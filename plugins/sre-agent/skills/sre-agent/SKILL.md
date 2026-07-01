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
dispatch, material Specialist dispatches, Grader verdict, RUN-STATE DIGEST, the
`model_tiering` record, and Report artifact — or an explicit abort/degraded-mode
reason in `run.md`.

General path: changed/affected surfaces -> questions -> observations/gaps -> judged
verdict -> report.

Local user responses (not incident posts): lead with 3 sentences — verdict, the one fact that proves it, and
the owner-routed next step. Offer the full report or deeper evidence on request instead of dumping it.
Keep the same honesty floor: disclose confidence caps, blockers, and required manual handoffs.

## Coordinator contract

The coordinator is orchestration-only. It may ONLY create/own the run, inventory
available capabilities and write the CAPABILITY MAP, dispatch subagents, merge and
cite returned observations, run the Grader pursuit loop, and assemble the final
handoff/Report dispatch.

The coordinator MUST NOT perform deep evidence collection, file/log/code searching,
or hypothesis analysis inline in the main context. Dispatch that work; keep inline
work to mechanical intake, status, merge, and handoff.

Coordinator action taxonomy (local receipt before action): every coordinator tool
call fits one allowed class: (a) intake/run-artifact read (including the intake
recurrence-cluster read for the fast-lane decision), (b) dispatch a
subagent, (c) await/read a subagent's output, (d) update the run-state digest, (e)
merge/adjudicate/synthesize cited observations, (f) assemble the handoff/Report,
or (g) inventory capability metadata from host-provided handles, or (h)
target-independent capability access-confirmation — the cheapest
capability-level health/list/schema/status check per § Access confirmation (no
guessed incident-specific target). Anything else — live telemetry query,
code/log/source search or read, recursive filesystem search, or independent
hypothesis evidence collection — is Specialist work: dispatch it instead of
running it inline. Before any coordinator query/search/source-read, confirm it is
one of (a)-(h); if not, dispatch. The intake fast-lane (§ Intake fast-lane) composes
with these classes and adds no new inline-evidence license: its lane decision reasons over
the intake recurrence cluster (a), its wave-1 duplicate-verification specialist an awaited dispatch (b), and its disposition a
handoff/Report (f) routed through the existing poster — the specialist collects the
confirming evidence, never the coordinator.

Synthesis stays owned by the coordinator: it may compare Specialist claims, build
the cited timeline, spot gaps/conflicts, dispatch targeted follow-up, and do
bounded validation that claim support actually exists. That is synthesis, not
primary collection. The direct evidence-probe budget is 0 by default. Use at most
2 direct probes per run, total, only to unblock dispatch routing or adjudicate a
Specialist conflict whose resolution changes the verdict/severity; each probe
records one line in `run.md`: purpose · why dispatch is invalid/too-slow ·
source · how the result changes routing/verdict · remaining probe budget. Class
(h) target-independent access-confirmation is outside this evidence-probe budget.
Do not spend probes for corroboration, confidence, or provenance; assign those.

Maintain a compact RUN-STATE DIGEST in `run.md`, initialized before Scout and
updated after each major artifact or Specialist output and before each later
stage advance, so static artifacts need not be reopened to recover context. A
missing or stale digest is a run defect. The schema lives in
`references/run-store.md#run-state-digest`.

Mandatory dispatch points:
- Scout is always dispatched after intake and the completed CAPABILITY MAP, unless the
  coordinator steers to the intake fast-lane (see § Intake fast-lane), which defers Scout
  and fails open to it on any uncertainty.
- At least one Specialist is dispatched per material hypothesis area from Scout, or
  an explicit gap is recorded when no matching capability or host support exists.
- Grader is always dispatched after the Specialist observation merge.
- Before dispatching the Grader, generate this run's **candidate evidence-query menu** — the executed query manifests filtered to evidence queries, listed by `runId` — using the incident-posting capability's evidence-menu function over the run's query-manifest store, and include the menu in the Grader brief. The Grader selects the 1-3 decisive verification queries by `runId` (it never runs queries or handles URLs); the Manual Investigation Kit's deep-links are resolved deterministically from those runIds. If no such menu capability is available, record a gap and proceed without the kit.
- Report is dispatched after the final Grader verdict and any required bounded
  introduction-provenance pass.

