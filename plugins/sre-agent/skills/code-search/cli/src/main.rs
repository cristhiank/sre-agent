use anyhow::{anyhow, Context, Result};
use clap::Parser;
use grep_regex::RegexMatcherBuilder;
use grep_searcher::{
    BinaryDetection, Searcher, SearcherBuilder, Sink, SinkContext, SinkContextKind, SinkMatch,
};
use ignore::{DirEntry, WalkBuilder, WalkState};
use regex::{Regex, RegexBuilder};
use serde::Serialize;
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering as AtomicOrdering};
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const DEFAULT_LIMIT: usize = 20;
const DEFAULT_CONTEXT: usize = 2;
const DEFAULT_MAX_PER_FILE: usize = 3;
const DEFAULT_MAX_FILES: usize = 100_000;
const WARN_FILES: usize = 20_000;
const SPARSE_RESULT_THRESHOLD: usize = 5;

const EXCLUDE_DIRS: &[&str] = &[
    ".git",
    "node_modules",
    "bin",
    "obj",
    "build",
    "target",
    "dist",
    "packages",
    ".vs",
    ".vscode",
    "TestResults",
    "__pycache__",
];

const SKIP_EXTS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "ico", "svg", "webp", "pdf", "zip", "gz", "tar", "7z",
    "rar", "dll", "exe", "so", "dylib", "lib", "pdb", "obj", "class", "csv", "tsv", "resjson",
    "map", "lock", "woff", "woff2", "ttf", "eot", "otf", "mp4", "mov", "mp3", "wav", "bin", "dat",
];

const CTAGS_EXTS: &[&str] = &[
    "c", "h", "cc", "cpp", "cxx", "hpp", "cs", "fs", "fsx", "vb", "java", "kt", "kts", "scala",
    "go", "rs", "py", "rb", "php", "swift", "js", "jsx", "ts", "tsx", "mjs", "cjs", "sh", "bash",
    "zsh", "ps1", "sql",
];

const SUBTOKEN_STOPWORDS: &[&str] = &[
    "get", "set", "id", "to", "of", "the", "is", "by", "for", "new", "async", "impl", "base",
    "data", "info", "name", "type", "value", "item", "list", "map",
];

#[derive(Parser, Debug)]
#[command(name = "srch", about = "Ranked live code and text search")]
struct SearchArgs {
    #[arg(value_name = "QUERY", required = true, num_args = 1..)]
    query: Vec<String>,

    #[arg(short = 'p', long = "path", value_name = "DIR")]
    paths: Vec<PathBuf>,

    #[arg(short = 'g', long = "glob", value_name = "GLOB")]
    globs: Vec<String>,

    #[arg(short = 'e', long = "ext", value_name = "CSV")]
    ext: Option<String>,

    #[arg(long = "regex")]
    regex: bool,

    #[arg(short = 'w', long = "word")]
    word: bool,

    #[arg(long = "no-expand")]
    no_expand: bool,

    #[arg(long = "expand")]
    expand: bool,

    #[arg(long = "and")]
    and_mode: bool,

    #[arg(short = 'i', long = "ignore-case")]
    ignore_case: bool,

    #[arg(short = 'n', long = "limit", default_value_t = DEFAULT_LIMIT)]
    limit: usize,

    #[arg(short = 'C', long = "context", default_value_t = DEFAULT_CONTEXT)]
    context: usize,

    #[arg(short = 'm', long = "max-per-file", default_value_t = DEFAULT_MAX_PER_FILE)]
    max_per_file: usize,

    #[arg(long = "max-files", default_value_t = DEFAULT_MAX_FILES)]
    max_files: usize,

    #[arg(long = "json")]
    json: bool,

    #[arg(long = "stats")]
    stats: bool,

    #[arg(long = "usages")]
    usages: bool,

    #[arg(skip)]
    quiet: bool,
}

#[derive(Parser, Debug)]
#[command(name = "srch def", about = "Find symbol definitions")]
struct DefArgs {
    symbol: String,

    #[arg(short = 'p', long = "path", value_name = "DIR")]
    paths: Vec<PathBuf>,

    #[arg(long = "db", value_name = "FILE")]
    db: Option<PathBuf>,

    #[arg(long = "json")]
    json: bool,
}

#[derive(Parser, Debug)]
#[command(name = "srch index", about = "Build a ctags symbol DB")]
struct IndexArgs {
    #[arg(short = 'p', long = "path", value_name = "DIR")]
    paths: Vec<PathBuf>,

    #[arg(long = "db", value_name = "FILE")]
    db: Option<PathBuf>,

    #[arg(long = "rebuild")]
    rebuild: bool,
}

#[derive(Clone, Debug)]
struct QueryPlan {
    regex_text: String,
    original_terms: Vec<String>,
    sub_terms: Vec<String>,
    case_insensitive: bool,
    search_re: Regex,
    original_re: Option<Regex>,
    term_res: Vec<(String, Regex)>,
    subtoken_res: Vec<(String, Regex)>,
    def_res: Vec<(String, Regex)>,
}

#[derive(Clone, Debug)]
struct RawMatch {
    line: u64,
    text: String,
    before: Vec<String>,
    after: Vec<String>,
}

#[derive(Clone, Debug)]
struct RankedMatch {
    line: u64,
    col: usize,
    kind: String,
    text: String,
    before: Vec<String>,
    after: Vec<String>,
    score: f64,
}

#[derive(Clone, Debug)]
struct FileResult {
    path: PathBuf,
    score: f64,
    mtime: u64,
    matches: Vec<RankedMatch>,
}

#[derive(Serialize)]
struct JsonOutput {
    elapsed_ms: u128,
    files_scanned: usize,
    files_matched: usize,
    usages_scanned: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    definitions: Vec<DefEntry>,
    results: Vec<JsonFileResult>,
}

