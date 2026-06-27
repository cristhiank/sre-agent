# Artifact Contracts

Compact expectations for stage outputs. Shared honesty rules live in
[investigation-invariants.md](investigation-invariants.md); this file only says what each
stage should produce.

Internal artifacts (intake/scout/evidence/grader) cite a stable `OBS###` observation id when evidence exists. The external `6_report/` does not expose `OBS###` ids; it cites the same evidence in plain source terms (observed signal, source type, affected entity, time window, and query/pointer/path when available). Incident-report text,
including reported cause and timeline narrative, stays a claim until corroborated; see the
honesty floor.

## `1_intake/` — capture

Produces `incident-context.md`: the incident record as claims plus pointers.
Include the id, source, declared window, affected/symptom description, and any
reported or claimed cause as **CLAIMS**. Include source links or artifact pointers and
intake gaps. Also capture a discussion-thread summary: the material human comments, transfers, owner notes, prior RCA/mitigation, and linked change/rollout notes — each as a CLAIM with who and when — or an explicit note that the thread was empty or unavailable. The thread is primary orientation evidence, not authority; a human-stated cause or mitigation is corroborated like any other claim. One path, present-or-absent: when a pre-staged intake bundle exists in the run work area, reconcile the staged record + thread against its provenance manifest (per item: source/provenance and complete/redacted/gapped status) and capture them as CLAIMS + pointers with who/when, not verbatim copies; a manifest entry marked redacted, gapped, or absent is a fetch trigger — re-fetch it the authorized way via the read-only incident-record/discussion capability (redacted is not absent; align with the redacted-is-not-absent honesty floor). After that honest fetch attempt, if the source still returns a redaction sentinel, record an ACCESS-BLOCKED LEAD per that floor (name the redacted field and the richer-access surface that can read it), never an absence gap; record a plain intake gap only when the item is genuinely unavailable or empty. When no bundle is pre-staged, fetch the full record + thread the same way. The intent frame, CAPABILITY MAP, and recurrence identity are agent reasoning products, never pre-staged. Bootstrap also produces an intent frame before Scout with exactly three
fields:
- `literal_trigger`: verbatim alert, incident record, or human ask.
- `rca_target`: underlying failure in the measured or impacted operation the signal evidences (see investigation-invariants A1). Specific exception types/mechanisms are discovery findings, never the intake target. Use the smallest evidence-anchored operational failure target that makes the answer remediation-useful; for vague asks with no derivable target, set `rca_target: clarification_required (<ambiguity>)`.
- `success_definition`: what makes the answer remediation-useful for an on-call engineer: evidence of cause/owner/mitigation for those failures, or an evidence-backed clean no-failure closure.
Intake also records the incident's recurrence identity: the stable identifiers that would match sibling incidents when present — signal/error signature, affected operation/component, affected entity/resource/cohort, scope boundary, and owning service/team. This is identity capture for later correlation, not a hypothesis or a cause.
Bootstrap also produces the CAPABILITY MAP before Scout: available capabilities
inventoried, matched to stages, and gaps recorded. For each CRITICAL-EVIDENCE-PATH
capability, include one ACCESS STATUS line:
- `status`: `confirmed` | `unconfirmed-nondiagnostic(self-misuse | wrong-target | auth-cold-start)` | `blocked(hard-absent | denied-after-valid-auth)`
- fields: `canonical_source_read` (help/metadata/error-guidance source), `corrected_invocation_attempted` (yes/no + generic description), `target_dependency: none | guessed | provided | discovered`, `last_error` (short verbatim); for `blocked` only, `human_command` (one concrete command a human/specialist should run).
- Wrong-target, self-misuse, and auth/cold-start failures are non-diagnostic for availability: record `unconfirmed-nondiagnostic`, not `confirmed` or `blocked`. Non-critical capabilities default to `unconfirmed / not-probed` and need no access status.
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
Also names the failing-unit class and the correlation/identity/lineage keys that would
join the symptom to its producing layer, when discoverable, so specialists can enumerate
failing units and follow them (see the "Aggregate is not mechanism" invariant in
[investigation-invariants.md](investigation-invariants.md)).
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
Keep gaps visible. Scout stays neutral: no verdicts and no root-cause language.