These mandatory dispatch points have two exceptions: (1) a recorded iteration early-exit
(see § Iteration mode) — when an iteration carries no genuine new information, the run
terminates report-only with an `early_exit` note in `run.md` and dispatches nothing; and
(2) the coordinator's intake fast-lane decision (see § Intake fast-lane), which substitutes
one awaited
wave-1 duplicate-verification specialist + a disposition/Report for the Scout→Grader deep-lane on a
SAME (verified-duplicate) recognized-recurrence match, and still fails open to the full deep-lane (Scout, Specialists,
Grader, Report) on any wave-1 disagreement or uncertainty.

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
Scout, Specialist, the intake fast-lane wave-1 duplicate-verification specialist, Grader, Report, or
dispatched Knowledge Curator step is still incomplete.

A dispatch is complete ONLY after the coordinator has received the subagent's
finished output and merged it into the run state. If a dispatch returns only
an id, handle, promise, or a "started/running" status, it is NOT complete,
and the coordinator's very next action MUST be to await/poll/collect that
result in the same turn. Stating "waiting for results" in the response is not
waiting — it ends the turn and orphans the work.

Therefore, in unattended runs, fire-and-forget / background / detached
dispatch is FORBIDDEN for the required pipeline stages (Scout, Specialist,
the intake fast-lane wave-1 duplicate-verification specialist, Grader, Report, and a dispatched
Knowledge Curator). Dispatch is blocking:
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

Keep batch membership tight: include only verdict-relevant Specialists in the
awaited batch. When the host requires explicit collection, minimize poll count:
prefer one long awaited read per agent/batch over many short polls. Use in-turn
wait time only for non-evidence work such as updating the RUN-STATE DIGEST or
preparing the merge/contradiction matrix; do not start new dependent
investigations during a wait, because their inputs may be invalidated by the
pending batch.

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
or gap). Flag every read-only orientation/knowledge/history/documentation/search capability so
Scout's known-issue discovery pass is dispatched with ALL matching ones, never only the most
obvious knowledge source. A ranked code/knowledge-search capability counts here: classify its
find/rank use as a Scout orientation capability AND its deep-read use as a specialist
source-verification capability, and pass the find/rank use to Scout (full-evidence) for fast
discovery over the KB/docs/repo text while specialists retain deep code mechanism verification.

Inventory concrete entrypoints from host-provided tool handles and capability
descriptions (for example, environment variables or metadata that name the
telemetry-query, code-search, or incident-posting CLI handles). Do not recursively
search the filesystem to locate tool binaries; absence from a guessed path is not
a capability result.

Load and use every available capability whose description matches an immediate
investigation obligation. If a material need has no matching capability, record an
explicit gap. Do NOT proceed to Scout until the run exists, the CAPABILITY MAP is
complete, the intake intent frame is set, intake is verified-or-fetched (staged items
reconciled against the manifest and every redacted/gapped item either fetched or recorded
as an explicit gap), AND the RUN-STATE DIGEST is initialized.

Do not mark a description-matching capability gap/skip from an unverified environment guess ("no data here", "wrong working dir").
Availability is confirmed by invoking it and reading its self-report, not by guessing inputs exist; coordinator confirmation uses class (h) when it is target-independent. Assumed-unavailable-from-environment-guess is non-diagnostic (see Access confirmation).
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

A capability appearing absent inside a `reasoning-only`/restricted worker is non-diagnostic about availability and MUST NOT change a capability's ACCESS STATUS; re-confirm from a `full-evidence` context before recording it absent or blocked. Use coordinator context only as class (h) target-independent capability access-confirmation.

Read the capability's own help, metadata, or error guidance first. Confirm through its canonical invocation, starting with the cheapest capability-level health/list/schema/status check that does not depend on a guessed incident-specific target; do not first-probe by firing a guessed incident-specific query.

If that control probe exits 0 with EMPTY stdout/stderr, it is a silent no-op, not a confirmation: record `unconfirmed-nondiagnostic(probe-defect)` (never `confirmed` or `blocked`), retry once via an alternate host-supported invocation (a real absolute path rather than a wrapper/symlink, or a documented alternate entry point), and carry the alternate-invocation next step for a specialist.

Record `blocked` only when ACCESS STATUS cites all four: canonical invocation attempted; auth/cold-start handled or inapplicable; no dependency on a guessed incident-specific target (target-independent check or discovered/provided target); terminal missing-command or denied-after-valid-auth error.

