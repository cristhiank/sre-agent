# Memory Trust Model

The safety core for all persistent and cached state. Every rerun starts with skepticism
toward prior state; trust is earned by re-verification against fresh evidence.

## Prior observations (`observations.toon`)

`observations.toon` is a `kind`-discriminated append-only union: entries are either
`kind: observation` (Stage 2 findings) or `kind: deferred-lead` (deferred candidates
carried across runs). Both share `obs_id` and `valid_until`; see the full union schema in
`references/work-store.md`. All rules below apply to both entry kinds.

Prior entries are CLAIMS and LEADS, not truth.

- Cite prior observations as `<run-id>/OBS###` in the current run's candidates and receipts.
- Every cited prior observation must be re-verified against fresh evidence in the current
  run before being promoted to a finding.
- A re-verification failure demotes a prior observation to `suspect`; append a new entry
  with the demotion referencing the original `obs_id` as `refutes: <prior-obs-id>`.
  Never modify the original entry.
- `valid_until` is an advisory freshness bound. An entry past `valid_until` is treated
  as a lead, not a finding, even if not explicitly demoted.
- Append-only ledger: the coordinator never edits or deletes prior entries.

## Deferred-lead persistence (`observations.toon` carry-forward)

When a candidate is deferred (disposition `unverified-lead`) by the confirm budget, the
coordinator appends a deferred-lead entry to `observations.toon` (schema matches the
`kind: deferred-lead` variant of the union; see `references/work-store.md`):

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

**Aging boost:** on each subsequent run, if the deferred lead reappears in Stage 1, the
coordinator adds an age bonus to its `confirm_priority` proportional to `runs_deferred`.
The bonus formula is service-configurable; the required invariant is that sufficiently
old leads overtake fresh low-priority candidates and surface through the reserved aging
slots (see `references/confirm.md`).

**Append-only:** following the ledger discipline, each deferral appends a new entry with
the updated `runs_deferred` count; prior entries are never modified. The `first_seen`
field records the original first-deferral run-id and is copied unchanged across
subsequent entries.

**Reserved aging slots:** the confirm budget reserves `confirm_budget_aging_slots` slots
for the highest-`runs_deferred` leads each run, draining the oldest first. This guarantees
any persisting deferred lead is caught within a bounded number of runs; a real regression
is never permanently deferred.

**Resolution:** when a deferred lead is eventually confirmed or cleared, append a
resolution entry to `observations.toon` referencing the original `obs_id`; do NOT modify
the original entry. After resolution, the lead is no longer carried forward.

**Stale deferred leads:** a lead that has not reappeared in Stage 1 for a configurable
number of consecutive runs is marked `stale` and removed from the aging carry-forward
pool. Its entries remain in the append-only ledger.

## Cached probes (`cache/probes.toon`)

Before executing any probe in Stage 1, validate ALL of the following:
1. `cache_version` matches `service.toml`.
2. `valid_until` is in the future.
3. Probe schema fields are present and parseable.

On any validation failure: do NOT run the stale probe.
- If `cache_version` mismatches `service.toml`: unconditionally COLD — full rebuild of
  all cache files (see Cache version and migration below); Stage 1 cannot proceed until
  the full rebuild completes.
- If `cache_version` matches but the probe is past its `valid_until` TTL or has a schema
  gap: targeted mini-onboard for this probe only; re-validate before Stage 1 proceeds.
Running a stale probe against a schema-drifted signal produces misleading burst signals
that pollute the candidate list.

## Baselines (`cache/baselines.toon`)

Each baseline entry carries:
- `window_start_utc` / `window_end_utc` — exact window used.
- `exclusion_record` — any anomaly or incident windows excluded from the baseline math.
- `valid_until` — TTL for this baseline (configurable per signal; default: 24h).
- `points_count` — number of data points in the baseline window.

Rules:
- MUST NOT include any active-incident or anomaly window in baseline math. If an
  exclusion record is absent, treat the baseline as suspect.
- A baseline past `valid_until` is stale; treat as `thin_baseline` for `data_quality`.
- The baseline window must be at least 3× the recent window to have statistical meaning;
  shorter baselines → `thin_baseline`.
- A baseline with `points_count` below the minimum materiality floor → `thin_baseline`.

## Suppressions (`cache/suppressions.toon`)