#[derive(Serialize)]
struct JsonFileResult {
    path: String,
    score: i64,
    mtime: u64,
    matches: Vec<JsonMatch>,
}

#[derive(Serialize)]
struct JsonMatch {
    line: u64,
    col: usize,
    kind: String,
    text: String,
    before: Vec<String>,
    after: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
struct DefEntry {
    path: String,
    line: u64,
    kind: String,
    language: Option<String>,
    name: String,
}

#[derive(Serialize)]
struct DefOutput {
    note: Option<String>,
    results: Vec<DefEntry>,
}

struct CollectSink {
    matches: Vec<RawMatch>,
    pending_before: Vec<String>,
    cap: usize,
}

impl Sink for CollectSink {
    type Error = io::Error;

    fn matched(
        &mut self,
        _searcher: &Searcher,
        mat: &SinkMatch<'_>,
    ) -> std::result::Result<bool, io::Error> {
        let line = mat.line_number().unwrap_or(0);
        let text = String::from_utf8_lossy(mat.bytes())
            .trim_end_matches(['\r', '\n'])
            .to_string();
        let before = std::mem::take(&mut self.pending_before);
        self.matches.push(RawMatch {
            line,
            text,
            before,
            after: Vec::new(),
        });
        Ok(self.matches.len() < self.cap)
    }

    fn context(
        &mut self,
        _searcher: &Searcher,
        context: &SinkContext<'_>,
    ) -> std::result::Result<bool, io::Error> {
        let text = String::from_utf8_lossy(context.bytes())
            .trim_end_matches(['\r', '\n'])
            .to_string();
        match context.kind() {
            SinkContextKind::Before => self.pending_before.push(text),
            SinkContextKind::After => {
                if let Some(last) = self.matches.last_mut() {
                    last.after.push(text);
                }
            }
            SinkContextKind::Other => {}
        }
        Ok(true)
    }

    fn context_break(&mut self, _searcher: &Searcher) -> std::result::Result<bool, io::Error> {
        self.pending_before.clear();
        Ok(true)
    }
}

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "def" => {
                args.remove(1);
                let parsed = DefArgs::parse_from(args);
                return run_def(parsed);
            }
            "index" => {
                args.remove(1);
                let parsed = IndexArgs::parse_from(args);
                return run_index(parsed);
            }
            _ => {}
        }
    }
    let parsed = SearchArgs::parse_from(args);
    run_search(parsed).map(|_| ())
}

fn run_search(args: SearchArgs) -> Result<JsonOutput> {
    let roots = normalize_roots(&args.paths)?;
    let definitions = symbol_definitions_for_search(&args, &roots);
    let symbol_fast_path = should_fast_path(!definitions.is_empty(), args.usages, args.expand);
    if symbol_fast_path {
        let mut output = to_json_output(0, 0, 0, &definitions, &[]);
        output.usages_scanned = false;
        if !args.quiet {
            print_search_output(&args, &roots, &output)?;
        }
        return Ok(output);
    }

    let plan = build_query_plan(&args, false)?;
    let should_try_broaden =
        !args.regex && !args.expand && !args.no_expand && !plan.sub_terms.is_empty();
    let mut output = execute_search(&args, &roots, plan, &definitions)?;
    let mut broadened = false;

    if should_try_broaden && output.files_matched < SPARSE_RESULT_THRESHOLD {
        let broadened_plan = build_query_plan(&args, true)?;
        output = execute_search(&args, &roots, broadened_plan, &definitions)?;
        broadened = true;
    }

    if !args.quiet {
        if broadened {
            eprintln!("note: broadened (sparse results)");
        }
        print_search_output(&args, &roots, &output)?;
    }
    Ok(output)
}

fn should_fast_path(defs_present: bool, usages: bool, expand: bool) -> bool {
    defs_present && !usages && !expand
}

fn execute_search(
    args: &SearchArgs,
    roots: &[PathBuf],
    plan: QueryPlan,
    definitions: &[DefEntry],
) -> Result<JsonOutput> {
    let started = Instant::now();
    let include_exts = parse_exts(args.ext.as_deref());
    let glob_res = compile_globs(&args.globs)?;
    let matcher = RegexMatcherBuilder::new()
        .case_insensitive(plan.case_insensitive)
        .build(&plan.regex_text)
        .with_context(|| format!("invalid search regex: {}", plan.regex_text))?;

    let files_scanned = Arc::new(AtomicUsize::new(0));
    let exceeded_max_files = Arc::new(AtomicBool::new(false));
    let results = Arc::new(Mutex::new(Vec::<FileResult>::new()));
    let matcher = Arc::new(matcher);
    let plan = Arc::new(plan);
    let include_exts = Arc::new(include_exts);
    let glob_res = Arc::new(glob_res);
    let roots_arc = Arc::new(roots.to_vec());
    let context = args.context;
    let max_per_file = args.max_per_file;
    let and_mode = args.and_mode;
    let max_files = args.max_files;

    let mut builder = WalkBuilder::new(&roots[0]);
    for root in roots.iter().skip(1) {
        builder.add(root);
    }
    builder
        .hidden(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .parents(true);
    builder.threads(num_cpus::get().max(1));
    builder.filter_entry(is_included_entry);

    builder.build_parallel().run(|| {
        let matcher = Arc::clone(&matcher);
        let plan = Arc::clone(&plan);
        let files_scanned = Arc::clone(&files_scanned);
        let exceeded_max_files = Arc::clone(&exceeded_max_files);
        let results = Arc::clone(&results);
        let include_exts = Arc::clone(&include_exts);
        let glob_res = Arc::clone(&glob_res);
        let roots = Arc::clone(&roots_arc);
        Box::new(move |entry_result| {
            let entry = match entry_result {
                Ok(entry) => entry,
                Err(_) => return WalkState::Continue,
            };
            if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                return WalkState::Continue;
            }
            let path = entry.path();
            if should_skip_file(path, &include_exts, &glob_res) {
                return WalkState::Continue;
            }
            let scanned = files_scanned.fetch_add(1, AtomicOrdering::Relaxed) + 1;
            if scanned > max_files {
                exceeded_max_files.store(true, AtomicOrdering::Relaxed);
                return WalkState::Quit;
            }
            if let Ok(Some(result)) = search_one_file(
                path,
                &matcher,
                &plan,
                context,
                max_per_file,
                and_mode,
                &roots,
            ) {
                if let Ok(mut guard) = results.lock() {
                    guard.push(result);
                }
            }
            WalkState::Continue
        })
    });

    if exceeded_max_files.load(AtomicOrdering::Relaxed) {
        return Err(anyhow!(
            "max-files cap exceeded after {} files; use --path/--ext/--glob to scope tighter, increase --max-files, or run srch index plus srch def for symbol lookup",
            files_scanned.load(AtomicOrdering::Relaxed)
        ));
    }

    let elapsed_ms = started.elapsed().as_millis();
    let mut results = match Arc::try_unwrap(results) {
        Ok(mutex) => mutex.into_inner().unwrap_or_default(),
        Err(arc) => arc.lock().map(|guard| guard.clone()).unwrap_or_default(),
    };
    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.path.cmp(&b.path))
    });
    let files_matched = results.len();
    results.truncate(args.limit);

    Ok(to_json_output(
        elapsed_ms,
        files_scanned.load(AtomicOrdering::Relaxed),
        files_matched,
        definitions,
        &results,
    ))
}