After a usage/parse, target/resource resolution, or auth/init failure, make at least one corrected re-probe before any block. The correction removes the original defect and must not introduce another guessed target; if no target is known or discovered, record `unconfirmed-nondiagnostic(wrong-target)`, never `blocked`. Cap corrected attempts at two.

`blocked` is provisional: include failure class, last error, and the one concrete command a human/specialist should run. A specialist on that evidence path may re-confirm later; one bad early probe never permanently disables an evidence source.

Confirmation is per evidence source, not global: a capability confirmed against one target (e.g., the incident-record source) is not confirmed against a different source the lead needs. When the incident question requires the service's own telemetry, confirm or attempt that source as its own ACCESS STATUS line before recording a signal or table unknown (see the access invariant in `references/investigation-invariants.md` for the documentation-gap rule).

Source navigation composes with ACCESS STATUS: when assigning current-code navigation, specialists prefer read-only local navigation of the code the service knowledge points to (search, read, follow references locally). When assigning current-code source-navigation work, prefer an available ranked source-navigation/code-lookup capability for symbol definitions and multi-term identifier hunts when it returns scoped, grouped candidate locations; use bounded line-oriented search for true regex, exhaustive text scans, or when no such capability is available.
Assign remote source-control capability for provenance a local checkout may lack: commits, pull requests, branches, history, and blame.
Before code-dependent conclusions for a service-specific incident, resolve the local source location from service knowledge and record `source access: resolved-local|unavailable|not-needed`; if unavailable, state the gap rather than substituting remote current-code search or asserting code-level mechanism.

## Dispatch routing

Each dispatch declares three independent dimensions:
- Model tier: choose by capability CLASS, not a fixed model name. Scout and
  breadth-first orientation DEFAULT to the fast/economical class; Grader, Report,
  and synthesis DEFAULT to the reasoning-heavy class. Specialists take NO blanket
  class — each specialist's class is resolved per dispatch from its declared
  `model_affinity` contract combined with the Scout's per-hypothesis demand (see
  Specialist model resolution below). Within the
  chosen class, select the newest STABLE generation the dispatch tool advertises:
  resolve from that live model list at run time (never a remembered or hardcoded
  name), compare versions only WITHIN one family lineage (a sonnet-4.6 supersedes
  sonnet-4.5) and never version-rank across unrelated families; keep the class's
  default family and step to another only when the default lacks a required capability
  (effort level, toolset). Skip preview/unsupported models; if class/family metadata or
  version ordering is absent, use the host default for the class and record the
  fallback. Resolve once during the CAPABILITY MAP — recording the advertised models
  seen and the per-class choice — then USE that resolved model in the actual dispatch
  parameter on every dispatch when the dispatch tool exposes one (else record
  `harness-limited`): a model written into the Tier Record but not set on an available
  dispatch parameter is an incomplete handoff.
  Every dispatch records a Tier Record: role · chosen class · resolved model + basis ·
  default class · escalation reason when above default · fallback when the preferred
  class or model is unavailable.
  The intake fast-lane's wave-1 duplicate-verification specialist and its disposition/Report DEFAULT to the
  economical/mid class and record a Tier Record like any dispatch; the deep-lane is
  unchanged (reasoning-heavy Grader/Report). An escalation from the fast-lane runs the
  deep-lane at its normal classes.
  A non-economical Scout/orientation dispatch REQUIRES a named escalation reason
  (claim-gated reasoning, synthesis, or high ambiguity); a missing Tier Record is an
  incomplete handoff. Efficiency detail: `references/operational-discipline.md`.
  **Specialist model resolution (two-layer, adaptive).** A specialist's class is
  resolved from two layers, never a fixed default. Layer 1 is the specialist
  skill's `model_affinity` frontmatter contract: `default_class` (the
  eval-calibrated baseline), `minimum_allowed_class` (a hard floor), and the
  skill-authored `escalate_when` signals. Layer 2 is the Scout's per-hypothesis
  `model_demand` tag (`low` | `high`); the Scout raises it to `high` exactly when
  it observes one of the owning skill's `escalate_when` conditions for that
  hypothesis. All three operands resolve onto the single
  `{economical | mid | reasoning-heavy}` lattice before combining: `default_class`
  as declared; `model_demand` projects as `high` → the reasoning-heavy class and
  `low`/absent → the `default_class` (no raise); and `coordinator_risk_override` is
  an OPTIONAL, upward-only class the coordinator may name from its own risk read
  (recorded with basis `coordinator-escalated`), never a downward move and omitted
  by default. Resolve `effective_class = max(default_class, model_demand→class,
  coordinator_risk_override)`; the result is never below `minimum_allowed_class`
  and never below `default_class` — escalation is the ONLY permitted movement
  (Scout demand, coordinator, or a Grader re-dispatch may RAISE the class above the
  default; nothing lowers it below the declared default). A missing, malformed,
  stale, or invalid `model_affinity` (e.g. `default_class` below
  `minimum_allowed_class`, an unknown class) FAILS SAFE to the reasoning-heavy
  class and records the contract fault in the Tier Record. Lowering a skill's
  `default_class` or `minimum_allowed_class` is a doctrine change requiring eval
  evidence, never a per-run choice. The resolved class then follows the same
  newest-stable-within-family resolution above.
