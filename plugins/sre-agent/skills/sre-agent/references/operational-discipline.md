# Operational Efficiency Floor

This floor is SUBORDINATE to the honesty floor (`investigation-invariants.md`): when efficiency and correctness conflict, correctness wins. It cuts waste; it never licenses under-probing.

## Evidence-driven probing (the spine)

- Before each search, read, or dispatch, name the claim, discriminator, or routing decision it will close. Probes are gap-closing, not exploratory momentum: no probe without a named target it would resolve or falsify.

## Cost budget & stop rule

Precedence: **honesty floor > budget > efficiency.** A budget may stop additional source-hunting, but it NEVER converts missing, empty, unavailable, or weak evidence into evidence of absence. To state that a candidate cause did not occur, did not contribute, or is ruled out, you MUST have a fit-for-purpose authoritative probe for that specific claim, with scope/time/entity alignment sufficient for the claim. If the budget is exhausted before that standard is met, the ONLY allowed output is a named-unchecked / unknown gap — never "no evidence of", "ruled out", or causal-absence wording. One extra probe beyond budget is REQUIRED when it is the smallest probe needed to avoid recording false causal absence OR to satisfy the failing-unit gate; name that reason locally.

Default per-specialist budget for one assigned discriminator (defaults, tuned by telemetry — make the tripwire visible, do not treat as advisory):
- **≤ 12 evidence probes** (live queries/reads that could close the discriminator) to resolve it; stop when it is resolved, when a claim-safe unknown is reached, or when budget+escape is exhausted.
- **Empty/negative path: 1 primary fit-for-purpose probe + at most 2 meaningful retries (3 attempts total).** A retry MUST change the failure mode — re-scope, re-lens, or re-source — not a cosmetic query variant; you may move to ONE alternate source family when the first is known incomplete. If all retries hit the same source-family limit, stop and record source-family exhaustion — do not keep pivoting across sources for confidence.
- **Escape: at most +3 probes beyond the budget**, each allowed only under the honesty-floor exception above (avoid false absence / failing-unit gate) and each justified locally.
- Do not add probes for confidence, wording, adjacent questions, or provenance.

You run in a parallel wave; spend probes only while they can MATERIALLY resolve your assigned discriminator, because over-spending gates every sibling. If the next probe would likely close the discriminator or prevent a false-absence claim, take it and name why; otherwise stop. Minimizing sibling wait never licenses weakening evidence.

When you stop with the discriminator unresolved, hand off a compact unresolved-discriminator receipt (the unresolved-lead extension of your claim-readiness ledger, not a separate artifact, and not a vague gap) so the Grader can adjudicate it: assigned discriminator · why it matters to the verdict · probes attempted (source / scope / time / key filters → observed result) · why the result is not absence · best next probe if refinement chooses to spend · verdict-risk (high | medium | low) as the specialist's PRIOR · claim cap = unknown only. The specialist's `verdict-risk` only prioritizes WITHIN the Grader's follow-up budget; it never gates a lead out — the Grader re-rates materiality independently and runs its follow-up round on any receipt IT judges material and reachable, so a `low` self-rating can never silently drop a lead the Grader judges material (no material lead is silently dropped — see the lead ledger in [grading-rubric.md](grading-rubric.md) and the coordinator follow-up round).

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