fn print_search_output(args: &SearchArgs, roots: &[PathBuf], output: &JsonOutput) -> Result<()> {
    if args.json {
        println!("{}", serde_json::to_string_pretty(output)?);
    } else {
        if !output.definitions.is_empty() {
            print_definitions_block(&output.definitions, roots);
        }
        print_human_results(output, roots);
    }
    if !output.usages_scanned {
        eprintln!("note: usages not scanned (add --usages to scan)");
    }
    if output.files_scanned > WARN_FILES {
        eprintln!(
            "warning: scanned {} files; use --ext/--path/--glob for tighter scope, or srch index plus srch def for symbol lookup",
            output.files_scanned
        );
    }
    if args.stats {
        eprintln!(
            "stats: elapsed_ms={} files_scanned={} files_matched={}",
            output.elapsed_ms, output.files_scanned, output.files_matched
        );
    }
    Ok(())
}

fn symbol_definitions_for_search(args: &SearchArgs, roots: &[PathBuf]) -> Vec<DefEntry> {
    if args.regex || args.query.len() != 1 || !is_identifier_like(&args.query[0]) {
        return Vec::new();
    }
    let db = default_db_path(roots);
    let db = if db.exists() {
        db
    } else if let Some(db) = find_db_for_roots(roots) {
        db
    } else {
        return Vec::new();
    };
    let Some(meta_roots) = db_meta_roots(&db) else {
        return Vec::new();
    };
    if !db_covers_roots(&meta_roots, roots) {
        return Vec::new();
    }
    query_db(&db, &args.query[0])
        .unwrap_or_default()
        .into_iter()
        .filter(|entry| def_entry_under_roots(entry, roots))
        .collect()
}

fn normalize_roots(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let raw = if paths.is_empty() {
        vec![env::current_dir()?]
    } else {
        paths.to_vec()
    };
    raw.into_iter()
        .map(|p| {
            let canonical =
                fs::canonicalize(&p).with_context(|| format!("path not found: {}", p.display()))?;
            Ok(PathBuf::from(clean_path(&canonical)))
        })
        .collect()
}

fn build_query_plan(args: &SearchArgs, force_expand: bool) -> Result<QueryPlan> {
    let joined = args.query.join(" ");
    let case_insensitive = args.ignore_case || !joined.chars().any(|c| c.is_uppercase());
    let (original_terms, sub_terms, regex_text) = if args.regex {
        (
            vec![joined.clone()],
            Vec::new(),
            wrap_word(&joined, args.word, true),
        )
    } else {
        let originals = split_terms(&joined);
        if originals.is_empty() {
            return Err(anyhow!("query cannot be empty"));
        }
        let mut all = Vec::<String>::new();
        let mut seen = HashSet::<String>::new();
        let mut sub_terms = Vec::<String>::new();
        let mut sub_seen = HashSet::<String>::new();
        for term in &originals {
            push_unique(&mut all, &mut seen, term.clone(), case_insensitive);
            if is_identifier_like(term) {
                for sub in split_identifier(term)
                    .into_iter()
                    .filter(|s| keep_subtoken(s))
                {
                    push_unique(&mut sub_terms, &mut sub_seen, sub, case_insensitive);
                }
            }
        }
        if (force_expand || args.expand) && !args.no_expand {
            for sub in &sub_terms {
                push_unique(&mut all, &mut seen, sub.clone(), case_insensitive);
            }
        }
        all.sort_by_key(|s| std::cmp::Reverse(s.len()));
        let escaped = all
            .iter()
            .map(|s| regex::escape(s))
            .collect::<Vec<_>>()
            .join("|");
        let body = format!("(?:{})", escaped);
        (originals, sub_terms, wrap_word(&body, args.word, false))
    };

    let search_re = RegexBuilder::new(&regex_text)
        .case_insensitive(case_insensitive)
        .build()
        .with_context(|| format!("invalid search regex: {}", regex_text))?;
    let original_re = if args.regex {
        None
    } else {
        Some(build_union_re(
            &original_terms,
            case_insensitive,
            args.word,
        )?)
    };
    let mut term_res = Vec::new();
    let mut subtoken_res = Vec::new();
    let mut def_res = Vec::new();
    if !args.regex {
        for term in &original_terms {
            term_res.push((
                term.clone(),
                build_term_re(term, case_insensitive, args.word)?,
            ));
            def_res.push((term.clone(), build_def_re(term, case_insensitive)?));
        }
        for term in &sub_terms {
            subtoken_res.push((
                term.clone(),
                build_term_re(term, case_insensitive, args.word)?,
            ));
        }
    }
    Ok(QueryPlan {
        regex_text,
        original_terms,
        sub_terms,
        case_insensitive,
        search_re,
        original_re,
        term_res,
        subtoken_res,
        def_res,
    })
}

