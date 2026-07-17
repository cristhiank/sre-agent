# Work Store

Two tiers: a **persistent service root** (survives across runs) and a **per-run subtree**
(isolated per invocation).

## Work dir resolution (priority order)

1. `service_work_path` — an explicit, already-resolved service directory injected by the
   caller; used VERBATIM (it already includes the service id; do not append `<service-id>`
   again). Priority 1.
2. `LIVESITE_WORK_DIR` — a base work root supplied as an environment variable; resolve
   the service dir as `LIVESITE_WORK_DIR` joined with `<service-id>`. Priority 2.
3. Neither available — write no state; return a `no-work-dir` degraded receipt/abort.
   Do not guess a path.

Resolve to an absolute path before any file operation. In both cases the result is the
**resolved service dir** — the root used for all layout paths below; never append
`<service-id>` again after this resolution. Record the resolved path and the resolution
source (`service_work_path` verbatim, or `LIVESITE_WORK_DIR`+`<service-id>`) in
`run-state.toon` before advancing to COLD/WARM evaluation.

## Layout

```
<resolved-service-dir>/
  service.toml                      # identity, cache_version, last-run pointers — SERVICE ROOT ONLY
  observations.toon                 # PERSISTENT: durable cross-run memory ledger
  cache/
    schema.toon                     # discovered telemetry schema; version + freshness
    probes.toon                     # tuned probe definitions; version + freshness
    baselines.toon                  # per-signal baselines; TTL + window math + exclusions
    suppressions.toon               # conditional suppression rules
    component-map.toon              # binary-to-component attribution map
    coverage.toon                   # namespace coverage: watch_set per namespace, rotation cursor, per-run full-sweep budget, coverage status
  runs/<UTC-run-id>/
    run-state.toon                  # in-run working digest (coordinator-owned)
    stage1/
      candidates.toon               # Stage 1 output: candidates only, no verdicts
      receipts.toon                 # suppression + degraded-capability audit trail
    stage2/<candidate-id>/
      verdict.toon                  # Stage 2 output: written by the confirm subagent
      _scratch/                     # OPTIONAL: transient working files for this subagent only
    final/
      manifest.toon                 # merged cross-plane candidate index
      alert-draft.toon              # alert draft (DRAFT; not sent in v1)
```

## Canonical paths — write ONLY these paths

Hard rules. Any deviation is a layout violation.

1. **`service.toml` lives at the SERVICE ROOT only** — `<resolved-service-dir>/service.toml`.
   It MUST NOT be written into `cache/` or any other subdirectory.

2. **`cache/` holds exactly these six files and nothing else:**
   `schema.toon`, `probes.toon`, `baselines.toon`, `suppressions.toon`, `component-map.toon`,
   `coverage.toon`.
   `coverage.toon` is written by the coordinator (post-full-sweep merge), not by the
   onboarding dispatch directly. No scratch files, working queries, or intermediate outputs
   belong in `cache/`.

3. **Scratch and working query files** (coordinator or subagent, if any) go ONLY under
   `runs/<run-id>/<stage>/<candidate-id>/_scratch/`. They are transient, not part of the
   contract output, and not read by any other component.

4. **No improvised top-level directories.** Only `cache/` and `runs/` may be created under
   the service root. Directories such as `_probes/`, `tmp/`, or `scratch/` at the service
   root are forbidden.

5. **Run subtree is exactly:**
   `runs/<run-id>/run-state.toon`,
   `runs/<run-id>/stage1/{candidates,receipts}.toon`,
   `runs/<run-id>/stage2/<candidate-id>/verdict.toon` (plus optional `_scratch/`),
   `runs/<run-id>/final/{manifest,alert-draft}.toon`.
   No other files are written into a run subtree.

## Run ID format

`run-<YYYYMMDDThhmmssZ>` — normalized UTC timestamp.
Example: `run-20240315T143022Z`.