## `3_evidence/` — observations

Produces normalized observations, usually under `3_evidence/observations/`, plus any
small index or timeline that helps later readers.
Each observation is one factual statement with a stable `OBS###` id, a source or
provenance pointer, and what entity/time/surface it is about.
Observation/evidence rows include the observation id, factual statement, source/provenance
pointer, entity/time/surface, and optional `evidence_link`. For telemetry-query-backed
observations, specialists capture `evidence_link` from the shareable deep-link the
telemetry-query capability surfaced for the query that produced this observation (its
`DeepLink:` output / run-manifest `deepLink`). Omit `evidence_link` when no shareable
link was surfaced or when the underlying query embeds restricted identifiers; carry
de-identified raw query text in Details instead.
Describe the observation's kind in prose when useful; do not force a fixed taxonomy.
Keep raw rows or bulky artifacts behind pointers.

## `4_specialists/` — theory notes

Each specialist writes only its own stage area and produces per-specialist theory notes.
A theory note names a cause and mechanism, cites the observations it relies on, answers
or leaves open the scout questions it addressed, and lists gaps.
It may include alternatives or negative evidence, but should not overstate beyond the
observations it cites.
A theory note also pre-registers its discriminator — predicate plus expected
favored/rival stated and kept visibly separate before the checked value (with the OBS id
when checked) — and ends with the compact claim-readiness ledger (failing-population
bound / mechanism named / discriminator pre-registered / observed value checked / result
/ confidence ceiling).

## `5_grader/` — judged assessment

