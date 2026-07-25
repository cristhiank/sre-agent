# Refresh without clobbering curated knowledge

Treat a refresh as reconciliation, not regeneration. The current knowledge base
contains decisions as well as extracted facts; starting from a blank template
silently discards what a human or prior run already promoted.

## Re-mine the change, not the whole world

Compare the prior pinned inputs with the new ones and re-mine the surfaces whose
evidence changed, plus the dependent surfaces needed to interpret that change.
Leave unrelated records alone because broad re-rendering creates churn, stale
duplicates, and overwrite opportunities without adding evidence.

Follow a changed claim far enough to preserve meaning. A changed guard may alter
entry-point coverage, observability, blast radius, and failure knowledge even
when only one source file changed. File-level locality is not evidence-level
locality.

## Preserve before replacing

Treat every promoted fact in the prior knowledge base as an input to the
refresh. New evidence replaces it only when it addresses the same claim and is
stronger for that claim; “newer” alone is not stronger.

Judge competing evidence through these lenses:

| Lens | Decision rule | Why |
|---|---|---|
| Trust floor | Do not let a fresh inference overwrite an observed or owner-verified fact. | Lower-grade evidence cannot strengthen the same claim. |
| Exact applicability | Compare only evidence for the same entry point, deployment boundary, and failure variant. | Nearby evidence can silently broaden a claim beyond its proven scope. |
| Evidence fit | Match source evidence to implementation intent, live evidence to observed behavior, and documentation to declared behavior. | One evidence class cannot prove what only another class can show. |
| Freshness and anchors | Prefer current, re-resolved evidence after trust, scope, and fit are comparable. | A newer citation is useful only when it supports the same claim at equal or greater strength. |

A confirming live observation may strengthen the grade without changing the
fact. Record that as stronger support rather than rewriting the claim and losing
its history.

| New evidence | Refresh decision | Why |
|---|---|---|
| Confirms the same claim | Preserve the wording unless clarity is wrong; refresh evidence and freshness | Unnecessary rewriting makes curated meaning harder to review. |
| Proves a narrower or stronger replacement | Supersede only the affected claim and retain the prior disposition | The reader needs to know whether truth changed or merely gained support. |
| Is newer but weaker | Preserve the stronger fact and mark the new uncertainty or verification need | Recency cannot compensate for lower-grade evidence. |
| Conflicts at comparable strength | Preserve both as an explicit unresolved conflict | Silent adjudication hides the exact question that needs verification. |
| Loses its source anchor | Mark the fact stale or removed with the old anchor, new search scope, and follow-up probe | Source deletion does not prove that runtime behavior or downstream copies disappeared. |

## Never let absence become a silent delete

Account for every previously promoted fact that is missing from the refreshed
candidate. Preserve it, supersede it with stronger evidence, or state why it is
stale or removed. This is not bookkeeping ceremony: silent omission is
indistinguishable from accidental data loss to the next responder.

When a path, declaration, or signal disappears, retain enough context to explain
the old claim and the deletion evidence. Add a bounded verification route when
the behavior may persist outside the changed source, because source and runtime
can age at different rates.

## Keep curated additions grounded

Preserve promoted human and prior-run contributions, but do not upgrade a claim
merely because of who supplied it. Trust comes from applicable evidence,
resolvable anchors, and review. An ungrounded contribution remains a candidate;
a grounded one participates in the same comparison as every other fact.

Preservation never requires retaining secrets, raw sensitive payloads, or
restricted samples. Keep the durable fact and a resolvable non-sensitive pointer
because copying the payload creates a larger incident than losing the prose.

## Keep refresh debris out of search

Do not leave delta files, old copies, migration notes, or generation scratch in
the searched knowledge plane. They compete with current answers and can rank
above them. Keep history and replay evidence outside the responder-facing
corpus, while the current record carries the disposition needed to understand
what changed.