fn split_terms(query: &str) -> Vec<String> {
    query
        .split(|c: char| c.is_whitespace() || c == '|')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn is_identifier_like(term: &str) -> bool {
    !term.starts_with('-')
        && term.chars().any(|c| c.is_ascii_alphanumeric())
        && term
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '-' || c == ':')
}

fn split_identifier(term: &str) -> Vec<String> {
    let mut out = Vec::new();
    for part in term.split(|c: char| c == '_' || c == '.' || c == '-' || c == ':') {
        if part.is_empty() {
            continue;
        }
        let chars: Vec<char> = part.chars().collect();
        let mut start = 0usize;
        for i in 1..chars.len() {
            let prev = chars[i - 1];
            let cur = chars[i];
            let next = chars.get(i + 1).copied();
            let boundary = (prev.is_ascii_lowercase() && cur.is_ascii_uppercase())
                || (prev.is_ascii_alphabetic() && cur.is_ascii_digit())
                || (prev.is_ascii_digit() && cur.is_ascii_alphabetic())
                || (prev.is_ascii_uppercase()
                    && cur.is_ascii_uppercase()
                    && next.map(|n| n.is_ascii_lowercase()).unwrap_or(false));
            if boundary {
                let token: String = chars[start..i].iter().collect();
                if token.len() > 1 {
                    out.push(token);
                }
                start = i;
            }
        }
        let token: String = chars[start..].iter().collect();
        if token.len() > 1 {
            out.push(token);
        }
    }
    out
}

fn keep_subtoken(token: &str) -> bool {
    let lower = token.to_ascii_lowercase();
    lower.len() >= 4 && !SUBTOKEN_STOPWORDS.contains(&lower.as_str())
}

fn push_unique(out: &mut Vec<String>, seen: &mut HashSet<String>, term: String, ci: bool) {
    let k = key(&term, ci);
    if seen.insert(k) {
        out.push(term);
    }
}

fn key(s: &str, ci: bool) -> String {
    if ci {
        s.to_ascii_lowercase()
    } else {
        s.to_string()
    }
}

fn wrap_word(body: &str, word: bool, raw: bool) -> String {
    if !word {
        return body.to_string();
    }
    if raw {
        format!(r"\b(?:{})\b", body)
    } else {
        format!(r"\b{}\b", body)
    }
}

fn build_union_re(terms: &[String], ci: bool, word: bool) -> Result<Regex> {
    let escaped = terms
        .iter()
        .map(|s| regex::escape(s))
        .collect::<Vec<_>>()
        .join("|");
    let pat = wrap_word(&format!("(?:{})", escaped), word, false);
    RegexBuilder::new(&pat)
        .case_insensitive(ci)
        .build()
        .context("building original term regex")
}

fn build_term_re(term: &str, ci: bool, word: bool) -> Result<Regex> {
    RegexBuilder::new(&wrap_word(&regex::escape(term), word, true))
        .case_insensitive(ci)
        .build()
        .context("building term regex")
}

fn build_def_re(term: &str, ci: bool) -> Result<Regex> {
    let t = regex::escape(term);
    let pat = format!(
        r"(?x)(\b(class|struct|interface|enum|trait|record|fn|def|func|function|public|private|protected|internal|static|val|let|const|var|type|module|namespace)\b[^\n]*\b{}\b|\b{}\s*[:=]|^\s*(?:[\w<>\[\],.?]+\s+)*{}\s*\()",
        t, t, t
    );
    RegexBuilder::new(&pat)
        .case_insensitive(ci)
        .build()
        .context("building definition regex")
}

fn parse_exts(ext: Option<&str>) -> HashSet<String> {
    ext.unwrap_or("")
        .split(',')
        .map(|s| s.trim().trim_start_matches('.').to_ascii_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

fn compile_globs(globs: &[String]) -> Result<Vec<Regex>> {
    globs.iter().map(|g| glob_to_regex(g)).collect()
}

fn glob_to_regex(glob: &str) -> Result<Regex> {
    let mut pat = String::from("(?i)(?:^|.*/)");
    let normalized = glob.replace('\\', "/");
    let mut chars = normalized.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '*' => {
                if chars.peek() == Some(&'*') {
                    chars.next();
                    if chars.peek() == Some(&'/') {
                        chars.next();
                        pat.push_str("(?:.*/)?");
                    } else {
                        pat.push_str(".*");
                    }
                } else {
                    pat.push_str("[^/]*");
                }
            }
            '?' => pat.push('.'),
            '.' => pat.push_str(r"\."),
            '/' => pat.push('/'),
            c => pat.push_str(&regex::escape(&c.to_string())),
        }
    }
    pat.push('$');
    Regex::new(&pat).with_context(|| format!("invalid glob: {glob}"))
}

fn is_included_entry(entry: &DirEntry) -> bool {
    if entry.depth() == 0 {
        return true;
    }
    if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
        let name = entry.file_name().to_string_lossy();
        !EXCLUDE_DIRS.iter().any(|d| name.eq_ignore_ascii_case(d))
    } else {
        true
    }
}

