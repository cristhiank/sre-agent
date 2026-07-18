# Artifact Contracts

Compact expectations for stage outputs. Shared honesty rules live in
[investigation-invariants.md](investigation-invariants.md); this file only says what each
stage should produce.

Internal artifacts (intake/scout/evidence/grader) cite a stable `OBS###` observation id when evidence exists. The external `6_report/` does not expose `OBS###` ids; it cites the same evidence in plain source terms (observed signal, source type, affected entity, time window, and query/pointer/path when available). Incident-report text,
including reported cause and timeline narrative, stays a claim until corroborated; see the
honesty floor.

## Reusable guidance receipt (canonical)

`curated_guidance_reuse` is observability-only: it adds no verdict penalty and no bounce.
Per-lead specialist-authored values are `reused|adapted|rejected-because:<reason>|not-applicable`;
`reused`/`adapted` cite the asset id + re-grounded live OBS id,
`rejected-because:<reason>` names the specific asset element inspected and why it did not fit, and
`not-applicable` states which capability class was queried and returned nothing on-point. The
high-severity shelf bypass is stamped once at run level as
`curated_guidance_reuse=bypassed:high-severity-triage`.

## Material Claim Integrity Receipt (canonical)

The Grader writes `5_grader/claim-integrity.toon` before Report for every
**consequence-bearing claim**: a causal mechanism or attribution, exclusion/refutation,
population or blast-radius statement, incident state/mitigation/recovery statement,
cross-source shared-cause statement, or Manual Investigation Kit branch that changes
the operator action. Ordinary factual observations do not need a row unless selected
as an operator exemplar or as the premise for the owner action.

Each compact row has this shape:
```toon
claim_id: C###
claim: <reportable wording>
kind: mechanism|exclusion|population|incident-state|recovery|cross-source-attribution|manual-branch
evidence: <OBS ids>
source_fit: direct|proxy|unfit
source_fit_basis: <why the cited source observes this claim>
state_plane: incident|investigation-execution|synthetic|unknown|n/a
population_semantics: entity=<request|trace|session|user|tenant|resource|other|n/a>; key=<observed key role|n/a>; key_fit=<direct|authoritative-map|mismatch|unknown|n/a>; empty_policy=<excluded|unknown|n/a>
protocol_semantics: observed=<layer|n/a>; claimed=<layer|n/a>; fit=<yes|no|unknown|n/a>
time_basis: <observed_at + as_of/freshness + source|n/a>
rival_test: <favored predicts ...; rival predicts ...; observed ...; discriminates=yes|no|n/a>
attribution_scope: <authoritatively closed sources/entities + open sources/entities|n/a>
post_candidate: yes|no
terminal_attempt_refs: <attempt ids|n/a>
ref_resolution: resolved-exact|missing|invalid|identity-mismatch|n/a
availability: <observed|typed field gaps|n/a>
publish: exact|qualified:<allowed wording>|blocked:<reason>
```

All rows require `evidence`, `source_fit`, `post_candidate`, and `publish`. Conditional facets are required
only when their claim class triggers them: state/time for incident state or recovery;
population semantics for counts/cohorts; protocol semantics for a cross-layer
projection; rival test for exclusions, causal selection, or a decisive manual branch;
and attribution scope for a shared cause across independently owned sources. Do not
leave a triggered facet blank or mark it `n/a`.

`proxy`, `unfit`, a state-plane mismatch/unknown, `key_fit=mismatch|unknown`, a protocol-layer
mismatch, `discriminates=no`, or open per-source attribution cannot publish as a causal
fact, exclusion, incident recovery, population size, or decisive branch. Preserve the
underlying observation only through exact qualified wording or block it. Report and
Knowledge may weaken or omit a row; they may never strengthen it. A missing/incomplete
row is a `claim_integrity_gap`, not permission to re-derive or silently publish the
claim.

Set `post_candidate: yes` only for Grader-selected evidence-bearing facts that can
change the verdict, incident state, exemplar carriage, or owner-action premise.
Stage-derived facts require attempt-bound evidence and `ref_resolution:
resolved-exact`. Direct authoritative intake metadata may use attempt/ref `n/a` only
when `evidence` cites its Bootstrap/source pointer and the wording reproduces that
metadata without causal inference; human comments, advisor content, worker-derived
facts, and family/population claims cannot use this path. The final audit freezes these
fields. One observed confirmation permits only one-unit wording, never unsupported
family/population wording. Report may omit or weaken a row, but never repair its
lineage in prose or strengthen it.

## Awaited Stage Attempt Receipt (canonical)

Every dispatched Scout, Specialist, Grader, Report, or Curator attempt has one
coordinator-owned terminal receipt in `run.md`:

```toon
stage_attempt:
  attempt_id: <stable run-local id>
  stage: <stage/role>
  obligation_ids: <ids or none>
  terminal_event: finished|host-timeout|host-cancelled|failure
  terminal_event_at: <host-recorded UTC|unknown>
  returned_at: <UTC when the coordinator collected the terminal result|none>
  result_state: complete|partial|none
  durable_delta: <artifact pointer|none>
  merge_disposition: merged|not-merged
  integrity_gap: none|host-timeout|host-cancelled|failure|missing-artifact|invalid-artifact
```

`complete|partial` may be `merged` when the pointer names the exact valid durable output
produced by this `attempt_id` under the run's source identity. `terminal_event_at` and
`returned_at` are immutable chronology only: neither timestamp, their ordering, nor
elapsed runtime changes merge, Report, or post eligibility, and they never create
late, embargo, cutoff, or deadline states. An attempt-bound pointer that resolves to a
different attempt or source identity is `not-merged` with
`integrity_gap=invalid-artifact`; its claim row remains
`ref_resolution=identity-mismatch`. A host timeout,
host cancellation, or failure is an observed terminal event with `result_state=none`;
a still-running worker cannot be converted to a partial result. A finished attempt
with a missing or invalid required artifact is `not-merged` with the matching
integrity gap. The receipt resolves ownership; it never claims prompt-level
cancellation.

### Incomplete mandatory-stage successor rule

The coordinator writes this record when mandatory synthesis or audit has no valid
artifact:

```toon
mandatory_stage_integrity:
  trigger_attempt: <attempt id>
  stage: synthesis|audit
  gap: host-timeout|host-cancelled|failure|missing-artifact|invalid-artifact
  report_source: continuity-capsule|provisional-ranking-and-claims
  live_post: prohibited
```

- Missing/invalid synthesis cannot feed audit. Skip audit and produce a
  causal-claim-free local report from the continuity capsule with every causal/verdict
  gap explicit.
- Missing/invalid audit produces a report-only incomplete assessment from the
  provisional ranking and claim rows; no consequence claim is strengthened or treated
  as final.
- A finished valid synthesis, audit, or Report remains admissible regardless of elapsed
  runtime. No `mandatory_stage_integrity` path may mutate the incident.

### Routing-blocked successor rule

When a classified roster has no eligible candidate for a mandatory role, the
coordinator writes:

```toon
routing_blocked:
  role: <Scout|evidence-obligation:<id>|Grader|Report>
  reason: no-eligible-candidate
  required_tier: <balanced|advanced|frontier>
  required_effort: <high|max>
  rejected_candidates: <ids with typed rejection reasons|none advertised>
  verdict: Inconclusive-blocked
  live_post: prohibited
```

