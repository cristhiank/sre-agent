# Operational Efficiency Floor

This floor is SUBORDINATE to the honesty floor (`investigation-invariants.md`): when efficiency and correctness conflict, correctness wins. It cuts waste; it never licenses under-probing.

## Evidence-driven probing (the spine)

- Before each search, read, or dispatch, name the claim, discriminator, or routing decision it will close. Probes are gap-closing, not exploratory momentum: no probe without a named target it would resolve or falsify.

## Run pursuit governor (canonical)

At intake, write this compact structural record to `run.md`:
```toon
pursuit_limits:
  initial_wave_fanout_cap: 5
  refinement_wave_cap: 1
  refinement_wave_fanout_cap: 5
  coordinator_direct_probe_cap: 2
  specialist_probe_cap: 12
  specialist_negative_attempt_cap: 3
  specialist_honesty_escape_probe_cap: 3
  residual_discovery_cap: 1
  consequence_audit_additional_claim_cap: 2
  provenance_history_read_cap: 8
  post_initial_batch: available|consumed|not-dispatched
  current_stage: <stage>
```

Before every post-intake **evidence-producing** dispatch, record a compact
`pursuit_gate`: named obligation/discriminator; cited positive evidence signal; how the
result could change verdict, owner, mitigation, or the operator's next step; fitting
capability; duplicate-owner check; wave/slot availability; probe/read/output caps; and
stop condition. Do not dispatch confidence/wording-only work, a duplicate discriminator,
or work with no reachable material discriminator. Mandatory synthesis, consequence
audit, and Report are artifact-integrity stages rather than evidence-signal-gated work.

After the initial Specialist batch, the run has at most ONE additional awaited evidence
batch. It contains all independent material refinement obligations, including any
eligible residual-discovery or core-output edge obligation. No second/third batch,
recursion, or overflow wave exists. Rank the complete obligation set by consequence and
dispatch at most five independent attempts.
Record every capacity overflow obligation as `open-answerable` with reason
`fanout-cap`, its activation evidence, and its next discriminator. If no obligation
survives the materiality, reachability, duplicate-owner, and slot checks,
set `post_initial_batch=not-dispatched`; otherwise set it `consumed`. Initial Specialist
fanout uses the same max-five rule: dispatch the five highest-consequence activated
hypotheses and record every overflow hypothesis as activated but not dispatched, with
gap reason `fanout-cap` and its next discriminator. No class is silently dropped and
no sequential overflow chunk exists. Once the refinement wave is consumed, a newly
exposed reachable lead remains `open-answerable` with reason `wave-cap`; limit
exhaustion is never absence or refutation.

Every dispatch has the canonical Awaited Stage Attempt Receipt from
`artifact-contracts.md`. A dispatch remains pending until the host emits a terminal event.
A finished `complete|partial` durable result is merged regardless of elapsed runtime.
Host timeout, host cancellation, and failure are observed terminal facts, not
prompt-enforced controls; record them and keep the obligation visible. A running worker
never becomes a synthetic partial result, and no required worker is abandoned or
orphaned.

After refinement settlement, run the mandatory bounded consequence audit and then
Report. If mandatory synthesis or audit has no valid artifact because its terminal
receipt is timeout, cancellation, failure, missing-artifact, or invalid-artifact, use
the canonical `mandatory_stage_integrity` successor. That path preserves a bounded
local report but prohibits live posting. A valid accepted report remains post-eligible
regardless of elapsed runtime; only the Poster's non-time safety and integrity gates
decide mutation eligibility.

Default authored-output caps are: Scout `~6KB`; each initial Specialist `~8KB`; Grader
synthesis `~8KB`; each refinement worker `~4KB`; bounded consequence audit `~3KB`;
Knowledge Curator `~2KB and one item`. Raw tool artifacts stay behind pointers and do
not count toward authored output. A load-bearing proof may exceed its output target
only with `output_exception: <claim + why compression would lose proof>`; ceremony,
restated context, and settled branches never qualify.

## Probe caps & stop rule

Precedence: **honesty floor > structural limits > efficiency.** A limit may stop
additional source-hunting, but it NEVER converts missing, empty, unavailable, or weak
evidence into evidence of absence. To state that a candidate cause did not occur, did
not contribute, or is ruled out, you MUST have a fit-for-purpose authoritative probe
for that specific claim, with scope/time/entity alignment sufficient for the claim.
If the applicable probe/wave cap is exhausted first, the ONLY allowed output is a
named-unchecked / unknown gap — never "no evidence of", "ruled out", or causal-absence
wording.

