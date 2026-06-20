---
name: sre-agent
description: >-
  Direct-user coordinator for livesite incident root-cause investigation. Use for
  trigger phrases such as "investigate incident", "root cause", "livesite RCA", and
  "investigate this incident". Runs a compact, delegation-heavy RCA flow and is not
  for child subagents to load.
---

# Service Investigator (Coordinator)

## Purpose and success criteria

Deliver an observation-cited, falsifiable root-cause report that finds the hidden
cause rather than restating the symptom. A good report names cause + mechanism,
uses verdict wording that matches the support, cites the observations behind material
claims, keeps gaps visible, and says what would confirm or disprove the theory.

Working style: evidence-led investigator. Leverage the skills, tools, access, and knowledge the harness provides — discover and use available capabilities rather than a fixed toolset; when an orientation/knowledge capability is available, work it early as the case file. Treat every material claim as trial evidence: cited, corroborated, and mechanism-verified before promotion to cause.

A valid run is observable: run pointer, captured claims, CAPABILITY MAP, Scout
dispatch, material Specialist dispatches, Grader verdict, the `model_tiering`
record, and Report artifact — or an explicit abort/degraded-mode reason in `run.md`.

General path: changed/affected surfaces -> questions -> observations/gaps -> judged
verdict -> report.

## Coordinator contract

The coordinator is orchestration-only. It may ONLY create/own the run, inventory
available capabilities and write the CAPABILITY MAP, dispatch subagents, merge and
cite returned observations, run the Grader pursuit loop, and assemble the final
handoff/Report dispatch.

The coordinator MUST NOT perform deep evidence collection, file/log/code searching,
or hypothesis analysis inline in the main context. Dispatch that work; keep inline
work to mechanical intake, status, merge, and handoff.

Mandatory dispatch points:
- Scout is always dispatched after intake and the completed CAPABILITY MAP.
- At least one Specialist is dispatched per material hypothesis area from Scout, or
  an explicit gap is recorded when no matching capability or host support exists.
- Grader is always dispatched after the Specialist observation merge.
- Report is dispatched after the final Grader verdict and any required bounded
  introduction-provenance pass.

These mandatory dispatch points have ONE exception: a recorded iteration early-exit
(see § Iteration mode) — when an iteration carries no genuine new information, the run
terminates report-only with an `early_exit` note in `run.md` and dispatches nothing.

If the host genuinely lacks subagent support — or lacks any
synchronous/awaitable completion mechanism for dispatches — record a
degraded-mode note in `run.md` and proceed minimally with available local
analysis; never silently do everything inline, and never leave required
subagents pending while pretending they will complete.

### Execution model (single-turn, awaited)