No lower-tier or skipped-role successor is admissible. If Report has an eligible
route, it renders this receipt. If Report is the blocked role, the coordinator writes
`6_report/investigation-report.md` from a deterministic template containing only the
incident id, `Inconclusive-blocked`, the blocked role and routing gap, and the next
action to supply an eligible classified route. It contains no causal finding,
confidence claim, evidence interpretation, Manual Kit, or live-post instruction.
The receipt is terminal and report-only; no Grader artifacts are implied.


## `1_intake/` — capture

Produces `incident-context.md`: the incident record as claims plus pointers.
Include the id, source, declared window, affected/symptom description, and any
reported or claimed cause as **CLAIMS**. Include source links or artifact pointers and
intake gaps. When an incident-context capability decoded embedded telemetry/monitor alert-query deep-links from the incident body, record them as pointers (a decoded reproduce-the-metric query artifact plus an `alert_queries` manifest section) and classify any other staged reference links (TSG/wiki/docs/dashboard/runbook) by kind, URLs only; both are orientation pointers the Scout consumes, never authority and never fetched here. Also capture a discussion-thread summary: the material human comments, transfers, owner notes, prior RCA/mitigation, and linked change/rollout notes — each as a CLAIM with who and when — or an explicit note that the thread was empty or unavailable. The thread is primary orientation evidence, not authority; a human-stated cause or mitigation is corroborated like any other claim. One path, present-or-absent: when a pre-staged intake bundle exists in the run work area, reconcile the staged record + thread against its provenance manifest, normalize each item through the Bootstrap Evidence Manifest below, and capture them as CLAIMS + pointers with who/when, not verbatim copies; a `present-redacted`, `partial`, `absent`, or `unverified` item is a fetch trigger — re-fetch it the authorized way via the read-only incident-record/discussion capability (`present-redacted` is not `absent`; align with the redacted-is-not-absent honesty floor). After that honest fetch attempt, if the source still returns a redaction sentinel, record an ACCESS-BLOCKED LEAD per that floor (name the redacted field and the richer-access surface that can read it), never an absence gap; record a plain intake gap only when the item is genuinely unavailable or empty. When no bundle is pre-staged, fetch the full record + thread the same way. The intent frame, CAPABILITY MAP, and recurrence identity are agent reasoning products, never pre-staged.

### Bootstrap Evidence Manifest (canonical)

`incident-context.md` carries one compact row per staged or authorized-live evidence item:
```
item_id: <native stable id, else a stable run-local id retained across reconciliation>
evidence_class: <what kind of evidence the item contains>
capability_class: <generic read capability; never a provider or tool name>
source: <source identity>
provenance: <native source/call/manifest identity>
artifact_pointer: <native manifest, artifact, or result pointer>
captured_at: <source capture time|not supplied>
freshness_as_of: <source-declared freshness time|not supplied>
declared_time_scope: <source-declared window|not supplied>
population_scope: <source-declared cohort/entity scope|not supplied>
acquisition_state: staged|fetched|reconciled
coverage_state: complete|partial|present-redacted|absent|unavailable|unverified
gap_or_next_probe: <none|explicit gap and authorized next probe>
```
The coordinator writes the manifest after reconciling staged native manifests with any required
authorized re-fetches, before Scout. Pre-staged and live-fetched evidence use this
one shape. Retain native manifest/artifact pointers instead of copying provider payloads into
`incident-context.md`; never invent missing source, time, freshness, population, or coverage
semantics — preserve what the source declares, use `not supplied`, and name the gap or next
probe. `present-redacted` records content that exists but is not visible; it never projects to
`absent`. Every final row requires `acquisition_state: fetched|reconciled`; `staged` is
pre-reconciliation and cannot satisfy pre-Scout admission. `reconciled` records staged
provenance verification and may also record that an authorized capability was confirmed
unavailable when `gap_or_next_probe` names that explicit access gap. The daemon's existing
terminal redacted `CustomerReported` skip is out of scope and unchanged.

Bootstrap also produces an intent frame before Scout with exactly three
fields:
- `literal_trigger`: verbatim alert, incident record, or human ask.
- `rca_target`: underlying failure in the measured or impacted operation the signal evidences (see investigation-invariants A1). Specific exception types/mechanisms are discovery findings, never the intake target. Use the smallest evidence-anchored operational failure target that makes the answer remediation-useful; for vague asks with no derivable target, set `rca_target: clarification_required (<ambiguity>)`.
- `success_definition`: what makes the answer remediation-useful for an on-call engineer: evidence of cause/owner/mitigation for those failures, or an evidence-backed clean no-failure closure.
Intake also records the incident's recurrence identity: the stable identifiers that would match sibling incidents when present — signal/error signature, affected operation/component, affected entity/resource/cohort, scope boundary, and owning service/team. This is identity capture for later correlation, not a hypothesis or a cause.
Intake also holds the **intake recurrence cluster** for the fast-lane decision (`SKILL.md` § Six-stage flow → Bootstrap / FAST-LANE DECISION): the sibling incidents the read-only incident-record/context fetch returns via its recurrence include — each row carrying signature/title, owning team, `ClaimedRootCause`, `ClaimedMitigation`, `IsNoise`, `Severity`, and `Status`, held as orientation CLAIMS (never authority). When that capability or include is unavailable the cluster is an explicit GAP and the fast-lane cannot fire (deep-lane, fail open).
Bootstrap also produces the CAPABILITY MAP before Scout: available capabilities
inventoried, matched to stages, and gaps recorded. It also labels the per-service reusable-guidance shelf when a capability can resolve service docs, runbooks, failure modes, observability pointers, and curated guidance content for Scout coverage scans. For each key failing signal, the map should name any catalog/runbook-known adjacent producer/dependency observability classes likely to consume the failing units' identity/time/scope/operation pivots, including the dependency's own logs and exported metrics, to aim later discovery probes; this list is non-authoritative, and omission never closes a candidate or licenses a reachability cap under the reachability floor. For each CRITICAL-EVIDENCE-PATH
capability, include one ACCESS STATUS line:
- `status`: `confirmed` | `unconfirmed-nondiagnostic(self-misuse | wrong-target | auth-cold-start | probe-defect)` | `blocked(hard-absent | denied-after-valid-auth)`
- fields: `canonical_source_read` (help/metadata/error-guidance source), `corrected_invocation_attempted` (yes/no + generic description), `target_dependency: none | guessed | provided | discovered`, `last_error` (short verbatim); for `blocked` only, `human_command` (one concrete command a human/specialist should run).
- Wrong-target, self-misuse, and auth/cold-start failures are non-diagnostic for availability: record `unconfirmed-nondiagnostic`, not `confirmed` or `blocked`. An expected-output control probe (help/version/health/list/schema/status) that exits 0 with EMPTY stdout/stderr is `unconfirmed-nondiagnostic(probe-defect)` — a silent no-op, never `confirmed`, `blocked`, `reached`, or `reached_empty`; record `corrected_invocation_attempted` and the alternate-invocation next step for a specialist. Non-critical capabilities default to `unconfirmed / not-probed` and need no access status.
Intake captures the literal trigger and measured/impacted failure target; it does not
hypothesize causes.

## `2_scout/` — scope and discriminate

