# srch benchmarks

Two reproducible harnesses. Both are corpus-agnostic: the queries, the corpus
and the "good result" / "noise" patterns are all parameters, so the same scripts
can measure any tree without carrying knowledge of it.

## `run-bench.ps1` — A/B ranking quality

Runs a query list against one or more binaries and reports, per query, the rank
of the first result matching a `-TargetPattern`, the share of the top-N slots
taken by `-NoisePattern` results, and latency (best of `-Repeat` warm runs).

```powershell
.\run-bench.ps1 `
  -Exe 'old=C:\tmp\srch-old.exe','new=..\bin\srch.exe' `
  -Root  <corpus-root> `
  -QueryFile .\queries-sample.txt `
  -TargetPattern '^docs[\\/]' `
  -NoisePattern  '\.log$','-ledger\.'
```

Summary line per binary: `target@1`, `target@3`, `found`, `MRR`, `noise`
(fraction of all emitted slots matching a noise pattern), `wall_avg`, `scan_avg`.

Use it with prose-shaped queries — whole alert titles, sentences, error strings —
not just single identifiers; that is where ranking differences show up.

## `run-code-regression.ps1` — definition-ranking gate

Asserts that a known symbol's defining file still ranks at or above `max_rank`.
It passes `--usages` so the ctags fast path cannot mask a ranking regression.
Exits non-zero on any failure, so it can gate a change to ranking defaults.

```powershell
.\run-code-regression.ps1 -Exe ..\bin\srch.exe -Root <code-root> -CasesFile .\cases-sample.csv
```

`cases-sample.csv` columns: `query,expect,max_rank`. `expect` is a .NET regex
matched against the full result path; anchor it with `$` so a same-named file in
another directory cannot satisfy the case.

## Conventions

- Point `-Root` at a scoped subtree, not a multi-gigabyte repository root; both
  harnesses run every query serially and a huge root makes them unusable.
- Keep real query sets and real corpus paths **outside** this repository; pass
  them with `-QueryFile` / `-CasesFile`. The committed samples are placeholders.
- Output lands in `bench\out\` (git-ignored) as timestamped `.json` and `.md`.