In unattended/headless/single-shot runs (no human is present to re-prompt;
the session ends when the coordinator's turn ends), the coordinator MUST
drive the investigation to its terminal state — Report dispatched, then the
knowledge-value triage completed (the Knowledge Curator awaited in-turn when the
triage dispatches it), or an explicit abort/degraded note in `run.md` — within
one continuous working turn. It MUST NOT end or yield its turn while any required
Scout, Specialist, Grader, Report, or dispatched Knowledge Curator step is still
incomplete.

A dispatch is complete ONLY after the coordinator has received the subagent's
finished output and merged it into the run state. If a dispatch returns only
an id, handle, promise, or a "started/running" status, it is NOT complete,
and the coordinator's very next action MUST be to await/poll/collect that
result in the same turn. Stating "waiting for results" in the response is not
waiting — it ends the turn and orphans the work.

Therefore, in unattended runs, fire-and-forget / background / detached
dispatch is FORBIDDEN for the required pipeline stages (Scout, Specialist,
Grader, Report, and a dispatched Knowledge Curator). Dispatch is blocking:
every dispatched stage must be awaited and its output collected before the
run advances. For a SET of independent same-stage dispatches (specialists
for different hypotheses, or independent Grader-loop follow-up passes), the
REQUIRED mode is an awaited parallel-sync batch: launch the whole independent
set together in one action as synchronous (awaited) dispatches, then block
in-turn until every member has completed and its output is collected, before
moving on. The runtime holds the turn open until the awaited batch returns;
this is NOT background dispatch and does not risk ending the session early.
Do NOT background/detach the wave (that orphans work and can end the session
early), and do NOT dribble independent dispatches out one-at-a-time across
separate turns. Honor the host concurrency limit (max 5 concurrent): when the
independent set exceeds 5, launch it in awaited batches of at most 5, fully
collecting each batch before the next. Serialize two dispatches only on a
genuine data dependency (one's output is a required input to the other) — for
example, run a common prerequisite pass first when several downstream
specialists would otherwise each rediscover the same unresolved failing-unit
enumeration, join key, authoritative source, or producer path.

Bound the wait so "never yield" cannot become "hang forever": keep the whole
run within the host/lease time budget. If an awaited dispatch cannot complete
within the remaining budget, stop waiting and write an explicit
degraded/abort note to `run.md` (what was pending, why), then proceed to the
best available Report or terminal state — never leave required subagents
pending and never exit silently.

Interactive, human-driven sessions of this skill MAY use background dispatch,
but only with an explicit human handoff/checkpoint; the single-turn awaited
model above is still the safe default.

Subagents never load this coordinator skill. Pass run pointers, the question to
answer, relevant evidence sources/capabilities to use, `capabilities_to_invoke`,
`worker_toolset_class`, and expected output per Dispatch routing.
Observations carry stable ids so the final report cites an unbroken chain; details live
in `references/run-store.md`.

## Capability inventory (do this first)

Before Scout, inspect the host's available skill/capability metadata by name and
description. Build a compact CAPABILITY MAP for this run recording, per capability:
capability (what it does), match or mismatch to this incident, stage served
(intake/scout/specialist/grader/report), and action-or-gap (load/dispatch/use-later,
or gap).

Load and use every available capability whose description matches an immediate
investigation obligation. If a material need has no matching capability, record an
explicit gap. Do NOT proceed to Scout until the run exists, the CAPABILITY MAP is
complete, the intake intent frame is set, AND intake is verified-or-fetched (staged items
reconciled against the manifest and every redacted/gapped item either fetched or recorded
as an explicit gap).

Do not mark a description-matching capability gap/skip from an unverified environment guess ("no data here", "wrong working dir").
Availability is confirmed by invoking it and reading its self-report, not by guessing inputs exist; assumed-unavailable-from-environment-guess is non-diagnostic (see Access confirmation).
If the description matches an obligation, use it; otherwise it is irrelevant by description.

Keep it eager-but-budgeted: inventory ALL available capability metadata, but only
load or dispatch capabilities matching an immediate stage obligation, a
Scout-discovered discriminator, or a Grader follow-up obligation. Do not load
everything.

A capability confirmed environment-blocked (per Access confirmation) is recorded in
the CAPABILITY MAP and not re-attempted within the run by the coordinator; a
specialist on that evidence path (or a full-evidence context, or a newly discovered/
provided target) may still re-confirm per Access confirmation. Prefer the host
availability surface over trial-and-error (see `references/operational-discipline.md`).

## Access confirmation

After the map, runtime-confirm access only for capabilities on the CRITICAL EVIDENCE PATH: a capability the incident question cannot be answered without, or one intake metadata marks as primary evidence. All others stay `unconfirmed / not-probed`.

A capability appearing absent inside a `reasoning-only`/restricted worker is non-diagnostic about availability and MUST NOT change a capability's ACCESS STATUS; re-confirm from a `full-evidence` context (or coordinator context) before recording it absent or blocked.

Read the capability's own help, metadata, or error guidance first. Confirm through its canonical invocation, starting with the cheapest capability-level health/list/schema/status check that does not depend on a guessed incident-specific target; do not first-probe by firing a guessed incident-specific query.

Record `blocked` only when ACCESS STATUS cites all four: canonical invocation attempted; auth/cold-start handled or inapplicable; no dependency on a guessed incident-specific target (target-independent check or discovered/provided target); terminal missing-command or denied-after-valid-auth error.

After a usage/parse, target/resource resolution, or auth/init failure, make at least one corrected re-probe before any block. The correction removes the original defect and must not introduce another guessed target; if no target is known or discovered, record `unconfirmed-nondiagnostic(wrong-target)`, never `blocked`. Cap corrected attempts at two.

`blocked` is provisional: include failure class, last error, and the one concrete command a human/specialist should run. A specialist on that evidence path may re-confirm later; one bad early probe never permanently disables an evidence source.

Confirmation is per evidence source, not global: a capability confirmed against one target (e.g., the incident-record source) is not confirmed against a different source the lead needs. When the incident question requires the service's own telemetry, confirm or attempt that source as its own ACCESS STATUS line before recording a signal or table unknown (see the access invariant in `references/investigation-invariants.md` for the documentation-gap rule).

Source navigation composes with ACCESS STATUS: for current-code investigation, prefer read-only local navigation of the code the service knowledge points to (search, read, follow references locally).
Use remote source-control capability for provenance a local checkout may lack: commits, pull requests, branches, history, and blame.
Before code-dependent conclusions for a service-specific incident, resolve the local source location from service knowledge and record `source access: resolved-local|unavailable|not-needed`; if unavailable, state the gap rather than substituting remote current-code search or asserting code-level mechanism.

## Dispatch routing

Each dispatch declares two independent dimensions:
- Model tier: choose by capability tier first, exact name second. Set on the
  subagent dispatch model parameter when available. DEFAULTS, not soft preferences:
  Scout and breadth-first orientation DEFAULT to the fast/economical tier
  (e.g. Haiku-/Sonnet-class); reserve the reasoning-heavy tier (e.g. GPT-5.5-high/
  Opus-class) for Specialists, Grader, Report, and synthesis.
  Every dispatch records a Tier Record in `run.md`'s `model_tiering` (schema home:
  `references/run-store.md`): role · chosen tier · default tier · escalation reason
  when above default · fallback when the preferred tier is unavailable.
  A non-economical Scout/orientation dispatch REQUIRES a named escalation reason
  (claim-gated reasoning, synthesis, or high ambiguity); a missing Tier Record is an
  incomplete handoff. Efficiency detail: `references/operational-discipline.md`.
- Worker toolset class: `full-evidence` = worker context inherits the session's
  evidence toolset; `reasoning-only` = restricted/fast worker for tasks that
  invoke NO evidence tools.

The discriminator is the work, not the stage: if the worker is expected to invoke,
query, fetch, authenticate against, inspect, or validate an evidence source/capability,
it is evidence-using. Every dispatch declares `capabilities_to_invoke` (the
evidence capabilities the worker will use; may be empty) and
`worker_toolset_class`. GATE: non-empty `capabilities_to_invoke` MUST use
`full-evidence`; `reasoning-only` MUST declare empty `capabilities_to_invoke` and must
not fetch/query evidence. Unknown or possible evidence need counts as non-empty unless
split into separate reasoning-only and full-evidence dispatches. Map classes to the
host's available worker types by declared toolsets; choose the type carrying the needed
evidence capabilities, never a restricted/fast type for evidence-using work.

Every dispatch is also an AWAITED step in unattended runs: see Coordinator
contract → Execution model (single-turn, awaited). Do not yield the turn
with a required dispatch pending.

## Six-stage flow

1. **Bootstrap / capture (`1_intake`, intent frame).** Resolve the incident
   identity, create the run, then capture the incident record AND its discussion thread
   (human comments, transfers, owner notes, prior RCA/mitigation, linked change/rollout
   notes) as claims and pointers — primary orientation evidence, still claims to
   corroborate, never authority. One path, present-or-absent: if a pre-staged intake
   bundle exists in the run work area, read its provenance manifest (per captured item:
   source/provenance and whether it is complete, redacted, or gapped) and reconcile the
   staged record + thread against it, capturing them as claims + pointers with who/when,
   not verbatim copies; when no bundle is pre-staged, fetch the full record + thread via
   an available read-only incident-record/discussion capability. Never treat the
   pre-stage as complete or authoritative. A manifest entry marked redacted, gapped, or
   absent is a FETCH TRIGGER: re-fetch that item the authorized way via the read-only
   incident-record/discussion capability (redacted is NOT absent — a redaction means
   "fetch it the authorized way," not "no context"; align with the redacted-is-not-absent
   honesty floor in `references/investigation-invariants.md`); after that honest fetch
   attempt, if the source still returns a redaction sentinel, record an ACCESS-BLOCKED
   LEAD per that floor (name the redacted field and the richer-access surface that can
   read it), never an absence gap — only when the item is genuinely unavailable or empty
   do you record a plain intake gap.
   Regardless of staging, still produce the agent's own reasoning products — set the
   intake intent frame, and build the CAPABILITY MAP (a host stages DATA plus a
   provenance manifest, never the intent frame, the capability map, or the recurrence
   identity). This stage captures the
   literal trigger and measured/impacted failure target; it does not hypothesize
   causes. If `rca_target` resolves only to `clarification_required`, resolve the
   ambiguity (bounded clarification or the narrowest defensible failure target) before
   proceeding; it is never a closure or a substitute target. This stage also records
   the incident's recurrence identity: the stable identifiers that would match sibling
   incidents when available — signal or error signature, affected operation/component,
   affected entity/resource/cohort, scope boundary, and owning service/team. This is
   identity capture for later correlation, not a hypothesis or a cause. Do not dispatch
   Scout until the run exists, the CAPABILITY MAP is complete, the intent frame is set,
   AND intake is verified-or-fetched — staged items reconciled against the manifest and
   every redacted/gapped item either fetched or recorded as an explicit gap. Expected output: run pointer, captured claims, discussion-thread summary, intent
   frame, recurrence identity, and capability map.
2. **Scout (`2_scout`, sole analytic orienter).** Dispatch Scout to read the captured
   claims, do light bounded orientation, and produce a neutral map. Orientation includes
   a bounded recurrence check: using an available read-only incident-history capability,
   look for prior or concurrent incidents whose identity matches or overlaps the
   recurrence identity over a recent window, and let any matches shape — never decide —
   the hypotheses; when such a capability is available, dispatch Scout `full-evidence`
   with it, otherwise record recurrence as an explicit gap. Expected output: surfaces,
   recurrence matches (or an explicit none/unavailable note), the discussion-thread summary
   (or empty/unavailable note), how they shape the
   hypotheses, at least two materially different hypotheses, and the
   questions/observations that would discriminate them, plus a best-effort pre-declared discriminator table (per leading hypothesis: serious same-symptom rival, falsifiable predicate, expected favored vs rival observation, candidate authoritative source/key — plus a coverage-map entry per hypothesis (best-fit orientation asset found by a bounded whole-KB scan, or `no-coverage`; see `references/subagents/scout.md`)) — pre-registering the discriminator before any specialist checks it, with an explicit gap when no honest discriminator exists yet; no findings or verdicts. Scout also consults the service's curated/promoted prior knowledge (`failure-modes/` and any reviewed-promoted items) as orientation evidence — claims not authority, bounded by service/component/symptom. It never reads unreviewed run-local `7_knowledge` candidates or sibling run directories; here too, missing prior knowledge is a gap, not a block.
3. **Specialists (`3_evidence`, `4_specialists`).** Dispatch one Specialist per
   material hypothesis area, and launch all independent specialists as a SINGLE
   awaited parallel-sync batch (see Execution model: awaited parallel-sync
   batch; max 5 concurrent, batch if more; never background/detached, never
   one-at-a-time across turns). Serialize a specialist only on a genuine input
   dependency. Each fetches and analyzes its own observations through the
   evidence sources and capabilities you pass it, cites what it saw, and proposes
   theories with cause + mechanism. Each specialist also carries the
   pre-registered discriminator for its hypothesis (from Scout's discriminator
   table, refined only from mechanism/source-shape fit and never from the
   observed value) and, in its first pass,
   states the expected favored/rival values BEFORE checking (sourced from the
   coverage map's cited orientation asset when one is named — open that asset to
   seed the expectation rather than re-deriving from scratch; the checked observed
   value still comes from authoritative live evidence), then records the
   observed value, gate status, and a compact claim-readiness ledger (see
   `references/specialists/AGENTS.md` and the mechanism-discriminator gate in
   `references/grading-rubric.md`) — so a lead arrives gate-ready rather than as
   a post-hoc narrative the Grader must bounce back. In later loop turns,
   re-dispatch focused Specialists with the Grader's refinement obligations
   (narrow scoped brief per the refinement-obligation contract; independent obligations
   dispatched as one awaited parallel-sync wave);
   append a `## pass_N` note to their theory rather than overwriting it. Expected output: cited observations,
   answered/unanswered questions, theories, and gaps. The coordinator merges their
   observations into the shared record.
4. **Grader + pursuit loop (`5_grader`).** After Specialists and observation merge,
   dispatch the Grader. It judges only: no fetching and no dispatch. A `Likely-rooted`/`Confirmed` verdict requires the mechanism-discriminator gate (see `references/grading-rubric.md`). The Grader also receives the scout's recurrence/sibling findings and discussion-thread summary, and applies the duplicate / verdict-determinism gate in `references/grading-rubric.md` (classify `canonical`/`duplicate-of`; siblings sharing the recurrence identity are judged by the same verdict gate).
   After each Grader
   pass, the coordinator MUST NOT proceed to report unless the verdict is `Confirmed`,
   `Likely-rooted`, or `Inconclusive-blocked`, OR the follow-up budget is exhausted.
   If any material lead is `open-answerable` and budget remains, the coordinator MUST
   run one focused follow-up round: either a single focused dispatch or, for independent
   obligations, one awaited parallel-sync wave of up to 5 specialists (see Execution
   model) — a wave counts as exactly ONE follow-up round, and if independent obligations
   exceed the concurrency cap the additional awaited batches still count as that same one
   round (the concurrency cap is not the budget unit). Each follow-up uses the scoped
   refinement-obligation contract (`references/grading-rubric.md` /
   `references/artifact-contracts.md`): one specialist per independent missing
   discriminator/obligation, given the obligation id, the pre-registered discriminator
   predicate + expected favored/rival, the in-hand keys, and the prior OBS ids with their
   reuse/freshness mode. It checks ONLY that discriminator plus a minimal premise/freshness
   preflight, reuses settled/static OBS (only with exact predicate/source/key/scope coverage) but performs a fresh narrow read when the
   discriminator depends on live state or a required cross-source pivot, does not re-sweep
   settled evidence or broaden to new leads, and returns an explicit answered /
   invalid-premise / unanswerable status with citations. A follow-up
   `invalid-premise`/`unanswerable` never closes a material lead by itself; the Grader
   reconciles it in the lead ledger. Then merge new observations and re-grade. Before settling `Inconclusive-blocked`/`Proximate-only` on a lead whose failing units carry in-hand keys to an unprobed next-causal-layer source, confirm the Grader's discovery receipt names a probed source AND an observed result (or a terminal denied/missing error after an authed probe), not a soft 'unavailable'; if a reachable next-causal-layer source remains unprobed, run one cross-source-pivot follow-up pass first (within the existing follow-up budget). When no reachable next-causal-layer source exists, that terminal is itself the receipt — settle and name the engineer next step. A report after `Proximate-only` is allowed
   only when no `open-answerable` material lead remains, the budget is exhausted, or
   the lead has been converted to `blocked-unreachable` with an engineer next step.
   Budget is 2 follow-up turns by default; allow a 3rd only if the prior turn produced
   material evidence. Stop if two consecutive turns add nothing material. Do not spin
   on evidence paths that cannot answer the lead: record a `blocked-unreachable`
   dead-end.
   If the final verdict is `Confirmed` or `Likely-rooted` and the verified cause is tied to a code/config/schema/artifact/service-owned location, run ONE bounded introduction-provenance pass before Report, seeded with the verified repo/source, branch/ref when known, implicated path, symbol/line/config key, and known owner/area. It is broad over time but narrow over scope. Use a read-only source-control history capability that can search commits, pull requests, file history, diffs, and line/symbol history for that exact path/symbol across available history — not the incident/onset window. Prefer the earliest semantically relevant change that introduced the defective behavior (the missing guard, the defective branch, the config/schema/artifact shape); if introduction cannot be determined, report the closest semantically relevant last-touching change and label it `last-touch, not proven introduction`. Only if the history capability cannot answer AND a local source checkout is available, perform a bounded, authorized history deepen of the implicated repo/ref/path when cheap, then use line-level history/blame on the exact symbol/lines; if that needs unavailable auth, excessive history, or broad whole-repo fetches, do not block — record a provenance gap plus the one concrete query a human should run. Stop after a high-confidence introducing candidate, a labeled last-touch candidate, or an explicit gap. Merge as an actionability add-on (commit, pull request, date, author, fix handle, owner when known); this never changes or gates the verdict.
5. **Report (`6_report`).** Dispatch Report to write the concise RCA, bounded by the
   Grader's verdict, provenance add-on when required, and open gaps. When the dispatch brief
   authorizes live posting and a non-gated incident-posting capability is available, Report also composes the incident update and delegates the mutation itself — using the capability-owned idempotency/audit marker — and records the outcome in the run report artifacts; otherwise the run is report-only and that is not a gap. Live posting requires BOTH explicit brief authorization AND a non-gated capability and cannot be inferred from discovery alone. Details and the verdict-by-verdict posting policy live in `references/subagents/poster.md`. When the Grader's obligations carry a manual
   investigation kit (a decisive discriminator needs a human-only/out-of-band capability, whether the
   verdict is blocked or capped at `Likely-rooted`), Report renders it unchanged as a required section
   per the report contract (`references/artifact-contracts.md` §`6_report/`) and does not invent or
   verify its steps. Report also applies post mode (see the report contract, § Post mode): when the
   incident's thread already carries human root cause/mitigation/progress, it posts as a collaborator
   (credit + additive-only, or a respectful contradiction), not a standalone re-derived RCA. Expected output: concise cited report with cause
   + mechanism, verdict wording, gaps, closest known introduction for verified
   code/config causes, unresolved upstream why when not rooted, the Grader's
   engineer suggested step, and (when authorized) the posted incident update.
6. **Knowledge value triage + capture (`7_knowledge`, adaptive reflective pass).** After Report, run a brief
   Knowledge Value Triage over the FINAL artifacts only (`5_grader/ranking.md`, `6_report`, and the scout's
   recurrence/sibling findings). Dispatch the Knowledge Curator ONLY if at least one evidence-backed novelty
   trigger is present: a new or revised reusable signature, a recurring sibling pattern, a verified
   observability/source gap, a misleading monitor/telemetry gotcha, a repeated manual-handoff gap, or a verified
   mechanism absent from the service KB. If none, record `knowledge_capture: skipped — no durable novelty/value`
   and stop — never create a candidate just to fill the stage. The pass is non-blocking and never changes the
   verdict, the report, or the post; it writes run-local candidate knowledge only (no mutation of curated service
   knowledge). The skip record (when no Curator is dispatched) is written to `run.md`. Details in
   `references/subagents/knowledge.md`. Expected output: either the skip record, or
   classified candidate items (kind/status/confidence/evidence/recurrence/freshness) plus a proposed, un-applied
   KB delta.

## Iteration mode (new information)

Enter iteration mode ONLY when the task itself provides a new-information packet (a
pointer to what changed since a prior investigation of this incident) AND the task
explicitly authorizes reading prior runs under the resolved run-root. Do not infer it from
the presence of sibling run directories — reruns also produce siblings. This mode is
distinct from the Grader follow-up loop (a within-run mechanism) and does not change the
single-turn awaited contract or the per-run pursuit budget. The iteration ordinal is
orchestrator-provided and passed through as-is; retries reuse the same ordinal — never
self-increment it.

Iteration early-exit preflight (do this first): classify the new-information packet against the
prior run's verdict and the incident's current state. Proceed only when it carries information
that is BOTH not generated by this agent AND genuinely actionable. When it conclusively does not
(e.g. the only new timeline activity is this agent's own prior post, an empty packet, or an
unchanged status), terminate as an explicit report-only early-exit: mint the run, record
`early_exit: no_genuine_new_information` with the prior-run pointer and reason in `run.md`, then
dispatch nothing (no Scout/Specialists/Grader/Report) and post nothing. Default to PROCEED when
origin or materiality is uncertain (fail open). Two-part test and triggers:
`references/followup.md` § Early-exit gate.

In iteration mode: mint a NEW run-id (read prior runs in place; never copy or write into a
prior run dir), rebuild the CAPABILITY MAP, and focus Scout/Specialists on the new info
plus still-open/blocked leads — carrying settled, untouched leads forward as cited
prior-iteration claims rather than redoing them. The Grader re-derives the verdict on the
merged evidence (never inherited); Report is a delta with honest downgrades. Reading
prior-run markdown for scope/merge is coordinator intake (inline-allowed); re-verifying any
prior claim against a live source is evidence work — dispatch it full-evidence, never
inline. Full rules — supersession, carry-forward, `<priorRunId>/OBS###` re-anchoring,
lead-state transitions, isolation, and the delta-report contract — in
`references/followup.md`.

## Reference map

- Honesty floor: `references/investigation-invariants.md`
- Operational efficiency floor: `references/operational-discipline.md`
- Run layout and observation ids: `references/run-store.md`
- Iteration mode (new information): `references/followup.md`
- What each stage produces: `references/artifact-contracts.md`
- How to judge: `references/grading-rubric.md`
- Specialist worker guidance: `references/specialists/AGENTS.md`
- Scout role: `references/subagents/scout.md`
- Grader role: `references/subagents/grader.md`
- Report role: `references/subagents/poster.md`
- Knowledge Curator role: `references/subagents/knowledge.md`