Produces `scout-report.md` with surfaces in scope: services, regions, rings,
dependencies, time windows, and adjacent issues worth bounding.
Also reports recurrence: prior or concurrent incidents whose identity matches or
overlaps the recurrence identity — each with its stable incident id, trigger/start time,
which identity dimension(s) matched, and any parent/duplicate link the incident-history
exposes, with any reported verdict or mitigation carried as
claims — how they shape the hypotheses, and an explicit note when none was found or no
incident-history capability was available (recurrence shapes hypotheses but never sets a
verdict, and the per-sibling id and time let the grader classify canonical/duplicate — see
the recurrence invariant in [investigation-invariants.md](investigation-invariants.md)).
Also reports the discussion-thread summary: material human comments, transfers, owner notes,
prior RCA/mitigation, and linked change/rollout notes (carried from intake as claims) and how
they shape the hypotheses, or an explicit empty/unavailable note.
Also names the failing-unit class and the keys that would join the symptom to its
producing layer. For each candidate key, record `population_entity`, `key_role`,
`key_fit` (`direct|authoritative-map|mismatch|unknown|n/a`), `empty_policy`, and what unit it can
legitimately count. A request/trace/correlation key is not a session/user/tenant key
without an authoritative mapping. This arms enumeration without silently changing the
population semantics (see the "Aggregate is not mechanism" invariant in
[investigation-invariants.md](investigation-invariants.md)).
Optional typed field: `producer_resolution_lead` when a package/assembly-backed symbol is
absent from the primary repo — predicate `which service, if any, publishes this symbol?`
for the mechanism-verification path to resolve, never resolved during scope discovery.
Name at least two materially different hypotheses when possible.
For each, note what observations or questions would distinguish it from the others.
Also includes a best-effort pre-declared discriminator table: per leading hypothesis,
the serious same-symptom rival, a falsifiable predicate, the expected favored vs rival
observation, and a candidate authoritative source/key — pre-registered questions, not
checked results, with an explicit gap when no honest discriminator exists yet
(neutrality preserved: no expected value is asserted as the answer).
Also instantiates the discriminator ladder: bind the incident's signature to its
canonical ladder (a known-issue signature template discovered from any available read-only knowledge capability — service/failure knowledge, AI assets, TSGs, runbooks, wiki/doc search, not only the local KB) or bootstrap one from the service's
pipeline/capability map when the family is novel (consumer→producer→upstream lineage),
pre-registering each rung's `claim` / `favored_value` / `rival_values` / `evidence_shape`
/ `dispositive` and hanging the rungs on the coverage map above; any prior AI- or
human-claim hypothesis is recorded as an `inherited`, `open` rival rung, never as a seed
for the favored value (see the discriminator ladder and Gate C in
[grading-rubric.md](grading-rubric.md)).
Coverage-map disposition is typed per leading hypothesis:
`consulted:<capability> -> covered:<asset> | no-coverage`; when covered, carry the
returned guidance content lead (encoded checks/queries/decision-path/expected outcomes)
that the specialist consumes. Shared specialist reuse mode:
`reused | adapted | rejected-because:<reason> | not-applicable`; receipt semantics are canonical in § Reusable guidance receipt.
Keep gaps visible. Scout stays neutral: no verdicts and no root-cause language.
The report SHAPE is a REQUIRED compact structure — typed rows/fields (surfaces, recurrence
rows, hypotheses + discriminator table), NOT a prose narrative; keep it terse
(cap-prose-not-proof, [operational-discipline.md](operational-discipline.md)). Scout emits
no proof sketch — it stays neutral.

## `3_evidence/` — observations

Produces normalized observations, usually under `3_evidence/observations/`, plus any
small index or timeline that helps later readers.
Each observation is one factual statement with a stable `OBS###` id, a source or
provenance pointer, and what entity/time/surface it is about.
Observation/evidence rows include the observation id, factual statement, source/provenance
pointer, entity/time/surface, and optional `evidence_link`. For telemetry-query-backed
observations, include the query execution `runId` so the candidate menu can map the
manifest to this `OBS###`; specialists also capture `evidence_link` from the shareable deep-link the
telemetry-query capability surfaced for the query that produced this observation (its
`DeepLink:` output / run-manifest `deepLink`). Omit `evidence_link` when no shareable
link was surfaced or when the underlying query embeds restricted identifiers; carry
de-identified raw query text in Details instead.
The primary clickable-query surface is the **Failure path**: the Grader selects the
1-3 decisive verification queries by `runId` (from a deterministic candidate menu) into
`5_grader/decisive-queries.toon`, and the posting capability resolves those runIds to deep-links mechanically
— so the links never depend on a specialist copying a URL. When a safe selected query's
`observation_ref` proves one Failure path node, attach `[query]` to that node and include the observed
result in the node label; do not duplicate it. Render a fallback **Evidence Kit** immediately after the
path only for a selected query that cannot map to one node or when no chain exists. The kit is the decisive
few, never every query. `6_report/evidence-kit.md` and `6_report/incident-update-with-kit.md` are
**reserved** for the report-only finalize output (the assembled concise draft + fallback kit); the report
agent must not author files with those names. Unsafe links remain omitted and their de-identified raw query
text stays in Technical details.
Describe the observation's kind in prose when useful; do not force a fixed taxonomy.
Keep raw rows or bulky artifacts behind pointers.

## `4_specialists/` — theory notes

Each specialist writes only its own stage area and produces per-specialist theory notes.
A theory note names a cause and mechanism, cites the observations it relies on, answers
or leaves open the scout questions it addressed, and lists gaps.
It may include alternatives or negative evidence, but should not overstate beyond the
observations it cites.
Optional typed field: `producer_resolution` for a package/assembly-backed symbol that
was resolved: `completed-external | completed-internal | gap:<step>`; present only when
internal-first producer resolution was triggered.
A theory note also pre-registers its discriminator — predicate plus expected
favored/rival stated and kept visibly separate before the checked value (with the OBS id
when checked) — and ends with the compact claim-readiness ledger (failing-population
bound / mechanism named / discriminator pre-registered / coverage-map disposition
`consulted:<capability> -> covered:<asset> | no-coverage` / specialist reuse mode
`reused | adapted | rejected-because:<reason> | not-applicable` / reusable-guidance receipt per § Reusable guidance receipt / observed value
checked / result / confidence ceiling).
For a consequence-bearing claim, the ledger also supplies the conditional semantic
inputs the Grader needs for the Material Claim Integrity Receipt: observed state plane
and time basis, population entity/key role and empty policy, observed-versus-claimed
protocol layer, rival emission predictions, and per-source attribution scope as
applicable. The specialist supplies evidence; the Grader owns the final receipt and
publish disposition.
The note SHAPE is a REQUIRED compact structure, not a stylistic default: typed fields +
the claim-readiness ledger rows, with a SINGLE optional **proof-sketch** section as the
ONLY unbounded part — required and never compressed for a load-bearing causal chain
(mechanism-discriminator; change-arrival resolution INCLUDING an arrival-DISPROVEN
refutation; onset-signature→cause-class; or Gate E), per cap-prose-not-proof in
[operational-discipline.md](operational-discipline.md). No prose narrative, methodology
recap, or ruled-out-branch essay outside those fields.

## `5_grader/` — judged assessment

