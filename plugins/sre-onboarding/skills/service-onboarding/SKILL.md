---
name: service-onboarding
description: >-
  Use when building or refreshing a knowledge base for a service so an incident responder — human or
  agent — can get from a symptom to a cause fast: "onboard a service", "build a service knowledge
  base", "refresh the service KB", "map this service for livesite", "what breaks in this service and
  how would I see it", "why can't the agent find anything about X". Produces evidence-cited knowledge
  under services/<service>/ from pinned source and, where a capability exists, live telemetry. Optimised
  for a consumer that retrieves by ranked search under a clock, not by browsing. Read-only over source
  and production; host-agnostic. Not for mutating a service, one-off code lookups, or authoring prompts.
---

# Service Onboarding

<goal>
A responder arrives with an alert title and a clock. The knowledge base earns its
place if that responder reaches **symptom → route → discriminator → cause/owner**
before the budget runs out.

Everything below serves that. The KB is distilled context, never authority —
pinned source and live systems remain ground truth.
</goal>

<the_consumer>
Assume the reader is an agent that **searches, it does not browse**. It issues a
ranked lexical query and receives roughly twenty files with a couple of lines of
context each. It has no patience, no memory of your folder structure, and no way
to ask a colleague.

Two consequences drive most authoring decisions:

- **A fact that cannot be retrieved does not exist.** Put the literal
  operational strings a responder would paste — monitor names, scenario names,
  routes, error codes, exception types — in the record that explains them. A
  paraphrase does not match. Prefer the stable stem of a generated name over one
  instance of it.
- **A record that ranks but cannot be read is not a hit.** Keep lines short
  enough to survive a narrow context window.
</the_consumer>

<durability_rule>
**Record what stays true between incidents. Anything that changes between
incidents is a probe, not a fact.**

Contracts, guard semantics, enforcement scopes, identifiers, thresholds,
topology, cadence — durable. Current versions, live policy values, today's
counts — volatile: record how to ask, never the answer.

A stale fact is worse than an absent one, because the reader trusts it.
</durability_rule>

<method>
**Fan out to collect, converge to judge, then try to break it.** These are three
different jobs and merging them loses the value of each.

- **Scouts** take disjoint surfaces and collect candidate facts with citations.
  Give each one the surface, why it matters — name the real incident that failed
  without it — and the durability rule. Then let them work. They should surface
  conflicts and leave them unresolved; a scout that adjudicates hides the
  disagreement.
- **A consolidator** merges, resolves or records conflicts, decides what earns a
  place, and writes. It has the whole map; scouts never do.
- **A verifier** tries to falsify what was written. Ask it what the others got
  wrong. Scouts are good at finding candidate mechanisms and poor at judging
  intent — they cannot cheaply see tests, comments, and call sites while sweeping
  breadth-first, so permissive-looking code reads as accidental when the codebase
  says deliberate.

Breadth before depth. Cheap parallel collection over a wide surface beats deep
serial reading of a narrow one, because you cannot tell which surface mattered
until you have seen them all.

If parallel workers are unavailable, run the three as **separate passes** and
treat each earlier pass's output as untrusted input rather than as your own
conclusion. Record which you did. A single pass that collects, judges, and
confirms in one motion produces a knowledge base that agrees with itself and has
been checked by nobody.
</method>

<families>
Cover the four questions. Report which ones this service cannot answer rather
than filling the gap with prose:

| | |
|---|---|
| **Where it runs, who owns it** | hosting, rings, regions, escalation |
| **Who calls whom, blast radius** | edges, dependencies, and what is *not* affected |
| **How to see it in production** | sources, coordinates, field taxonomy, probes |
| **What breaks and why** | symptom, discriminator, mechanism, mitigation |

Then check the symptom families a responder actually arrives with. Availability,
latency, throttling and dependency failure are usually well covered. These are
routinely missing entirely, and each has produced a real incident:

**privacy and opt-out · consent · data residency · caller abuse and enumeration ·
deployment and rollout · capacity**

A family with no route is a finding. Say so; do not invent one.
</families>

<what_good_looks_like>
A **record** answers *"here is what is true"* and must stand alone — state the
fact in its own words even when another artifact is its canonical home, because a
pointer stripped of the fact's vocabulary cannot be retrieved at all.

An **index** answers *"go here"*. It stays pointer-shaped and carries no facts.

A **route** is complete only with coordinates, the exact field and dimension
names, and any trap that returns empty rather than failing loudly. Coordinates
alone do not terminate a route; "compose a query yourself" is not a route.

The highest-value facts are the ones an investigator cannot rediscover under
time pressure: a query that returns nothing when written the obvious way, a
status code with several causes, a health check that does not prove what it
appears to prove, a comment that contradicts its own code. Hunt those
deliberately.
</what_good_looks_like>

<stable_surface>
You are not the only writer. Other capabilities append curated knowledge from
later investigations, and read this KB to decide what is already known — and
they bind to **locations**, not to prose. Shape the interior however the service
warrants, but keep these reachable at predictable paths:

- an **evidence/grade record** other writers read to learn what is already
  established and at what confidence — and which they never overwrite
- a **symptom → route index** a responder or a later writer enters through
- a **home for failure knowledge**, so curated findings have somewhere to land
- a **freshness and provenance header**, so a reader can tell how old this is

Without them a later writer has nothing to compare against, and its own
trust floor silently degrades to "whatever I just mined". This is an interface,
not a schema: names and internals are yours, existence and stability are not.
</stable_surface>

<invariants>
**Absence is not evidence.** Never claim nothing calls X, no edge exists, or a
signal is absent without naming what you searched and what remains unverified. A
term missing from one plane may be present under a different name in another.

**Unknown beats invented closure.** Every material thread ends as promoted,
rejected, or explicitly escalated. Never guess an owner, a limit, a table, a
dimension, or an abbreviation expansion.

**Grade what you assert.** Separate what source proves from what you inferred
from what a live probe confirmed, and never promote across those boundaries
silently. Thin evidence produces a thin record, and that is the correct outcome.

**Citations are load-bearing.** Cite the pinned identity you actually read. Never
extend a partial reference into a complete-looking one — a plausible fabricated
citation is worse than an honest gap, because it cannot be checked.
</invariants>

<done>
Not a checklist of artifacts. Three questions, and the **verifier** answers
them — not the author who wrote the records:

1. **Does it retrieve?** Take real alert titles for this service and search the
   KB as the agent would. Titles must come from monitor and alert definitions or
   incident history — **never from the records just written**, which only proves
   the KB matches its own vocabulary. Record the score and name the misses; a
   miss is the most useful output this test produces.
2. **Does it terminate?** Can a responder act on what they find, or does the
   trail end in a pointer?
3. **Is it honest?** Are the gaps stated, the grades earned, and the volatile
   facts expressed as probes?

Record what you pinned: the source revisions read, whether live evidence was
available, and the date. A refresh has nothing to compare against otherwise, and
will either re-mine everything or quietly call stale facts current.

On a refresh, add a fourth question: **did anything curated get clobbered?**
Re-mine what changed; preserve what a human or a prior run promoted.
</done>

<boundaries>
Read-only over source and production. Steering here names capability classes
only; the KB itself records concrete evidence-cited coordinates, because that is
what makes it useful. Never copy secrets, credentials, or raw sensitive payloads
into the KB — and never sanitise a coordinate into uselessness. If a value must
be withheld, leave a resolvable pointer to where it lives.
</boundaries>

<references>
| Need | Read |
|---|---|
| record shape, retrieval mechanics, anti-patterns that get work rejected | `references/authoring.md` |
| the four questions and the symptom families, with what each must answer | `references/families.md` |
| refreshing without clobbering curated work | `references/refresh.md` |
</references>
