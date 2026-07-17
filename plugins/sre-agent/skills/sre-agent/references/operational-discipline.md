# Operational Efficiency Floor

This floor is SUBORDINATE to the honesty floor (`investigation-invariants.md`): when efficiency and correctness conflict, correctness wins. It cuts waste; it never licenses under-probing.

## Evidence-driven probing (the spine)

- Before each search, read, or dispatch, name the claim, discriminator, or routing decision it will close. Probes are gap-closing, not exploratory momentum: no probe without a named target it would resolve or falsify.

## Run pursuit governor (canonical)

At intake, write this compact budget record to `run.md`:
```toon
pursuit_budget:
  started_at: <wall clock noted before incident-identity resolution; back-date this value when run.md is created>
  deadline_at: <explicit user/runtime/host deadline, else started_at + 75m>
  deadline_basis: user|runtime|host-lease|default-75m
  bootstrap_envelope: 7m
  planned_deep_lane_budget: 65m
  deep_lane_remaining_floor: 58m
  initial_evidence_stop_at: <deadline_at - 35m>
  synthesis_stop_at: <deadline_at - 25m>
  evidence_stop_at: <deadline_at - 15m>
  report_tail: 15m
  deadline_enforcement: host-bounded|cooperative
  budget_state: active|incompatible-deep-lane-envelope
  post_initial_batch: available|consumed|not-dispatched
  current_stage: <stage>
```

The run schedule reserves every mandatory envelope in execution order: Bootstrap/capture
`7m`, Scout `8m`, the parallel initial Specialist wave `15m`, Grader synthesis `10m`,
the possible single post-synthesis batch `10m`, bounded consequence audit `3m`, optional
introduction-provenance `4m`, Report `6m`, and `2m` of collection/write slack. An
explicit deep-lane plan therefore needs 65 minutes from `started_at`, but this planning
floor does not reject a shorter run that is independently admitted to the cheaper fast
lane. After Bootstrap and the FAST-LANE DECISION, the authoritative deep-lane gate is
the actual remaining time: less than 58 minutes is incompatible. Record
`budget_state=incompatible-deep-lane-envelope`, launch no deep-lane worker, and return a
causal-claim-free budget blocker requesting a compatible deadline. Never silently extend
the supplied deadline.

The three stop times make the conditional batch reachable without consuming the terminal
tail:

- `initial_evidence_stop_at`: Scout and the initial Specialist wave are terminally
  settled; no new initial evidence worker starts. The wave admits at most five
  independent attempts in parallel.
- `synthesis_stop_at`: Grader synthesis is terminally settled. If it emits material
  obligations, the complete set may use the reserved `10m` post-synthesis window.
- `evidence_stop_at`: that batch is terminally settled or retired; no new evidence work
  starts. The remaining `15m` `report_tail` is reserved only for audit, optional
  provenance, Report, and collection/write slack.

The terminal envelope is protected. Before every post-intake **evidence-producing** dispatch,
record a compact `pursuit_gate`: named obligation/discriminator; cited evidence signal;
how the result could change verdict, owner, mitigation, or the operator's next step;
expected wall-clock/output cost; mandatory successor reserve; worker stop time; and
`budget_fit=yes|no`. Scout/initial-Specialist attempts must fit before
`initial_evidence_stop_at`; a post-synthesis attempt must fit before `evidence_stop_at`.
The worker stop time plus every still-required successor envelope must fit before
`deadline_at`. Do not dispatch a worker whose full envelope and collection allowance do
not fit before its stage stop.
Do not dispatch evidence work when it is confidence/wording-only, lacks a positive
evidence signal for its hypothesis class, duplicates an owned discriminator, or would
consume the protected tail. Mandatory synthesis/audit/Report stages use their reserved
envelopes rather than this evidence-signal test.