Produces `ranking.md`: a judged verdict in one of `Confirmed` / `Likely-rooted` /
`Proximate-only` / `Inconclusive-blocked` / `Refuted`, with rationale and the
observations relied on.
Keeps a lead ledger (each material lead closed as `closed-supported` /
`closed-refuted` / `open-answerable` / `blocked-unreachable`) so no lead is silently
dropped.
Also produces the canonical `claim-integrity.toon`. After the optional single
post-synthesis batch, the bounded consequence audit writes
`consequence-audit.toon`, content-checks all claim rows for semantic mismatches, and
independently re-derives at most two claims whose failure would change the verdict,
owner, mitigation, or incident-state assertion. It then finalizes the affected
`publish` dispositions and ranking; it does not fetch, create another obligation, or
re-grade the whole case.
Also records these Grader-owned lines in `ranking.md`:
```toon
Residual discovery: unconsumed | consumed
Residual discovery outcome: not-triggered | obligation:<id> | no-eligible-class | recovered:<id> | malformed-prior-receipt
```
A residual obligation in `refinement-obligations.md` carries
`origin: independent-residual-discovery`; that obligation is the durable recovery
witness. The receipt is monotonic across Grader passes. Absence is valid only before
the first `ranking.md`; when the receipt is absent or `unconsumed`, recover from an
existing residual-obligation witness first. An existing ranking without a receipt and
without that witness is malformed and cannot reopen residual discovery. The Grader
owns both the receipt and residual obligation; the coordinator only dispatches the
obligation.

### Decisive-query Evidence Frame (canonical)

The candidate evidence-query menu uses one row per executed evidence query:

```toon
runId: <execution run id>
query_summary: <compact purpose>
source: <source identity>
observation_ref: <OBS###|none>
selection_eligible: yes|no
ineligible_reason: <none|missing-observation-ref|incomplete-frame-inputs>
```

Only `selection_eligible=yes` rows may be selected. Eligibility requires a normalized
query-backed observation carrying the same `runId` and enough inputs to materialize the
canonical frame below. Manifest-only rows remain visible as gaps but cannot be selected.

Also produces `5_grader/decisive-queries.toon`, with one compact frame per selected query.
The file begins with `schema_version: sre-agent.decisive-queries.v2`; each record uses:
```toon
runId: <execution run id>
why: <why this query was selected>
```
Version 1 used one pipe-delimited line, `<runId> | <why>`. Version 2 is an explicit
wire-format migration because deterministic placement also requires
`observation_ref`. The incident-posting capability must advertise v2 key/value-block
support before live posting; a v1-only or unknown selector is a visible capability gap
and forces report-only output rather than dropping enrichment or guessing. A v2
selector may ignore the remaining enrichment fields that follow in the same record:
```toon
observation_ref: <OBS###>
claim_predicate: <material claim or discriminator tested>
source_fit: <authoritative|adjacent|unresolved>
source_fit_basis: <why this source observes the predicate>
time_frame: <exact start..end with timezone>
time_relation: <declared|preceding|centered|custom>
returned_time_semantics: <provider-returned semantics|unresolved>
population_cohort: <failing population or cohort>
population_keys: <identity/correlation keys>
population_coverage: <coverage statement and caveat>
provenance_chain: <query-origin pointer -> query artifact pointer -> execution-manifest pointer>
reachability_probe_fitness_ref: <lead/receipt pointer to existing reachability_outcome and probe_fitness|n/a-not-required:<reason>>
material_gap: <none|gap that prevents decisiveness>
```
The query executor supplies the inputs from query and run artifacts; the Grader validates
and finalizes the selected frames. If source, time, or population semantics required to
interpret the result remain unresolved, the query is non-decisive and the existing lead
stays `open-answerable` or retains its probe-fitness gap as applicable. Do not infer
provider semantics or create new status enums; the frame references existing outcomes.
Use `n/a-not-required:<reason>` only when the selected query targets an already reachable
source and neither durable-entity handoff nor blocked-reachability reasoning contributes
to its decisiveness. Never leave the field blank or fabricate a receipt pointer.