- Context-window tier: a large/long-context tier on the top-level session does NOT
  propagate to dispatched subagents — each defaults to the standard window. A heavy-read
  worker DEFAULTS to the large-context tier — Scout whenever it receives
  KB/history/documentation/search orientation capabilities, and evidence specialists
  enumerating telemetry or source. When the dispatch tool exposes a context/window
  parameter, the dispatch call MUST set it (recording the intent without setting the
  parameter is incomplete), so the worker does not exhaust its window and force mid-run
  compaction. This is independent of the model-class escalation above — large context on
  the economical model class is NOT a model escalation; record a separate context reason.
  Use the standard window only with an explicit bounded-input reason; when the host
  exposes no override for an over-budget heavy reader, bound or stage the worker's reads
  (or record a visible context-capability gap) rather than silently dispatching over
  budget. Record the chosen window tier in the Tier Record.
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

Scout/Specialist brief contract: each dispatch carries a compact orientation
packet with what the coordinator already established — incident facts, candidate
known-issue/KB hits with pointers, and resolved cluster/db/join-key targets when
known — so the worker verifies rather than rediscovers them. Pass compact facts
plus pointers, not raw bulk context. The brief also names the exact single
question, evidence budget, capability handles to use, and asks for partial
findings plus named gaps rather than a long-running sweep.

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
   identity capture for later correlation, not a hypothesis or a cause. Alongside the
   incident record, this stage also holds the **intake recurrence cluster**: the
   read-only incident-record/context capability's fetch INCLUDES a recurrence set (the
   recurrence include) — the sibling incidents matched to this incident over a recent
   window, each row carrying signature/title, owning team, `ClaimedRootCause`,
   `ClaimedMitigation`, `IsNoise`, `Severity`, and `Status`. The coordinator holds those
   sibling rows as orientation evidence (claims, not authority) for the FAST-LANE DECISION
   below. If that capability or include is unavailable, the cluster is an explicit GAP —
   the fast-lane cannot fire and the run takes the deep-lane (fail open). Do not dispatch
   Scout until the run exists, the CAPABILITY MAP is complete, the intent frame is set,
   AND intake is verified-or-fetched — staged items reconciled against the manifest and
   every redacted/gapped item either fetched or recorded as an explicit gap.
   **FAST-LANE DECISION (coordinator self-governs the lane at intake, with judgment).**
   With intake complete, before dispatching Scout, the coordinator reasons over the cheap
   orientation it already holds — the captured recurrence identity plus the intake recurrence
   cluster (the read-only incident-history recurrence pass surfaces, at intake, the sibling
   incidents matched to this incident's recurrence identity over a recent window, each
   carrying its claimed root cause, claimed mitigation, noise flag, severity, and status; an
   intake-artifact read, action class (a)). It treats the cluster rows as orientation
   evidence — claims, not authority, the same honesty floor as failure-knowledge grounding
   and Scout's known-issue consultation; never run-local `7_knowledge` candidates or sibling
   run dirs (the same isolation rule as Scout). The job is to RECOGNIZE whether this incident
   is a member of a convergent recurrence FAMILY worth a bounded WAVE-1 DUPLICATE VERIFICATION
   rather than the full investigation, following RECOGNIZE → VERIFY LIVE → RECONCILE (never
   import): the cluster supplies family membership, ONE wave-1 specialist re-establishes the
   family's previous root cause on THIS incident's live evidence, and the coordinator
   reconciles SAME vs. escalate — the prior root cause is only the hypothesis, never imported.
   Recognize the family and dispatch the wave-1 specialist only when the coordinator can affirm
   ALL of: (1) a CONVERGENT cluster — multiple siblings sharing the recurrence identity
   (signature + ≥1 more identity axis; title/team alone is insufficient); (2) at least one
   family member carries a READABLE PRIOR ROOT CAUSE to verify — a recently-resolved sibling's
   discussion/RCA, or this incident's own prior agent RCA in-thread — from which a falsifiable
   discriminator can be formed (the prior conclusion MAY be the agent's own earlier post: that
   is a valid hypothesis to re-verify live, not a disqualifier); and (3) Sev ∈ {3,4} with scope
   consistent with the family bound (the wave-1 check re-confirms scope live). When it does,
   take the fast-lane (§ Intake fast-lane) and defer Scout — EXCEPT on the periodic re-validation
   FLOOR (§ Intake fast-lane): the coordinator forces the FULL deep-lane even when the wave-1 check
   would confirm when EITHER no `family-validated` marker for this recurrence identity is visible in
   the intake recurrence cluster / thread within a ~30-day recency window, OR N=10 consecutive
   fast-lane SAME dispositions have run for the family with no intervening deep-lane validation —
   computable from IcM state at intake, so silent drift cannot perpetuate. On no convergent cluster, no readable prior conclusion, no falsifiable
   discriminator, an out-of-bound Sev/scope, or any doubt, proceed to Scout exactly as below —
   the decision lives in the coordinator's judgment and fails open to the deep-lane rather than
   guessing. The decision is a relationship
   recognition for routing only; it never sets `canonical`/`duplicate-of`. Full contract:
   `references/fast-lane.md`. Expected output: run pointer, captured claims, discussion-thread summary, intent
   frame, recurrence identity, the intake recurrence cluster (sibling rows, or an explicit
   unavailable/gap note), the capability map, and the fast-lane decision
   (`fast-lane: recognized` | `deep-lane: <reason>`).
