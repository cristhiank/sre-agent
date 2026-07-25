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
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering as AtomicOrdering};
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

/// Extensions whose `#`/`--`/`//` line prefixes are prose markup, not comments, and whose
/// definition anchors are headings and leading `term:` lines rather than C-family declarations.
const PROSE_EXTS: &[&str] = &[
    "md", "markdown", "mdx", "rst", "adoc", "asciidoc", "txt", "text", "org", "wiki", "tex",
];

/// Extensions where a definition is a key in key position at the start of a line.
const STRUCTURED_EXTS: &[&str] = &[
    "json", "jsonc", "yaml", "yml", "toml", "ini", "cfg", "conf", "config", "properties", "env",
    "toon",
];

const SUBTOKEN_STOPWORDS: &[&str] = &[
    "get", "set", "id", "to", "of", "the", "is", "by", "for", "new", "async", "impl", "base",
    "data", "info", "name", "type", "value", "item", "list", "map",
];

// ---------------------------------------------------------------------------
// BM25F ranking parameters.
//
// File score = BM25F over three line zones (head / body / comment) plus the path
// as its own zone, then two bounded structural bonuses. IDF and average document
// length are derived from the same walk that collects the matches, so ranking
// stays a single pass with no prose index.
// ---------------------------------------------------------------------------

/// Term-frequency saturation point. Higher = repetition keeps paying for longer.
const BM25_K1: f64 = 1.2;
/// Length-normalization strength for ordinary body text.
const BM25_B_BODY: f64 = 0.75;
/// Anchors are position-bound, so length matters less there.
const BM25_B_HEAD: f64 = 0.5;
/// Zone weights folded into the normalized term frequency before saturation.
const W_HEAD: f64 = 2.4;
const W_BODY: f64 = 1.0;
const W_COMMENT: f64 = 0.55;
const W_PATH: f64 = 3.0;
/// Expanded subtokens are weaker evidence than the terms the caller typed.
const W_SUBTOKEN: f64 = 0.3;
/// Weight of a term exactly as the caller typed it.
const W_TERM_ORIGINAL: f64 = 1.0;
/// Weight of an identifier recovered from inside a punctuated term. Nearly first-class:
/// in an alert title the discriminative identifier is usually glued to scaffolding.
const W_TERM_SEGMENT: f64 = 0.6;

/// Shortest cumulative prefix of a compound identifier worth searching for.
const MIN_PREFIX_GRAM_LEN: usize = 8;
/// Cap on prefixes per compound identifier, so a deeply compounded name cannot flood the
/// union regex.
const MAX_PREFIX_GRAMS: usize = 5;
/// Bounded bonus for covering a large share of the query's total IDF mass.
const COVERAGE_BONUS: f64 = 2.0;
/// Weight on the mean IDF of terms that appear in a definition-shaped line.
const ANCHOR_BONUS: f64 = 0.75;
/// Multiplicative, scale-free demotion for test/spec paths.
const TEST_PATH_FACTOR: f64 = 0.85;
/// Per-line occurrence cap so one pathological line cannot dominate a file's TF.
const MAX_TF_PER_LINE: usize = 4;
/// Line length past which the *display* ranking of a line starts to decay.
const LINE_DAMP_START: f64 = 400.0;
/// Zone multipliers used to order the lines shown for a file.
const LINE_ZONE_HEAD: f64 = 1.5;
const LINE_ZONE_BODY: f64 = 1.0;
const LINE_ZONE_COMMENT: f64 = 0.8;
/// Default clip width for emitted match/context lines. 0 disables clipping.
const DEFAULT_MAX_LINE_WIDTH: usize = 400;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FileClass {
    /// C-family/scripting source: declaration keywords, `//`/`#`/`--` comments.
    Code,
    /// Key/value data: a definition is `key:`/`key=` in key position.
    Structured,
    /// Prose/markup: `#` is a heading, a definition is a heading or a leading `term:`.
    Prose,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Zone {
    Head,
    Body,
    Comment,
}

impl Zone {
    fn index(self) -> usize {
        match self {
            Zone::Head => 0,
            Zone::Body => 1,
            Zone::Comment => 2,
        }
    }

    fn kind(self) -> &'static str {
        match self {
            Zone::Head => "def",
            Zone::Body => "usage",
            Zone::Comment => "comment",
        }
    }

    fn line_multiplier(self) -> f64 {
        match self {
            Zone::Head => LINE_ZONE_HEAD,
            Zone::Body => LINE_ZONE_BODY,
            Zone::Comment => LINE_ZONE_COMMENT,
        }
    }
}

fn classify_file(path: &Path) -> FileClass {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if PROSE_EXTS.contains(&ext.as_str()) {
        FileClass::Prose
    } else if STRUCTURED_EXTS.contains(&ext.as_str()) {
        FileClass::Structured
    } else {
        FileClass::Code
    }
}

#[derive(Parser, Debug)]
#[command(name = "srch", about = "Ranked live code and text search")]
struct SearchArgs {
    #[arg(value_name = "QUERY", required = true, num_args = 1..)]
    query: Vec<String>,

    #[arg(short = 'p', long = "path", value_name = "DIR")]
    paths: Vec<PathBuf>,

    #[arg(short = 'g', long = "glob", value_name = "GLOB")]
    globs: Vec<String>,

    #[arg(short = 'x', long = "exclude", value_name = "GLOB")]
    exclude: Vec<String>,

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

    #[arg(long = "max-line-width", default_value_t = DEFAULT_MAX_LINE_WIDTH)]
    max_line_width: usize,

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
    sub_terms: Vec<String>,
    case_insensitive: bool,
    search_re: Regex,
    original_re: Option<Regex>,
    /// Terms the caller typed plus the identifiers recovered from inside them.
    term_res: Vec<(String, Regex)>,
    /// Per entry in `term_res`: 1.0 for a typed term, less for a recovered segment.
    term_weights: Vec<f64>,
    subtoken_res: Vec<(String, Regex)>,
    /// Whether tier-3 fragments are part of this plan's search regex and scoring.
    fragments_used: bool,
    /// Definition-anchor regex per file class: code, structured, prose.
    anchor_res: [Option<Regex>; 3],
}

impl QueryPlan {
    fn anchor_re(&self, class: FileClass) -> Option<&Regex> {
        let idx = match class {
            FileClass::Code => 0,
            FileClass::Structured => 1,
            FileClass::Prose => 2,
        };
        self.anchor_res[idx].as_ref()
    }
}

/// One retained match line plus the features needed to score it once IDF is known.
#[derive(Clone, Debug)]
struct LineFeat {
    line: u64,
    col: usize,
    zone: Zone,
    text: String,
    before: Vec<String>,
    after: Vec<String>,
    /// (original-term index, occurrences on this line, capped)
    term_hits: Vec<(usize, usize)>,
    /// (subtoken index, occurrences on this line, capped)
    sub_hits: Vec<(usize, usize)>,
    len: usize,
}

/// A matched file plus its zone term frequencies, held until corpus statistics exist.
#[derive(Clone, Debug)]
struct FileCandidate {
    path: PathBuf,
    mtime: u64,
    size: u64,
    is_test: bool,
    /// Per original term, term frequency in [head, body, comment] line zones.
    tf: Vec<[f64; 3]>,
    /// Per original term, occurrences in the file path.
    tf_path: Vec<f64>,
    /// Per subtoken, term frequency across all matched lines.
    tf_sub: Vec<f64>,
    /// Per original term, present anywhere in the file (matched line or path).
    present: Vec<bool>,
    /// Term-presence mask of the widest definition-shaped line seen in the file.
    anchor_mask: Vec<bool>,
    lines: Vec<LineFeat>,
    root_bonus: f64,
}

