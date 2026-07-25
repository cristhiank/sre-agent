# Questions and symptom families

Folders do not create coverage. A knowledge base is useful only when it answers
the questions a responder must resolve and exposes the families it cannot
route.

Across eight measured knowledge bases, entry-point routing contained **zero**
rows for opt-out, consent, or data residency. Both of the most serious real
incidents in that service family entered through that blind spot. Treat these
families as first-class investigation surfaces, not optional edge cases.

## The four questions

| Question | A useful answer resolves | A misleading substitute |
|---|---|---|
| **Where does it run, and who owns it?** | The runtime and deployment boundaries that change behavior; region or cohort differences; operational owner, escalation boundary, and any ownership uncertainty | A component inventory with no incident owner or affected scope |
| **Who calls whom, and what is the blast radius?** | Edge direction; runtime call versus embedded behavior; contract or guard on the edge; propagation, fallback, isolation, and what is explicitly not affected | A list of dependencies with no direction, failure semantics, or non-impact |
| **How can it be seen in production?** | A bounded route to the signal; exact source coordinates, fields, dimensions, and join keys; signal meaning; casing or view traps that return empty instead of failing | A dashboard or source pointer that leaves the responder to discover the query |
| **What breaks, and why?** | Arrival symptom; nearest look-alike; observation that separates them; mechanism; mitigation or owner; trust and unresolved cause | An error catalog that maps every symptom directly to one cause |

Let the service choose the representation. The four questions are stable because
they mirror investigation decisions; a fixed directory or document set is not.
The exception is the small stable surface other writers bind to — an evidence
and grade record, a symptom-to-route index, a failure-knowledge home, a
freshness header. Those keep predictable locations so a later writer can find
what is already established; everything inside them is yours to shape.

## Families a responder arrives with

Availability, latency, throttling, and dependency failure are commonly present,
but they still need discriminating answers. The remaining families are often
missing entirely and therefore deserve deliberate attention.

| Family | What the knowledge must actually answer |
|---|---|
| **Availability** | Which operation is failing, for which scope, and whether “down” means errors, no traffic, sparse-denominator noise, or a health signal that does not exercise the failing path. Name the first observation that separates those states so an alert does not become the diagnosis. |
| **Latency** | Where time is spent—queueing, local execution, retry, dependency, or propagation—and which budget expires first. Include the measured scope and percentile or deadline semantics because an average cannot distinguish a tail stall from broad slowness. |
| **Throttling** | Which boundary imposes the limit, what identity or resource key it counts, the window or concurrency model, and how rejection differs from saturation. State retry and backoff behavior because a safe retry at one boundary can amplify another. |
| **Dependency failure** | Which edge failed, how its error is translated, whether fallback or isolation engaged, and which callers remain unaffected. Name the local evidence and the dependency-side evidence separately so correlation is not promoted to ownership. |
| **Privacy and opt-out** | Where exclusion is declared, where it is enforced, which read, write, compute, indexing, or export paths it covers, and which paths are intentionally exempt. Record propagation or cache delay and fail-open or fail-closed behavior because a correct setting is not proof that every entry point honored it. |
| **Consent** | What grant, denial, expiry, or revocation state is evaluated; at which entry point; for which actor and scope; and how a changed decision propagates. Show how denial appears in telemetry because consent failures often resemble empty results or ordinary authorization failures. |
| **Data residency** | What placement or routing rule governs ingestion, processing, storage, replication, and operational telemetry; where crossing a boundary is allowed; and what proves enforcement. Include fallback and migration behavior because nominal placement does not describe degraded or transitional paths. |
| **Caller abuse and enumeration** | Which caller identity, tenant, resource, or network boundary is controlled; how listing, search, pagination, and repeated misses are bounded; and whether responses leak object existence. Give the signal and mitigation route because abuse can look like organic load while privacy leakage can look like harmless probing. |
| **Deployment and rollout** | What version, configuration, policy, or feature state changed; which cohorts received it; how mixed states interact; and what rollback can and cannot reverse. Tie symptoms to rollout timing and cohort boundaries because fleet-wide summaries hide partial deployment failures. |
| **Capacity** | Which finite resource is exhausted, its accounting unit and scope, the saturation signal, scaling delay, and hard versus soft limit. Separate local capacity from dependency quota because both can produce the same latency, rejection, or backlog shape but require different owners. |

Do not force every incident into one family. A rollout may expose a consent-cache
bug, or caller abuse may consume capacity. Cross-link the mechanisms in their
own words because each arrival query uses different vocabulary.

## When a family has no route

A missing route is a result, not permission to write generic guidance. Record
the gap where a responder searching that family will find it, because silence
looks indistinguishable from accidental omission.

State what is known, what surfaces were searched, what could not be established,
and the next evidence that would close the gap. Naming the search scope prevents
“not found” from becoming “does not exist”; naming the missing endpoint keeps a
placeholder from pretending to terminate an investigation.

```text
# consent — no established production route

known: source declares a consent decision before the read path
searched: entry-point declarations, guard call sites, telemetry catalog, incident history
not-established: enforcement coverage for background processing and cached decisions
not-established: production signal, exact fields, and bounded probe
risk: a denial may surface as an empty result rather than an explicit error
next-evidence: trace the guard through background entry points and resolve its signal taxonomy
```

Do not guess the owner, source, field, or query to make the gap look complete.
An honest unrouted family tells the next investigation where the knowledge base
is blind; an invented route sends it confidently to the wrong place.