2. **Scout (`2_scout`, sole analytic orienter).** Dispatch Scout to read the captured
   claims, do light bounded orientation, and produce a neutral map. Orientation includes
   a bounded recurrence check: using an available read-only incident-history capability,
   look for prior or concurrent incidents whose identity matches or overlaps the
   recurrence identity over a recent window, and let any matches shape — never decide —
   the hypotheses; when such a capability is available, dispatch Scout `full-evidence`
   with it, otherwise record recurrence as an explicit gap. Orientation also includes a
   bounded known/ongoing-issue discovery pass across whatever read-only knowledge,
   orientation, and documentation capabilities the run exposes — discovered by description
   from the CAPABILITY MAP (service knowledge, failure knowledge, AI assets, TSGs, runbooks,
   wiki/doc search, and the like), never a single hardcoded knowledge source; dispatch Scout
   `full-evidence` with the ones it must search, and record an explicit gap when none is
   available. Expected output: surfaces,
   recurrence matches (or an explicit none/unavailable note), the discussion-thread summary
   (or empty/unavailable note), how they shape the
   hypotheses, at least two materially different hypotheses, and the
   questions/observations that would discriminate them, plus a best-effort pre-declared discriminator table (per leading hypothesis: serious same-symptom rival, falsifiable predicate, expected favored vs rival observation, candidate authoritative source/key — plus a coverage-map entry per hypothesis (best-fit orientation/known-issue asset found by a bounded scan across the discovered read-only knowledge capabilities — not a single hardcoded knowledge source — or `no-coverage`; see `references/subagents/scout.md`)) — pre-registering the discriminator before any specialist checks it, with an explicit gap when no honest discriminator exists yet; no findings or verdicts. Scout also consults whatever curated/promoted prior knowledge the discovered capabilities expose (failure knowledge and any reviewed-promoted items among them) as orientation evidence — claims not authority, bounded by service/component/symptom. When a discovered asset describes a known or ongoing issue whose signature matches this incident AND supplies (or lets Scout derive) a falsifiable discriminator, Scout names it the leading candidate to test first — recorded as an inherited/open rung, never a settled answer — so the first specialist wave checks it before broad fanout; a weaker symptom-only match stays a non-leading orientation lead. It never reads unreviewed run-local `7_knowledge` candidates or sibling run directories; here too, missing prior knowledge is a gap, not a block.
