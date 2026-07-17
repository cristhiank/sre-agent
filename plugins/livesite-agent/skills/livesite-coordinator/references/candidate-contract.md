# Candidate Contract

The Stage 1 → Stage 2 hand-off schema. Host-agnostic; no concrete tool, cluster, product,
or table names appear here.

## Per-candidate fields

| Field | Type | Description |
|---|---|---|
| `id` | string | Stable candidate id for this run: `cand-<NNN>` (zero-padded). |
| `triage_order` | int | Burst-rank order from Stage 1. Ordering only — NOT severity. |
| `confirm_priority` | float | Deterministic confirm-priority score (burst × direction × blast-radius × data_quality blend). See `references/funnel-triage.md`. Distinct from `triage_order`. |
| `plane` | enum | `logs` or `metrics`. |
| `service` | string | Service id (from invocation). |
| `component` | string | Component from `cache/component-map.toon`; `unknown` if unresolved. |
| `signal` | string | Cluster signature: `<component>/<normalized-prefix>`. No volatile ids. |
| `direction` | enum | `spike` / `drop` / `new`. |
| `deviation` | object | See Deviation by direction below. |
| `observed_scope` | string | Window description: `last <N> minutes from <window_anchor_utc>`. |
| `data_quality` | enum | `ok` / `thin_baseline` / `empty-ambiguous` / `partial_coverage`. |
| `why_candidate` | string | Observation statement only. See candidates-only discipline. |
| `evidence_locator` | object | Deterministic re-pull recipe. See Evidence locator below. |
| `suggested_checks` | list[string] | Routing hints for Stage 2. Not conclusions. |
| `related_candidates` | list[string] | Mechanical links to other `cand-<NNN>` ids (same burst window, same component, or correlated direction). |

## Deviation by direction

**spike:**
```
burst_factor:      <Nx vs the signal's own baseline>
recent_value:      <aggregated value in the observed window>
baseline_value:    <baseline p50 or mean>
baseline_points:   <number of points in the baseline window>
baseline_note:     <if contaminated: describe and state recommended anchor>
```

**drop:**
```
recent_value:      <aggregated value in the observed window>
baseline_p50:      <baseline median>
drop_pct:          <percentage below baseline p50>
```

**new:**
```
first_seen:        <UTC timestamp of first occurrence in the observed window>
recent_count:      <count in the observed window>
```

## Evidence locator

A deterministic re-pull recipe sufficient for Stage 2 to execute independently without
any Stage 1 state.

```
capability_class:   <generic class, e.g. "bounded read-only telemetry log-query capability">
source_handle:      <host-local routing handle; concrete and deterministic is preferred>
signal:             <cluster signature (same as candidate.signal)>
sampling_shape:     <time bucket size and aggregation type>
window_anchor_utc:  <fixed UTC anchor — same as the run's window_anchor_utc>
recent_window:      <duration, e.g. "30m">
baseline_window:    <duration, e.g. "7d">
repull_recipe:      <CAPABILITY-EXECUTABLE INVOCATION, window-parameterized — see hard requirement below>
```

`source_handle` — the work-store is HOST-LOCAL RUNTIME STATE, so a concrete,
deterministic, host-local routing handle IS allowed and PREFERRED here; it produces
deterministic re-pulls. The host-agnosticism rule binds the SKILL text, NOT the runtime
artifacts a run writes. A concrete handle in a written candidate is not a violation.

`repull_recipe` — **HARD REQUIREMENT**: this field MUST be a capability-executable
invocation runnable verbatim by a confirm subagent. It is the exact command/query in
the form the capability accepts: fully wrapped, all parameters filled, with the time
window expressed as a single substitutable placeholder. The recipe is
**window-parameterized**: the confirm subagent executes it verbatim for BOTH the fresh
re-pull (substituting `recent_window`) AND the chronic long-window first-seen check
(substituting the long-window value, e.g. `baseline_window × 3`). The coordinator
(which already knows the concrete capability syntax) writes the ready-to-run recipe at
hand-off time; the confirm subagent substitutes only the window value — it does NOT
re-derive capability syntax. A `repull_recipe` that is raw query text not yet wrapped
in the capability's required invocation form is a malformed hand-off.

## Candidates-only discipline

Stage 1 candidates are OBSERVATIONS, not conclusions. The `why_candidate` field is
strictly scoped to mechanical observation:

| Allowed — observation | Forbidden — verdict |
|---|---|
| "Signal rate is Nx above its own baseline in the observed window." | "This is causing the issue." |
| "Signal has no prior baseline presence; first appeared N minutes ago." | "This is the root cause." |
| "Signal burst exceeds all matching suppression rule envelopes." | "Severity is critical." |
| "Signal is co-located in time with cand-002 (same component)." | "These two signals confirm each other." |
| "Data quality is thin_baseline: only K baseline points." | "Insufficient evidence; refuted." |
| "Component attribution: component-A." | "Component-A is responsible." |

Verdicts are Stage 2 property. Stage 1 candidates carry only observations, mechanical
links, and re-pull recipes.

## Learned refinements

**Refinement 1 — Incident longer than the baseline window:**
If an ongoing incident has been active longer than the baseline window, the baseline is
contaminated: it incorporates the incident itself, deflating the burst factor. In this case:
- Do NOT anchor burst to a contaminated baseline.
- Anchor to a CLEAN pre-incident baseline (a window fully before the incident start), or
  apply an absolute floor (e.g. "signal was absent for N days prior").
- Record the contamination evidence and anchor choice in `deviation.baseline_note` (Stage 1)
  and `verdict.toon baseline_note` (Stage 2).

**Refinement 2 — Chronic-vs-fresh check (mandatory before `confirmed`):**
A chronic signal (continuously present for weeks or months) can produce a burst against a
short baseline even when nothing changed. Stage 2 MUST execute a long-window first-seen
pull before issuing any `confirmed` verdict. If the fresh re-pull does not re-confirm the
burst (signal cleared or normalized), the verdict is `recovered` or `refuted` WITHOUT a
chronic pull (cheap early exit — see `references/confirm.md`). The chronic check is
required only when the fresh re-pull still shows the burst. A signal continuously present
for longer than the baseline window is a chronic condition, not a fresh regression. Emit
`inconclusive` or `recovered`; do not confirm as a release regression without a clean
pre-incident anchor.