Carries forward observability-only guidance telemetry when present, not authored fresh by the Grader:
per-lead `curated_guidance_reuse=reused|adapted|rejected-because:<reason>|not-applicable`,
alongside the coverage-map disposition when present; run-level
`curated_guidance_reuse=bypassed:high-severity-triage` for the whole-shelf high-severity bypass.
Receipt semantics are canonical in § Reusable guidance receipt.
Records a duplicate classification (`canonical` / `duplicate-of <incident>` / none) when the scout reports a recent or concurrent sibling sharing the recurrence identity, and applies the verdict-determinism rule so such siblings are judged by the same verdict gate (see `grading-rubric.md`).
When `Proximate-only` with `open-answerable` leads, also writes
`refinement-obligations.md` for the coordinator's single post-synthesis batch,
carrying per obligation: id; lead id(s); pre-registered discriminator predicate +
expected favored/rival; acceptable evidence shape; in-hand keys; prior OBS ids with
reuse/freshness mode; stop condition; dependency-ids/independence-reason; and allowed
outcome statuses (answered/invalid-premise/unanswerable). Independent obligations are
dispatched as the single awaited parallel-sync post-synthesis batch (one specialist per
independent discriminator, maximum five); when `blocked-unreachable`, records the dead-end
plus an engineer suggested next step.
Discovery receipt for causal `blocked-unreachable` leads, a final `Proximate-only` whose unresolved upstream/mechanism lead still had reachable in-hand keys, or clean/no-failure target closures:
- `needed_evidence_type`: metric | log | trace | config | code | other
- `candidate_provenance`: observability/service-knowledge reference, catalog/schema result, config/code reference, infra/control-plane change-event reference (cert/secret/DNS/config/flag rotation or push event), query history, dashboard pointer, or user-provided; invented or guessed targets remain nondiagnostic and are not probed
- `observability_service_knowledge_lookup`: result or explicit absence
- `schema_catalog_discovery_attempted`: source + scope + result
- `log_vs_metric_distinction`: why available evidence is insufficient and what evidence type is needed
- `access_result`: the result of an actual canonical probe attempted against the source that answers this lead — a correct/authed/non-guessed query result, or a recorded terminal authorization/missing-command/missing-source/schema error. A documentation gap (a signal/table/source named-but-not-confirmed) is not an access result and cannot satisfy `blocked-unreachable` (see the access invariant in [investigation-invariants.md](investigation-invariants.md)). To record a source unreachable, canonical missing-command after entrypoint discovery, authenticated authorization denial, missing-source, or schema-absent qualify; token-acquisition/auth-init, usage, entrypoint, output-capture, a soft or unnamed "unavailable", an exhausted probe/fanout cap, or assumed provider ownership do not. When such a provisional failure comes from a worker and would cap the whole verdict, cite the coordinator's phase-current canonical capability-level control recheck from `full-evidence`; without it the lead stays `open-answerable`.
- `evidence_source_probed`: which source/endpoint was actually queried for this lead. An unfit or never-probed source — including an unprobed service telemetry source the lead needs — leaves the lead `open-answerable`, not blocked; a fit-for-purpose probe of the source that answers the lead which comes back genuinely empty is `reached_empty` — it rules out only what a fit probe in the correct scope/window rules out (it may close/refute on the merits when fit) but NEVER licenses a block per "Empty is not absent"; only a `denied` probe licenses `blocked-unreachable`.
- `probe_fitness`: `fit=yes|no`; the claim being supported; signal/source used and why it observes that claim; basis (documented source, schema/catalog discovery, runbook, owner hint, or explicit rationale); claim scope vs queried scope (+ justification if narrower); result. A guessed signal or unjustified sub-window is `fit=no`; a lens that cannot observe the symptom class (an error-/failure-level filter for a latency/late-completion symptom) is `fit=no`; a `fit=no` result is nondiagnostic and cannot support `blocked-unreachable` or clean closure.
- `correlation_follow_through`: discriminating ids/joins the symptom exposed (if any), whether followed to the next causal layer, and the result or why-not.
- `verdict_ceiling_lead`: the open causal lead targeting `rca_target` whose reachability sets the verdict ceiling (its block would downgrade the verdict, e.g. Likely-rooted → Proximate-only). Named by lead id; if two co-equal leads each set the ceiling, each carries its own receipt (cannot split to dodge). Non-decisive/contributing leads and `closed-supported`/`closed-refuted` dispositions carry no typed receipt and settle as today. A named change with `arrival_status=disproven` is a `closed-refuted` disposition, NOT a `verdict_ceiling_lead` — it cannot be carried as a capped-but-named ceiling lead.
- `arrival_status`: for a named introducing change carried by any lead (code commit/PR/build OR infra/control-plane rotation/push event), the three-valued change-arrival resolution — `verified | unverified | disproven` — per the change-arrival gate in grading-rubric.md. `disproven` ⇒ the lead is `closed-refuted` (`correlation-not-causal`), not a `verdict_ceiling_lead`; `unverified` with reachable build/branch/deploy provenance is `unattempted_open` ⇒ the `verdict_ceiling_lead`, MUST-dispatch a bounded arrival probe (not a Proximate-only soften) per Gate B; `verified` ⇒ eligible UNLESS post-onset or a revert/rollback/mitigation (Gate E), which is ineligible regardless of arrival. Absence of arrival evidence is `unverified`, never `disproven`. Logic lives in grading-rubric.md; this field records the resolved value.
- `reachability_outcome`: the typed resolution, for the `verdict_ceiling_lead`, that the `access_result` / `evidence_source_probed` / `probe_fitness` / `correlation_follow_through` / `generic_pivot_ladder` / `alternate_capability_pivot` evidence above consolidates into — exactly one of: `reached` (probe issued, nonempty result; cite {call_id, result handle}) · `reached_empty` (probe issued, zero rows; states what a fit probe in the correct scope/window RULES OUT — empty ≠ absent; weighed on probe fitness, NEVER licenses a block, and does not auto-refute — sole exception: the change-arrival gate resolution against an authoritative, COMPLETE serving-build/deploy/control-plane manifest that positively EXCLUDES the change resolves to `DISPROVEN`, see grading-rubric.md § Mechanism-discriminator gate → Change-arrival gate) · `denied` (canonical entrypoint resolution returned missing-command, or a canonical probe issued after auth/init resolution and returned an authenticated authorization/missing-source/schema-absent error or redaction sentinel; cite {probe call_id, verbatim returned error/sentinel string}; provisional failures listed in `access_result` are nondiagnostic, not `denied`) · `discovery_exhausted` (bounded discovery — actual schema/catalog discovery probe(s) of candidate sources — found no source consuming the pivot dimensions; a DEMONSTRATED exhaustion of the causal chain via spent probes, not a reachability denial; MUST cite the schema/catalog discovery probe call_id(s) + verbatim result(s) for each candidate checked; absence from a discovery artifact — CAPABILITY MAP, scout enumeration, self-built map — does NOT satisfy this; only spent probe call_ids do) · `unattempted_open` (no probe issued; the lead stays `open-answerable`). There is NO `unattempted_blocked` member — "I didn't try" projects only to `unattempted_open`, so a 0-probe "no access established" block is unrepresentable; a bare "I couldn't find a source" assertion (no cited spent schema/catalog discovery probe, required discovery sequence not completed) is likewise `unattempted_open`, never `discovery_exhausted`. Only `denied` (cited returned error/sentinel + call_id) or `discovery_exhausted` (cited spent schema/catalog discovery probe call_id(s) + results, required bounded discovery sequence completed) licenses `blocked-unreachable` or a Proximate-only-due-to-block cap; `reached_empty` and `unattempted_open` CANNOT cap and keep the lead open. Cross-source consolidation: the ceiling lead's `reachability_outcome` is the WEAKEST / MOST-OPEN resolution across the primary probe AND every fit discovery-artifact-listed alternate able to reach the decisive scope — its listing or omission in the CAPABILITY MAP never closes it (see `alternate_capability_pivot` below) — it is `denied` or `discovery_exhausted` ONLY when the primary AND all fit alternates each resolved `denied`/`discovery_exhausted`; if any fit alternate is `unattempted_open`, the consolidated outcome is `unattempted_open` and CANNOT cap (this closes the primary-`denied`-while-map-named-alternate-`unattempted_open` and arrival-lead-as-ceiling bypasses). Canonical verdict logic: [grading-rubric.md](grading-rubric.md) § blocked-unreachable. Reuse-before-probe: a prior in-run probe artifact (call_id) for that same evidence source already satisfies the access component — no new query.
- `discriminator_rung`: the dispositive ladder rung(s) this receipt resolves — the typed close that the `access_result` / `evidence_source_probed` / `probe_fitness` / `correlation_follow_through` / `reachability_outcome` evidence above consolidates into, exactly as `reachability_outcome` already consolidates the access sub-fields. Per rung: `dispositive` (decisive-for-verdict; orientation-proposed, grader content-verified), `observed_identity{scenario, signal, owner, correlation_key}` (each projected non-null from an authoritative probe, or `n/a-not-exposed` with cited justification where the capability does not expose it), and `status` (`open | closed_confirmed | closed_refuted`). Rung close/refute rules live solely in grading-rubric.md (Gate A); this field RECORDS the Gate A result — the cited identity projection and the resulting status — and does not restate the rule.
- `held_branch_freshness`: one row for every held or conditionally closed branch whose
  exit carries a time-dependent reactivation predicate. Record `lead_id`;
  `predicate_class` (`recurrence | still-firing | still-degraded | other-live`);
  `own_signature`; `prior_evidence_end`; `refresh_obligation`
  (`not-required | required:<id> | completed:<id> | skipped:<reason>`);
  `window_covered` (`<prior_evidence_end>..<report-time target>` or `unproven`);
  `coverage_status` (`proven | unproven`, including ingestion/watermark basis);
  `outcome` (`pending | fired | not-fired | predicate-unproven |
  predicate-unclosed`); and `lead_disposition`. The behavioral mapping lives solely in
  grading-rubric.md § Time-dependent held-branch closure; this field records its
  per-branch decision and never treats an empty/unproven or skipped read as closure.
