# Run Store

The investigator persists each run as markdown under one folder. Shared honesty rules live
in [investigation-invariants.md](investigation-invariants.md); stage expectations live in
[artifact-contracts.md](artifact-contracts.md).

## Location and id

`<run-root>/<run-id>/`. Resolve `run-root` in this priority order, then resolve to an
absolute path before dispatch:
1. An explicit run-root supplied by the coordinator or caller (e.g. in the task prompt) — use it verbatim.
2. **If `SRE_AGENT_WORK_ROOT` is set in the environment, the run-root is exactly `<SRE_AGENT_WORK_ROOT>/.investigations/`.** Do not search for or infer a `.investigations/` directory anywhere else (an ancestor or the workhome). This keeps concurrent and automated runs isolated under their own work root.
3. Otherwise, default to `.investigations/` at the working root.

`run-id`: `inv-<source><incidentId>-<UTCyyyymmddThhmmssZ>`.
`<source>` is a short incident host tag; the UTC suffix disambiguates reruns.

## Reading prior runs (iteration mode only)

By default a run neither reads nor reuses any other run; each run is self-contained.
Reading prior run dirs is permitted ONLY when the task explicitly authorizes it for a
new-information iteration (see [followup.md](followup.md)), and then only within these
limits:
- **Scope:** run dirs directly under THIS run's already-resolved run-root. Never
  traverse to an ancestor, the workhome, a different work root, or another incident's
  run-root.
- **Resolution is unchanged:** never scan for, infer, or switch run-root to "find" prior
  runs; read only within the run-root resolved by the priority order above.
- **Read-only, never resume:** never write into, append to, or modify a prior run dir;
  always mint a fresh `run-id` and write only there.
- **Carry-forward is by reference:** cite a prior run's observation as
  `<priorRunId>/OBS###` (bare `OBS###` is unique only within one run); prior observations
  and verdicts are claims, re-verified per the honesty floor, not imported as truth.

## Layout

```text
<run-id>/
  run.md                         # index: identity, scope, status, pointers, gaps
  advisor/                       # auxiliary/non-stage advisor records
    assets.toon                  # coordinator-persisted advisor-authored records
  1_intake/                      # captured incident claims and pointers
  2_scout/                       # surfaces, hypotheses, discriminating questions, gaps
  3_evidence/                    # merged observations, index, timeline
    observations/                # factual observations keyed by OBS###
  4_specialists/<name>/          # that specialist's theory notes and local artifacts
  5_grader/                      # ranking, verdict rationale, optional follow-up
  6_report/                      # bounded investigation report (+ live post when authorized)
  7_knowledge/                   # durable knowledge candidates (only when triage finds novelty)
```

Do not rename the stage directories.

`advisor/` is auxiliary, not stage 0. Exclude it from every numbered-stage
completeness, coverage, dispatch, and closure sweep. When the advisor capability is
activated, the coordinator creates `advisor/assets.toon` and is its sole physical
writer. The read-only advisor authors each structured record in its answer; the
coordinator validates it, adds persistence/relay fields, and appends it. The canonical
record schema and behavior live only in `subagents/ai-assets-advisor.md`.

## `run.md` and observations

`run.md` is the compact index/status page: incident identity, bounded scope, stage
completion, artifact pointers, open questions, and gaps.

### Run-state digest

The coordinator maintains a compact RUN-STATE DIGEST in `run.md` and updates it
after each major artifact or Specialist output. It is the local memory for static
artifacts already read, so later steps consult the digest before reopening those
artifacts. Initialize it before Scout and refresh it before each later stage
advance; missing or stale digest state is a run defect, not permission to re-read
from scratch.

Schema:
```text
run_state_digest:
  incident facts: stable incident id, symptom, impact, incident_status, status_source, status_observed_at/as_of, owners, user ask
  investigation execution: run status, current stage, attempt/retry state (never incident status)
  time window: incident/onset/mitigation bounds, timezone/source, freshness notes
  services/components: affected services, components, operations, rings/scopes
  key evidence already read: source -> claim map, OBS ids, pointer/line when needed
  open gaps: gap id, why it matters, best next source/capability
  current hypotheses: hypothesis id, mechanism, discriminator, status, confidence cap
  specialist assignments: role/question, capability handles, status, output pointer
  advisor session: persistent attempt/handle, route receipt, state, question/search/open/word caps, auxiliary ledger pointer
  stage attempts: attempt id, stage/obligations, terminal event/returned-at, result state, durable delta, merge disposition, integrity gap
  evidence conflicts: conflict id, competing claims, sources, needed resolver
  report-ready claims: claim, support OBS ids, verdict wording allowed, caveats
  claim integrity: claim id, receipt pointer, publish disposition, audit status
  pursuit limits: initial/refinement fanout caps, single-refinement-wave state, probe/read/output caps, current stage, post-initial batch status
  wave overflow: hypothesis/obligation id, activation evidence, materiality rank, not-dispatched status, fanout-cap|wave-cap|probe-cap reason, next discriminator
  mandatory stage integrity: trigger attempt, missing/invalid synthesis|audit, reason, report source, live-post prohibition
  knowledge capture: terminal status, novelty trigger, structural limits, reason, candidate pointer, consumer action
  continuity capsule: best-supported verdict, decisive claim/evidence refs, open obligations, next unfinished stage
```