After the initial Specialist batch, the run has at most ONE additional awaited evidence
batch. It contains all independent material refinement obligations, including any
eligible residual-discovery or core-output edge obligation. No second/third batch,
recursion, or overflow wave exists. Rank the complete obligation set by consequence and
dispatch at most five independent attempts whose full envelopes fit the reserved window.
Record every capacity overflow obligation as `open-answerable` with reason
`open-budgeted`, its activation evidence, and its next discriminator. If no attempt fits,
set `post_initial_batch=not-dispatched`; otherwise set it `consumed`. Initial Specialist
fanout uses the same max-five rule: dispatch the five highest-consequence activated
hypotheses and record every overflow hypothesis as activated but not dispatched, with
gap reason `open-budgeted` and its next discriminator. No class is silently dropped and
no sequential overflow chunk consumes a later stage's window.

Every dispatch has the canonical Awaited Stage Attempt Receipt from
`artifact-contracts.md`. With `deadline_enforcement=host-bounded`, a host timeout or
cancellation at the recorded cutoff is a terminal event: retire the attempt
`open-budgeted` and advance. With `deadline_enforcement=cooperative`, the coordinator
cannot abandon a still-running awaited worker; it must await the host return and record a
runtime deadline overrun. A late evidence-producing result is embargoed from downstream
artifacts. A late reasoning-only Grader synthesis, audit, or Report may be
`merged-late-terminal`; the canonical deadline-degraded successor rule admits it while
forcing report-only finalization. A running worker never becomes a synthetic partial
result. This preserves the awaited-dispatch invariant while making clear that a hard
deadline requires runtime support.

At `evidence_stop_at`, launch no new evidence work. Audit consumes only deltas whose
attempt receipts are `complete|partial` and `merge_disposition=merged`; every retired or
late obligation remains `open-budgeted`. Then run the remaining mandatory terminal
stages and produce Report. Budget exhaustion is never absence or refutation.

Default stage envelopes are stop signals, not licenses to truncate proof: Scout
`8m / ~6KB`; each initial Specialist `15m / ~8KB`; Grader synthesis `10m / ~8KB`;
each post-initial worker `10m / ~4KB`; bounded consequence audit `3m / ~3KB`; Report
`6m`; Knowledge Curator `2m / ~2KB and one item`. Raw tool artifacts stay behind
pointers and do not count toward authored output. A load-bearing proof may exceed its
output target only with `output_exception: <claim + why compression would lose proof>`;
ceremony, restated context, and settled branches never qualify.

## Cost budget & stop rule

Precedence: **honesty floor > budget > efficiency.** A budget may stop additional source-hunting, but it NEVER converts missing, empty, unavailable, or weak evidence into evidence of absence. To state that a candidate cause did not occur, did not contribute, or is ruled out, you MUST have a fit-for-purpose authoritative probe for that specific claim, with scope/time/entity alignment sufficient for the claim. If the budget is exhausted before that standard is met, the ONLY allowed output is a named-unchecked / unknown gap — never "no evidence of", "ruled out", or causal-absence wording. Before `evidence_stop_at`, one extra probe beyond a worker's local probe budget is REQUIRED when it is the smallest probe needed to avoid recording false causal absence OR to satisfy the failing-unit gate; name that reason locally. At/after `evidence_stop_at`, protect Report instead: retain the unknown gap and launch no new probe.

Default per-specialist budget for one assigned discriminator (defaults, tuned by telemetry — make the tripwire visible, do not treat as advisory):
- **≤ 12 evidence probes** (live queries/reads that could close the discriminator) to resolve it; stop when it is resolved, when a claim-safe unknown is reached, or when budget+escape is exhausted.
- **Empty/negative path: 1 primary fit-for-purpose probe + at most 2 meaningful retries (3 attempts total).** A retry MUST change the failure mode — re-scope, re-lens, or re-source — not a cosmetic query variant; you may move to ONE alternate source family when the first is known incomplete. If all retries hit the same source-family limit, stop and record source-family exhaustion — do not keep pivoting across sources for confidence.
- **Escape: at most +3 probes beyond the budget**, each allowed only under the honesty-floor exception above (avoid false absence / failing-unit gate) and each justified locally.
- Do not add probes for confidence, wording, adjacent questions, or provenance.
- **Output ceiling.** Return a COMPACT structured record — a claim-readiness ledger / typed rows, not a prose narrative (§ Cap prose, not proof). Generate no output tokens for restating the brief, narrating steps taken, hedging, or re-summarizing settled context.