- `promotion_precondition` (Gate B): records the grader's Gate B evaluation per grading-rubric.md — the dispositive rung ids + statuses, each open dispositive rung's `reachability_outcome`, and the dispatch/cap disposition. Do not restate the rule here.
- `remaining_gap`: what evidence would change the RCA
- `structural_cap_used`: which bounded discovery steps were spent
- `generic_pivot_ladder`: the VERTICAL next-causal-layer pivot — the time ∧ affected-scope/entity ∧ operation/step pivot was attempted into a capability-map-named next-causal-layer source, recorded as source + result. The cited probe MUST — by DEFAULT for every vertical-pivot citation, whether the layer is a separately-named source OR a signal/namespace within a source already `reached` for the primary — be a spent probe schema-attributable to that next causal layer's own instrumentation, the attribution established from a schema/catalog probe call_id or an explicit read-shape citation of that layer's own interface/schema (same provenance rigor as `alternate_capability_pivot`'s capability class; a bare attribution sentence never satisfies it), and carry a call_id distinct from the primary's already-`reached` call_id; when the target is within an already-`reached` source, re-citing the primary probe or a second differently-filtered query against the SAME already-`reached` producer schema additionally does not satisfy the vertical pivot. The alternative is a terminal `no such source named` — and that terminal value MUST cite the schema/catalog discovery probe call_id(s) + verbatim result(s) that established no source consuming the pivot dimensions (same provenance rigor as `access_result`/`probe_fitness`: a bare assertion of absence is invalid, exactly as a documentation gap cannot satisfy `blocked-unreachable`; absence from a discovery artifact — CAPABILITY MAP, scout enumeration, self-built map — does not satisfy this terminal). The HORIZONTAL same-scope alternate-capability pivot is a distinct obligation recorded in `alternate_capability_pivot` below.
- `alternate_capability_pivot`: the HORIZONTAL same-scope pivot, required whenever the primary source for the decisive scope is unreachable/failed — for EACH capability-class-fit or scope-derived same-class alternate read capability able to reach that same scope (an alternate cluster/endpoint, a log/trace-grep capability vs a metrics-query cluster, an elevated-entitlement read vs a GUI-only reader): the alternate + its probed outcome (`reached`/`reached_empty`/`denied`/`discovery_exhausted`), OR a terminal `none-named` with a cited spent schema/catalog discovery probe call_id + result showing no candidate with the needed capability class reaches this scope — absence from a discovery artifact (CAPABILITY MAP, scout enumeration, self-built map) does NOT satisfy `none-named`. A candidate leaves the pivot only by reachability-floor conditions: (a) its own cited probe terminal error, or (c) a gated cross-class structural non-fit — the candidate's capability class (from the taxonomy `{metrics-query | log/trace-grep | GUI-only-reader | elevated-entitlement-read | control-plane/config}`, established from a schema/catalog probe call_id or explicit read-shape citation of the candidate's own interface/schema) plus a stated reason why that class cannot carry the decisive-scope data model; the class + citation are recorded in this field. NEVER by a KB note, discovery-artifact scope-match absence, enumeration omission, or same-class non-fit assertion. A fit alternate left `unattempted_open` keeps the consolidated `reachability_outcome` `unattempted_open` (cannot cap); canonical verdict logic and terminology: [grading-rubric.md](grading-rubric.md) § blocked-unreachable.
- `in_hand_branches_dispositioned`: `yes` (list each zero-cost in-context branch + the OBS id / observation that disposes it) | `n/a-with-reason`.
- `signal_validity`: CONDITIONAL — when the trigger is a measurement/alert/monitor signal, disposed via the existing signal-validity / alert-semantics capability → `real-failure | evaluation-artifact | unresolved`; when the trigger is not signal-shaped, record `n/a-trigger-not-signal-shaped`. An `unresolved` value is an `open-answerable` lead only when it meets the materiality bar (could materially change the verdict); otherwise it is a confidence-capping gap, never by itself a basis for `Inconclusive-blocked`.
- For a final `Proximate-only`-with-reachable-keys settle, the load-bearing fields are `correlation_follow_through`, `evidence_source_probed`, `remaining_gap`, and `structural_cap_used` — showing the cross-source pivot was spent or why no reachable next-causal-layer source exists; `access_result`/`probe_fitness` apply only once that pivot is actually probed, so a receipt marking them N/A without a spent pivot does not license the settle.
Detailed judging rules live in `grading-rubric.md`.

## `6_report/` — bounded RCA

Produces a concise external-facing report bounded by the grader verdict. No internal
observation ids; cite material claims in plain source terms. Never copy unredacted customer content that the run obtained by bypassing a source redaction (anything marked do-not-republish / enumerated by a run `pii-marker`) verbatim into the report — the report is projected into the incident post. Carry such content only as DE-IDENTIFIED facts (service/operation terms; customer-identifying values by category + count, never verbatim), per "Do not republish redacted customer content" in [investigation-invariants.md](investigation-invariants.md). A material numeric/aggregate
claim carries its source and a coverage caveat when the figure is partial, sampled, or
measured differently by another source over the same window.
<example>"N per <source A>; <source B> showed M, same window — unreconciled"</example>

For deep-lane runs, every consequence-bearing report claim must map to a final
`claim-integrity.toon` row with `publish: exact|qualified`. Render the exact claim or its
allowed qualified wording; omit `blocked` claims and name the integrity gap when it
changes the handoff. Report never creates a new consequence-bearing claim. Fast-lane
known-recurrence output remains bounded by its existing SAME + gate-pass receipt and may
not add recovery, exclusion, or causal attribution beyond that receipt.

**Report completeness.** When the Grader selects Operator exemplars, carry the ranked
set and #1 owner action into the existing Answer/Facts/Proof envelope. Omitting a
reachable selected exemplar or its owner/action fails acceptance. Preserve typed field
gaps and the single executable next check; an unreachable field does not suppress an
otherwise supported report/post. This changes content completeness, not the visual
layout.

A source's last observed failure is not incident recovery. Render it as
`last observed at <time> in <source/window>` unless a publishable claim has
`state_plane=incident` and an authoritative recovery time basis. Never translate an
ended query window or missing later sample into `stopped`, `ceased`, `recovered`, or
`mitigated`.

In iteration mode (`followup.md`), the report and any incident post are a delta/update —
what changed since the last iteration plus the updated verdict (including honest
downgrades) — bounded by the current grader verdict.

### Adaptive operator projection

The report optimizes for rapid comprehension, not artifact completeness. Internal
receipts stay in their stage files. Choose one operator shape:

- **Decision Brief** — the default when the result is actionable without a
  human-only discriminator: `Confirmed`, `Likely-rooted`, `Refuted`, and
  known-recurrence outcomes. Use it for collaborator/additive posts too.
- **Mechanism Handoff** — use for `Proximate-only`, `Inconclusive-blocked`, or any
  verdict whose next decision depends on a human-only discriminator or unresolved
  upstream why.

Lead with exactly one plain-language status label: `Confirmed` -> **Confirmed cause**,
`Likely-rooted` -> **Likely cause**, `Proximate-only` -> **Cause not confirmed**,
`Inconclusive-blocked` -> **Investigation
blocked**, `Refuted` -> **Suspected cause ruled out**, and fast-lane SAME ->
**Known recurrence**. The exact internal class may appear once in Technical details
as metadata; do not lead with it. The shape name is metadata, never the title.

Use one of these heading skeletons; omit an optional heading when it has no content.
A Decision Brief may use `Failure path` instead of `Proof`, never both:

```markdown
# <plain-language status label>
<Answer, Confidence when needed, Facts, owner action>
## Proof
## Still open
## Technical details
```

```markdown
# <plain-language status label>
<Answer, visible Confidence, Facts, owner action>
## Failure path
## Evidence Kit
## OCE next checks
## Technical details
```

Both shapes preserve this information hierarchy:

1. **Answer** — one plain sentence stating what happened and the #1 owner-routed
   action, at most 50 words. Summarize the result; do not expand the causal chain here
   when Failure path owns it.
2. **Confidence** — when hedged, capped, or downgraded, keep one visible line naming
   the plain-language cap reason and the evidence that would lift it. Never collapse
   this into Technical details.
3. **Facts** — 2-3 labeled one-liners. Use the existing stable labels:
   confirmed/likely/proximate -> **Impact** and **Fix**; blocked -> **Impact**,
   **Blocked**, and **Do next**; refuted/closure -> **Checked**, **Finding**, and
   **Residual risk**; collaborator/additive -> **Builds on**, **Delta**, and
   **Why it matters**. Do not repeat the answer. When present, compress Operator
   exemplars into these existing Facts or the existing Proof bullets, never both and
   never a new heading; move only unique telemetry to Technical details.
4. **Proof** — represent the mechanism once. A local Decision Brief uses at most
   three unique proof bullets OR a compact failure path; a Mechanism Handoff uses a
   compact failure path. A live incident-management projection with a causal chain always re-projects
   the proof as one compact `Failure path`; it replaces proof bullets rather than
   adding a second representation. For an implementation-backed chain, render exactly
   one code pointer per known hop in causal order:
   `file[:line] · symbol -- input -> output`. A pointer established only from current
   source is `source-inferred`; it does not prove the deployed version, the introducing
   change, or root ownership. Leave unresolved upstream hops and ownership open rather
   than inferring them. A delta-only additive post with no new chain may omit the path.
   Never re-narrate the same mechanism in a callout, alternatives, mitigation, or gaps.
5. **Action and uncertainty** — keep one owner-routed next action prominent. Name
   only the uncertainty that changes confidence, owner, mitigation, or the next
   check.
6. **Evidence Kit** — fallback only for a safe decisive query that cannot map to one
   Failure path node or when no chain exists; place it immediately after the path and
   never duplicate an inline query link.
7. **OCE next checks** — the compact operator-facing projection of the internal
   Manual Investigation Kit; visible when the human-only gate below applies.
8. **Technical details** — progressively disclosed when rendering supports it.
   Include only unique timeline rows, ruled-out items, population caveats,
   provenance, related incidents, references, and raw-query fallback text. Do not
   preserve stage receipts or repeat the answer/proof/failure path.

Keep authored narrative before Technical details at approximately 350 words or less.
This is a prose target, not permission to omit a causal caveat, privacy control,
manual branch distinction, or posting safeguard. Use no more than six report headings;
thin optional fields stay inline instead of creating sections.

Write for a junior OCE. Use short concrete sentences, familiar system nouns, active
verbs, and explicit locations. Translate internal verdict and receipt terms into what
is known, what is still unknown, and which observable result would settle it. Use
plain external labels (`CONFIRMED`, `LIKELY CAUSE`, `CAUSE NOT CONFIRMED`, `BLOCKED`,
or `CHECKED - NOT CAUSE`); the exact internal verdict class may appear once in
Technical details only when useful. The owner action names what to inspect, where,
and what each result means. Vague actions such as "repair or repoint the state" or
"verify recovery" are not actionable.

Evidence pointers use owner-resolvable source terms. On each proving Failure path
node, retain every relevant anchor already carried by its evidence lineage. An
implementation-backed hop uses the canonical code-pointer form above; preserve its
supplied source/ref and code/commit/PR/work-item/incident-management/ADO URLs plus selected
telemetry-query evidence. Preserve supplied URLs exactly; never normalize hosts, guess
an organization/project/repository, or replace a resolvable link with a bare id,
private run id, or prose summary. Omit categories that were not established. A
material code locator remains useful for `Proximate-only` evidence and must survive at
its established confidence. `Closest known introduction / provenance` remains limited
to its verified-cause rule and qualifier discipline below.

For telemetry-query evidence, reuse the `evidence_link` that rode on the proving
OBS/evidence row and place it with the observed result on the exact Failure path node
it proves; when it is the primary proof, keep it on the first proving node. Do not
repeat it in Evidence Kit. The report/post never constructs or encodes that link and
never hunts query manifests to recover it. If the proving OBS has no safe
`evidence_link`, carry de-identified raw query text in Technical details instead and
never fabricate a link. If neither safe link nor attributable query lineage exists,
state the lineage gap at the claim's existing strength. These anchors stay inline;
do not create a standalone evidence section. Surface `TSG/KB consulted` with links
only when those sources were actually used; omit the line when none were used.
Hyperlink related incident ids when an owner-resolvable incident URL is available.

Conditional content:
- Add a highlighted **latent bug / important finding** only for a confirmed defect
  distinct from the primary mechanism. Label real-but-unproven defects plainly; do
  not create a second callout that repeats the primary finding.
- For a Mechanism Handoff, name why the upstream cause remains open. When a cited
  redaction sentinel hides the decisive value, state that the content exists and
  requires an authorized human or source UI; never imply that redacted means absent.
