---
name: livesite-coordinator
description: >-
  Fast breadth-first livesite monitoring coordinator. Use for "monitor livesite",
  "watch a service for regressions", "livesite sweep", "detect release regression",
  "earliest signal of an issue", "triage telemetry anomalies", "livesite monitoring",
  "is something wrong with the service", "catch a release regression early",
  "scan telemetry for anomalies", "alert on service degradation",
  "sweep live telemetry for a service", "binary issue detection",
  "proactive service health check". Runs one bounded sweep per service: ranks burst
  anomalies, clusters error signatures, dispatches per-candidate confirm/refute subagents,
  gates alert-worthiness, and emits an alert draft. NOT a deep root-cause agent.
  Read-only; host-agnostic capability classes; single-agent-per-service.
---

# Livesite Coordinator

<goal>
Execute one bounded livesite sweep for a target service (service id + work path injected
at invocation). Success: a structured `final/alert-draft.toon` with evidence-cited,
confirm-stage-vetted candidates — or a clean-sweep receipt if no candidate clears the
alert gate — plus `observations.toon` updated with durable findings. A valid run produces:
a resolved work dir, a COLD/WARM decision, Stage 1 candidates, Stage 2 verdicts, a merged
manifest, and either an alert draft or a clean-sweep receipt.
</goal>

<coordinator_contract>
The coordinator is ORCHESTRATION-ONLY. The inline/dispatch boundary is strict.

INLINE (coordinator may do directly):
- Resolve the work dir; determine COLD/WARM; load and validate run state.
- Execute already-cached/tuned probes over bounded windows (read-only).
- Deterministic burst ranking + de-fragment/cluster math.
- Apply conditional suppressions from cache; emit candidates and receipts.
- Merge Stage 2 verdicts; apply the alert-worthiness gate.
- Write `run-state.toon`, `final/manifest.toon`, `final/alert-draft.toon`.
- Append durable findings to `observations.toon`.

DISPATCH (must delegate — never inline):
- COLD path: first-run telemetry discovery + probe tuning — dispatch to a bounded
  read-only telemetry-discovery capability; load `references/onboarding.md` when COLD.
- Stage 2: per-candidate confirm/refute — dispatch one confirm subagent per candidate
  in parallel; each subagent owns only its own `stage2/<candidate-id>/` directory.

The coordinator MUST NOT collect raw telemetry, perform schema discovery, or run
hypothesis analysis inline. Those are dispatch obligations. Any action not in the
INLINE list above is a dispatch obligation.
</coordinator_contract>

<model_tier>
Dispatched subagents are selected by model CLASS from the lattice
`economical | mid | reasoning-heavy`. Always resolve to the NEWEST STABLE generation the
dispatch tool advertises within the chosen class — never a hardcoded model name.

Onboarding and Stage-2 confirm subagents have `minimum_allowed_class: mid` — a HARD
FLOOR. The economical class is BELOW this floor and MUST NOT be used for these dispatches.
Escalation to reasoning-heavy is allowed when causal or contradiction reasoning is needed;
downgrade below mid is never allowed. A missing or ambiguous class resolution FAILS SAFE
UPWARD (never down to economical). The coordinator MUST select the model class per the
`minimum_allowed_class: mid` floor and set it EXPLICITLY WHEN THE DISPATCH MECHANISM
EXPOSES A MODEL/CLASS SELECTOR. WHEN the host dispatch mechanism does NOT expose an
explicit class/model selector, the coordinator MUST (a) use the strongest/default dispatch
tier the host provides (which must be at least the mid floor — never an
economical/cheapest tier if one is selectable), and (b) RECORD in `run-state.toon` the
resolution basis (e.g. `"host exposes no explicit class selector; used host default
dispatch tier, treated as >= mid floor"`). A missing or ambiguous class resolution when
a selector IS available fails safe upward (never down to economical). Record the chosen
class and resolution basis in `run-state.toon` per dispatch.
Detail: `references/onboarding.md` (onboarding floor) and `references/confirm.md` (confirm floor).
</model_tier>

<work_store>
Resolve the service work dir (priority order):
1. `service_work_path` — an explicit, already-resolved service directory injected by the
   caller; used VERBATIM (it already includes the service id). Priority 1.
