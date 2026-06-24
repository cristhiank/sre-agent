# srch

`srch` is a generic live code and plain-text search CLI. It walks ignored source trees, expands identifier-like queries, ranks likely definitions above usages, groups results by file, and emits compact human or JSON output.

## Usage

```powershell
srch GetManagerSummaryStatistics --path C:\repos\sre-agent-repos\services --stats
srch GetManagerSummaryStatistics --path C:\repos\sre-agent-repos\services --usages --stats
srch DocumentDbConnector IDocumentDbConnector CosmosDb --path C:\repos\sre-agent-repos\services --json
srch --regex "OBS[0-9]{3}" --path C:\repos\sre-agent-repos\services\insights
srch index --path C:\repos\sre-agent-repos\services\insights --rebuild
srch def GetManagerSummaryStatistics
```

## Common options

- `-p, --path <DIR>`: search root; repeatable. Defaults to the current directory.
- `-g, --glob <GLOB>`: include filter; repeatable.
- `-e, --ext <CSV>`: include extensions such as `cs,ts,md`.
- `--regex`: treat the joined query as a raw regex and disable expansion.
- `-i, --ignore-case`: match without case sensitivity.
- `-w, --word`: require whole-word matches.
- `--usages`: for a single identifier with an available ctags DB, scan and rank text usages; by default `srch` prints definitions only instantly.
- `--expand`: force identifier subtoken expansion and run a full scan. By default, `srch` searches original terms first and only broadens sparse results.
- `--no-expand`: disable automatic sparse-result broadening.
- `--and`: require all original query terms in a matched file.
- `-n, --limit <N>`: max files in output. Default: 20.
- `-C, --context <N>`: context lines per match. Default: 2.
- `-m, --max-per-file <N>`: max match lines per file. Default: 3.
- `--json`: emit machine-readable JSON.
- `--max-files <N>`: abort scans past this file cap. Default: 100000.
- `--stats`: print elapsed milliseconds and file counts.

`srch index` stores native sorted ctags databases under this CLI project's `.srch\\` directory so searched repositories are not polluted.

## Build

Prerequisites: Rust/cargo and Universal Ctags (`ctags` and `readtags`) on `PATH`.

```powershell
# one-step: builds and stages the binary where the launcher's SRCH_CLI resolves it
.\build.ps1
```

Or manually:

```powershell
cargo build --release
Copy-Item target\release\srch.exe bin\srch.exe -Force
```

`cargo build --release` produces `target\release\srch.exe`; it must be copied to `bin\srch.exe`
(the path the launcher's `SRCH_CLI` resolves to — `bin\` is git-ignored, so each environment builds
its own). `srch index` builds a ctags DB under this CLI project's `.srch\` directory.

