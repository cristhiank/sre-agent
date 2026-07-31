---
name: code-search
description: >-
  Fast ranked code-and-text search and symbol-definition lookup across local source trees and
  knowledge files, as a lower-friction alternative to raw grep. Use when you need to find where a
  symbol (class, function, method, type, config key) is defined, find its usages, locate a concept
  across many files, or run a scoped ranked search that surfaces the likely definition first and
  groups results by file instead of returning a flat wall of line matches. Good for code-symbol
  lookups (CamelCase/snake_case/dotted identifiers), multi-term OR queries, and searching both
  source code and plain-text/markdown knowledge. Invokes the `srch` CLI (a ripgrep-core walker plus
  a universal-ctags symbol index). Boundary: read-only; generic text+code only with no
  knowledge-base schema awareness; results are evidence to verify by opening the cited file:line,
  not authority; indexed definition results reflect the last index build, so treat them as
  possibly stale against the working tree.
---

# Code Search

Locate code and knowledge faster than reformulating greps: one ranked, scoped, grouped query that
puts the likely **definition** first, plus an indexed symbol lookup that answers "where is X
defined?" without scanning files.

Invoke the `srch` CLI. Use the absolute path in `SRCH_CLI` when set; otherwise use `srch` on PATH
or the built `srch` binary under this skill's `cli/bin/`.

Honesty floor: read-only. Every result is a candidate to confirm by opening the cited `path:line`;
a text-search hit is evidence, not proof. Indexed lookups reflect the last `srch index` and may lag
the working tree — say so when it matters, and rebuild when in doubt.

## When to use

- "Where is `<Symbol>` defined?" → `srch def` (indexed, no file scan, fastest).
- "Find `<Symbol>` / a concept across the tree" → `srch <query>` (ranked, grouped).
- "Show usages of `<Symbol>`" → `srch <Symbol> --usages`.
- A multi-term OR hunt (e.g. an interface and its impls) → `srch A B C`.
- Prefer this over hand-written `A|IA|AImpl` grep alternations and over paging through unranked
  match walls.

Not for: mutating files, structured queries over a specific KB schema, or true ad-hoc regex over a
huge tree where a single `rg` is simpler (use `--regex`, or just grep).

## Commands

```
srch def <Symbol> [-p DIR]…              # indexed definition lookup (fastest; needs an index)
srch index -p <DIR> [--rebuild]          # build/refresh the ctags symbol DB for a root
srch <query…> [-p DIR] [-e cs,ts,md]     # ranked search; single indexed symbol → defs only
srch <Symbol> --usages [-p DIR]          # …add the scoped live usage scan
srch <query…> -x "*-ledger.*" -x "*.min.js"   # drop bulk/append-only files from the results
srch "<alert title as pasted>" [-p DIR]  # punctuation-split into terms; ranked by rarity
srch <query…> --no-expand                # literal: only the terms exactly as typed
srch <query…> --regex "<re>"             # raw regex, no tokenization or expansion
srch <query…> --json | --stats           # machine output / timing + files scanned|matched
```

Scope every search with `-p/--path` (and `-e/--ext` or `-g/--glob`) — see operating notes.
Full flag list: `cli/README.md`.

## Operating notes

- **Single-symbol search is definition-first.** With an index covering the scope, `srch <Symbol>`
  returns the definition(s) instantly and does **not** scan for usages; add `--usages` to extend.
  Multi-term, `--regex`, and `--expand` queries always run the live scan.
- **Whole-tree code scans are slow by environment, not by tool.** File opens are scanned by host
  endpoint protection (~1 ms/file), so an unscoped scan of a large code tree takes tens of seconds
  for `srch`, `rg`, or any scanner alike. Prefer `srch def` (indexed, opens no files) or scope
  tightly with `--path`/`--ext`. Small text/knowledge trees scan instantly.
- **Index freshness.** `srch def` and the single-symbol fast path read the last `srch index`; after
  source/submodule advances, `srch index --rebuild` for the affected root. Live searches
  (`--usages`, multi-term, regex) always reflect the current working tree.
- **Paste alert titles verbatim.** Query terms are split on punctuation, so a discriminative
  identifier glued to scaffolding still matches: `[Service][QOS]` searches `Service` and `QOS`,
  `*POST-/tenant/api/v1.0/parserecordsfromrequest*` reaches `parserecordsfromrequest`. Terms are
  OR-ed and ranked by IDF — common fragments cost nothing, rare ones decide the order. Short
  operational codes (`NA`, `EU`, `TBD`, `403`) are kept. Use `--no-expand` to search the literal
  punctuated string, or `--regex` for a raw pattern.
- **Ranking is BM25F** over heading / body / comment / path zones: rare query terms outweigh common
  ones, repetition saturates, and long files are length-normalized so a large aggregate cannot win
  on volume. Terms the caller typed outweigh terms recovered from inside them. A definition-shaped
  line adds a bonus scaled by how rare its term is. Test paths are
  mildly demoted. Definition shape and comment markers are read **per file type** — in Markdown a
  leading `#` is a heading (boosted), while in code it is a comment (demoted). `--stats --json`
  reports each term's document frequency and IDF.
- **Write for retrieval.** In prose, put the identifying term in a heading or at the start of its
  line, keep one subject per file, and use the rare exact token (an ID, a processor name) rather
  than a generic paraphrase. Long append-only ledgers rank poorly by design; `-x` removes them.
- **Long lines are clipped** to `--max-line-width` (default 400) around the hit, so `-C` context
  stays readable on minified or table-packed files; `--max-line-width 0` restores full lines.
- **Smart case** by default (case-insensitive unless the query has an uppercase letter); `-i` forces
  insensitive. Common junk (binaries, build/output dirs, generated assets) is excluded; a
  `--max-files` cap aborts an accidental giant scan with guidance.

## Index hygiene

Symbol databases are written under the CLI's own `.srch/` directory, never inside the searched
repositories. Rebuild per searched root after its sources advance; one DB per root, matched to the
search scope by stored root metadata.