Produces `ranking.md`: a judged verdict in one of `Confirmed` / `Likely-rooted` /
`Proximate-only` / `Inconclusive-blocked` / `Refuted`, with rationale and the
observations relied on.
Keeps a lead ledger (each material lead closed as `closed-supported` /
`closed-refuted` / `open-answerable` / `blocked-unreachable`) so no lead is silently
dropped.
Records a duplicate classification (`canonical` / `duplicate-of <incident>` / none) when the scout reports a recent or concurrent sibling sharing the recurrence identity, and applies the verdict-determinism rule so such siblings are judged by the same verdict gate (see `grading-rubric.md`).
When `Proximate-only` with `open-answerable` leads, also writes
`refinement-obligations.md` for the coordinator's pursuit loop (budget ~2-3 rounds),
carrying per obligation: id; lead id(s); pre-registered discriminator predicate +
expected favored/rival; acceptable evidence shape; in-hand keys; prior OBS ids with
reuse/freshness mode; stop condition; dependency-ids/independence-reason; and allowed
outcome statuses (answered/invalid-premise/unanswerable). Independent obligations are
dispatched as one awaited parallel-sync wave (one specialist per independent
discriminator), counting as one focused follow-up round; when `blocked-unreachable`, records the dead-end
plus an engineer suggested next step.
Discovery receipt for causal `blocked-unreachable` leads, a final `Proximate-only` whose unresolved upstream/mechanism lead still had reachable in-hand keys, or clean/no-failure target closures:
- `needed_evidence_type`: metric | log | trace | config | code | other
- `candidate_provenance`: observability/service-knowledge reference, catalog/schema result, config/code reference, query history, dashboard pointer, or user-provided; invented or guessed targets remain nondiagnostic and are not probed
- `observability_service_knowledge_lookup`: result or explicit absence
- `schema_catalog_discovery_attempted`: source + scope + result
- `log_vs_metric_distinction`: why available evidence is insufficient and what evidence type is needed
- `access_result`: the result of an actual canonical probe attempted against the source that answers this lead — a correct/authed/non-guessed query result, or a recorded terminal auth/missing-command/schema error. A documentation gap (a signal/table/source named-but-not-confirmed) is not an access result and cannot satisfy `blocked-unreachable` (see the access invariant in [investigation-invariants.md](investigation-invariants.md)). To record a source unreachable, the terminal error must be from an authenticated probe: auth-denied, missing-source, or schema-absent qualify; a soft or unnamed "unavailable", a time-budget constraint, or assumed provider ownership do not. If no authenticated probe was attempted and a reachable in-hand-keyed source remains, the receipt is incomplete and the lead stays `open-answerable`.
- `evidence_source_probed`: which source/endpoint was actually queried for this lead. An unfit or never-probed source — including an unprobed service telemetry source the lead needs — leaves the lead `open-answerable`, not blocked; a fit-for-purpose probe of the source that answers the lead which comes back genuinely empty is `reached_empty` — it rules out only what a fit probe in the correct scope/window rules out (it may close/refute on the merits when fit) but NEVER licenses a block per "Empty is not absent"; only a `denied` probe licenses `blocked-unreachable`.
- `probe_fitness`: `fit=yes|no`; the claim being supported; signal/source used and why it observes that claim; basis (documented source, schema/catalog discovery, runbook, owner hint, or explicit rationale); claim scope vs queried scope (+ justification if narrower); result. A guessed signal or unjustified sub-window is `fit=no`; a lens that cannot observe the symptom class (an error-/failure-level filter for a latency/late-completion symptom) is `fit=no`; a `fit=no` result is nondiagnostic and cannot support `blocked-unreachable` or clean closure.
- `correlation_follow_through`: discriminating ids/joins the symptom exposed (if any), whether followed to the next causal layer, and the result or why-not.
- `verdict_ceiling_lead`: the open causal lead targeting `rca_target` whose reachability sets the verdict ceiling (its block would downgrade the verdict, e.g. Likely-rooted → Proximate-only). Named by lead id; if two co-equal leads each set the ceiling, each carries its own receipt (cannot split to dodge). Non-decisive/contributing leads and `closed-supported`/`closed-refuted` dispositions carry no typed receipt and settle as today.
- `reachability_outcome`: the typed resolution, for the `verdict_ceiling_lead`, that the `access_result` / `evidence_source_probed` / `probe_fitness` / `correlation_follow_through` evidence above consolidates into — exactly one of: `reached` (probe issued, nonempty result; cite {call_id, result handle}) · `reached_empty` (probe issued, zero rows; states what a fit probe in the correct scope/window RULES OUT — empty ≠ absent; weighed on probe fitness, NEVER licenses a block, and does not auto-refute) · `denied` (probe issued, returned an access/permission/missing-source/schema-absent error — or a returned redaction sentinel; cite {probe call_id, verbatim returned error/sentinel string}) · `discovery_exhausted` (bounded discovery found NO capability-map-named next-causal-layer source AND no schema/catalog-discovered source/key/path consuming the pivot dimensions — a DEMONSTRATED exhaustion of the causal chain, not a reachability denial; MUST cite the capability-map line / explicit absence with the same provenance rigor as `generic_pivot_ladder`, plus any schema/catalog discovery call_id+result) · `unattempted_open` (no probe issued; the lead stays `open-answerable`). There is NO `unattempted_blocked` member — "I didn't try" projects only to `unattempted_open`, so a 0-probe "no access established" block is unrepresentable; a bare "I couldn't find a source" assertion (no cited capability-map absence, budget not actually spent) is likewise `unattempted_open`, never `discovery_exhausted`. Only `denied` (cited returned error/sentinel + call_id) or `discovery_exhausted` (cited capability-map absence, bounded budget actually spent) licenses `blocked-unreachable` or a Proximate-only-due-to-block cap; `reached_empty` and `unattempted_open` CANNOT cap and keep the lead open. Reuse-before-probe: a prior in-run probe artifact (call_id) for that same evidence source already satisfies the access component — no new query.
- `discriminator_rung`: the dispositive ladder rung(s) this receipt resolves — the typed close that the `access_result` / `evidence_source_probed` / `probe_fitness` / `correlation_follow_through` / `reachability_outcome` evidence above consolidates into, exactly as `reachability_outcome` already consolidates the access sub-fields. Per rung: `dispositive` (decisive-for-verdict; orientation-proposed, grader content-verified), `observed_identity{scenario, signal, owner, correlation_key}` (each projected non-null from an authoritative probe, or `n/a-not-exposed` with cited justification where the capability does not expose it), and `status` (`open | closed_confirmed | closed_refuted`). Rung close/refute rules live solely in grading-rubric.md (Gate A); this field RECORDS the Gate A result — the cited identity projection and the resulting status — and does not restate the rule.
- `promotion_precondition` (Gate B): records the grader's Gate B evaluation per grading-rubric.md — the dispositive rung ids + statuses, each open dispositive rung's `reachability_outcome`, and the dispatch/cap disposition. Do not restate the rule here.
- `remaining_gap`: what evidence would change the RCA
- `budget_used`: which bounded discovery steps were spent
- `generic_pivot_ladder`: the time ∧ affected-scope/entity ∧ operation/step pivot was attempted into a capability-map-named next-causal-layer source (source + result), OR a terminal `no such source named` — and that terminal value MUST cite the CAPABILITY MAP line / explicit absence it was derived from (same provenance rigor as `access_result`/`probe_fitness`: a bare assertion of absence is invalid, exactly as a documentation gap cannot satisfy `blocked-unreachable`).
- `in_hand_branches_dispositioned`: `yes` (list each zero-cost in-context branch + the OBS id / observation that disposes it) | `n/a-with-reason`.
- `signal_validity`: CONDITIONAL — when the trigger is a measurement/alert/monitor signal, disposed via the existing signal-validity / alert-semantics capability → `real-failure | evaluation-artifact | unresolved`; when the trigger is not signal-shaped, record `n/a-trigger-not-signal-shaped`. An `unresolved` value is an `open-answerable` lead only when it meets the materiality bar (could materially change the verdict); otherwise it is a confidence-capping gap, never by itself a basis for `Inconclusive-blocked`.
- For a final `Proximate-only`-with-reachable-keys settle, the load-bearing fields are `correlation_follow_through`, `evidence_source_probed`, `remaining_gap`, and `budget_used` — showing the cross-source pivot was spent or why no reachable next-causal-layer source exists; `access_result`/`probe_fitness` apply only once that pivot is actually probed, so a receipt marking them N/A without a spent pivot does not license the settle.
Detailed judging rules live in `grading-rubric.md`.