Suppressions are CONDITIONAL, not blanket. Each rule envelope defines all five conditions:
1. Component match.
2. Cluster signature match (exact or pattern).
3. Signal direction (spike / drop / new).
4. `rate_ceiling` — absolute upper bound on the recent rate.
5. `burst_ceiling` — upper bound on the burst factor (recent vs the signal's own baseline).

**Dual-ceiling escape:** a candidate ESCAPES suppression if EITHER ceiling is breached —
absolute rate exceeds `rate_ceiling`, OR burst factor exceeds `burst_ceiling`. Either
excess is sufficient. This ensures a large relative spike whose absolute rate stays within
a noisy envelope still escapes on `burst_ceiling` and is never wrongly suppressed.
Burst always wins when either ceiling is breached.

Every suppression evaluation — whether the rule fires or not — produces an auditable
receipt row in `stage1/receipts.toon`. There are no invisible drops. Burst-escape
receipts record which ceiling was breached.

A suppression rule missing EITHER `rate_ceiling` OR `burst_ceiling`, or with an
unparseable value for either, is treated as DISABLED for this run; write a
`malformed-suppression` receipt row.

## Coverage (`cache/coverage.toon`)

`coverage.toon` records per-namespace watch-set membership, rotation cursor, per-run
full-sweep budget, and coverage status. It is coordinator-owned.

**Freshness validation — before using any `watch_set`:**
- `coverage.toon` is present and `cache_version` matches `service.toml`.
- Each namespace's `watch_set` was produced by a completed full sweep
  (`coverage_status: covered`, not `overdue` or `never`).
- The `watch_set` data is within the namespace's `resweep_due` bound.

**Watch-set staleness:** a `watch_set` entry is stale if the originating full sweep is
past the namespace's `resweep_due` bound. A stale `watch_set` is a trust failure → treat
as drift → dispatch a mini-onboard for that namespace; never pull a stale watch-set as
if fresh.

**Watch-set aging:** metric eviction from the `watch_set` (age-out after successive
no-burst sweeps) is a cache-drift event. After eviction a newly-materializing signal may
be missed until the next full sweep covers it. The coverage caveat in the alert output
must state the WATCH tier's coverage scope and the oldest `resweep_due` age explicitly.

**Partial-sweep failure semantics:**
- A namespace whose full sweep was attempted but failed retains its prior `watch_set` unchanged.
- `coverage_status` is set to `overdue`; a `partial-sweep` receipt is written.
- The rotation cursor is NOT advanced past a failed namespace.
- A partial-sweep namespace is NOT reported as "covered" in the coverage caveat.

**Never-covered namespaces:** namespaces with `coverage_status: never` have no `watch_set`
and are not burst-ranked in the WATCH tier. They must be reported explicitly in the
coverage caveat — they are a gap, not a clean pass.

## Cache version and migration

`service.toml` carries a `cache_version` field. All six cache files carry a matching
`cache_version` field (including `coverage.toon`).

**Version mismatch or missing `coverage.toon` → ALWAYS full rebuild (never a targeted migration):**
ANY `cache_version` mismatch between any cache file and `service.toml`, OR `coverage.toon`
absent or version-mismatched, is unconditionally COLD: dispatch a full first-run
telemetry-discovery rebuild of ALL six cache files from scratch. A targeted mini-onboard
CANNOT resolve a version mismatch — it applies only to individually-stale files within a
fully matching `cache_version`. Do not execute Stage 1 until the full rebuild completes
and all six files pass validation at the current `cache_version`.

**Targeted mini-onboard scope:** applies ONLY when `cache_version` matches across
`service.toml` AND all present cache files, AND `coverage.toon` is present and
version-valid, AND one or more of the five discovery files are individually stale (past
their `valid_until` TTL) or absent. In that case, re-tune only the stale/absent discovery
files; leave fresh files untouched; re-validate all files before Stage 1.

`cache_version` is a monotonic integer. Increment it when probe schema, baseline math,
or coverage tracking changes in a way that makes old cached values incompatible with the
new computation. Adding the two-tier cadence (`coverage.toon`) is a version bump that
requires a full onboard to rebuild the cache.

## Degraded / blocked receipts

When a telemetry capability is unavailable or returns an error:
- Do NOT proceed silently with zero data.
- Write a `degraded-capability` receipt to `stage1/receipts.toon`, recording:
  - `capability_class` — the generic class of the unavailable capability.
  - `error_class` — the category of failure (e.g. timeout, auth, not-found). Do not copy
    raw error payloads that may contain sensitive data.
- If the degraded capability covers a required signal plane, mark affected candidates
  as `data_quality: empty-ambiguous`.
- If ALL signal-plane capabilities are degraded → write a `run-blocked` receipt and
  terminate Stage 1 cleanly. Do not emit spurious zero-evidence candidates.

## Honesty floor

- Never claim "no anomaly" when a capability was unavailable for part of the sweep.
- Never claim a burst is "confirmed" without a Stage 2 `confirmed` verdict.
- Never suppress a burst silently.
- Never import prior observations as truth without re-verification.
- Always make gaps explicit; prefer an explicit `inconclusive` over an unsupported finding.
- Confidence is graded (`high | medium | low`); strong claims require gate-passed evidence.