**Fresh-run-id is mandatory and unique — invariants:**
- Every invocation MUST mint a NEW run-id and create a NEW `runs/<run-id>/` directory that DID NOT EXIST before this invocation. The coordinator MUST NOT write into, resume, adopt, repair, or re-finalize any pre-existing `runs/<...>/` directory — a run owns ONLY the directory it created this invocation.
- If the minted `run-<UTC>` id collides with an existing directory (e.g. two runs starting in the same second), regenerate a unique id (e.g. append a short disambiguator such as `-a`, `-b`) until the target directory does not exist. The new run dir is ALWAYS freshly created — never pre-existing.
- A run's `window_anchor_utc` and `run_id` are established fresh at THIS invocation's start; they are NEVER inherited from a pre-existing run directory.

## Prior runs and orphaned runs

A prior `runs/<id>/` directory that lacks `final/manifest.toon` OR `final/alert-draft.toon` is INCOMPLETE/ORPHANED.

**The current run MUST NOT:**
- Adopt, finalize, or repair an orphaned run directory.
- Merge its `stage1/candidates.toon` or `stage2/` verdict files as this run's evidence.
- Write any artifact into a pre-existing run directory.

Leave orphaned directories untouched (write-isolation extends to all pre-existing run dirs). Note the orphaned directory as a process gap in the current run's own `run-state.toon` summary.

Prior-run VERDICTS and CANDIDATES are never imported as this run's fresh evidence. Cross-run continuity flows ONLY through `observations.toon` claims that are re-verified this run — never by reusing another run's verdict or candidate files. This extends the prior-observations-are-CLAIMS rule: prior run directories are stale evidence containers, not adoption targets.

Optional GC: the coordinator MAY note orphaned run dirs for cleanup in `run-state.toon` but MUST NOT delete, repair, or modify them.

## COLD vs WARM (deterministic; evaluated before any probe execution)

**COLD** — any of the following is true:
- `service.toml` is absent or fails schema parse.
- `cache/schema.toon` is absent, fails version validation, or its `cache_version` does
  not match `service.toml`.
- `cache/probes.toon` is absent, fails validation, or is stale (past its `valid_until`
  timestamp).
- `cache/baselines.toon` is absent, fails validation, or has exceeded its TTL.
- `cache/suppressions.toon` is absent or fails validation.
- `cache/component-map.toon` is absent or fails validation.
- `cache/coverage.toon` is absent, fails version validation, or its `cache_version` does
  not match `service.toml`. An old-version cache that predates `coverage.toon` (no file
  present) is unconditionally COLD.

COLD action: load `references/onboarding.md` and dispatch a first-run
telemetry-discovery capability. Do not execute Stage 1 until all required cache files
are written, validated, and version-matched.

**WARM** — all cache files are present, schema-valid, version-matched, and within their
freshness TTL.

WARM action: skip onboarding; proceed directly to Stage 1.

**Partial-cache** — applies ONLY when ALL of the following hold: (a) `cache_version`
matches across `service.toml` and all present cache files; (b) `coverage.toon` is present
and version-valid; (c) one or more of the five discovery files (`schema.toon`,
`probes.toon`, `baselines.toon`, `suppressions.toon`, `component-map.toon`) are
individually stale or absent. In this case dispatch a targeted mini-onboard for only the
stale or absent discovery files; leave fresh files untouched; re-validate all files before
Stage 1.

**EXEMPTIONS — always COLD → full rebuild, never a targeted migration:**
- Any `cache_version` mismatch on any cache file or `service.toml`.
- `coverage.toon` absent, fails validation, or is version-mismatched (pre-P5 cache or
  corrupted state).
These conditions CANNOT be resolved by a targeted re-tune — they always require a full
rebuild of all six cache files from scratch.

## Write isolation