## `6_report/` — bounded RCA

Produces a concise external-facing report bounded by the grader verdict. No internal
observation ids; cite material claims in plain source terms. Never copy unredacted customer content that the run obtained by bypassing a source redaction (anything marked do-not-republish / enumerated by a run `pii-marker`) verbatim into the report — the report is projected into the incident post. Carry such content only as DE-IDENTIFIED facts (service/operation terms; customer-identifying values by category + count, never verbatim), per "Do not republish redacted customer content" in [investigation-invariants.md](investigation-invariants.md). A material numeric/aggregate
claim carries its source and a coverage caveat when the figure is partial, sampled, or
measured differently by another source over the same window.
<example>"N per <source A>; <source B> showed M, same window — unreconciled"</example>

In iteration mode (`followup.md`), the report and any incident post are a delta/update —
what changed since the last iteration plus the updated verdict (including honest
downgrades) — bounded by the current grader verdict.

Always include content-bearing sections that project cleanly into the Hybrid incident post. The report keeps
full depth, but its shape maps to:
1. **Band answer** — verdict-bounded plain failure statement plus the #1 owner-routed action; short enough to
   project as the TL;DR / verdict band.
2. **Facts header inputs** — verdict-shaped labeled facts. Use exactly these label sets: confirmed/likely/
   proximate -> **Impact** and **Fix**; blocked -> **Impact**, **Blocked**, and **Do next**; refuted/closure
   -> **Checked**, **Finding**, and **Residual risk**; collaborator/additive -> **Builds on**, **Delta**, and
   **Why it matters**. Keep the verdict in the band answer, not repeated in these labels.