**Early stop (the stop condition is discriminator-resolved, NOT coverage-complete).** Return the moment your assigned discriminator is RESOLVED (favored / rival / claim-safe-unknown) OR the budget is hit — whichever first. Accounting for every obligation, corroborating an already-resolved discriminator, or hunting confidence/wording/provenance is NOT a reason to keep going: coverage / obligation-completeness is not a stop condition. Honesty floor still overrides — never record causal absence to stop early; an unresolved discriminator returns the compact unresolved receipt below. Early-stop and "coverage is not a stop condition" are Worker-scoped to a single assigned discriminator; they NEVER authorize skipping the Grader synthesis's conditional arrival / onset-signature / Gate E checks or the final audit's PINNED always-re-derived set for those triggered consequence-bearing claims ([grading-rubric.md](grading-rubric.md)), nor softening a MUST-dispatch arrival probe while the single post-synthesis batch remains available. The honesty override bars stopping on a false-PRESENCE miscausation (a rollback treated as causal per Gate E, or a non-arrived change named as the cause) as much as on false absence.

**Stay in your lane (breadth is not a worker's decision).** A specialist resolves ONLY its assigned discriminator; it does NOT spawn sibling hypotheses, widen scope, or fan out to adjacent causes — breadth is the coordinator's evidence-gated core-and-edge decision (`SKILL.md` § Six-stage flow → Specialists, **Core-and-edge roster**), not a worker's.

**Residual-discovery allowance (coordinator execution only).** Specialists reserve no residual-search slice and do no off-discriminator scan. The canonical trigger, selection, and obligation rule lives in [grading-rubric.md](grading-rubric.md) § Independent residual discovery. At dispatch time:

`residual allowance = evidence_stop_at - now - already-committed mandatory work`.

Dispatch the single residual obligation only when that allowance covers one bounded discriminator worker plus collection inside the one post-initial batch; never create an overflow batch solely for residual work. Otherwise retain the lead as `open-answerable` with gap reason `open-budgeted` and the pre-registered next discriminator. `open-budgeted` is a gap reason, not a ledger status, and is never absence, refutation, or `blocked-unreachable`. One worker, one class, one discriminator; no recursion, broadening, sibling spawn, or extra pursuit batch.

You run in a parallel wave; spend probes only while they can MATERIALLY resolve your assigned discriminator, because over-spending gates every sibling. If the next probe would likely close the discriminator or prevent a false-absence claim, take it and name why; otherwise stop. Minimizing sibling wait never licenses weakening evidence.

When you stop with the discriminator unresolved, hand off a compact unresolved-discriminator receipt (the unresolved-lead extension of your claim-readiness ledger, not a separate artifact, and not a vague gap) so the Grader can adjudicate it: assigned discriminator · why it matters to the verdict · probes attempted (source / scope / time / key filters → observed result) · why the result is not absence · best next probe if refinement chooses to spend · verdict-risk (high | medium | low) as the specialist's PRIOR · claim cap = unknown only. The specialist's `verdict-risk` only prioritizes WITHIN the Grader's follow-up budget; it never gates a lead out — the Grader re-rates materiality independently and may select the receipt into the single post-synthesis batch when material and reachable, so a `low` self-rating can never silently drop a lead the Grader judges material (no material lead is silently dropped — see the lead ledger in [grading-rubric.md](grading-rubric.md) and the coordinator's bounded refinement).

## Cap prose, not proof