| Owner | Writes to |
|---|---|
| Coordinator | `run-state.toon`, `final/`, `observations.toon`, `cache/coverage.toon` (post-full-sweep merge — coordinator merges per-namespace watch-set findings returned by the mini-onboard dispatch); other `cache/` files written on initial post-onboard merge only; `stage2/<candidate-id>/verdict.toon` ONLY as the tooling-blocked fallback (see `references/confirm.md`) |
| Onboarding dispatch | `service.toml` (service root); `cache/` five files (`schema.toon`, `probes.toon`, `baselines.toon`, `suppressions.toon`, `component-map.toon`); returns per-namespace watch-set findings as output receipts for the coordinator to merge into `cache/coverage.toon` |
| Confirm subagent (one per candidate) | `stage2/<candidate-id>/verdict.toon`; optionally `stage2/<candidate-id>/_scratch/` (transient only) |

No subagent writes outside its assigned subtree. No two subagents share a write target.
The coordinator writes inside `stage2/` only for the single tooling-blocked fallback
documented in `references/confirm.md`; it never modifies a subagent's completed output.

## `observations.toon` — persistent memory ledger

`observations.toon` is the durable cross-run ledger. Entries are `kind`-discriminated;
two entry types coexist in the same append-only file:

**Observation entry** (`kind: observation`):
```
kind:         observation
run_id:       <originating run-id>
obs_id:       <run-id>/OBS<NNN>   # zero-padded; stable cross-run reference
signal:       <component>/<normalized-signal-name>
verdict:      confirmed | refuted | recovered | inconclusive
summary:      <one-line finding>
valid_until:  <UTC timestamp; advisory freshness bound>
```

**Deferred-lead entry** (`kind: deferred-lead`):
```
kind:             deferred-lead
run_id:           <originating run-id>
obs_id:           <run-id>/OBS<NNN>
candidate_id:     <candidate id from stage1/candidates.toon>
confirm_priority: <priority score at time of deferral>
first_seen:       <run-id of the run in which this lead was FIRST deferred>
runs_deferred:    <count of consecutive runs this lead has been deferred without resolution; start: 1>
signal:           <cluster signature>
direction:        <spike | drop | new>
valid_until:      <UTC timestamp; advisory>
```

Prior entries are CLAIMS to re-verify each run, cited as `<run-id>/OBS###`.
Never import them as truth; re-verify before promoting to a finding.
Append-only: never modify prior entries. When a prior observation is re-verified,
emit a new entry referencing the prior `obs_id` as `confirms: <prior-obs-id>` or
`refutes: <prior-obs-id>`; do not overwrite the original.

## `coverage.toon` — namespace coverage state

`coverage.toon` is the coordinator-owned coverage manifest, written by the coordinator
after each full-sweep mini-onboard merge. Structure (TOON, host-agnostic):

Per-namespace entries:
```
namespace:            <namespace identifier>
watch_set:            [<signal-name>, ...]  # signals ever-bursted in a full sweep OR pinned high-value
last_full_sweep_utc:  <UTC timestamp>
resweep_due:          <UTC timestamp>       # risk-weighted: shorter for high-churn/high-value/sparse namespaces
coverage_status:      covered | overdue | never
pinned:               true | false          # whether this namespace is in the pinned_critical set
```

Global:
```
cursor:           <index into ordered namespace list>  # rotation position; advances only on completed sweeps
budget:           <derived count>                      # self-tuning: target fast-path latency share ÷ observed per-namespace sweep cost
pinned_critical:  [<namespace>, ...]                   # always swept every run; never skipped for budget
cache_version:    <monotonic integer matching service.toml>
```

Rules:
- `resweep_due` is risk-weighted: shorter for namespaces with recent burst activity,
  high churn, sparse coverage, or high-value/critical components; longer for
  stable/quiet namespaces. Never a hardcoded constant — derived from observed behavior.
- `budget` is self-tuning: derived from the target fast-path latency share and the
  observed per-namespace sweep cost accumulated across prior runs.
- The cursor advances only past namespaces whose full sweep completed successfully
  in the current run. A failed or partial sweep does not advance the cursor.
- `pinned_critical` is kept small; it is the safety guard for never-watched surfaces.