fn should_skip_file(path: &Path, include_exts: &HashSet<String>, glob_res: &[Regex]) -> bool {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if SKIP_EXTS.contains(&ext.as_str()) {
        return true;
    }
    if !include_exts.is_empty() && !include_exts.contains(&ext) {
        return true;
    }
    if !glob_res.is_empty() {
        let normalized = path.to_string_lossy().replace('\\', "/");
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if !glob_res
            .iter()
            .any(|re| re.is_match(&normalized) || re.is_match(file_name))
        {
            return true;
        }
    }
    false
}

fn search_one_file(
    path: &Path,
    matcher: &grep_regex::RegexMatcher,
    plan: &QueryPlan,
    context: usize,
    max_per_file: usize,
    and_mode: bool,
    roots: &[PathBuf],
) -> Result<Option<FileResult>> {
    let cap = if and_mode {
        usize::MAX
    } else {
        (max_per_file.saturating_mul(20)).max(80)
    };
    let mut searcher = SearcherBuilder::new()
        .line_number(true)
        .before_context(context)
        .after_context(context)
        .binary_detection(BinaryDetection::quit(b'\x00'))
        .build();
    let mut sink = CollectSink {
        matches: Vec::new(),
        pending_before: Vec::new(),
        cap,
    };
    if searcher.search_path(matcher, path, &mut sink).is_err() || sink.matches.is_empty() {
        return Ok(None);
    }

    let mut distinct_terms = HashSet::new();
    let mut ranked = Vec::new();
    let path_boost = path_original_boost(path, plan);
    let is_test = is_test_path(path);

    for raw in sink.matches {
        let text = raw.text.trim_end().to_string();
        for (term, re) in &plan.term_res {
            if re.is_match(&text) || path_contains(path, term, plan.case_insensitive) {
                distinct_terms.insert(key(term, true));
            }
        }
        let original = plan
            .original_re
            .as_ref()
            .map(|re| re.is_match(&text))
            .unwrap_or(true);
        let subtoken_hits = plan
            .subtoken_res
            .iter()
            .filter(|(_, re)| re.is_match(&text))
            .count();
        let def = !plan.def_res.is_empty() && plan.def_res.iter().any(|(_, re)| re.is_match(&text));
        let comment = looks_like_comment(&text);
        let col = first_col(&text, plan).unwrap_or(1);
        let mut score = 10.0;
        if original {
            score += 18.0;
        } else {
            score += 5.0;
        }
        score += (subtoken_hits as f64 * 1.5).min(6.0);
        if def {
            score += 45.0;
        }
        if comment {
            score -= 8.0;
        }
        if is_test {
            score -= 5.0;
        }
        score += path_boost.min(18.0);
        ranked.push(RankedMatch {
            line: raw.line,
            col,
            kind: if comment {
                "comment"
            } else if def {
                "def"
            } else {
                "usage"
            }
            .to_string(),
            text,
            before: raw.before,
            after: raw.after,
            score,
        });
    }

    if and_mode && !plan.original_terms.is_empty() {
        let required = plan
            .original_terms
            .iter()
            .map(|t| key(t, true))
            .collect::<HashSet<_>>();
        if !required.is_subset(&distinct_terms) {
            return Ok(None);
        }
    }

    ranked.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.line.cmp(&b.line))
    });
    let mut seen_lines = HashSet::new();
    ranked.retain(|m| seen_lines.insert(m.text.trim().to_string()));
    ranked.truncate(max_per_file);
    let density = distinct_terms.len() as f64 * 8.0;
    let best = ranked.first().map(|m| m.score).unwrap_or(0.0);
    let mtime = file_mtime(path).unwrap_or(0);
    let root_bonus = roots
        .iter()
        .position(|r| path.starts_with(r))
        .map(|i| (roots.len() - i) as f64 * 0.01)
        .unwrap_or(0.0);
    Ok(Some(FileResult {
        path: path.to_path_buf(),
        score: best + density + root_bonus,
        mtime,
        matches: ranked,
    }))
}

fn first_col(text: &str, plan: &QueryPlan) -> Option<usize> {
    if let Some(re) = &plan.original_re {
        if let Some(m) = re.find(text) {
            return Some(m.start() + 1);
        }
    }
    plan.search_re.find(text).map(|m| m.start() + 1)
}

fn path_original_boost(path: &Path, plan: &QueryPlan) -> f64 {
    plan.original_terms
        .iter()
        .filter(|term| path_contains(path, term, plan.case_insensitive))
        .count() as f64
        * 10.0
}

fn path_contains(path: &Path, term: &str, ci: bool) -> bool {
    let p = path.to_string_lossy();
    if ci {
        p.to_ascii_lowercase().contains(&term.to_ascii_lowercase())
    } else {
        p.contains(term)
    }
}

fn is_test_path(path: &Path) -> bool {
    let p = path.to_string_lossy().to_ascii_lowercase();
    p.contains("test")
        || p.contains("spec")
        || p.contains("__tests__")
        || p.contains(".test.")
        || p.contains(".spec.")
}

fn looks_like_comment(text: &str) -> bool {
    let t = text.trim_start();
    t.starts_with("//")
        || t.starts_with('#')
        || t.starts_with(';')
        || t.starts_with("--")
        || t.starts_with("/*")
        || t.starts_with('*')
}

fn file_mtime(path: &Path) -> Result<u64> {
    Ok(fs::metadata(path)?
        .modified()?
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs())
}

fn to_json_output(
    elapsed_ms: u128,
    files_scanned: usize,
    files_matched: usize,
    definitions: &[DefEntry],
    results: &[FileResult],
) -> JsonOutput {
    JsonOutput {
        elapsed_ms,
        files_scanned,
        files_matched,
        usages_scanned: true,
        definitions: definitions.to_vec(),
        results: results
            .iter()
            .map(|r| JsonFileResult {
                path: clean_path(&r.path),
                score: r.score.round() as i64,
                mtime: r.mtime,
                matches: r
                    .matches
                    .iter()
                    .map(|m| JsonMatch {
                        line: m.line,
                        col: m.col,
                        kind: m.kind.clone(),
                        text: m.text.clone(),
                        before: m.before.clone(),
                        after: m.after.clone(),
                    })
                    .collect(),
            })
            .collect(),
    }
}