The smallest additional probe needed to avoid false causal absence or satisfy the
failing-unit gate is allowed only while either the worker's honesty-escape probe cap or
an unspent slot in the single refinement wave remains. Name that reason locally. Once
neither allowance remains, retain the unknown gap with reason `probe-cap` or
`wave-cap`; never convert limit exhaustion into absence/refutation.

Default per-specialist caps for one assigned discriminator (make the tripwire visible,
do not treat it as advisory):
- **≤ 12 evidence probes** (live queries/reads that could close the discriminator) to resolve it; stop when it is resolved, when a claim-safe unknown is reached, or when the probe and escape caps are exhausted.
- **Empty/negative path: 1 primary fit-for-purpose probe + at most 2 meaningful retries (3 attempts total).** A retry MUST change the failure mode — re-scope, re-lens, or re-source — not a cosmetic query variant; you may move to ONE alternate source family when the first is known incomplete. If all retries hit the same source-family limit, stop and record source-family exhaustion — do not keep pivoting across sources for confidence.
- **Escape: at most +3 probes beyond the ordinary probe cap**, each allowed only under the honesty-floor exception above (avoid false absence / failing-unit gate) and each justified locally.
- Do not add probes for confidence, wording, adjacent questions, or provenance.
- **Output ceiling.** Return a COMPACT structured record — a claim-readiness ledger / typed rows, not a prose narrative (§ Cap prose, not proof). Generate no output tokens for restating the brief, narrating steps taken, hedging, or re-summarizing settled context.

**Early stop (the stop condition is discriminator-resolved, NOT coverage-complete).** Return the moment your assigned discriminator is RESOLVED (favored / rival / claim-safe-unknown) OR the applicable probe cap is hit — whichever first. Accounting for every obligation, corroborating an already-resolved discriminator, or hunting confidence/wording/provenance is NOT a reason to keep going: coverage / obligation-completeness is not a stop condition. Honesty floor still overrides — never record causal absence to stop early; an unresolved discriminator returns the compact unresolved receipt below. Early-stop and "coverage is not a stop condition" are Worker-scoped to a single assigned discriminator; they NEVER authorize skipping the Grader synthesis's conditional arrival / onset-signature / Gate E checks or the final audit's PINNED always-re-derived set for those triggered consequence-bearing claims ([grading-rubric.md](grading-rubric.md)), nor softening a MUST-dispatch arrival probe while the single post-synthesis batch remains available. The honesty override bars stopping on a false-PRESENCE miscausation (a rollback treated as causal per Gate E, or a non-arrived change named as the cause) as much as on false absence.

**Stay in your lane (breadth is not a worker's decision).** A specialist resolves ONLY its assigned discriminator; it does NOT spawn sibling hypotheses, widen scope, or fan out to adjacent causes — breadth is the coordinator's evidence-gated core-and-edge decision (`SKILL.md` § Six-stage flow → Specialists, **Core-and-edge roster**), not a worker's.

**Residual-discovery allowance (coordinator execution only).** Specialists reserve no
residual-search slice and do no off-discriminator scan. The canonical trigger,
selection, and obligation rule lives in [grading-rubric.md](grading-rubric.md) §
Independent residual discovery. Dispatch the single residual obligation only when the
single refinement wave remains unspent and one of its max-five slots is available;
never create an overflow batch solely for residual work. Otherwise retain the lead as
`open-answerable` with gap reason `wave-cap` or `fanout-cap` and the pre-registered next
discriminator. A structural gap reason is never absence, refutation, or
`blocked-unreachable`. One worker, one class, one discriminator; no recursion,
broadening, sibling spawn, or extra pursuit batch.

You run in a parallel wave; spend probes only while they can MATERIALLY resolve your assigned discriminator, because over-spending gates every sibling. If the next probe would likely close the discriminator or prevent a false-absence claim, take it and name why; otherwise stop. Minimizing sibling wait never licenses weakening evidence.

When you stop with the discriminator unresolved, hand off a compact unresolved-discriminator receipt (the unresolved-lead extension of your claim-readiness ledger, not a separate artifact, and not a vague gap) so the Grader can adjudicate it: assigned discriminator · why it matters to the verdict · probes attempted (source / scope / time / key filters → observed result) · why the result is not absence · best next probe if refinement chooses to spend · verdict-risk (high | medium | low) as the specialist's PRIOR · claim cap = unknown only. The specialist's `verdict-risk` only prioritizes WITHIN the Grader's single refinement allowance; it never gates a lead out — the Grader re-rates materiality independently and may select the receipt into the single post-synthesis batch when material and reachable, so a `low` self-rating can never silently drop a lead the Grader judges material (no material lead is silently dropped — see the lead ledger in [grading-rubric.md](grading-rubric.md) and the coordinator's bounded refinement).

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
