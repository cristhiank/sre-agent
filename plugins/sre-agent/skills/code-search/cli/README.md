# srch

`srch` is a generic live code and plain-text search CLI. It walks ignored source trees, expands identifier-like queries, ranks results with a BM25F scoring model, groups results by file, and emits compact human or JSON output.

## Usage

```powershell
srch GetAccountSummaryStatistics --path C:\repos\sre-agent-repos\services --stats
srch GetAccountSummaryStatistics --path C:\repos\sre-agent-repos\services --usages --stats
srch DocumentDbConnector IDocumentDbConnector CosmosDb --path C:\repos\sre-agent-repos\services --json
srch --regex "OBS[0-9]{3}" --path C:\repos\sre-agent-repos\services\insights
srch index --path C:\repos\sre-agent-repos\services\insights --rebuild
srch def GetAccountSummaryStatistics
```

## Common options

- `-p, --path <DIR>`: search root; repeatable. Defaults to the current directory.
- `-g, --glob <GLOB>`: include filter; repeatable.
- `-x, --exclude <GLOB>`: exclude filter; repeatable. Applied after `--glob`/`--ext`. Use it to drop bulk generated or append-only files (`-x "*.min.js" -x "*-ledger.*"`) that would otherwise take result slots.
- `-e, --ext <CSV>`: include extensions such as `cs,ts,md`.
- `--regex`: treat the joined query as a raw regex and disable all tokenization and expansion.
- `-i, --ignore-case`: match without case sensitivity.
- `-w, --word`: require whole-word matches.
- `--usages`: for a single identifier with an available ctags DB, scan and rank text usages; by default `srch` prints definitions only instantly.
- `--expand`: force fragment expansion and run a full scan.
- `--no-expand`: search only the terms exactly as typed — no segments, no fragments, no sparse-result broadening. This is how you search a literal punctuated string.
- `--and`: require all typed query terms in a matched file; recovered segments stay optional.
- `-n, --limit <N>`: max files in output. Default: 20.
- `-C, --context <N>`: context lines per match. Default: 2.
- `-m, --max-per-file <N>`: max match lines per file. Default: 3.
- `--max-line-width <N>`: clip emitted match and context lines to `N` characters around the hit; `0` disables clipping. Default: 400. Without it a single minified or table-packed line can be tens of thousands of characters and `-C` becomes unusable.
- `--json`: emit machine-readable JSON.
- `--max-files <N>`: abort scans past this file cap. Default: 100000.
- `--stats`: print elapsed milliseconds and file counts; in `--json` mode it also emits a `ranking` object with the corpus size, the average document length used for normalization, and each term's document frequency and IDF.

`srch index` stores native sorted ctags databases under this CLI project's `.srch\\` directory so searched repositories are not polluted.

## Query terms

Real queries are often pasted verbatim from an alert, a stack trace or a log
line, so the discriminative identifier arrives glued to punctuation
(`*POST-/tenant/api/v1.0/parserecordsfromrequest*`, `[Service][QOS]`). Whole-token
matching alone would never reach it and the search returns nothing. `srch`
therefore expands each query into three tiers, each weighted differently:

| Tier | Produced by | Weight | Always on |
|---|---|---|---|
| 1. Typed | the terms exactly as given | 1.0 | yes |
| 2. Segment | maximal `[A-Za-z0-9_]` runs inside a typed term | 0.6 | unless `--no-expand` |
| 3. Fragment | camelCase / `_` / `.` / `-` splits of tiers 1–2, ≥4 chars, minus stopwords, plus cumulative ≥3-part prefixes of compound names | 0.3 (`W_subtoken`) | phrase-shaped queries, `--expand`, or a sparse first pass |

Tiers are **OR-ed**, not AND-ed: requiring every term of a 12-token alert title
guarantees zero results. IDF then decides what actually mattered — a segment
like `Service` (df 90, idf 0.39) contributes almost nothing while `QOS`
(df 16, idf 2.09) dominates. Run `--stats --json` to see the exact df/idf split.

Generated names — scenarios, monitors, jobs, test cases — are built by appending
qualifiers, so a corpus often records a *shorter* form of the name the caller
pastes. Tier 3 therefore also emits cumulative prefixes of a compound
identifier: `GetRecordsShowUIArchivedShowOrgEnabledScenario` yields
`GetRecordsShowUIArchived`. Prefixes start at three parts and eight characters
(a two-part prefix such as `AsyncLLM` matches every neighbouring type) and are
capped per identifier.

Segments are deliberately **not** length-filtered: short operational codes
(`WW`, `DF`, `QOS`, `ACE`, `TBD`, `403`) are highly discriminative. Only single
characters and bare one/two-digit numbers are dropped. Fragments *are* filtered,
because they are guesses rather than things the caller typed.