fn print_definitions_block(definitions: &[DefEntry], roots: &[PathBuf]) {
    println!("definitions:");
    for def in definitions.iter().take(DEFAULT_LIMIT) {
        let display = display_path(&PathBuf::from(&def.path), roots);
        println!("  {}:{}: {} {}", display, def.line, def.kind, def.name);
    }
}

fn print_human_results(output: &JsonOutput, roots: &[PathBuf]) {
    for (idx, file) in output.results.iter().enumerate() {
        let rel = display_path(&PathBuf::from(&file.path), roots);
        println!(
            "#{} {}  (score={}, mtime={})",
            idx + 1,
            rel,
            file.score,
            file.mtime
        );
        for m in &file.matches {
            for before in &m.before {
                println!("    | {}", before.trim_end());
            }
            println!("  {}:{}: {}", m.line, m.col, m.text.trim_end());
            for after in &m.after {
                println!("    | {}", after.trim_end());
            }
        }
    }
}

fn display_path(path: &Path, roots: &[PathBuf]) -> String {
    for root in roots {
        if let Ok(stripped) = path.strip_prefix(root) {
            return stripped.to_string_lossy().to_string();
        }
    }
    clean_path(path)
}

fn clean_path(path: &Path) -> String {
    path.to_string_lossy()
        .trim_start_matches(r"\\?\")
        .to_string()
}

fn run_index(args: IndexArgs) -> Result<()> {
    let started = Instant::now();
    let roots = normalize_roots(&args.paths)?;
    let db = args.db.unwrap_or_else(|| default_db_path(&roots));
    if db.exists() && !args.rebuild {
        println!(
            "DB already exists: {} (use --rebuild to overwrite)",
            db.display()
        );
        return Ok(());
    }
    if let Some(parent) = db.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp_db = tmp_path_for(&db);
    let file_list = tmp_path_for(&db.with_extension("files.txt"));
    let meta_path = db_meta_path(&db);
    let tmp_meta = tmp_path_for(&meta_path);
    let _ = fs::remove_file(&tmp_db);
    let _ = fs::remove_file(&file_list);
    let _ = fs::remove_file(&tmp_meta);
    let indexed_files = write_ctags_file_list(&roots, &file_list)?;
    let mut cmd = Command::new("ctags");
    cmd.arg("--sort=yes")
        .arg("--fields=+nKzS")
        .arg("-f")
        .arg(&tmp_db)
        .arg("-L")
        .arg(&file_list);
    let status = cmd
        .stderr(Stdio::null())
        .status()
        .context("failed to run ctags; ensure Universal Ctags is on PATH")?;
    let count = fs::read_to_string(&tmp_db)
        .map(|s| {
            s.lines()
                .filter(|line| !line.starts_with("!_") && !line.trim().is_empty())
                .count()
        })
        .unwrap_or(0);
    let _ = fs::remove_file(&file_list);
    if !status.success() || count == 0 {
        let _ = fs::remove_file(&tmp_db);
        let _ = fs::remove_file(&tmp_meta);
        if !status.success() {
            return Err(anyhow!("ctags failed with status {status}"));
        }
        return Err(anyhow!("ctags produced no symbols"));
    }
    let elapsed_ms = started.elapsed().as_millis();
    let meta = serde_json::json!({
        "timestamp_unix": SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
        "roots": roots.iter().map(|p| p.to_string_lossy().to_string()).collect::<Vec<_>>(),
        "db": db.to_string_lossy().to_string(),
    });
    fs::write(&tmp_meta, serde_json::to_string_pretty(&meta)?)?;
    // Publish fail-closed: drop any stale sidecar BEFORE swapping the DB so no crash window can
    // pair a freshly-narrowed DB with an older meta that still lists a dropped root (which would
    // let a spanning query fast-path against a DB that no longer covers it). Absent meta => the
    // DB is simply not selected for the fast path (live scan), which is the safe degradation.
    let _ = fs::remove_file(&meta_path);
    fs::rename(&tmp_db, &db).with_context(|| format!("publishing ctags DB to {}", db.display()))?;
    fs::rename(&tmp_meta, &meta_path)
        .with_context(|| format!("publishing ctags metadata to {}", meta_path.display()))?;
    println!(
        "symbols={} files_indexed={} elapsed_ms={} db={}",
        count,
        indexed_files,
        elapsed_ms,
        db.display()
    );
    Ok(())
}

fn write_ctags_file_list(roots: &[PathBuf], file_list: &Path) -> Result<usize> {
    let mut builder = WalkBuilder::new(&roots[0]);
    for root in roots.iter().skip(1) {
        builder.add(root);
    }
    builder
        .hidden(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .parents(true);
    builder.filter_entry(is_included_entry);
    let include_exts = HashSet::new();
    let glob_res: Vec<Regex> = Vec::new();
    let mut paths = Vec::new();
    for entry in builder.build().filter_map(|e| e.ok()) {
        let ext = entry
            .path()
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)
            && CTAGS_EXTS.contains(&ext.as_str())
            && !should_skip_file(entry.path(), &include_exts, &glob_res)
        {
            paths.push(clean_path(entry.path()));
        }
    }
    paths.sort();
    fs::write(file_list, paths.join("\n"))?;
    Ok(paths.len())
}

fn run_def(args: DefArgs) -> Result<()> {
    let mut note = None;
    let db = if let Some(db) = args.db.clone() {
        Some(db)
    } else if !args.paths.is_empty() {
        let roots = normalize_roots(&args.paths)?;
        let exact = default_db_path(&roots);
        if exact.exists() {
            Some(exact)
        } else {
            find_db_for_roots(&roots)
        }
    } else {
        find_default_db()
    };
    let mut results = if let Some(db_path) = db {
        if db_path.exists() {
            query_db(&db_path, &args.symbol)?
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };
    if results.is_empty() {
        note = Some("ctags DB was not used; falling back to live definition heuristic".to_string());
        results = fallback_def(&args.symbol, &args.paths)?;
    }
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&DefOutput { note, results })?
        );
    } else {
        if let Some(note) = note {
            println!("note: {}", note);
        }
        for r in results.iter().take(DEFAULT_LIMIT) {
            println!(
                "{}:{}: {} {} ({})",
                r.path,
                r.line,
                r.kind,
                r.name,
                r.language.clone().unwrap_or_default()
            );
        }
    }
    Ok(())
}