- When the verdict rests on an aggregate, state population coverage inline: fully,
  partially, or not enumerated, with resolved and missing keys. Translate internal
  homogeneity tokens into plain language (for example, "sampled units followed the
  same pattern"); keep the exact token only in Technical details metadata.
- When a competing surface could change ownership or mislead the operator, explain
  the chosen surface in the Answer if verdict-shaping, otherwise under ruled-out
  Technical details. Do not create a standalone section.
- Add **Closest known introduction / provenance** only for verified code/config or
  infra/control-plane rotation causes, using the report-writer qualifier discipline.
- Add an internal **Manual Investigation Kit** when a decisive discriminator genuinely requires
  a human-only or out-of-band capability after the honesty, access, and probe-fitness
  gates. A reachable unchecked lead stays `open-answerable`; a check unreachable to
  everyone produces no kit.

  The internal receipt remains complete: one decisive question, access/effort,
  1-3 read-only steps with verification status and citation, favored and rival
  predictions, discrimination result, owner/action, mitigation, and at most three
  reply-back values. Do not copy that receipt verbatim into the report.

  Render the operator-facing projection as **OCE next checks**:
  1. Keep 1-3 checks and approximately 120 authored words, excluding a
     safety-critical command.
  2. Use `check in source -> result -> meaning/action`; name what to inspect and where.
  3. For a decisive two-result check, keep both favored and rival result arms visible.
     If a result does not choose between causes, say so and route to the next safe
     check.
  4. Use an outcome table only when three or more materially different results make
     inline branches ambiguous.
  5. Mark `unverified` or `missing-citation` guidance as
     `confirm before relying on this step`. Include access/effort only when it changes
     who can run the check.
  6. State mitigation once, warn before a state-changing action, and request at most
     three exact reply-back values.
  7. Keep Kusto links on their proving Failure path nodes when safe; use the fallback
     **Evidence Kit** or Technical details otherwise. Never put them in `OCE next checks`.

  Keep optional rivals and scope checks in Technical details, not the visible checks. For
  no-emission/missing-data/SLA-by-absence symptoms, the decisive question is producer
  eligibility when producer evidence is reachable; a monitor-definition export is
  only a non-blocking confirmation. Brevity never removes a rival meaning, trust
  warning, owner handoff, or state-changing caution. If any required field is missing,
  label the visible checks **incomplete** and name the gap.
- Add a **Duplicate / related incidents** note when the grader recorded a `duplicate-of` or `canonical` classification: render it from `5_grader/ranking.md` and cross-link the sibling incident id(s). This run's verdict follows its own evidence and may differ from the sibling's; do not infer, match, or import the sibling's cause, verdict, or mitigation.
- Add a **Known / ongoing issue note** when the run took the intake fast-lane and the wave-1 duplicate verification returned SAME (see `SKILL.md` § Six-stage flow → Bootstrap / FAST-LANE DECISION and `fast-lane.md`): render the recurrence-family disposition — this incident is the SAME recurring issue as the family (a verified duplicate of the known recurrence family), the live confirming observation (the wave-1 discriminator result for THIS incident), and the engineer next-step. It is a known-issue disposition, NOT a Grader verdict band, and never a `duplicate-of`/`canonical` claim. A SAME disposition carries a cited live confirming OBS and is eligible for a live post under the poster's authorization gate — posted as a collaborator/additive duplicate-reference ("same as the known recurrence family, verified live"), never a standalone re-derived RCA; with no authorization or no non-gated posting capability it is composed into this report-only finalize instead (reusing `6_report/incident-update-with-kit.md`). A wave-1 run that did NOT return SAME does not reach this finalize — it is escalated to the full deep-lane. See `subagents/poster.md` § Live incident-system posting (fast-lane precondition) / § Verdict policy.

### Post status receipt

Report writes one nonempty `6_report/post-status.md` on every terminal path,
including report-only and suppressed posting. This is the canonical posting receipt:

```toon
outcome: posted-verified | already-present | submitted-unverified | report-only | failed
reason: none | <existing gate, suppression, refusal, or failure reason>
incident_id: <provided incident id>
marker: <exact capability-owned run+ordinal marker | none>
ordinal: <orchestrator-provided ordinal>
post_pointer: <owner-resolvable capability-returned pointer | none>
recorded_at_utc: <RFC3339 UTC>
```

Only `posted-verified` and `already-present` are successful live-post outcomes.
`posted-verified` requires marker presence on the authoritative post-submission incident-management
read-back; `already-present` requires an authoritative pre-read match for the same
marker and performs no mutation. `submitted-unverified` means submission may have
occurred but the read-back did not prove marker presence; it is non-success and never
authorizes an automatic repost or ordinal advance. `report-only` records a gate or
suppression before mutation, and `failed` records a definite refusal or failure before
any possible submission. See `subagents/poster.md` § Live incident-system posting for
the lifecycle; do not create a parallel receipt.

### Post mode (collaborate when the incident already has human progress)

Before composing any incident post, classify the incident's discussion thread (captured at intake) for material human progress — a stated root cause, a mitigation, a transfer to an owning team, or a prior RCA — using the grader's adjudication in `5_grader/ranking.md` (lead ledger + verdict) for whether the evidence corroborates or contradicts each human claim, and choose how to contribute:

- **No meaningful human progress** → post the full RCA (or `OCE next checks` when blocked). The default first-investigation case.
- **Human progress present and consistent with the evidence** → collaborator mode: do not restate or re-derive their RCA. Lead with one line that credits and builds on the existing work ("building on <who>'s root cause and <who>'s mitigation; this adds, it does not restate"), then contribute ONLY the genuinely additive finding — the exact mechanism, code/config path, or signal levels the humans stated at a high level; a missing mechanism, blast-radius, or correlation; or a forward-looking preventive gap — plus concrete owner-routed next/preventive actions that are additive, not duplicative, and references tying the additive claims to evidence AND to the thread items they extend.
- **Human progress contradicted by the evidence** → state the contradiction respectfully: name the discrepancy, cite the corroborating evidence, and frame it as a correction to verify — never an override of the owners' decision, and never silently drop an existing mitigation. The contradiction is admissible ONLY when GROUNDED IN THE ALERT'S OWN AUTHORITATIVE SOURCE (the same source that drives the alerted metric), with any timestamp/aggregate conflict against owner-supplied telemetry first reconciled per the owner-supplied-telemetry reconciliation obligation in `references/investigation-invariants.md`; a contradiction computed on an adjacent/non-authoritative surface, or against owner-supplied telemetry not first reconciled (binning/rounding/aggregation/field-semantics), is inadmissible.
- **Human progress present but not yet corroborated** → collaborator mode with explicit corroboration-status labels on each human claim (corroborated / not-yet-corroborated / contradicted).

Never restate or override the humans, and never ignore an existing mitigation. Post mode chooses whether to contribute a standalone RCA versus a collaborative addition; the grader's verdict still bounds the strength of what you assert. Across all branches, the internal Manual Investigation Kit is driven by the grader's verdict (a blocked or `Likely-rooted`-capped decisive discriminator), not by the post-mode branch — its `OCE next checks` projection can accompany a collaborator post.

## `7_knowledge/` — durable knowledge (candidate)

The coordinator always writes this canonical terminal record to `run.md`:

```toon
knowledge_capture:
  status: completed|skipped-no-value|not-dispatched|truncated
  novelty_trigger: <trigger id/summary, or none>
  structural_limits: <one candidate; <=2 cited OBS rows; narrow target read; ~2KB>
  reason: none|no-eligible-route|source-unreachable|read-cap|output-cap
  candidate: 7_knowledge/knowledge.md|none
  consumer_action: candidate-review|none
```

`skipped-no-value` means triage found no evidence-backed durable novelty.
`not-dispatched` means novelty existed but no eligible route/source was reachable; it
writes no `knowledge.md` and is not equivalent to no novelty. `truncated` means the
Curator reached a structural read/output cap; `candidate` points to a grounded file only
when one was safely completed. `completed` may still have `candidate: none` when the
bounded extract/generalize pass rejects every candidate.

`7_knowledge/knowledge.md` is written only when the coordinator's Knowledge Value Triage
(see `SKILL.md` § Six-stage flow) found evidence-backed novelty and dispatched the
Curator. It is compact markdown (not JSON), run-local and candidate-only - it never
mutates curated `services/<svc>/` knowledge. Produced by the Knowledge Curator
(`subagents/knowledge.md`).

Always-sections:
- **Run summary** — one line: service, incident, verdict, and whether new durable knowledge was found. If the Curator hit its read cap before finishing, it emits only a safely completed grounded candidate, if any, and the coordinator records `status: truncated`; a truncated capture never delays or alters the report.
- **Atomic candidate** — at most ONE item with `id`, `kind` (`signature-candidate | reusable-gotcha | knowledge-gap | already-known | follow-up | recurrence-known-issue-candidate`), an interrogative `check`, `applies-when`, `does-not-apply-when`, `status`, `confidence`, `evidence` (eligible claim-integrity row + cited OBS/source summary), `recurrence`, and `freshness`. It is generalized and de-identified only after extracting the incident-bound fact. A `signature-candidate` still requires signature plus recurrence or a reusable mechanism.
- **Proposed KB delta** — at most one suggestion: which `failure-modes/` signature or `observability/` binding a human could add or update, with the evidence to validate first. It remains unapplied; reviewed curation owns promotion.

Empty-but-honest is valid: if nothing durable cleared the bar, state what prior knowledge was reused and stop. Never fabricate a signature to fill the file.

Keep sections concise and omit any section that carries no content. A scoped or partial run remains scoped or partial; do not turn gaps into an all-clear.