/// Corpus statistics derived from the same walk that collected the matches.
#[derive(Debug)]
struct Corpus {
    n_docs: f64,
    avgdl: f64,
    idf: Vec<f64>,
    idf_sub: Vec<f64>,
    df: Vec<usize>,
    /// Total IDF mass of query terms that exist in the corpus at all.
    idf_total: f64,
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
    size: u64,
    matches: Vec<RankedMatch>,
}

#[derive(Serialize)]
struct JsonOutput {
    elapsed_ms: u128,
    files_scanned: usize,
    files_matched: usize,
    usages_scanned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    ranking: Option<RankingStats>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    definitions: Vec<DefEntry>,
    results: Vec<JsonFileResult>,
}

#[derive(Serialize)]
struct RankingStats {
    n_docs: f64,
    avgdl_bytes: f64,
    terms: Vec<TermStat>,
}

#[derive(Serialize)]
struct TermStat {
    term: String,
    df: usize,
    idf: f64,
}

#[derive(Serialize)]
struct JsonFileResult {
    path: String,
    score: f64,
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
    let should_try_broaden = !args.regex
        && !args.expand
        && !args.no_expand
        && !plan.fragments_used
        && !plan.sub_terms.is_empty();
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
    let exclude_res = compile_globs(&args.exclude)?;
    let matcher = RegexMatcherBuilder::new()
        .case_insensitive(plan.case_insensitive)
        .build(&plan.regex_text)
        .with_context(|| format!("invalid search regex: {}", plan.regex_text))?;