3. **Specialists (`3_evidence`, `4_specialists`).** Dispatch one Specialist per
   material hypothesis area, and launch all independent specialists as a SINGLE
   awaited parallel-sync batch (see Execution model: awaited parallel-sync
   batch; max 5 concurrent, batch if more; never background/detached, never
   one-at-a-time across turns). Serialize a specialist only on a genuine input
   dependency. Known-issue-first staged dispatch (decision rule): when Scout designates a
   leading known/ongoing-issue candidate carrying a falsifiable discriminator, the first wave
   MAY be narrowed to the specialist(s) that check that discriminator, its serious same-symptom
   rival, and the failing-population enumeration — recording the other material hypotheses in the
   lead ledger as `deferred-by-known-issue-first` (held, not dropped), dispatched as the normal one-per-hypothesis
   wave the moment any fail-open trigger fires or the candidate is refuted/unverified (see the
   known-issue acceleration settle rule in `references/grading-rubric.md`). Absent such a
   candidate, dispatch the normal one-Specialist-per-hypothesis wave. (Ordering: the
   coordinator may steer to the intake fast-lane FIRST at intake on a recognized recurrence-family
   match, skipping Scout; known-issue-first is a deep-lane mechanism that applies
   only when the fast-lane did not fire — they are complementary and non-conflicting.) Each
   fetches and analyzes its own observations through the
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
   When the run took the known-issue-first staged path, the Grader applies the known-issue
   acceleration settle rule (`references/grading-rubric.md`): settle in fewer rounds once the
   candidate's discriminator passes the full gates on this incident's live evidence, or fail open
   to the deferred normal fanout — never a lower verdict bar.
   If the final verdict is `Confirmed` or `Likely-rooted` and the verified cause is tied to a code/config/schema/artifact/service-owned location, run ONE bounded introduction-provenance pass before Report, seeded with the verified repo/source, branch/ref when known, implicated path, symbol/line/config key, and known owner/area. It is broad over time but narrow over scope. **Hard-cap it at ≤ ~8 history/source-control reads or ~4 minutes** — it is a post-verdict actionability add-on that must NEVER dominate the run; if the introducing change is not found within that cap, especially across renames/path-moves where a paged history API needs many version-filtered reads, STOP and record the provenance gap plus the one concrete query a human should run, rather than chasing it across the paged API. If the exact introducing change is actually load-bearing for the verdict, owner selection, mechanism discrimination, regression-vs-drift, or rollback-vs-forward-fix, it is no longer actionability-only — route it through the normal pre-verdict discriminator/refinement budget instead of this bounded post-verdict pass. Use a read-only source-control history capability that can search commits, pull requests, file history, diffs, and line/symbol history for that exact path/symbol across available history — not the incident/onset window. Prefer the earliest semantically relevant change that introduced the defective behavior (the missing guard, the defective branch, the config/schema/artifact shape); if introduction cannot be determined, report the closest semantically relevant last-touching change and label it `last-touch, not proven introduction`. Only if the history capability cannot answer AND a local source checkout is available, perform a bounded, authorized history deepen of the implicated repo/ref/path when cheap, then use line-level history/blame on the exact symbol/lines; if that needs unavailable auth, excessive history, or broad whole-repo fetches, do not block — record a provenance gap plus the one concrete query a human should run. Stop after a high-confidence introducing candidate, a labeled last-touch candidate, or an explicit gap. Merge as an actionability add-on (commit, pull request, date, author, fix handle, owner when known); this never changes or gates the verdict.
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
   observability/source gap, a misleading monitor/telemetry gotcha, a repeated manual-handoff gap, a verified
   mechanism absent from the service KB, or a deep-lane-confirmed NEW recurring known/benign disposition that
   carries a falsifiable discriminator — for which the Knowledge Curator proposes an un-applied
   recurrence/known-issue knowledge candidate (recurrence-identity + discriminator + verdict + evidence, written
   run-local as a suggestion only; schema in `references/subagents/knowledge.md`). If none, record `knowledge_capture: skipped — no durable novelty/value`
   and stop — never create a candidate just to fill the stage. The pass is non-blocking and never changes the
   verdict, the report, or the post; it writes run-local candidate knowledge only (no mutation of curated service
   knowledge). The skip record (when no Curator is dispatched) is written to `run.md`. Details in
   `references/subagents/knowledge.md`. Expected output: either the skip record, or
   classified candidate items (kind/status/confidence/evidence/recurrence/freshness) plus a proposed, un-applied
   KB delta.