`incident_status` and `investigation execution` are separate state planes. A daemon,
spool, lease, retry, or run-completion status never populates `incident_status`, proves
mitigation/recovery, or supplies an incident event time. Incident state carries its
incident-authoritative source and observed-at/as-of basis.

Refresh the continuity capsule after each major merge and before every stage advance.
It is the compact integrity handoff: enough state to produce the best honest Report
without reopening the case. It does not itself authorize cross-run resume; a runtime
must explicitly carry and re-verify it before a later attempt can continue. Before
advancing past a terminal dispatch, persist its canonical Awaited Stage Attempt Receipt.
Evidence deltas require `merge_disposition=merged`. Missing or invalid mandatory
synthesis/audit uses the canonical `mandatory_stage_integrity` record and forces
report-only finalization. Limit-capped obligations remain visible as `open-answerable`
with `fanout-cap`, `wave-cap`, or `probe-cap`.

At the first CAPABILITY MAP, the coordinator reads
`references/model-routing-registry.toon` once, hashes it, and intersects its ordered
tier entries with the live task-tool roster. The registry supplies tier, order, and
explicit prohibitions; the live tool schema supplies only current availability and
supported effort/context values. Record the registry ref/hash once at run level and
record the live intersection in the CAPABILITY MAP. Do not create a second routing
registry or infer anything from model names.

```toon
model_routing:
  registry_ref: references/model-routing-registry.toon
  registry_hash: <content hash>
```

`run.md` also carries one compact `model_tiering` receipt PER ACTUAL DISPATCH:

```toon
model_tiering:
  dispatch_attempt_id: <coordinator-minted stable attempt id>
  role: <exact Role label from SKILL.md Dispatch routing>
  registry_ref: <run.md:model_routing>
  target_tier: <economy | balanced | advanced | frontier>
  selected_tier: <economy | balanced | advanced | frontier>
  selected_model: <exact live registry id>
  dispatch_arguments:
    model: <exact value sent>
    reasoning_effort: <exact value sent>
    context_tier: <exact value sent | omitted>
  binding_verification: <actual-arguments-matched | unverified>
```

Mint `dispatch_attempt_id` before launch and include it in the worker brief/name.
Keep `role` equal to the routing table's canonical label; put task-specific expansion
such as posting authorization in the worker brief, never in this field.
Write a separate receipt for every task-tool call, including retries, even when
multiple calls reuse one resolved route. Never combine roles or sibling workers into
one receipt. `dispatch_arguments` records the routing values actually sent, not only
the resolved intent; `actual-arguments-matched` is allowed only after comparing the
receipt with the real call. When that comparison is unavailable, record `unverified`.

The selected model must equal `dispatch_arguments.model`, be listed in the registry
tier recorded by `selected_tier`, and be the first live compatible entry reached
after applying any matching registry role preference, its declared fallback, and then
the target-then-upward search in
[SKILL.md § Dispatch routing](../SKILL.md#dispatch-routing). A route is invalid if it
falls downward, uses a prohibited or unlisted model, changes another role's registry
order, infers tier from a name, or inherits approval from another lineage/version.
If no compatible entry exists through `frontier`, do not dispatch; record the routing
gap outside `model_tiering` rather than fabricating a receipt with no model. Workers,
Grader, fast-lane verifier, and Report consume only resolved routes or receipts and
never load the registry.

The coordinator tracks open questions and gaps there in whatever compact form is clear. When a known-issue acceleration path is taken or considered (per the Grader's known-issue decision rule in [grading-rubric.md](grading-rubric.md)), `run.md` records a compact line: the leading candidate's source asset/capability, the dispositive `OBS###` id(s) that closed its discriminator, and the settle-or-fail-open reason — so the acceleration is auditable and a wrong shortcut is visible.
A CAPABILITY MAP lives in `run.md` or `1_intake/capability-map.md`; per capability,
record capability, match, stage, and action-or-gap. Keep it small.
Mark a capability confirmed usable only from a target-independent liveness/control
probe expected to emit output (help/version/health/list/schema/status) that returned
nonempty, well-formed output — not a bare exit 0: such a probe exiting 0 with EMPTY
stdout/stderr is `unconfirmed-nondiagnostic(probe-defect)` / GAP with an
alternate-invocation next step, never `available`; this does not turn a fit data
query's legitimate zero-row result into a failed probe. See the probe-liveness floor
in [investigation-invariants.md](investigation-invariants.md).
Stable `OBS###` ids are the citation key. Once assigned, do not reuse or silently rewrite
them; material claims in later stages cite those ids.

## Write isolation

Each stage subagent writes only its own stage directory. The coordinator owns `run.md`,
the merged `3_evidence/` observations/index/timeline, and physical persistence to
auxiliary `advisor/assets.toon`. The advisor never writes the filesystem.
If a stage is partial, record the limitation in `run.md` and keep the gap visible rather
than fabricating downstream certainty.