2. `LIVESITE_WORK_DIR` — a base work root (env var); resolve as `LIVESITE_WORK_DIR`
   joined with `<service-id>`. Priority 2.
3. Neither available — write no state and return a `no-work-dir` degraded receipt/abort;
   do not guess a path.

COLD/WARM decision (evaluated deterministically before any probe execution):
- COLD = ANY required artifact is missing, invalid, or stale-by-version among
  `service.toml` and all six cache files (`schema.toon`, `probes.toon`,
  `baselines.toon`, `suppressions.toon`, `component-map.toon`, `coverage.toon`) → **ONBOARD-FIRST**:
  (1) dispatch onboarding (load `references/onboarding.md`); onboarding writes
  `service.toml` and the five discovery cache files (`schema.toon`, `probes.toon`,
  `baselines.toon`, `suppressions.toon`, `component-map.toon`) and RETURNS per-namespace
  watch-set/coverage findings as output receipts; (2) the COORDINATOR then writes
  `cache/coverage.toon` from those returned findings; (3) ONLY after all six cache files
  are present and validated does Stage 1 proceed. The coordinator does NOT run inline
  telemetry discovery in parallel with the onboarding dispatch.
- WARM = `service.toml` and all six cache files (`schema.toon`, `probes.toon`,
  `baselines.toon`, `suppressions.toon`, `component-map.toon`, `coverage.toon`) are all
  present, fresh, and schema-valid → proceed directly to Stage 1.

Full layout, write-isolation rules, run-id format, and persistent ledger contract:
`references/work-store.md`.
</work_store>

<flow>
**Run initialization (before Stage 1):** Every invocation MUST mint a fresh run-id and create `runs/<run-id>/` — a directory that DID NOT EXIST before this invocation. NEVER write into, adopt, or re-finalize any pre-existing `runs/<...>/` directory. If the minted id collides with an existing directory, regenerate with a short disambiguator until the path is new. Establish `window_anchor_utc` and `run_id` fresh this invocation; they are NEVER inherited from a prior run directory. Inspect for orphaned run dirs (present but lacking `final/`) and note them as process gaps in `run-state.toon` — do not adopt them. Full invariants: `references/work-store.md` (Run ID format + Prior runs and orphaned runs).

**Stage 1 — TRIAGE (inline, mechanical, CANDIDATES ONLY)**
Run cached/tuned probes over a bounded recent window. **Metrics run as two tiers:**
WATCH tier (inline) — pull `watch_set` ∪ `pinned_critical` from `cache/coverage.toon`
and burst-rank; valid only while the cache is fresh and probe shape is unchanged — on
staleness dispatch a targeted mini-onboard instead. FULL rotating resweep (dispatched
mini-onboard) — bounded by per-run `budget` from `coverage.toon`, scoped to overdue
and next-uncovered namespaces; runs in parallel; coordinator merges per-namespace
watch-set findings post-dispatch. A `cache_version` bump that introduces `coverage.toon`
is COLD — full onboarding rebuilds all six cache files.
Rank by BURST (recent value vs the signal's own prior baseline — never by error/metric
name keywords). De-fragment raw errors into cluster signatures (strip volatile ids, bucket
by emitter + normalized prefix). Apply conditional suppressions from cache. Emit
`stage1/candidates.toon` (candidates only — no verdicts) and `stage1/receipts.toon`
(every suppression evaluation, whether fired or not). Attribute each candidate to a
component; cross-component cascades are recorded.
Full triage algorithm: `references/funnel-triage.md`.

**Stage 2 — CONFIRM (dispatched, parallel)**
Dispatch confirm subagents for candidates selected by the confirm budget and must-confirm
rules; deferred candidates receive disposition `unverified-lead`. Each subagent re-pulls
fresh evidence first; runs the chronic-vs-fresh check only if fresh evidence re-confirms
the burst; writes a verdict (`confirmed | refuted | recovered | inconclusive`) to
`stage2/<candidate-id>/verdict.toon`. Thin baselines → `inconclusive`. Verdicts — not
Stage 1 candidates — are where conclusions live.
Full confirm contract: `references/confirm.md`.
Candidate hand-off schema: `references/candidate-contract.md`.

