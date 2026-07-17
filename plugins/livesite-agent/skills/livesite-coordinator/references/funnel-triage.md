# Stage 1 — Funnel Triage

Stage 1 is INLINE and MECHANICAL. It transforms cached/tuned probe output into a ranked
candidate list. It produces CANDIDATES ONLY — no verdicts, no severity labels.

## Inputs

- `cache/probes.toon` — tuned probe definitions (validated WARM before Stage 1 begins).
- `cache/baselines.toon` — per-signal baselines (freshness-checked; treat past `valid_until`
  as `thin_baseline`).
- `cache/suppressions.toon` — conditional suppression rules.
- `cache/component-map.toon` — binary-to-component attribution.
- **Bounded recent window** — fixed UTC anchor set at Stage 1 start; recorded as
  `window_anchor_utc` in `run-state.toon` before the first probe execution. Never open-ended;
  never allowed to drift between probe executions within the same run.

## Two-tier metrics: WATCH tier (inline) + FULL rotating resweep (dispatched)

Stage 1 runs metrics as two tiers concurrently:

**WATCH tier — inline (this stage):**
Pull ONLY the `watch_set` signals of all covered namespaces from `cache/coverage.toon`,
PLUS the `watch_set` signals of all namespaces in the `pinned_critical` set (pinned
namespaces contribute their watch-set signals, not the namespace identifiers). Burst-rank
these signals using the algorithm below. This is the fast path and runs inline.

Inline preconditions — ALL must hold before pulling the watch-set:
- `cache/coverage.toon` is present, schema-valid, `cache_version`-matched, and within
  its freshness TTL.
- Each covered namespace has a non-empty `watch_set` produced by a completed full sweep
  (`coverage_status: covered`).
- The probe shape for watched signals is unchanged since the last full sweep
  (version-matched probes).
- The total watch-set pull is estimated to stay within the fast-path latency budget
  (as recorded in `coverage.toon`).

If any precondition fails: treat as drift for the affected namespace(s) → dispatch a
targeted mini-onboard for those namespaces (load `references/onboarding.md`); do NOT
pull a stale watch-set as if fresh.

**FULL rotating resweep — dispatched (this run, bounded):**
In parallel with the WATCH-tier burst ranking, trigger a bounded dispatched mini-onboard
for the namespace resweep (scope from `cache/coverage.toon`): namespaces whose
`resweep_due` has passed (overdue) plus the next uncovered namespaces per the rotation
cursor, within the per-run `budget`. This dispatch does not block Stage 1 WATCH-tier
candidates. The coordinator merges the mini-onboard's per-namespace watch-set findings
into `coverage.toon` after the full-sweep dispatch completes.

## Burst ranking (the primary filter)

Rank signals by BURST: compare each signal's recent value against that signal's OWN prior
baseline. Burst is always signal-relative — never compare across different signals and never
apply a global threshold.

Burst direction:
- `spike` — recent rate or count significantly exceeds the signal's own baseline
  (burst factor ≥ the tuned threshold from `probes.toon`).
- `drop` — recent rate or count significantly below the signal's own baseline p50.
- `new` — signal has no prior baseline presence and recent count exceeds the minimum
  materiality floor.

**Do NOT filter by error or metric name keywords.** Signal names are opaque identifiers;
burst math is the filter.

## De-fragment / cluster

Raw errors often emit many variants of the same root signal (trace IDs, request GUIDs,
numeric instance names embedded in messages). Before ranking:

1. Strip volatile identifiers (GUIDs, numeric ids, embedded timestamps) from error text.
2. Bucket by emitter + normalized message prefix.
3. Assign a cluster signature: `<component>/<normalized-prefix>` — no volatile ids.

Each candidate's `signal` field uses the cluster signature, not a raw log line.

## Conditional suppressions

Read `cache/suppressions.toon`. A suppression fires ONLY when ALL five conditions in the
rule envelope match the candidate:

1. Component matches.
2. Cluster signature matches the rule's signature pattern.
3. Signal direction (spike / drop / new) matches.
4. Recent absolute rate ≤ the rule's `rate_ceiling`.
5. Burst factor ≤ the rule's `burst_ceiling` (recent vs the signal's own baseline).

**Dual-ceiling escape:** a candidate ESCAPES suppression if EITHER ceiling is breached —
absolute rate exceeds `rate_ceiling`, OR burst factor exceeds `burst_ceiling`. Either
excess is sufficient. A large relative spike whose absolute rate stays within a noisy
envelope still escapes on `burst_ceiling` and is never wrongly suppressed. Burst always
wins when either ceiling is breached.

Every suppression evaluation — whether the rule fires or not — produces a row in
`stage1/receipts.toon`. There are no silent drops. Burst-escape receipts record which
ceiling was breached and by how much.

A suppression rule missing EITHER `rate_ceiling` OR `burst_ceiling`, or with an
unparseable value for either, is treated as DISABLED for this run; write a
`malformed-suppression` receipt row.

## Component attribution

Use `cache/component-map.toon` to assign each candidate to a component (binary, role, or
partition). A service is many components; cross-component cascades are materially different
from single-component bursts — record related cross-component candidates in
`related_candidates`. If a candidate's emitter cannot be attributed, mark
`component: unknown` and note it in receipts.

## Confirm-priority scoring

Stage 1 computes a `confirm_priority` score for each candidate at hand-off time. This
score is DISTINCT from `triage_order` (raw burst-rank by burst factor alone).
`confirm_priority` is the deterministic blend used by Stage 2 to select candidates within
the confirm budget.

The blend (higher weight = higher priority):
1. **Burst factor** — higher burst factor raises priority (primary weight).
2. **Direction** — `new` and `spike` weighted above `drop`; a fresh-appearing signal or
   rate surge is more likely to be a release regression than a rate drop.
3. **Blast radius** — wider scale-unit or version spread raises priority.
4. **Data quality** — `data_quality: ok` ranked above `thin_baseline` or
   `partial_coverage`.

The score is computed deterministically from Stage 1 outputs; the coordinator records it
in the candidate at hand-off time and does not recompute it at selection. The numeric
weights are configurable per service (`service.toml`); the factor order above is fixed.

## Outputs

`stage1/candidates.toon` — one entry per unsuppressed candidate. Full schema:
`references/candidate-contract.md`.

`stage1/receipts.toon` — one entry per evaluated suppression or degraded-capability event:

```
type:                  suppression-evaluated | degraded-capability | malformed-suppression
candidate_signal:      <cluster signature>
suppression_rule_id:   <rule id from suppressions.toon; or "n/a" for capability events>
fired:                 true | false
burst_escape:          none | rate_ceiling | burst_ceiling | both  # "none" when fired: true
reason:                <why fired or did not fire; which ceiling was breached and by how much if escaped>
suppressed_candidate:  <full candidate record, only when fired: true>
```

## Candidates-only discipline

Stage 1 candidates are OBSERVATIONS, not conclusions. See the allowed vs forbidden table
in `references/candidate-contract.md`. The `why_candidate` field states what was observed
mechanically; it never states a verdict, root cause, or severity.

## Window anchor discipline

Record `window_anchor_utc` in `run-state.toon` before executing the first probe. All
probes in Stage 1 use this same fixed anchor. Do not re-anchor mid-stage.