3. **Failure path** — an indented causal tree with evidence pointers on proving nodes. This is the single
   mechanism representation; do not add a separate Mechanism section.
4. **Manual Investigation Kit** — required in full when a decisive discriminator needs a human-only or
   out-of-band capability and the verdict is blocked or manual-handoff-capped; otherwise continuation notes
   can live under Details.
5. **Details** — full-depth timeline (`time | what happened | entity/node | effect`, timing labeled `exact`,
   `approximate`, or `unknown`), ruled-out items, provenance, duplicate/related incidents, sources/references,
   and raw-query fallback text.

Evidence pointers use owner-resolvable source terms. For telemetry-query evidence, reuse the `evidence_link`
that rode on the proving OBS/evidence row; the report/post never constructs or encodes that link and never
hunts query manifests to recover it. If the proving OBS has no safe `evidence_link`, carry de-identified raw
query text in Details instead and never fabricate a link. Surface `TSG/KB consulted` with links only when
those sources were actually used; omit the line when none were used. Hyperlink related incident ids when an
owner-resolvable incident URL is available.

Conditional sections:
- Add a highlighted **latent bug / important finding** callout when the run confirms a real code/config defect, even if not proven as this incident's trigger; label real-but-unproven when applicable.
- Add **Why we couldn't pin the cause** for `Proximate-only` / `Inconclusive-blocked`, naming the missing evidence in plain terms. When a verdict rests on a present-but-redacted field (a cited returned redaction sentinel, not genuine emptiness), the TL;DR and this section state that the content exists and an authorized human or the source's own UI can read it — never implying no context exists (per "Redacted is not absent" in [investigation-invariants.md](investigation-invariants.md)).
- Add a **Population coverage** line whenever the verdict rests on an aggregate/population signal — rendered from the grader's failing-unit enumeration result (the claim-readiness ledger / `5_grader` verdict, not re-derived): state whether the failing population is enumerated, partially enumerated (name the dimensions/keys resolved and those still missing), or not enumerated/blocked (name the missing dimensions). One line for a fully-enumerated cause. When a population-decomposition pass ran, also carry the representative drill-down result and the homogeneity label (`shared-pattern-supported` / `multiple-cohorts` / `per-entity-heterogeneous` / `unknown-insufficient` — a population-pattern-support label, NOT verdict confidence) with the sample basis/caveat for the OCE.
- Add a **Why this surface** line whenever the grader's target-alignment/discovery receipt records a competing surface that was salient in this incident — named in intake, the discussion thread, incident title/message, a cited TSG/KB or dashboard, a sibling incident, or adjacent telemetry — and that difference changes the owner-routed action or could mislead the operator. Render from the grader's receipt and failing-unit enumeration in `5_grader/ranking.md`; do not re-derive surface selection in Report. Skip when the chosen surface is uncontested. Line shape: `Why this surface: <competing frame> was salient, but <authoritative failing population/scope> drove the verdict because <one evidence pointer>; <adjacent surface> is <separate / less material / unproven> for this incident.` Project this line into the Band answer when it is verdict-shaping, or into the ruled-out block in Details when it is a secondary framing note; do not add it as a standalone section. If a human named the competing surface, handle it through Post mode (§ Post mode below) as corroboration/contradiction/additive context.
- Add **Closest known introduction / provenance** only for verified code/config causes, using the report-writer qualifier discipline.
- Add an **Manual Investigation Kit** whenever a decisive discriminator needs a human-only or out-of-band capability the agent lacked — whether that leaves the verdict `Inconclusive-blocked` or caps it at `Likely-rooted` — and only after the honesty, access, and probe-fitness gates establish it genuinely unreachable (a reachable unchecked lead stays `open-answerable`, not a kit; a block unreachable to everyone, with no human-executable check, needs no kit). Derive it from the specialist-produced core manual check (target/capability, required access, action, predicate, expected→meaning, written to `4_specialists/<name>/theory.md`), grader-adjudicated in `5_grader/ranking.md` (discovery receipt) and report-assembled, plus `5_grader/refinement-obligations.md` when present; it is required, not optional, when it applies. The grader carries the decisive parts (decisive check, operator steps, mitigation, reply-back); the report frames the block reason + delegated capability and appends optional non-blocking rivals. For a no-emission / missing-data / SLA-by-absence symptom, the report classifies the producer-run/eligibility lifecycle state for the OCE, and the Manual-Kit decisive predicate is the producer-eligibility question (not the monitor null/missing-bucket definition) whenever producer-side evidence is reachable — the monitor-definition export is rendered as a non-blocking confirmation / fix-authoring item, not the decisive check. Order:
  1. **Block reason + delegated capability** — one line: what is blocked and the capability the on-call engineer has that the agent lacked.
  2. **Decisive check first** — the single discriminator that flips the verdict, stated as a predicate.
  3. **Operator-executable steps** — each a read-only / least-privilege diagnostic action an authorized human can run (command, query, API call, portal/GUI navigation, dashboard view, runbook/handbook procedure, or owner/state-reader request), with its required access/elevation, the exact action, and expected-observation → meaning branches (confirms / refutes / alternate mechanism / inconclusive); route any state-changing action to **Mitigation** (step 4), never into the decisive check.
  4. **Mitigation once confirmed** — or an explicit `no verified mitigation known; escalate to <owner/capability>`.
  5. **Optional rivals / scope checks** — explicitly non-blocking, with rough effort.
  6. **Reply-back** — the exact evidence the on-call engineer should reply with on this incident: a value, export, query result, or owner confirmation. Do not mention the agent's run, re-run, run path, or workspace; the incident reply is what continues the investigation (see `followup.md` and the self-contained-post invariant in [investigation-invariants.md](investigation-invariants.md)).
  Each operator step carries its verification + citation status — `verified-with-citation` against the service knowledge and the authoritative operational source/runbook/handbook for that action (when service knowledge points to one), `unverified`, or `missing-citation` — and an `unverified`/`missing-citation` step is rendered as such, never as a trusted copy-paste command (see the producer obligation in `specialists/AGENTS.md`). A `verified-with-citation` step is sourced, not proven safe to run blind: lead the kit with a one-line caution (`AI-authored — validate against current state before running any elevated or state-changing step`), and render any elevated or state-changing step for the operator to validate before running. If a decisive blocked discriminator exists but the kit lacks the decisive predicate, an executable/manual step, expected→meaning branches, or verification/citation status, render the kit as **incomplete** with the missing fields named — do not bury it in generic next actions.