fn default_db_path(roots: &[PathBuf]) -> PathBuf {
    let root = roots
        .first()
        .cloned()
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    cli_dir()
        .join(".srch")
        .join(format!("{}.tags", sanitize_root(&root)))
}

fn cli_dir() -> PathBuf {
    if let Ok(exe) = env::current_exe() {
        for ancestor in exe.ancestors().skip(1).take(6) {
            if ancestor.join("Cargo.toml").exists() {
                return ancestor.to_path_buf();
            }
        }
        if let Some(parent) = exe.parent() {
            if parent
                .file_name()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("bin"))
                .unwrap_or(false)
            {
                if let Some(cli) = parent.parent() {
                    return cli.to_path_buf();
                }
            }
            return parent.to_path_buf();
        }
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn sanitize_root(root: &Path) -> String {
    let s = root.to_string_lossy();
    let mut out = String::new();
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else {
            out.push('_');
        }
    }
    out.trim_matches('_').to_string()
}

fn find_default_db() -> Option<PathBuf> {
    let dir = cli_dir().join(".srch");
    let mut candidates = fs::read_dir(dir)
        .ok()?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("tags"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|p| fs::metadata(p).and_then(|m| m.modified()).ok());
    candidates.pop()
}

fn find_db_for_roots(roots: &[PathBuf]) -> Option<PathBuf> {
    let dir = cli_dir().join(".srch");
    let mut candidates = fs::read_dir(dir)
        .ok()?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("tags"))
                .unwrap_or(false)
        })
        .filter(|p| db_meta_matches_roots(p, roots))
        .collect::<Vec<_>>();
    candidates.sort_by_key(|p| fs::metadata(p).and_then(|m| m.modified()).ok());
    candidates.pop()
}

fn db_meta_matches_roots(db: &Path, roots: &[PathBuf]) -> bool {
    let Some(meta_roots) = db_meta_roots(db) else {
        return false;
    };
    meta_roots.iter().any(|meta| {
        roots
            .iter()
            .any(|root| meta.starts_with(root) || root.starts_with(meta))
    })
}

fn db_meta_path(db: &Path) -> PathBuf {
    db.with_extension("tags.meta.json")
}

fn tmp_path_for(path: &Path) -> PathBuf {
    PathBuf::from(format!("{}.tmp", path.to_string_lossy()))
}

fn db_meta_roots(db: &Path) -> Option<Vec<PathBuf>> {
    let text = fs::read_to_string(db_meta_path(db)).ok()?;
    let value = serde_json::from_str::<Value>(&text).ok()?;
    let meta_roots = value.get("roots")?.as_array()?;
    Some(
        meta_roots
            .iter()
            .filter_map(Value::as_str)
            .map(|meta_root| PathBuf::from(normalize_output_path(meta_root)))
            .collect(),
    )
}

fn db_covers_roots(meta_roots: &[PathBuf], search_roots: &[PathBuf]) -> bool {
    !meta_roots.is_empty()
        && search_roots
            .iter()
            .all(|root| meta_roots.iter().any(|meta| root.starts_with(meta)))
}

fn def_entry_under_roots(entry: &DefEntry, roots: &[PathBuf]) -> bool {
    let path = PathBuf::from(normalize_output_path(&entry.path));
    roots.iter().any(|root| path.starts_with(root))
}

fn query_db(db: &Path, symbol: &str) -> Result<Vec<DefEntry>> {
    if symbol.trim().is_empty() || symbol.starts_with('-') {
        return Ok(Vec::new());
    }
    let mut entries = run_readtags(db, symbol, &[])?;
    if entries.is_empty() {
        entries = run_readtags(db, symbol, &["-i"])?;
    }
    if entries.is_empty() {
        entries = run_readtags(db, symbol, &["-i", "-p"])?;
    }
    entries.sort_by(|a, b| {
        rank_name(&a.name, symbol)
            .cmp(&rank_name(&b.name, symbol))
            .then_with(|| a.path.cmp(&b.path))
            .then_with(|| a.line.cmp(&b.line))
    });
    entries.truncate(DEFAULT_LIMIT);
    Ok(entries)
}

fn run_readtags(db: &Path, symbol: &str, mode_args: &[&str]) -> Result<Vec<DefEntry>> {
    let mut cmd = Command::new("readtags");
    cmd.arg("-t").arg(db).arg("-e").arg("-n");
    for arg in mode_args {
        cmd.arg(arg);
    }
    cmd.arg("-").arg(symbol);
    let output = match cmd.output() {
        Ok(output) => output,
        Err(_) => return Ok(Vec::new()),
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().filter_map(parse_readtags_line).collect())
}

