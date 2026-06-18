# Operational Efficiency Floor

This floor is SUBORDINATE to the honesty floor (`investigation-invariants.md`): when efficiency and correctness conflict, correctness wins. It cuts waste; it never licenses under-probing.

## Evidence-driven probing (the spine)

- Before each search, read, or dispatch, name the claim, discriminator, or routing decision it will close. Probes are gap-closing, not exploratory momentum: no probe without a named target it would resolve or falsify.

## Search discipline

- Prefer a structured search capability that returns empty gracefully over broad shell text-search.
- Scope searches with identifiers already in hand — scope/scenario/ring ids, the exact error string — rather than broad guesses.
- A no-match is SCOPED negative evidence: "not found in THIS source with THIS query," which constrains the next probe — NOT causal absence. It never means "absent from the system" or "not causal."
- Recording that something "didn't happen" still requires a fit-for-purpose authoritative probe (the honesty floor's "empty is not absent" governs); a search miss is not that probe.
- Keep a no-match distinct from a genuine tool error: an empty result is data, a failed invocation is not.

## Read discipline

- Default to ranged/targeted reads; lead with the per-service knowledge base for orientation before broad source reading.
- Do not re-read a static run/stage artifact already in context; this never licenses reusing a stale live-system read — re-fetch live state when correctness needs it.
- "Evidence sufficient" means the live discriminator for the claim has been resolved — the source that would falsify the claim was checked or named as explicitly unchecked — not merely that nearby or plausible evidence exists.
- Sufficiency only stops redundant READING. It never licenses recording causal absence, skipping the discriminator, or stopping before the source that would falsify the claim.

## Tooling / blocked capabilities

- Detect capability presence/inventory from the host's capability surface rather than trial-and-error; ACCESS is still confirmed by canonical invocation (Access confirmation).
- A capability is "blocked" only after environment-level confirmation; one failed call is non-diagnostic — defer to the Access-confirmation rules.
- Once confirmed blocked, the coordinator records it once in the run's CAPABILITY MAP and does not re-attempt it within the run; a specialist on that evidence path, a full-evidence context, or a newly discovered/provided target may still re-confirm per Access confirmation (`blocked` is provisional).