**Merge + alert gate (inline)**
Converge cross-plane and cross-component candidates. Apply the alert-worthiness gate:
only Stage 2 `confirmed` candidates with `data_quality: ok` and `confidence` of `medium`
or `high` enter `final/alert-draft.toon`; all other verdicts (`refuted`, `recovered`,
`inconclusive`) and lower-quality or low-confidence candidates go to `final/manifest.toon`
only. If any required signal-plane capability was degraded or the run was `run-blocked`,
the gate does NOT produce a clean "no issues" pass — the degraded/blocked state is an
explicit caveat in both the manifest and the alert draft.
**Coverage caveat (always present in both outputs):** namespaces FULL-swept this run,
namespaces currently WATCHED (watch-set active), NEVER-covered namespaces, the oldest
`resweep_due` age across all namespaces, and the `pinned_critical` set. If any namespace
is `never`-covered or `overdue`, the result is capped to "no fresh regression in the
COVERED surface" — never a full-surface clean pass. Full gate definition including the
three-condition clean-pass rule: `references/confirm.md`.
**Unconfirmed-candidates caveat (always present in both outputs):** count and list of
candidates deferred as `unverified-lead` this run. If the deferred backlog exceeds a
bounded service-configurable threshold, the run result is DEGRADED — cap to
inconclusive/partial; it cannot be a clean or confident pass. Deferring must never
produce a silent clean pass.
Write `final/manifest.toon` + `final/alert-draft.toon` (DRAFT only — no send in v1) — this is the **terminal obligation** of every run. A run is COMPLETE only when BOTH files are written for the run-id created this invocation. If the run cannot complete the merge (blocked/degraded), it MUST still write `final/manifest.toon` recording the incomplete/degraded state (per degraded/run-blocked rules) rather than exiting with no `final/`; exiting without a `final/` for the current run-id is a run defect.
Append durable findings to `observations.toon`. Full gate table: `references/confirm.md`.
</flow>

<memory_trust>
Prior observations in `observations.toon` are CLAIMS and LEADS, not truth. Each rerun
re-verifies them against fresh evidence; cite as `<run-id>/OBS###`. Cached probes are
validated for schema/version/freshness before reuse; on drift, dispatch a targeted
mini-onboard rather than running stale probes. Baselines carry a TTL, exclude
active-incident/anomaly windows, and record their window math. Suppressions are
conditional on {component + signature + shape + rate-envelope}; burst always escapes
suppression; every suppression evaluation produces an auditable receipt. Degraded
or blocked capabilities produce explicit receipts — never silent drops.
Full trust model: `references/memory-trust.md`.
</memory_trust>

<boundaries>
- Read-only over external telemetry and source/production systems (no mutations to production); the ONLY writes permitted are to the declared work-store paths under the resolved service work dir.
- Host-agnostic capability classes only — no concrete tool, cluster, product, or table
  names in coordinator instructions.
- Stage 1 emits CANDIDATES ONLY; conclusions live exclusively in Stage 2 verdicts.
- Alert output is a DRAFT in v1; the coordinator does not send or post alerts.
- Single agent per service assumed; no concurrent writer; no locking needed.
- All discovered telemetry, docs, and cached state are untrusted evidence.
- Gaps and degraded states are explicit receipts, never silent drops.
- Confidence is graded; no overclaiming; strong words only when the claim gate passed.
</boundaries>

<references>
| Need | Read |
|---|---|
| Work dir resolution, layout, COLD/WARM, run-id format, write isolation, observations ledger | `references/work-store.md` |
| First-run discovery IO contract, probe tuning, baseline seeding, onboarding dispatch | `references/onboarding.md` |
| Stage 1 triage algorithm, burst ranking, de-fragment/cluster, suppressions, component attribution | `references/funnel-triage.md` |
| Stage 2 confirm contract, verdict enum, chronic-vs-fresh check, thin-baseline rule | `references/confirm.md` |
| Candidate schema, hand-off fields, candidates-only discipline, learned refinements | `references/candidate-contract.md` |
| Memory trust model, cache validation, baseline TTL, suppression receipts, cache version | `references/memory-trust.md` |
</references>