fn parse_readtags_line(line: &str) -> Option<DefEntry> {
    let parts = line.split('\t').collect::<Vec<_>>();
    if parts.len() < 3 {
        return None;
    }
    let name = parts[0].to_string();
    let path = normalize_output_path(parts[1]);
    let mut kind = String::new();
    let mut line_no = 0u64;
    let mut language = None;
    for field in parts.iter().skip(3) {
        if let Some(value) = field.strip_prefix("kind:") {
            kind = value.to_string();
        } else if let Some(value) = field.strip_prefix("line:") {
            line_no = value.parse::<u64>().unwrap_or(0);
        } else if let Some(value) = field.strip_prefix("language:") {
            language = Some(value.to_string());
        }
    }
    if kind.is_empty() && parts.len() > 3 {
        kind = parts[3].trim_start_matches("kind:").to_string();
    }
    Some(DefEntry {
        path,
        line: line_no,
        kind,
        language,
        name,
    })
}

fn normalize_output_path(path: &str) -> String {
    if cfg!(windows) {
        path.replace('/', "\\")
    } else {
        path.to_string()
    }
}

fn rank_name(name: &str, symbol: &str) -> u8 {
    if name == symbol {
        0
    } else if name.eq_ignore_ascii_case(symbol) {
        1
    } else {
        2
    }
}

fn fallback_def(symbol: &str, paths: &[PathBuf]) -> Result<Vec<DefEntry>> {
    let args = SearchArgs {
        query: vec![symbol.to_string()],
        paths: paths.to_vec(),
        globs: Vec::new(),
        ext: None,
        regex: false,
        word: false,
        no_expand: true,
        expand: false,
        and_mode: false,
        ignore_case: false,
        limit: DEFAULT_LIMIT,
        context: 0,
        max_per_file: 1,
        max_files: DEFAULT_MAX_FILES,
        json: true,
        stats: false,
        usages: false,
        quiet: true,
    };
    let output = run_search(args)?;
    let mut entries = Vec::new();
    let mut seen = HashSet::new();
    for def in output.definitions {
        if seen.insert((def.path.clone(), def.line)) {
            entries.push(def);
        }
    }
    for file in output.results {
        for m in file.matches {
            if m.kind == "def" && seen.insert((file.path.clone(), m.line)) {
                entries.push(DefEntry {
                    path: file.path.clone(),
                    line: m.line,
                    kind: m.kind,
                    language: None,
                    name: symbol.to_string(),
                });
            }
        }
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_fast_path_truth_table() {
        for defs_present in [false, true] {
            for usages in [false, true] {
                for expand in [false, true] {
                    let expected = defs_present && !usages && !expand;
                    assert_eq!(
                        should_fast_path(defs_present, usages, expand),
                        expected,
                        "defs_present={defs_present} usages={usages} expand={expand}"
                    );
                }
            }
        }
        assert!(should_fast_path(true, false, false));
        assert!(!should_fast_path(true, false, true));
    }

    #[test]
    fn identifier_like_rejects_option_shapes() {
        assert!(is_identifier_like("GetX"));
        assert!(is_identifier_like("snake_case"));
        assert!(!is_identifier_like("-i"));
        assert!(!is_identifier_like(""));
        assert!(!is_identifier_like("a|b"));
        assert!(!is_identifier_like("-leading"));
    }

    #[test]
    fn split_identifier_handles_common_boundaries() {
        let parts = split_identifier("GetManagerSummaryStatistics")
            .into_iter()
            .map(|s| s.to_ascii_lowercase())
            .collect::<Vec<_>>();
        for expected in ["get", "manager", "summary", "statistics"] {
            assert!(parts.contains(&expected.to_string()));
        }
        assert_eq!(
            split_identifier("license-sync"),
            vec!["license".to_string(), "sync".to_string()]
        );
        assert_eq!(
            split_identifier("manager.settings"),
            vec!["manager".to_string(), "settings".to_string()]
        );
        assert_eq!(
            split_identifier("Version2Endpoint"),
            vec!["Version".to_string(), "Endpoint".to_string()]
        );
    }

    #[test]
    fn keep_subtoken_filters_noise() {
        assert!(!keep_subtoken("get"));
        assert!(!keep_subtoken("id"));
        assert!(!keep_subtoken("data"));
        assert!(keep_subtoken("manager"));
    }

    #[test]
    fn parse_readtags_line_parses_fields() {
        let line =
            "GetX\tC:\\repo\\src\\file.cs\t/^void GetX()$/;\"\tkind:method\tline:42\tlanguage:C#";
        let entry = parse_readtags_line(line).expect("entry");
        assert_eq!(entry.name, "GetX");
        assert_eq!(entry.path, "C:\\repo\\src\\file.cs");
        assert_eq!(entry.line, 42);
        assert_eq!(entry.kind, "method");
        assert_eq!(entry.language.as_deref(), Some("C#"));
        assert!(parse_readtags_line("too\tshort").is_none());
    }

    #[test]
    fn rank_name_orders_exact_case_insensitive_other() {
        assert!(rank_name("Symbol", "Symbol") < rank_name("symbol", "Symbol"));
        assert!(rank_name("symbol", "Symbol") < rank_name("Other", "Symbol"));
    }

    #[test]
    fn db_coverage_requires_index_superset() {
        let indexed = vec![PathBuf::from(r"C:\repo\services")];
        assert!(db_covers_roots(
            &indexed,
            &[PathBuf::from(r"C:\repo\services\insights")]
        ));
        let subset = vec![PathBuf::from(r"C:\repo\services\insights")];
        assert!(!db_covers_roots(
            &subset,
            &[PathBuf::from(r"C:\repo\services")]
        ));
        let disjoint = vec![PathBuf::from(r"C:\repo\other")];
        assert!(!db_covers_roots(
            &disjoint,
            &[PathBuf::from(r"C:\repo\services")]
        ));
    }

    #[test]
    fn glob_to_regex_matches_expected_extensions() {
        let re = glob_to_regex("*.cs").expect("glob");
        assert!(re.is_match("a.cs"));
        assert!(!re.is_match("a.ts"));
    }
}