Agent-to-agent artifacts (theory notes, claim-readiness ledgers, the grader ranking, the scout report) are COMPACT structured records — terse typed rows / ledgers — NOT prose narratives, because output volume is a first-order latency + cost driver. Compress ceremony: no restating the brief, no step-by-step narration, no hedging, no re-summary of settled context. The compact SHAPE is MANDATORY, not a stylistic default: `theory.md` uses typed fields + claim-readiness ledger rows with a proof sketch as its ONLY unbounded section, and `scout-report.md` uses typed rows and stays neutral (no proof sketch) — so verbosity has nowhere structural to go while the causal proof stays uncapped. The artifact shapes are canonical in [artifact-contracts.md](artifact-contracts.md) (`ranking.md` already carries a fixed schema).

A PROOF SKETCH is REQUIRED — and never compressed — for any load-bearing causal chain: the mechanism-discriminator; the change-arrival gate resolution INCLUDING an arrival-DISPROVEN refutation (show the serving-build-vs-merge + backport/cherry-pick check, not just "not causal"); the onset-signature→cause-class rung; Gate E (post-onset actor) eligibility; or any contested/consequential verdict. Spend words on the causal proof; spend none on ceremony.

<bad-example> A clean, single-cause run returns an 18-section narrative report re-explaining intake, methodology, and every ruled-out branch in prose. Correct: a compact verdict + ledger + the ONE proof sketch the finding actually needs. </bad-example>

Honesty floor > this rule: never drop the causal proof, cited evidence, or a visible gap to save tokens.

## Search discipline

- Prefer a structured search capability that returns empty gracefully over broad shell text-search.
- Scope searches with identifiers already in hand — scope/scenario/ring ids, the exact error string — rather than broad guesses.
- A no-match is SCOPED negative evidence: "not found in THIS source with THIS query," which constrains the next probe — NOT causal absence. It never means "absent from the system" or "not causal."
- Recording that something "didn't happen" still requires a fit-for-purpose authoritative probe (the honesty floor's "empty is not absent" governs); a search miss is not that probe.
- Keep a no-match distinct from a genuine tool error: an empty result is data, a failed invocation is not.

## Read discipline

- Default to ranged/targeted reads; lead with the per-service knowledge base for orientation before broad source reading.
- Before reopening a static run/stage artifact already read this run, consult the RUN-STATE DIGEST in `run.md`; if it is missing or stale, treat that as a run defect and repair it from in-context artifacts or a named missing-field reopen. Reopen only to cite exact lines, resolve a named contradiction, or fill a named missing field, and record the reason. This never licenses reusing a stale live-system read — re-fetch live state when correctness needs it.
- "Evidence sufficient" means the live discriminator for the claim has been resolved — the source that would falsify the claim was checked or named as explicitly unchecked — not merely that nearby or plausible evidence exists.
- Sufficiency only stops redundant READING. It never licenses recording causal absence, skipping the discriminator, or stopping before the source that would falsify the claim.
- Enumeration is gap-closing, not momentum: the test is a NEW named gap the next record would close, not "more of the same evidence." Once the live discriminator is resolved AND the authoritative failing population is already bound to its producer, paginating more records of the same shape and scope changes no answer — stop and prefer a bounded sample (a resolved discriminator with no failing unit yet keyed to a producer is NOT a stop — keep enumerating). Enumeration still binding that population, or closing another open discriminator, IS gap-closing and continues; this never licenses substituting an aggregate for the failing-unit enumeration gate ([grading-rubric.md](grading-rubric.md)) or recording causal absence.

## Tooling / blocked capabilities

- Detect capability presence/inventory from the host's capability surface rather than trial-and-error; ACCESS is still confirmed by canonical invocation (Access confirmation).
- A capability is "blocked" only after environment-level confirmation; one failed call is non-diagnostic — defer to the Access-confirmation rules.
- Once confirmed blocked, the coordinator records it once in the run's CAPABILITY MAP and does not re-attempt it within the run; a specialist on that evidence path, a full-evidence context, or a newly discovered/provided target may still re-confirm per Access confirmation (`blocked` is provisional).