A query is "phrase-shaped" when it has two or more typed terms or yields two or
more segments; a bare identifier stays a precise single-term lookup and only
broadens if the first pass comes back sparse.

- `--no-expand` keeps tier 1 only. That is how you search a literal punctuated
  string.
- `--regex` bypasses tokenization completely: one opaque term, no tiers, no
  anchors.
- `--and` requires every **typed** term; recovered segments stay optional.

## Ranking

Files are scored with BM25F over four zones, so rare terms, focused files and
declaration-shaped lines win. All statistics come from the same walk that
collects matches — there is no prose index to build or refresh.

```
score(file) = Σ_term  w_term · idf(term) · saturate( W_head·tf_head/norm_head
                                                   + W_body·tf_body/norm_body
                                                   + W_comment·tf_comment/norm_body
                                                   + W_path·tf_path )
            + Σ_subtoken  0.3 · idf · saturate(tf/norm_body)
            + 2.00 · (share of query IDF mass present in the file)
            + 0.75 · (weighted mean IDF of terms that appear in a definition-shaped line)
            × 0.85 if the path looks like test/spec
```

- `w_term` is the tier weight above: 1.0 for a term the caller typed, 0.6 for one
  recovered from inside it.
- `idf = ln(1 + (N − df + 0.5)/(df + 0.5))`, `N` = files scanned in this run.
- `saturate(x) = x(k1+1)/(k1+x)` with `k1 = 1.2`, so repetition has diminishing returns.
- `norm = 1 − b + b·(size / avgdl)` with `b = 0.75` for body and `0.5` for headings; `avgdl` is the **geometric** mean file size, which is robust to a few huge blobs in a tree.
- Zone weights: heading 2.4, body 1.0, comment 0.55, path 3.0.
- Ties break on smaller file first, then path — never on path alone.

Line zones and "definition-shaped" are decided per file class, because they are
syntax, not semantics. A matched line is scored in the **head** zone when it is
definition-shaped, the **comment** zone when it is a comment, otherwise **body**:

| Class | Extensions | Comment zone | Definition-shaped (head zone) line |
|---|---|---|---|
| Prose | `md`, `markdown`, `mdx`, `rst`, `txt`, `adoc` | `<!-- … -->` only | a heading naming the term, `term:` / `term —` / `term --` opening a line, or a table row whose first cell is the term |
| Structured | `toon`, `yaml`, `yml`, `toml`, `ini`, `json`, `cfg`, `env` | `#`, `;`, `//` | the term in **key** position at the start of a line |
| Code | everything else | `//`, `#`, `;`, `--`, `/*`, `*` | the term in a **naming** position: right after a declaring keyword, opening the line bound with `:`/`=`, the callable being declared, or the member declared after its type |

A leading `#` is a Markdown heading, not a comment, so headings are boosted
rather than penalized. Conversely a `key: value` line only counts as a
definition in a structured file when the term is the key, and `new Foo()` or
`void Bar(Foo f)` do not count as definitions of `Foo` in code.

## Build

Rust/cargo is required only on machines that build a release. Machines consuming
a frozen release use the prebuilt `bin\srch.exe` and do not need Rust.
Universal Ctags (`ctags` and `readtags`) is optional and needed only for
`srch index`; ordinary search and fallback definition lookup work without it.

```powershell
# one-step: builds and stages the binary where the launcher's SRCH_CLI resolves it
.\build.ps1
```

Or manually:

```powershell
cargo build --release
Copy-Item target\release\srch.exe bin\srch.exe -Force
```

`cargo build --release` produces `target\release\srch.exe`; it must be copied to
`bin\srch.exe` (the path `SRCH_CLI` resolves to). `bin\` is git-ignored, so the
release builder stages it into each frozen release. `srch index` builds a ctags
DB under this CLI project's `.srch\` directory.

## Tests and benchmarks

```powershell
cargo test                 # unit tests, including the ranking model
.\bench\run-bench.ps1 -h   # A/B ranking harness, see bench\README.md
```

## Build

Rust/cargo is required only on machines that build a release. Machines consuming
a frozen release use the prebuilt `bin\srch.exe` and do not need Rust.
Universal Ctags (`ctags` and `readtags`) is optional and needed only for
`srch index`; ordinary search and fallback definition lookup work without it.

```powershell
# one-step: builds and stages the binary where the launcher's SRCH_CLI resolves it
.\build.ps1
```

Or manually:

```powershell
cargo build --release
Copy-Item target\release\srch.exe bin\srch.exe -Force
```

`cargo build --release` produces `target\release\srch.exe`; it must be copied to
`bin\srch.exe` (the path `SRCH_CLI` resolves to). `bin\` is git-ignored, so the
release builder stages it into each frozen release. `srch index` builds a ctags
DB under this CLI project's `.srch\` directory.