## Intake fast-lane (wave-1 duplicate verification)

On recognizing a recurrence family at intake (§`1_intake`), the coordinator runs a bounded
WAVE-1 DUPLICATE VERIFICATION instead of the Scout→Grader deep-lane — self-governing the lane
with judgment, not firing a mechanical rule. It follows RECOGNIZE → VERIFY LIVE → RECONCILE
(never import): the intake recurrence cluster supplies family membership, ONE wave-1 specialist
re-establishes the family's previous root cause on THIS incident's live evidence, and the
coordinator reconciles SAME (fast-track + post) vs. escalate. The prior root cause is only the
hypothesis; it is never imported. This NEVER lowers a verdict bar and fails open to the
deep-lane on any uncertainty. Full contract — the backbone, recognition rule, the wave-1
specialist brief, the reconcile rule, escalation triggers, recall safeguards, and the periodic
re-validation backstop — in `references/fast-lane.md`.

- **RECOGNIZE** (coordinator self-governs, an intake read of class (a)): the recurrence-cluster
  reasoning above. Recognize the family and dispatch the wave-1 specialist only on ALL of — a
  convergent cluster (signature + ≥1 more identity axis across multiple siblings; title/team
  alone is insufficient), at least one family member carrying a readable prior root cause to
  verify (a recently-resolved sibling's discussion/RCA OR this incident's own prior agent RCA
  in-thread — the agent's own prior RCA is a valid hypothesis, not a disqualifier), and Sev ∈
  {3,4} with scope in the family bound; otherwise ESCALATE to Scout.
- **WAVE-1 DUPLICATE-VERIFICATION SPECIALIST** (one bounded `full-evidence` dispatch,
  economical/mid class, an awaited class (b) dispatch): its brief is "is THIS incident the SAME
  root cause as the family's previous incidents?". It (a) reads the family's previous root cause
  from a representative recently-resolved sibling's discussion/RCA (via the read-only
  incident-context/history capability) or this incident's own prior in-thread RCA, as a CLAIM,
  and extracts a falsifiable discriminator (`expected_favored` = the known root-cause signature;
  `expected_rival` = what a genuinely DIFFERENT / real root cause looks like on the same live
  evidence); (b) states expected favored vs rival BEFORE reading, then checks that ONE predicate
  against THIS incident's live evidence with the same mechanism-discriminator gate rigor as the
  deep-lane scoped to that predicate, recording the observed value, gate status, and a cited OBS
  — budget ≤ ~2 targeted reads / a tight time cap, and within it corroborating the incident's
  actual scope against the family bound; (c) returns **SAME** (live evidence matches
  `expected_favored` AND refutes the rival AND scope is in bound) or **DIFFERENT / ambiguous /
  can't-verify** (matches the rival, matches neither pre-named value, scope wider than the bound,
  no falsifiable discriminator formable, or the check was blocked).
