# Specialists - Discovery & Worker Guidance

A specialist owns one hypothesis class. The coordinator discovers sibling specialist skills from their frontmatter `name` and `description`, then dispatches selected specialists as subagents.

Honesty floor: [../investigation-invariants.md](../investigation-invariants.md).

Operational efficiency floor: [../operational-discipline.md](../operational-discipline.md). Probes are gap-closing (name the claim/discriminator each closes); "sufficient" = the live discriminator is resolved. Operate under both floors; honesty wins.

## Discovery

This file is not a registry. A specialist description should be enough for routing: identify the hypothesis class, include trigger terms or obligation themes, and state that it operates within an investigation workflow that supplies its context, not as a direct user entrypoint.

The coordinator routes from descriptions; it does not load specialist bodies for routing, and children do not load the coordinator skill.

## Worker guidance

Use [../run-store.md](../run-store.md), [../artifact-contracts.md](../artifact-contracts.md), and [../grading-rubric.md](../grading-rubric.md) as needed. Keep the local role simple:

- Run as a subagent. Do **not** load the coordinator skill.
- Own your assigned hypothesis class and write only under `4_specialists/<your-name>/`.
- Start from the coordinator's brief: your hypothesis area, output directory, the skills you were passed, and any existing observations. Assign new `OBS###` ids as you record observations so the report can cite an unbroken chain.
- Worker class follows the coordinator's Dispatch routing (evidence-using specialists run `full-evidence`).
- Fetch and analyze your own observations through the passed skills; reuse existing observation ids when they already answer the question.
- Cite observation ids for material claims.
- Name cause plus mechanism in any theory you advance.
- Close assigned questions honestly in natural language: answered, no issue found, open gap, or needs more.
- Never silently substitute weaker observations or sources for the requested discriminator; make the gap visible.
- Bind to the authoritative failing population (the honesty floor's lead, with "Aggregate is not mechanism" and "Empty is not absent" folded under it — see [../investigation-invariants.md](../investigation-invariants.md)): when real failures exist, do not advance OR refute a mechanism without enumerating the failing units (or a justified keyed cohort) and following their keys to the producer; a zero-row/negative probe is a defect to re-probe fit-for-purpose (signal/scope/join) within budget, not an answer, unless fit for that population. Make genuine enumeration gaps explicit rather than substituting the aggregate.
- A blocked decisive discriminator is a delegation to produce, not just a gap to note. First earn it with the access invariant's cross-source pivot ([../investigation-invariants.md](../investigation-invariants.md)) within the bounded follow-up budget — a block, or a final `Proximate-only`, that skipped this reachable next-causal-layer probe is an assumed-block, so leave the lead open-answerable instead. Then, when the decisive check is genuinely unreachable because it needs a human-only or out-of-band capability, produce the read-only operator-executable manual check the on-call engineer would run — target/capability, required access/elevation, the exact action (e.g. command, query, API call, portal/GUI navigation, dashboard, runbook/handbook procedure, or owner request), the predicate it evaluates, and the expected-observation → meaning branches — and verify the operational steps (commands, roles, navigation) against the service knowledge AND the authoritative operational source/runbook/handbook it points to, citing that source. Mark each step `verified-with-citation`, `unverified`, or `missing-citation` rather than asserting an unverified step. This feeds the grader's adjudication and the report's Manual Investigation Kit (`../artifact-contracts.md` §`6_report/`).