- Add a **Duplicate / related incidents** note when the grader recorded a `duplicate-of` or `canonical` classification: render it from `5_grader/ranking.md` and cross-link the sibling incident id(s). This run's verdict follows its own evidence and may differ from the sibling's; do not infer, match, or import the sibling's cause, verdict, or mitigation.

When live posting is authorized, Report also records the capability-owned post outcome in the run report artifacts for later-iteration idempotency/audit continuity (see `subagents/poster.md` § Live incident-system posting).

### Post mode (collaborate when the incident already has human progress)

Before composing any incident post, classify the incident's discussion thread (captured at intake) for material human progress — a stated root cause, a mitigation, a transfer to an owning team, or a prior RCA — using the grader's adjudication in `5_grader/ranking.md` (lead ledger + verdict) for whether the evidence corroborates or contradicts each human claim, and choose how to contribute:

- **No meaningful human progress** → post the full RCA (or the Manual Investigation Kit when blocked). The default first-investigation case.
- **Human progress present and consistent with the evidence** → collaborator mode: do not restate or re-derive their RCA. Lead with one line that credits and builds on the existing work ("building on <who>'s root cause and <who>'s mitigation; this adds, it does not restate"), then contribute ONLY the genuinely additive finding — the exact mechanism, code/config path, or signal levels the humans stated at a high level; a missing mechanism, blast-radius, or correlation; or a forward-looking preventive gap — plus concrete owner-routed next/preventive actions that are additive, not duplicative, and references tying the additive claims to evidence AND to the thread items they extend.
- **Human progress contradicted by the evidence** → state the contradiction respectfully: name the discrepancy, cite the corroborating evidence, and frame it as a correction to verify — never an override of the owners' decision, and never silently drop an existing mitigation.
- **Human progress present but not yet corroborated** → collaborator mode with explicit corroboration-status labels on each human claim (corroborated / not-yet-corroborated / contradicted).