- **RECONCILE (re-established, not imported)** (class (f) handoff/Report + poster):
  - **SAME → FAST-TRACK + AUTO-POST** (reconcile SAME ONLY when the receipt is internally
    consistent — `wave1_result: same` AND the wave-1 discriminator `gate: pass` with a cited live
    OBS; a `same` result whose gate is not `pass` is self-contradictory → escalate). Compose the
    disposition — this incident is the SAME
    recurring issue as the family (a duplicate of the known recurrence family), cause + mechanism,
    the wave-1 confirming OBS as the cited evidence (honesty floor: cite the live observation,
    never "a sibling said so"), and the engineer next-step. AUTO-POST the named `Known-recurrence`
    poster disposition (`references/subagents/poster.md` § Verdict policy) via the existing poster
    path as a **collaborator / additive duplicate-reference** contribution (additive "same as the
    known recurrence family, verified live", NEVER a standalone re-derived RCA), under the SAME
    authorization + idempotency/audit rules as deep-lane posting (live posting still requires
    brief authorization AND a non-gated capability; otherwise report-only). Apply the existing
    Poster post-mode check — contribute additively/respectfully when the thread already carries
    human root cause / mitigation / progress (disposition token `collaborator-duplicate-live`);
    otherwise post the additive duplicate-reference directly (`posted-duplicate-live`). The
    disposition does NOT classify
    `canonical`/`duplicate-of` — that linkage is exclusively a Scout(sibling discovery)+Grader
    (clock-ordering) product and is NEVER coordinator-inferred here; its duplicate assertion rests
    only on the recurrence-identity sibling relationship it verified live, and a sibling/duplicate
    linkage is carried only when already host-supplied (incident-system duplicate linkage in the
    intake bundle), rendered labeled "incident-system-linked". Emit an observable fast-lane verdict
    and a `6_report` so the run is auditable. Record a FAST-LANE receipt — including
    `wave1_result:` (same|different|ambiguous|cant-verify), the `wave1_check:` OBS + `gate:`
    status, the prior-root-cause source sibling, the drift-backstop state
    (`family_validated_marker` + `fast_lanes_since_deep_validation`), and the disposition — plus
    the `model_tiering` Tier Record in `run.md`.
  - **DIFFERENT / ambiguous / can't-verify → ESCALATE → DEEP-LANE.** Run the normal deep-lane
    (Scout → Specialists → Grader → Report) with NO verdict bar lowered. The cluster recognition +
    the wave-1 OBS carry forward as intake context (a pre-registered discriminator check the
    deep-lane reuses); never post a duplicate on doubt. Catch-all: any wave-1 return that is not an
    internally consistent `same` + `gate: pass` (including a missing/unexpected value) escalates
    (defensive fail-open).
- **PERIODIC RE-VALIDATION BACKSTOP (computable floor).** The backstop is the sole structural guard
  against a subtly-wrong prior root cause perpetuating as an auto-posted duplicate, so its firing is
  FLOORED and COMPUTABLE from IcM state the run already holds at intake (the recurrence cluster +
  threads) — never judgment alone, never a ledger/pipeline/sibling-run reads. The coordinator forces
  the FULL deep-lane instead of the wave-1 fast-lane when EITHER floor trips: (a) **marker
  staleness** — no `family-validated` marker for this recurrence identity is visible in the intake
  recurrence cluster / thread within a conservative **~30-day** recency window; or (b)
  **consecutive-fast-lane cap** — after **N = 10** consecutive fast-lane SAME dispositions for the
  family with no intervening deep-lane validation (counted from the cluster/thread markers). Only a
  DEEP-LANE disposition writes the machine-readable `family-validated` marker (a stable tag +
  recurrence-identity + UTC timestamp) into its IcM post — a fast-lane duplicate post never does — so
  the next sibling's intake read surfaces it and both floors are computable from IcM alone. The floor
  is judgment-INFORMED (deep-validate MORE often at will) but never judgment-ONLY. A deep-lane
  re-validation that diverges from the family's assumed root cause records a `recurrence-drift`
  candidate in `7_knowledge` for human attention and surfaces the corrected disposition in its IcM
  post. Full contract: `references/fast-lane.md` § Recall safeguards #7.

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
that is BOTH not generated by this agent AND plausibly changes what we would **conclude, check,
prioritize, or post** (verdict, confidence, scope/impact, mitigation state, investigative
direction, or posting obligation). When it conclusively does not (e.g. the only new timeline
activity is this agent's own prior post, an empty packet, an unchanged status, an acknowledgment,
a reassignment/routing/handoff ping, an automated service-account refresh of the same signal with
no material scope change, or thread housekeeping), terminate as an explicit report-only early-exit:
mint the run, record `early_exit: no_genuine_new_information` with the prior-run pointer and reason
(carry a `reason_class=` label) in `run.md`, then dispatch nothing (no Scout/Specialists/Grader/Report)
and post nothing. Default to PROCEED when origin or materiality is uncertain (fail open). Primary
materiality test, classes, and fail-open refresh comparator: `references/followup.md` § Early-exit gate.

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
- Intake fast-lane (wave-1 duplicate verification): `references/fast-lane.md`
- What each stage produces: `references/artifact-contracts.md`
- How to judge: `references/grading-rubric.md`
- Specialist worker guidance: `references/specialists/AGENTS.md`
- Scout role: `references/subagents/scout.md`
- Grader role: `references/subagents/grader.md`
- Report role: `references/subagents/poster.md`
- Knowledge Curator role: `references/subagents/knowledge.md`