    let files_scanned = Arc::new(AtomicUsize::new(0));
    // Sum of ln(1 + size) in fixed point: avgdl is a geometric mean so a handful of
    // multi-megabyte blobs cannot inflate it and neutralize length normalization.
    let log_size_sum = Arc::new(AtomicU64::new(0));
    let exceeded_max_files = Arc::new(AtomicBool::new(false));
    let results = Arc::new(Mutex::new(Vec::<FileCandidate>::new()));
    let matcher = Arc::new(matcher);
    let plan = Arc::new(plan);
    let include_exts = Arc::new(include_exts);
    let glob_res = Arc::new(glob_res);
    let exclude_res = Arc::new(exclude_res);
    let roots_arc = Arc::new(roots.to_vec());
    let context = args.context;
    let max_per_file = args.max_per_file;
    let and_mode = args.and_mode;
    let max_files = args.max_files;
    let max_line_width = args.max_line_width;

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
        let log_size_sum = Arc::clone(&log_size_sum);
        let exceeded_max_files = Arc::clone(&exceeded_max_files);
        let results = Arc::clone(&results);
        let include_exts = Arc::clone(&include_exts);
        let glob_res = Arc::clone(&glob_res);
        let exclude_res = Arc::clone(&exclude_res);
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
            if should_skip_file(path, &include_exts, &glob_res, &exclude_res) {
                return WalkState::Continue;
            }
            let (size, mtime) = entry
                .metadata()
                .ok()
                .map(|m| {
                    (
                        m.len(),
                        m.modified()
                            .ok()
                            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                            .map(|d| d.as_secs())
                            .unwrap_or(0),
                    )
                })
                .unwrap_or((0, 0));
            let scanned = files_scanned.fetch_add(1, AtomicOrdering::Relaxed) + 1;
            log_size_sum.fetch_add(encode_log_size(size), AtomicOrdering::Relaxed);
            if scanned > max_files {
                exceeded_max_files.store(true, AtomicOrdering::Relaxed);
                return WalkState::Quit;
            }
            if let Ok(Some(result)) = collect_file_candidate(
                path,
                size,
                mtime,
                &matcher,
                &plan,
                context,
                max_per_file,
                max_line_width,
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

    let candidates = match Arc::try_unwrap(results) {
        Ok(mutex) => mutex.into_inner().unwrap_or_default(),
        Err(arc) => arc.lock().map(|guard| guard.clone()).unwrap_or_default(),
    };
    let scanned = files_scanned.load(AtomicOrdering::Relaxed);
    let corpus = build_corpus(
        scanned,
        log_size_sum.load(AtomicOrdering::Relaxed),
        &plan,
        &candidates,
    );
    let mut results = rank_candidates(candidates, &corpus, &plan, max_per_file);
    let elapsed_ms = started.elapsed().as_millis();
    let files_matched = results.len();
    results.truncate(args.limit);

    let mut output = to_json_output(elapsed_ms, scanned, files_matched, definitions, &results);
    if args.stats {
        output.ranking = Some(ranking_stats(&plan, &corpus));
    }
    Ok(output)
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

    if args.regex {
        // A raw regex is opaque: one term, no tiers, no anchors.
        let regex_text = wrap_word(&joined, args.word, true);
        let search_re = RegexBuilder::new(&regex_text)
            .case_insensitive(case_insensitive)
            .build()
            .with_context(|| format!("invalid search regex: {}", regex_text))?;
        return Ok(QueryPlan {
            regex_text: regex_text.clone(),
            sub_terms: Vec::new(),
            case_insensitive,
            search_re: search_re.clone(),
            original_re: None,
            term_res: vec![(regex_text.clone(), search_re)],
            term_weights: vec![W_TERM_ORIGINAL],
            subtoken_res: Vec::new(),
            fragments_used: false,
            anchor_res: [None, None, None],
        });
    }

    let originals = split_terms(&joined);
    if originals.is_empty() {
        return Err(anyhow!("query cannot be empty"));
    }

    // Tier 1: exactly what the caller typed. Tier 2: the identifiers inside it once
    // punctuation is removed. Tier 3: camel/underscore fragments of those identifiers.
    // `--no-expand` keeps tier 1 only, which is how a literal punctuated string is searched.
    let mut core = Vec::<String>::new();
    let mut core_seen = HashSet::<String>::new();
    let mut term_weights = Vec::<f64>::new();
    for term in &originals {
        if push_unique(&mut core, &mut core_seen, term.clone(), case_insensitive) {
            term_weights.push(W_TERM_ORIGINAL);
        }
    }
    let mut n_segments = 0usize;
    if !args.no_expand {
        for term in &originals {
            for seg in segments_of(term) {
                if push_unique(&mut core, &mut core_seen, seg, case_insensitive) {
                    term_weights.push(W_TERM_SEGMENT);
                    n_segments += 1;
                }
            }
        }
    }

    let mut sub_terms = Vec::<String>::new();
    let mut sub_seen = core_seen.clone();
    if !args.no_expand {
        for term in &core {
            for sub in subtokens_of(term).into_iter().filter(|s| keep_subtoken(s)) {
                push_unique(&mut sub_terms, &mut sub_seen, sub, case_insensitive);
            }
        }
    }

    // A multi-word or punctuation-split query is a phrase and needs recall; a bare identifier
    // is a symbol lookup and needs precision, so it only broadens if it comes back sparse.
    let phrase_shaped = originals.len() >= 2 || n_segments >= 2;
    let fragments_used =
        !sub_terms.is_empty() && (force_expand || args.expand || phrase_shaped) && !args.no_expand;

    let mut all: Vec<String> = core.clone();
    if fragments_used {
        all.extend(sub_terms.iter().cloned());
    }
    all.sort_by_key(|s| std::cmp::Reverse(s.len()));
    let regex_text = union_pattern(&all, args.word);

    let search_re = RegexBuilder::new(&regex_text)
        .case_insensitive(case_insensitive)
        .build()
        .with_context(|| format!("invalid search regex: {}", regex_text))?;
    let original_re = Some(build_union_re(&core, case_insensitive, args.word)?);

    let mut term_res = Vec::new();
    for term in &core {
        term_res.push((
            term.clone(),
            build_term_re(term, case_insensitive, args.word)?,
        ));
    }
    let mut subtoken_res = Vec::new();
    if fragments_used {
        for term in &sub_terms {
            subtoken_res.push((
                term.clone(),
                build_term_re(term, case_insensitive, args.word)?,
            ));
        }
    }
    let anchor_res = [
        Some(build_anchor_re(&core, case_insensitive, FileClass::Code)?),
        Some(build_anchor_re(
            &core,
            case_insensitive,
            FileClass::Structured,
        )?),
        Some(build_anchor_re(&core, case_insensitive, FileClass::Prose)?),
    ];

    Ok(QueryPlan {
        regex_text,
        sub_terms,
        case_insensitive,
        search_re,
        original_re,
        term_res,
        term_weights,
        subtoken_res,
        fragments_used,
        anchor_res,
    })
}

fn split_terms(query: &str) -> Vec<String> {
    query
        .split(|c: char| c.is_whitespace() || c == '|')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Maximal alphanumeric/underscore runs inside a term. This is the fix for punctuation-glued
/// queries: an alert title glues the discriminative identifier to scaffolding
/// (`*POST-/tenant/api/v1.0/parserecordsfromrequest*`, `[Service][QOS]`), and whole-token
/// matching never reaches it. Segments are first-class terms, not weak fragments.
fn segments_of(term: &str) -> Vec<String> {
    term.split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|s| keep_segment(s))
        .map(|s| s.to_string())
        .collect()
}

/// Short codes carry real discriminative power in operational text (`WW`, `DF`, `QOS`, `403`),
/// so segments are deliberately not length-filtered the way expanded fragments are. Only
/// single characters and bare one/two-digit numbers are dropped as noise.
fn keep_segment(s: &str) -> bool {
    let len = s.chars().count();
    if len < 2 {
        return false;
    }
    !(len <= 2 && s.chars().all(|c| c.is_ascii_digit()))
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

/// Subtokens for query expansion. Unlike `is_identifier_like` (which gates the symbol-DB
/// fast path and must stay strict), this splits on *any* non-alphanumeric run first, so a
/// term the caller pasted with surrounding punctuation still expands into its identifier
/// parts instead of degrading to a literal that matches nothing.
fn subtokens_of(term: &str) -> Vec<String> {
    if term.starts_with('-') {
        return Vec::new();
    }
    let mut out = Vec::new();
    for chunk in term.split(|c: char| !c.is_ascii_alphanumeric() && c != '_') {
        if chunk.is_empty() {
            continue;
        }
        let parts = split_identifier(chunk);
        for sub in &parts {
            out.push(sub.clone());
        }
        out.extend(prefix_grams(&parts));
    }
    out
}

/// Cumulative leading concatenations of a compound identifier: `Get|Skills|Show|AI|Disabled`
/// also yields `GetRecords`, `GetRecordsShow`, `GetRecordsShowUI`, `GetRecordsShowUIArchived`.
///
/// Generated names — scenarios, monitors, test cases, jobs — are built by appending
/// qualifiers, so a corpus routinely records a shorter form of a longer name the caller
/// pastes. Whole-token matching misses that, and single-part fragments are too weak to
/// recover it. Prefixes are linear in the part count (not the quadratic all-n-grams set),
/// and they stay in the fragment tier because they are inferred, not typed.
fn prefix_grams(parts: &[String]) -> Vec<String> {
    if parts.len() < 4 {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut acc = String::new();
    // A 2-part prefix of a compound name is too generic (`AsyncLLM` of `AsyncLLMPathResult`
    // matches every neighbouring type), so start at 3 parts. The full token is tier 1/2.
    for (i, part) in parts.iter().take(parts.len() - 1).enumerate() {
        acc.push_str(part);
        if i >= 2 && acc.len() >= MIN_PREFIX_GRAM_LEN {
            out.push(acc.clone());
            if out.len() >= MAX_PREFIX_GRAMS {
                break;
            }
        }
    }
    out
}

fn keep_subtoken(token: &str) -> bool {
    let lower = token.to_ascii_lowercase();
    lower.len() >= 4 && !SUBTOKEN_STOPWORDS.contains(&lower.as_str())
}

fn push_unique(out: &mut Vec<String>, seen: &mut HashSet<String>, term: String, ci: bool) -> bool {
    let k = key(&term, ci);
    if seen.insert(k) {
        out.push(term);
        true
    } else {
        false
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

/// Terms at or above this length match as unbounded substrings, which is what lets a corpus
/// record of `getrecordsshowuiarchived` answer a query that pasted the shorter `getrecords`.
/// Shorter terms get word boundaries instead. Operational codes (`ICE`, `ACE`, `DF`, `WW`)
/// have no such prefix relationship, and as substrings they collide with ordinary corpus
/// words — `ice` inside `services`, `ace` inside `namespace`, `df` inside `sdfv2` — which
/// inflates their document frequency, collapses their IDF toward zero, and destroys exactly
/// the discrimination they exist to provide. Measured on the example-service KB: bounding moved
/// `ice` from df=125/idf=0.16 to df=9/idf=2.74, `ace` from 114/0.25 to 13/2.39, and `df`
/// from 91/0.47 to 10/2.64, while non-colliding `ww` and `staging` were unchanged.
const MIN_UNBOUNDED_TERM_LEN: usize = 5;

/// `-w` forces boundaries on every term; otherwise only short terms are bounded.
fn bound_term(term: &str, word: bool) -> bool {
    word || term.chars().count() < MIN_UNBOUNDED_TERM_LEN
}

/// Build an alternation that bounds each term independently. Wrapping the *joined* pattern
/// would force one policy on every term and lose the asymmetry.
fn union_pattern(terms: &[String], word: bool) -> String {
    let parts = terms
        .iter()
        .map(|s| wrap_word(&regex::escape(s), bound_term(s, word), false))
        .collect::<Vec<_>>()
        .join("|");
    format!("(?:{})", parts)
}

fn build_union_re(terms: &[String], ci: bool, word: bool) -> Result<Regex> {
    let pat = union_pattern(terms, word);
    RegexBuilder::new(&pat)
        .case_insensitive(ci)
        .build()
        .context("building original term regex")
}

fn build_term_re(term: &str, ci: bool, word: bool) -> Result<Regex> {
    RegexBuilder::new(&wrap_word(
        &regex::escape(term),
        bound_term(term, word),
        true,
    ))
    .case_insensitive(ci)
    .build()
    .context("building term regex")
}

/// Definition-anchor patterns, one per file class. `@T@` is replaced with an alternation over
/// the query's original terms (`@B@` with the word-bounded form). Running a C-family
/// declaration regex over Markdown is what made `term:` anywhere in a sentence look like a
/// definition, so each class gets the shape that is actually a definition in that syntax.
/// A code definition is the *declared name*, not any line that happens to mention a modifier
/// keyword. Each alternative requires the term to sit in a naming position: right after a
/// declaring keyword, opening the line and bound with `:`/`=`, being the callable declared,
/// or being the member declared after its type. `public Foo Bar { get; set; }` therefore
/// anchors `Bar` but not `Foo`, and `new Foo()` anchors nothing.
const CODE_ANCHOR: &str = concat!(
    r"(\b(class|struct|interface|enum|trait|record|union|impl|type|typedef|typealias|protocol",
    r"|actor|module|namespace|package|fn|def|func|function|sub|procedure|method",
    r"|let|var|const|val|property|event|delegate)[ \t]+@B@",
    r"|^[ \t]*(?:[-*][ \t]+)?@T@[ \t]*[:=](?:[^=]|$)",
    r"|^[ \t]*(?:[\w<>\[\],.?&*~]+[ \t]+)*@T@[ \t]*\(",
    r"|^[ \t]*(?:[\w<>\[\],.?&*]+[ \t]+){1,6}@T@[ \t]*(?:[;={]|$))",
);

/// Key position only: start of line (optionally a list dash / quote / bracket), then `:` or `=`.
const STRUCTURED_ANCHOR: &str =
    r#"^[ \t]*(?:[-*][ \t]+)?(?:["'\[][ \t]*)?@T@(?:[ \t]*["'\]])?[ \t]*[:=]"#;

/// A heading that names the term, a line that opens with `term:` / `term —`, or a table row
/// whose first cell is the term. These are the shapes that introduce a record in prose.
const PROSE_ANCHOR: &str = concat!(
    r"^[ \t]{0,3}(?:#{1,6}[ \t]+[^\n]*@B@",
    r#"|(?:[-*+][ \t]+|\d+\.[ \t]+|>[ \t]+)?[`*_"']{0,3}@T@[`*_"']{0,3}[ \t]*(?:[:=]|\u{2014}|\u{2013}|--)"#,
    r"|\|[ \t]*[`*_]{0,3}@T@[`*_]{0,3}[ \t]*\|)",
);

/// `\b` only where the term itself starts/ends with a word character: a term such as
/// `[Service]` or `*POST-/x*` has no word boundary to assert and would never match.
fn word_bounded(term: &str) -> String {
    let escaped = regex::escape(term);
    let is_word = |c: char| c.is_alphanumeric() || c == '_';
    let lead = term.chars().next().map(is_word).unwrap_or(false);
    let trail = term.chars().last().map(is_word).unwrap_or(false);
    format!(
        "{}{}{}",
        if lead { r"\b" } else { "" },
        escaped,
        if trail { r"\b" } else { "" }
    )
}

fn build_anchor_re(terms: &[String], ci: bool, class: FileClass) -> Result<Regex> {
    let raw = terms
        .iter()
        .map(|t| regex::escape(t))
        .collect::<Vec<_>>()
        .join("|");
    let bounded = terms
        .iter()
        .map(|t| word_bounded(t))
        .collect::<Vec<_>>()
        .join("|");
    let raw = format!("(?:{})", raw);
    let bounded = format!("(?:{})", bounded);
    let template = match class {
        FileClass::Code => CODE_ANCHOR,
        FileClass::Structured => STRUCTURED_ANCHOR,
        FileClass::Prose => PROSE_ANCHOR,
    };
    let pat = template.replace("@B@", &bounded).replace("@T@", &raw);
    RegexBuilder::new(&pat)
        .case_insensitive(ci)
        .build()
        .with_context(|| format!("building {:?} definition regex", class))
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

fn should_skip_file(
    path: &Path,
    include_exts: &HashSet<String>,
    glob_res: &[Regex],
    exclude_res: &[Regex],
) -> bool {
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
    if !glob_res.is_empty() || !exclude_res.is_empty() {
        let normalized = path.to_string_lossy().replace('\\', "/");
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if exclude_res
            .iter()
            .any(|re| re.is_match(&normalized) || re.is_match(file_name))
        {
            return true;
        }
        if !glob_res.is_empty()
            && !glob_res
                .iter()
                .any(|re| re.is_match(&normalized) || re.is_match(file_name))
        {
            return true;
        }
    }
    false
}

/// Collect one file's matches plus the zone term frequencies BM25F needs. Scoring itself is
/// deferred: IDF and average document length are only known once the whole walk is done.
#[allow(clippy::too_many_arguments)]
fn collect_file_candidate(
    path: &Path,
    size: u64,
    mtime: u64,
    matcher: &grep_regex::RegexMatcher,
    plan: &QueryPlan,
    context: usize,
    max_per_file: usize,
    max_line_width: usize,
    and_mode: bool,
    roots: &[PathBuf],
) -> Result<Option<FileCandidate>> {
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

    let n_terms = plan.term_res.len();
    let n_subs = plan.subtoken_res.len();
    let class = classify_file(path);
    let anchor_re = plan.anchor_re(class);
    let path_key = key(&path.to_string_lossy(), plan.case_insensitive);

    let mut tf = vec![[0.0f64; 3]; n_terms];
    let mut tf_sub = vec![0.0f64; n_subs];
    let mut tf_path = vec![0.0f64; n_terms];
    let mut present = vec![false; n_terms];
    let mut anchor_mask = vec![false; n_terms];
    let mut anchor_width = 0usize;

    // The path zone is matched with the term's own compiled regex, not a raw substring scan:
    // every ancestor directory contributes to `path_key`, so an unbounded short term (`ice`
    // inside `.../services/...`) would otherwise score a path hit on every candidate in the
    // corpus and flatten the zone it is supposed to discriminate with.
    for (i, (_, re)) in plan.term_res.iter().enumerate() {
        let count = re.find_iter(&path_key).take(MAX_TF_PER_LINE).count();
        if count > 0 {
            tf_path[i] = count as f64;
            present[i] = true;
        }
    }

    let mut feats: Vec<LineFeat> = Vec::with_capacity(sink.matches.len().min(cap));
    for raw in sink.matches {
        let text = raw.text.trim_end().to_string();
        let is_anchor = anchor_re.map(|re| re.is_match(&text)).unwrap_or(false);
        let zone = if is_anchor {
            Zone::Head
        } else if looks_like_comment(&text, class) {
            Zone::Comment
        } else {
            Zone::Body
        };
        let zi = zone.index();

        let mut term_hits = Vec::new();
        for (i, (_, re)) in plan.term_res.iter().enumerate() {
            let occ = re.find_iter(&text).take(MAX_TF_PER_LINE).count();
            if occ > 0 {
                tf[i][zi] += occ as f64;
                present[i] = true;
                term_hits.push((i, occ));
            }
        }
        let mut sub_hits = Vec::new();
        for (i, (_, re)) in plan.subtoken_res.iter().enumerate() {
            let occ = re.find_iter(&text).take(MAX_TF_PER_LINE).count();
            if occ > 0 {
                tf_sub[i] += occ as f64;
                sub_hits.push((i, occ));
            }
        }
        if is_anchor && term_hits.len() > anchor_width {
            anchor_width = term_hits.len();
            anchor_mask = vec![false; n_terms];
            for (i, _) in &term_hits {
                anchor_mask[*i] = true;
            }
        }
        if term_hits.is_empty() && sub_hits.is_empty() {
            continue;
        }
        let col = first_col(&text, plan).unwrap_or(1);
        let len = text.chars().count();
        feats.push(LineFeat {
            line: raw.line,
            col,
            zone,
            text: clip_match_line(&text, col, max_line_width),
            before: raw
                .before
                .into_iter()
                .map(|l| clip_context_line(&l, max_line_width))
                .collect(),
            after: raw
                .after
                .into_iter()
                .map(|l| clip_context_line(&l, max_line_width))
                .collect(),
            term_hits,
            sub_hits,
            len,
        });
    }

    if feats.is_empty() {
        return Ok(None);
    }
    // `--and` means every term the caller actually typed must be present. Recovered
    // segments are recall aids, not requirements, so they are excluded from the test.
    if and_mode
        && !present
            .iter()
            .enumerate()
            .all(|(i, p)| *p || term_weight(plan, i) < W_TERM_ORIGINAL)
    {
        return Ok(None);
    }

    // Retain a bounded candidate pool: the structurally strongest lines, plus the best line
    // for every term, so a rare term's line survives to the IDF-aware pass that follows.
    let retain = (max_per_file.saturating_mul(4)).max(12);
    if feats.len() > retain {
        let mut order: Vec<usize> = (0..feats.len()).collect();
        order.sort_by(|&a, &b| {
            prelim_line_score(&feats[b])
                .partial_cmp(&prelim_line_score(&feats[a]))
                .unwrap_or(Ordering::Equal)
                .then_with(|| feats[a].line.cmp(&feats[b].line))
        });
        let mut keep: Vec<bool> = vec![false; feats.len()];
        for &i in order.iter().take(retain) {
            keep[i] = true;
        }
        for t in 0..n_terms {
            if order
                .iter()
                .take(retain)
                .any(|&i| feats[i].term_hits.iter().any(|(ti, _)| *ti == t))
            {
                continue;
            }
            if let Some(&i) = order
                .iter()
                .find(|&&i| feats[i].term_hits.iter().any(|(ti, _)| *ti == t))
            {
                keep[i] = true;
            }
        }
        let mut idx = 0usize;
        feats.retain(|_| {
            let k = keep[idx];
            idx += 1;
            k
        });
    }

    let root_bonus = roots
        .iter()
        .position(|r| path.starts_with(r))
        .map(|i| (roots.len() - i) as f64 * 0.01)
        .unwrap_or(0.0);
    Ok(Some(FileCandidate {
        path: path.to_path_buf(),
        mtime,
        size,
        is_test: is_test_path(path),
        tf,
        tf_path,
        tf_sub,
        present,
        anchor_mask,
        lines: feats,
        root_bonus,
    }))
}

/// Structure-only ordering used to bound the retained pool before IDF exists.
fn prelim_line_score(f: &LineFeat) -> f64 {
    let coverage = f.term_hits.len() as f64 + 0.3 * f.sub_hits.len() as f64;
    coverage * f.zone.line_multiplier() * line_length_damp(f.len)
}

/// Long lines carry the same term in far more noise; damp them so a concise line wins the
/// display slot. Gentle and logarithmic — a long table row is still shown when it is the match.
fn line_length_damp(len: usize) -> f64 {
    let over = (len as f64 - LINE_DAMP_START).max(0.0);
    1.0 / (1.0 + (1.0 + over / LINE_DAMP_START).ln())
}

fn encode_log_size(size: u64) -> u64 {
    ((1.0 + size as f64).ln() * 4096.0) as u64
}

fn build_corpus(
    files_scanned: usize,
    log_size_sum: u64,
    plan: &QueryPlan,
    candidates: &[FileCandidate],
) -> Corpus {
    let n_docs = files_scanned.max(candidates.len()).max(1) as f64;
    let avgdl = if files_scanned > 0 {
        ((log_size_sum as f64 / 4096.0) / files_scanned as f64).exp() - 1.0
    } else {
        0.0
    }
    .max(1.0);

    let n_terms = plan.term_res.len();
    let n_subs = plan.subtoken_res.len();
    let mut df = vec![0usize; n_terms];
    let mut df_sub = vec![0usize; n_subs];
    for c in candidates {
        for (i, p) in c.present.iter().enumerate() {
            if *p {
                df[i] += 1;
            }
        }
        for (i, tf) in c.tf_sub.iter().enumerate() {
            if *tf > 0.0 {
                df_sub[i] += 1;
            }
        }
    }
    let idf: Vec<f64> = df.iter().map(|&d| idf_of(n_docs, d)).collect();
    let idf_sub: Vec<f64> = df_sub.iter().map(|&d| idf_of(n_docs, d)).collect();
    // Terms that appear nowhere cannot be covered, so they must not dilute coverage.
    // Weighted so a recovered segment counts for less of the query than a typed term.
    let idf_total = idf
        .iter()
        .zip(df.iter())
        .enumerate()
        .filter(|(_, (_, &d))| d > 0)
        .map(|(i, (v, _))| term_weight(plan, i) * *v)
        .sum::<f64>();
    Corpus {
        n_docs,
        avgdl,
        idf,
        idf_sub,
        df,
        idf_total,
    }
}

fn idf_of(n_docs: f64, df: usize) -> f64 {
    let df = df as f64;
    (1.0 + (n_docs - df + 0.5) / (df + 0.5)).ln()
}

/// Weight of query term `i`: full for a term the caller typed, reduced for one recovered
/// from inside it. Missing entries default to full weight so callers stay safe.
fn term_weight(plan: &QueryPlan, i: usize) -> f64 {
    plan.term_weights.get(i).copied().unwrap_or(W_TERM_ORIGINAL)
}

fn bm25_saturate(tfn: f64) -> f64 {
    if tfn <= 0.0 {
        0.0
    } else {
        tfn * (BM25_K1 + 1.0) / (BM25_K1 + tfn)
    }
}

fn rank_candidates(
    candidates: Vec<FileCandidate>,
    corpus: &Corpus,
    plan: &QueryPlan,
    max_per_file: usize,
) -> Vec<FileResult> {
    let mut out: Vec<FileResult> = candidates
        .into_iter()
        .map(|c| score_candidate(c, corpus, plan, max_per_file))
        .collect();
    out.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(Ordering::Equal)
            // Ties go to the more focused document, then to a stable path order, so a
            // near-tie is not resolved by whichever directory sorts first alphabetically.
            .then_with(|| a.size.cmp(&b.size))
            .then_with(|| a.path.cmp(&b.path))
    });
    out
}

fn score_candidate(
    c: FileCandidate,
    corpus: &Corpus,
    plan: &QueryPlan,
    max_per_file: usize,
) -> FileResult {
    let ratio = (c.size as f64 / corpus.avgdl).max(0.0);
    let norm_body = 1.0 - BM25_B_BODY + BM25_B_BODY * ratio;
    let norm_head = 1.0 - BM25_B_HEAD + BM25_B_HEAD * ratio;
    let norm_body = norm_body.max(0.05);
    let norm_head = norm_head.max(0.05);

    let mut bm25 = 0.0;
    for (i, zones) in c.tf.iter().enumerate() {
        let tfn = W_HEAD * zones[0] / norm_head
            + W_BODY * zones[1] / norm_body
            + W_COMMENT * zones[2] / norm_body
            + W_PATH * c.tf_path[i];
        bm25 += term_weight(plan, i) * corpus.idf[i] * bm25_saturate(tfn);
    }
    for (i, tf) in c.tf_sub.iter().enumerate() {
        let tfn = W_BODY * tf / norm_body;
        bm25 += W_SUBTOKEN * corpus.idf_sub[i] * bm25_saturate(tfn);
    }

    let mut covered = 0.0;
    let mut anchored = 0.0;
    let mut answerable_weight = 0.0;
    for (i, p) in c.present.iter().enumerate() {
        if corpus.df[i] == 0 {
            continue;
        }
        let w = term_weight(plan, i);
        answerable_weight += w;
        if *p {
            covered += w * corpus.idf[i];
        }
        if c.anchor_mask.get(i).copied().unwrap_or(false) {
            anchored += w * corpus.idf[i];
        }
    }
    let coverage = if corpus.idf_total > 0.0 {
        covered / corpus.idf_total
    } else {
        0.0
    };
    // Mean anchored IDF over the answerable query. Scaling by IDF is the point: a
    // definition-shaped line for a rare term is strong evidence, one for a common word is
    // not. Dividing by the query width keeps a wide prose query from stacking anchors.
    let anchor_gain = if answerable_weight > 0.0 {
        anchored / answerable_weight
    } else {
        0.0
    };

    let mut score = bm25 + COVERAGE_BONUS * coverage + ANCHOR_BONUS * anchor_gain;
    if c.is_test {
        score *= TEST_PATH_FACTOR;
    }
    score += c.root_bonus;

    let mut matches: Vec<RankedMatch> = c
        .lines
        .into_iter()
        .map(|f| {
            let mut w = 0.0;
            for (i, occ) in &f.term_hits {
                w += term_weight(plan, *i) * corpus.idf[*i] * (1.0 + (*occ as f64).ln());
            }
            for (i, occ) in &f.sub_hits {
                w += W_SUBTOKEN * corpus.idf_sub[*i] * (1.0 + (*occ as f64).ln());
            }
            let line_score = w * f.zone.line_multiplier() * line_length_damp(f.len);
            RankedMatch {
                line: f.line,
                col: f.col,
                kind: f.zone.kind().to_string(),
                text: f.text,
                before: f.before,
                after: f.after,
                score: line_score,
            }
        })
        .collect();
    matches.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.line.cmp(&b.line))
    });
    let mut seen_lines = HashSet::new();
    matches.retain(|m| seen_lines.insert(m.text.trim().to_string()));
    matches.truncate(max_per_file);
    FileResult {
        path: c.path,
        score,
        mtime: c.mtime,
        size: c.size,
        matches,
    }
}

fn ranking_stats(plan: &QueryPlan, corpus: &Corpus) -> RankingStats {
    let terms = plan
        .term_res
        .iter()
        .enumerate()
        .map(|(i, (term, _))| TermStat {
            term: term.clone(),
            df: corpus.df[i],
            idf: (corpus.idf[i] * 1000.0).round() / 1000.0,
        })
        .collect();
    RankingStats {
        n_docs: corpus.n_docs,
        avgdl_bytes: corpus.avgdl.round(),
        terms,
    }
}

/// Clip a match line to a window around the first match so a multi-kilobyte line cannot
/// swamp the output. `col` is the 1-based column in the *original* line and is preserved.
fn clip_match_line(text: &str, col: usize, max_width: usize) -> String {
    if max_width == 0 {
        return text.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= max_width {
        return text.to_string();
    }
    let lead = max_width / 4;
    let start = col.saturating_sub(1).saturating_sub(lead).min(
        chars
            .len()
            .saturating_sub(max_width)
            .max(0),
    );
    let end = (start + max_width).min(chars.len());
    let mut out = String::new();
    if start > 0 {
        out.push('…');
    }
    out.extend(chars[start..end].iter());
    if end < chars.len() {
        out.push('…');
    }
    out
}

fn clip_context_line(text: &str, max_width: usize) -> String {
    if max_width == 0 {
        return text.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= max_width {
        return text.to_string();
    }
    let mut out: String = chars[..max_width].iter().collect();
    out.push('…');
    out
}

fn first_col(text: &str, plan: &QueryPlan) -> Option<usize> {
    if let Some(re) = &plan.original_re {
        if let Some(m) = re.find(text) {
            return Some(m.start() + 1);
        }
    }
    plan.search_re.find(text).map(|m| m.start() + 1)
}


fn is_test_path(path: &Path) -> bool {
    let p = path.to_string_lossy().to_ascii_lowercase();
    p.contains("test")
        || p.contains("spec")
        || p.contains("__tests__")
        || p.contains(".test.")
        || p.contains(".spec.")
}

/// Comment markers are syntax, so they are read per file class. In Markdown a leading `#` is
/// a heading — the line canonical records open with — not a comment to be penalized.
fn looks_like_comment(text: &str, class: FileClass) -> bool {
    let t = text.trim_start();
    match class {
        FileClass::Code => {
            t.starts_with("//")
                || t.starts_with('#')
                || t.starts_with(';')
                || t.starts_with("--")
                || t.starts_with("/*")
                || t.starts_with('*')
        }
        FileClass::Structured => t.starts_with('#') || t.starts_with(';') || t.starts_with("//"),
        FileClass::Prose => t.starts_with("<!--"),
    }
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
        ranking: None,
        definitions: definitions.to_vec(),
        results: results
            .iter()
            .map(|r| JsonFileResult {
                path: clean_path(&r.path),
                score: (r.score * 100.0).round() / 100.0,
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
            "#{} {}  (score={:.2}, mtime={})",
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
    let exclude_res: Vec<Regex> = Vec::new();
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
            && !should_skip_file(entry.path(), &include_exts, &glob_res, &exclude_res)
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
        exclude: Vec::new(),
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
        max_line_width: 0,
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
        let parts = split_identifier("GetAccountSummaryStatistics")
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

    fn plan_for(terms: &[&str]) -> QueryPlan {
        let args = SearchArgs {
            query: terms.iter().map(|s| s.to_string()).collect(),
            paths: vec![PathBuf::from(".")],
            globs: Vec::new(),
            exclude: Vec::new(),
            ext: None,
            regex: false,
            word: false,
            no_expand: true,
            expand: false,
            and_mode: false,
            ignore_case: true,
            limit: DEFAULT_LIMIT,
            context: 0,
            max_per_file: 3,
            max_line_width: DEFAULT_MAX_LINE_WIDTH,
            max_files: DEFAULT_MAX_FILES,
            json: true,
            stats: false,
            usages: true,
            quiet: true,
        };
        build_query_plan(&args, false).expect("plan")
    }

    fn candidate(path: &str, size: u64, tf_body: f64, anchored: bool) -> FileCandidate {
        FileCandidate {
            path: PathBuf::from(path),
            mtime: 0,
            size,
            is_test: is_test_path(Path::new(path)),
            tf: vec![[0.0, tf_body, 0.0]],
            tf_path: vec![0.0],
            tf_sub: Vec::new(),
            present: vec![tf_body > 0.0],
            anchor_mask: vec![anchored],
            lines: Vec::new(),
            root_bonus: 0.0,
        }
    }

    #[test]
    fn classify_file_separates_prose_structured_and_code() {
        assert_eq!(classify_file(Path::new("a\\notes.md")), FileClass::Prose);
        assert_eq!(classify_file(Path::new("a\\notes.markdown")), FileClass::Prose);
        assert_eq!(classify_file(Path::new("a\\ledger.toon")), FileClass::Structured);
        assert_eq!(classify_file(Path::new("a\\conf.yaml")), FileClass::Structured);
        assert_eq!(classify_file(Path::new("a\\Program.cs")), FileClass::Code);
        assert_eq!(classify_file(Path::new("a\\noext")), FileClass::Code);
    }

    #[test]
    fn markdown_headings_are_not_comments() {
        // The inversion the audit measured: a `#` heading is where prose puts its record id.
        assert!(!looks_like_comment("## fk.tenant-settings-timeout", FileClass::Prose));
        assert!(looks_like_comment("<!-- editorial note -->", FileClass::Prose));
        assert!(looks_like_comment("# shell comment", FileClass::Code));
        assert!(looks_like_comment("// c comment", FileClass::Code));
        assert!(looks_like_comment("# toon comment", FileClass::Structured));
    }

    #[test]
    fn code_anchor_requires_a_naming_position() {
        let re = build_anchor_re(&["AoaiLatencyCheckpoint".to_string()], false, FileClass::Code)
            .expect("anchor");
        assert!(re.is_match("    public class AoaiLatencyCheckpoint"));
        assert!(re.is_match("public sealed record AoaiLatencyCheckpoint(int A);"));
        // the term is the *type*, not the declared name
        assert!(!re.is_match(
            "    private static void Populate(IDictionary<string, object> p, AoaiLatencyCheckpoint c)"
        ));
        assert!(!re.is_match("    var checkpoint = new AoaiLatencyCheckpoint"));
        assert!(!re.is_match("        payload[\"AoaiLatencyCheckpoint\"] = checkpoint;"));
    }

    #[test]
    fn code_anchor_still_matches_members_and_callables() {
        let re = build_anchor_re(&["LatencyCheckpoint".to_string()], false, FileClass::Code)
            .expect("anchor");
        assert!(re.is_match("        public AoaiLatencyCheckpoint LatencyCheckpoint { get; set; }"));
        let f = build_anchor_re(&["RunAsync".to_string()], false, FileClass::Code).expect("anchor");
        assert!(f.is_match("        public async Task RunAsync(CancellationToken token)"));
        assert!(f.is_match("def RunAsync(self):"));
    }

    #[test]
    fn structured_anchor_only_fires_in_key_position() {
        let re = build_anchor_re(&["opt-out".to_string()], true, FileClass::Structured)
            .expect("anchor");
        assert!(re.is_match("  opt-out: true"));
        assert!(re.is_match("  - \"opt-out\" = 1"));
        // the measured defect: a term in *value* position used to score as a definition
        assert!(!re.is_match("  reason: opt-out: handled upstream"));
        assert!(!re.is_match("  note = see opt-out"));
    }

    #[test]
    fn prose_anchor_covers_heading_lead_in_and_table_key() {
        let re =
            build_anchor_re(&["fk.dsapi-timeout".to_string()], true, FileClass::Prose).expect("anchor");
        assert!(re.is_match("### fk.dsapi-timeout — tenant settings read stalls"));
        assert!(re.is_match("fk.dsapi-timeout: tenant settings read stalls"));
        assert!(re.is_match("| fk.dsapi-timeout | DSApi | 5xx |"));
        assert!(!re.is_match("see also fk.dsapi-timeout for the retry budget"));
    }

    #[test]
    fn punctuated_terms_still_build_an_anchor() {
        // `\b` cannot be asserted around `[Service]`; the old builder emitted `\b\[Service\]\b`
        // and silently matched nothing.
        let re = build_anchor_re(&["[Service]".to_string()], true, FileClass::Prose).expect("anchor");
        assert!(re.is_match("## [Service] availability"));
    }

    #[test]
    fn idf_rewards_rarity_and_never_goes_negative() {
        let rare = idf_of(1000.0, 2);
        let common = idf_of(1000.0, 400);
        let everywhere = idf_of(1000.0, 1000);
        assert!(rare > common, "rare={rare} common={common}");
        assert!(common > everywhere);
        assert!(everywhere > 0.0, "idf must stay positive: {everywhere}");
    }

    #[test]
    fn bm25_saturates_repetition() {
        let one = bm25_saturate(1.0);
        let five = bm25_saturate(5.0);
        let fifty = bm25_saturate(50.0);
        assert!(five > one);
        assert!(fifty > five);
        // 5x the term frequency must not buy 5x the score
        assert!(five < 5.0 * one, "five={five} one={one}");
        assert!(fifty < BM25_K1 + 1.0);
    }

    #[test]
    fn long_lines_are_damped_then_clipped() {
        assert!((line_length_damp(80) - 1.0).abs() < 1e-9);
        assert!(line_length_damp(4000) < line_length_damp(400));
        let long = "x".repeat(21_246);
        let clipped = clip_match_line(&long, 12_000, 400);
        assert!(clipped.chars().count() <= 400 + 2, "got {}", clipped.chars().count());
        assert!(clipped.starts_with('…') && clipped.ends_with('…'));
        assert_eq!(clip_match_line("short", 1, 400), "short");
        assert_eq!(clip_match_line(&long, 0, 0).len(), long.len());
        assert!(clip_context_line(&long, 400).chars().count() <= 401);
    }

    #[test]
    fn clip_match_line_keeps_the_hit_visible() {
        let text = format!("{}NEEDLE{}", "a".repeat(2000), "b".repeat(2000));
        let clipped = clip_match_line(&text, 2001, 200);
        assert!(clipped.contains("NEEDLE"), "clip dropped the match: {clipped}");
    }

    #[test]
    fn subtokens_split_punctuated_query_terms() {
        let subs = subtokens_of("[WW JobName: GetPersonalizedTasksForbiddenScenario]");
        assert!(subs.iter().any(|s| s.eq_ignore_ascii_case("personalized")));
        assert!(subs.iter().any(|s| s.eq_ignore_ascii_case("forbidden")));
        assert!(subs.iter().any(|s| s.eq_ignore_ascii_case("job")));
        // noise subtokens are dropped by the caller's filter, not by the splitter
        assert!(!keep_subtoken("get"));
        assert!(subtokens_of("-i").is_empty());
    }

    #[test]
    fn length_normalization_prefers_the_focused_file() {
        let plan = plan_for(&["swssprocessor"]);
        let big = candidate("kb\\aggregate.md", 240_000, 6.0, false);
        let small = candidate("kb\\processor.md", 3_000, 3.0, false);
        let cands = vec![big, small];
        let corpus = build_corpus(200, encode_log_size(8_000) * 200, &plan, &cands);
        let ranked = rank_candidates(cands, &corpus, &plan, 3);
        assert_eq!(
            ranked[0].path,
            PathBuf::from("kb\\processor.md"),
            "a 240 KB aggregate should not outrank a focused file on raw hit count"
        );
    }

    #[test]
    fn anchored_file_outranks_a_denser_usage_file() {
        let plan = plan_for(&["clusteranalysiscontent"]);
        let usage = candidate("src\\Uses.cs", 4_000, 3.0, false);
        let def = candidate("src\\Types.cs", 6_000, 2.0, true);
        let cands = vec![usage, def];
        let corpus = build_corpus(500, encode_log_size(6_000) * 500, &plan, &cands);
        let ranked = rank_candidates(cands, &corpus, &plan, 3);
        assert_eq!(ranked[0].path, PathBuf::from("src\\Types.cs"));
    }

    #[test]
    fn test_paths_are_demoted_but_not_erased() {
        let plan = plan_for(&["adminauthorizationhandler"]);
        let prod = candidate("src\\Handler.cs", 5_000, 2.0, true);
        let tests = candidate("test\\HandlerTests.cs", 5_000, 2.0, true);
        assert!(tests.is_test && !prod.is_test);
        let cands = vec![tests, prod];
        let corpus = build_corpus(500, encode_log_size(5_000) * 500, &plan, &cands);
        let ranked = rank_candidates(cands, &corpus, &plan, 3);
        assert_eq!(ranked[0].path, PathBuf::from("src\\Handler.cs"));
        assert_eq!(ranked.len(), 2, "test files stay retrievable");
    }

    #[test]
    fn ties_break_on_focus_not_alphabetical_path() {
        let plan = plan_for(&["threshold"]);
        // identical evidence, different size: the audit's 68-file lexicographic tie
        let index = candidate("00-index\\ledger.md", 90_000, 2.0, false);
        let record = candidate("zz-records\\record.md", 4_000, 2.0, false);
        let cands = vec![index, record];
        let corpus = build_corpus(100, encode_log_size(9_000) * 100, &plan, &cands);
        let ranked = rank_candidates(cands, &corpus, &plan, 3);
        assert_eq!(ranked[0].path, PathBuf::from("zz-records\\record.md"));
    }

    #[test]
    fn exclude_globs_drop_matching_paths() {
        let exclude = compile_globs(&["*.toon".to_string()]).expect("globs");
        let exts = HashSet::new();
        let none: Vec<Regex> = Vec::new();
        assert!(should_skip_file(
            Path::new("kb\\00-index\\evidence-ledger.toon"),
            &exts,
            &none,
            &exclude
        ));
        assert!(!should_skip_file(
            Path::new("kb\\failure-knowledge\\dsapi.md"),
            &exts,
            &none,
            &exclude
        ));
    }

    fn plan_expanded_for(terms: &[&str]) -> QueryPlan {
        let args = SearchArgs {
            query: terms.iter().map(|s| s.to_string()).collect(),
            paths: vec![PathBuf::from(".")],
            globs: Vec::new(),
            exclude: Vec::new(),
            ext: None,
            regex: false,
            word: false,
            no_expand: false,
            expand: false,
            and_mode: false,
            ignore_case: true,
            limit: DEFAULT_LIMIT,
            context: 0,
            max_per_file: 3,
            max_line_width: DEFAULT_MAX_LINE_WIDTH,
            max_files: DEFAULT_MAX_FILES,
            json: true,
            stats: false,
            usages: true,
            quiet: true,
        };
        build_query_plan(&args, false).expect("plan")
    }

    fn term_list(plan: &QueryPlan) -> Vec<String> {
        plan.term_res.iter().map(|(t, _)| t.clone()).collect()
    }

    #[test]
    fn segments_recover_identifiers_glued_to_punctuation() {
        assert_eq!(
            segments_of("*POST-/tenant/api/v1.0/parserecordsfromrequest*"),
            vec!["POST", "tenant", "api", "v1", "parserecordsfromrequest"]
        );
        assert_eq!(segments_of("[Service][QOS]"), vec!["Service", "QOS"]);
        assert_eq!(
            segments_of("PUT-/user/external/api/v1.0/3pskills"),
            vec!["PUT", "user", "external", "api", "v1", "3pskills"]
        );
    }

    #[test]
    fn segments_keep_short_codes_but_drop_bare_noise() {
        // Short operational codes are highly discriminative and must survive.
        for code in ["WW", "DF", "QOS", "ACE", "ICE", "TBD", "403"] {
            assert!(keep_segment(code), "{code} should be kept");
        }
        // Single characters and bare small numbers are not.
        for noise in ["x", "0", "1", "12"] {
            assert!(!keep_segment(noise), "{noise} should be dropped");
        }
    }

    #[test]
    fn punctuated_query_yields_core_terms_that_can_match() {
        let plan = plan_expanded_for(&["[Service][QOS]"]);
        let terms = term_list(&plan);
        assert!(terms.contains(&"[Service][QOS]".to_string()));
        assert!(terms.contains(&"Service".to_string()));
        assert!(terms.contains(&"QOS".to_string()));
        // The union regex must reach the bare identifier in ordinary prose.
        assert!(plan.search_re.is_match("QOS policy enforcement returns 403"));
    }

    #[test]
    fn typed_terms_outweigh_recovered_segments() {
        let plan = plan_expanded_for(&["[Service][QOS]"]);
        let terms = term_list(&plan);
        let full = terms.iter().position(|t| t == "[Service][QOS]").expect("full");
        let seg = terms.iter().position(|t| t == "QOS").expect("segment");
        assert_eq!(plan.term_weights[full], W_TERM_ORIGINAL);
        assert_eq!(plan.term_weights[seg], W_TERM_SEGMENT);
        assert!(plan.term_weights[seg] < plan.term_weights[full]);
    }

    #[test]
    fn bare_identifier_is_not_diluted_by_tiers() {
        // A single-token symbol lookup must stay a precise one-term query.
        let plan = plan_expanded_for(&["AoaiLatencyCheckpoint"]);
        assert_eq!(term_list(&plan), vec!["AoaiLatencyCheckpoint".to_string()]);
        assert!(!plan.fragments_used);
    }

    #[test]
    fn phrase_shaped_query_enables_fragments_up_front() {
        // Several typed terms means a prose/alert phrase: recall matters more than precision.
        let plan = plan_expanded_for(&["SkillsEmbeddingGenerationSwssProcessor", "timeout"]);
        assert!(plan.fragments_used);
        assert!(plan.sub_terms.iter().any(|s| s.eq_ignore_ascii_case("embedding")));
        assert!(!plan.subtoken_res.is_empty());
    }

    #[test]
    fn no_expand_searches_the_literal_string() {
        let plan = plan_for(&["*POST-/tenant/api/v1.0/parserecordsfromrequest*"]);
        assert_eq!(
            term_list(&plan),
            vec!["*POST-/tenant/api/v1.0/parserecordsfromrequest*".to_string()]
        );
        assert!(!plan.fragments_used);
        assert!(plan.search_re.is_match("*POST-/tenant/api/v1.0/parserecordsfromrequest*"));
        assert!(!plan.search_re.is_match("parserecordsfromrequest"));
    }

    #[test]
    fn regex_mode_bypasses_tokenization_entirely() {
        let args = SearchArgs {
            query: vec!["skills?(v1|v2)".to_string()],
            paths: vec![PathBuf::from(".")],
            globs: Vec::new(),
            exclude: Vec::new(),
            ext: None,
            regex: true,
            word: false,
            no_expand: false,
            expand: false,
            and_mode: false,
            ignore_case: true,
            limit: DEFAULT_LIMIT,
            context: 0,
            max_per_file: 3,
            max_line_width: DEFAULT_MAX_LINE_WIDTH,
            max_files: DEFAULT_MAX_FILES,
            json: true,
            stats: false,
            usages: true,
            quiet: true,
        };
        let plan = build_query_plan(&args, false).expect("plan");
        assert_eq!(plan.term_res.len(), 1);
        assert!(!plan.fragments_used);
        assert!(plan.sub_terms.is_empty());
        assert!(plan.search_re.is_match("skillv2"));
    }

    #[test]
    fn segment_weight_reduces_its_share_of_the_score() {
        // Same document evidence, but attributed to a recovered segment rather than a
        // typed term, must score strictly lower.
        let plan = plan_expanded_for(&["[Service][QOS]"]);
        let terms = term_list(&plan);
        let n = terms.len();
        let mut c = candidate("kb\\a.md", 2_000, 0.0, false);
        c.tf = vec![[0.0, 0.0, 0.0]; n];
        c.tf_path = vec![0.0; n];
        c.present = vec![false; n];
        c.anchor_mask = vec![false; n];
        let full = terms.iter().position(|t| t == "[Service][QOS]").unwrap();
        let seg = terms.iter().position(|t| t == "QOS").unwrap();

        let corpus = Corpus {
            n_docs: 100.0,
            avgdl: 2_000.0,
            idf: vec![2.0; n],
            idf_sub: Vec::new(),
            df: vec![5; n],
            idf_total: 2.0 * n as f64,
        };
        let mut with_full = c.clone();
        with_full.tf[full][1] = 4.0;
        with_full.present[full] = true;
        let mut with_seg = c.clone();
        with_seg.tf[seg][1] = 4.0;
        with_seg.present[seg] = true;

        let a = score_candidate(with_full, &corpus, &plan, 3).score;
        let b = score_candidate(with_seg, &corpus, &plan, 3).score;
        assert!(a > b, "typed term {a} should outscore segment {b}");
    }

    #[test]
    fn prefix_grams_recover_a_shorter_recorded_form() {
        let parts: Vec<String> = split_identifier("GetRecordsShowUIArchivedShowOrgEnabledScenario");
        let grams = prefix_grams(&parts);
        assert!(
            grams.iter().any(|g| g == "GetRecordsShowUIArchived"),
            "expected the recorded short form, got {grams:?}"
        );
        // The full token belongs to tier 1/2 and must not be duplicated here.
        assert!(!grams
            .iter()
            .any(|g| g == "GetRecordsShowUIArchivedShowOrgEnabledScenario"));
        assert!(grams.len() <= MAX_PREFIX_GRAMS);
    }

    #[test]
    fn prefix_grams_stay_off_short_and_two_part_prefixes() {
        // A 3-part name has no prefix specific enough to be worth searching.
        assert!(prefix_grams(&split_identifier("SkillTaxonomyDto")).is_empty());
        // A 2-part prefix of a longer name is too generic and cost the code plane a rank.
        let grams = prefix_grams(&split_identifier("AsyncLLMPathResult"));
        assert!(
            !grams.iter().any(|g| g == "AsyncLLM"),
            "two-part prefix must not be emitted, got {grams:?}"
        );
        for g in &grams {
            assert!(g.len() >= MIN_PREFIX_GRAM_LEN);
        }
    }

    #[test]
    fn prefix_grams_are_fragment_tier_not_core() {
        let plan = plan_expanded_for(&["GetRecordsShowUIArchivedShowOrgEnabledScenario", "canary"]);
        assert!(plan.fragments_used);
        assert!(plan
            .sub_terms
            .iter()
            .any(|s| s == "GetRecordsShowUIArchived"));
        // It is inferred, so it must never be promoted into the typed/segment core.
        assert!(!term_list(&plan)
            .iter()
            .any(|t| t == "GetRecordsShowUIArchived"));
    }
}