Never restate or override the humans, and never ignore an existing mitigation. Post mode chooses whether to contribute a standalone RCA versus a collaborative addition; the grader's verdict still bounds the strength of what you assert. Across all branches, the Manual Investigation Kit is driven by the grader's verdict (a blocked or `Likely-rooted`-capped decisive discriminator), not by the post-mode branch — it can accompany a collaborator post.

## `7_knowledge/` — durable knowledge (candidate)

Written only when the coordinator's Knowledge Value Triage (see `SKILL.md` § Six-stage flow) found evidence-backed novelty; otherwise the run records `knowledge_capture: skipped — no durable novelty/value` in `run.md` and writes nothing here. One file, `knowledge.md`, compact markdown (not JSON), run-local and candidate-only — it never mutates curated `services/<svc>/` knowledge. Produced by the Knowledge Curator (`subagents/knowledge.md`).

Always-sections:
- **Run summary** — one line: service, incident, verdict, and whether new durable knowledge was found (or the skip record). If the Curator hit its read cap before finishing, it emits the candidates found so far and records `knowledge_capture: truncated` (partial capture — post-verdict and non-blocking, parallel to the `skipped` state); a truncated capture never delays or alters the report.
- **Items** — each carries `kind` (signature-candidate | reusable-gotcha | knowledge-gap | already-known | follow-up), `status` (verified | probable-unverified | single-incident-candidate), `confidence` (high|medium|low), service + component/scope + symptom, the claim, **evidence** (OBS id + a one-line source summary — external, never introspection), `recurrence` (sibling incidents or prior-signature link, or `single-incident candidate`), and `freshness` (run date). A `signature-candidate` must pass the two-key rule (signature + recurrence-or-reusable-mechanism); `already-known` names only material reused knowledge. Gate D — a `signature-candidate` from a rooted discriminator ladder writes/updates the canonical signature template (the pre-registered rungs plus their favored/rival values) so it feeds CP-1 on the next sibling, and records the sibling-reconciliation outcome (`converge | split | residual`, per CP-4 in [grading-rubric.md](grading-rubric.md)) on that item.
- **Proposed KB delta** — a suggestion list only: which `failure-modes/` signature or `observability/` binding a human could add or update, in the service's template, with the evidence to validate first. Explicitly NOT applied by this run; cross-run dedup and promotion into curated knowledge happen in a separate reviewed curation step.

Empty-but-honest is valid: if nothing durable cleared the bar, state what prior knowledge was reused and stop. Never fabricate a signature to fill the file.

Keep sections concise and omit any section that carries no content. A scoped or partial run remains scoped or partial; do not turn gaps into an all-clear.