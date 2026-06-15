# KB Layout

Size budget: ~200 lines. Canonical home for the committed KB tree, core artifacts, stable anchors, lens seams, freshness headers, and update pointers.

Two-layer rule: **repo-lower** `kb/<repo>/` is built first as the evidence floor; **service-higher** is derived from it and is the product. Every responder-useful service-higher implementation claim traces to a repo-lower cell unless `verification-and-evidence.md` explicitly allows a non-repo grounding type.

## CORE areas (incident-primary, lens-ready)

Preserve the four survivor questions while using these canonical areas:

| CORE area | Survivor question | Required content |
|---|---|---|
| `service/` | identity, ownership, access/escalation, support boundaries, concept-model/glossary | service identity, owner/on-call, escalation/access, support boundaries, concept model, glossary |
| `topology/` | where it runs + who calls whom | service-graph, endpoints/ports catalog, dependencies, blast-radius, deployable-unit coverage matrix |
| `observability/` | how to observe | source-catalog, join-keys, canonical-signals, restricted-sources, dashboards/recipes |
| `failure-knowledge/` | what breaks and why | signatures and mechanisms, discriminator-first |

Completeness is outcome-based: collectively, without leaving the KB, a responder can route symptom -> signal/source -> owner -> likely mechanism -> next discriminator. Gaps are explicit `unknown` with searched scope; never invent fields.

## Per-repo floor — `kb/<repo>/`

| File/slot | Contains |
|---|---|
| `entry-points.md` | main/host/startup, exposed surfaces, deployable-binary vs library classification |
| `modules.md` | load-bearing modules/subsystems, responsibilities, composition roots when source-visible |
| `concepts.md` | repo-local concept candidates and source classes; feeds `service/concept-model.md` |
| `deep/contracts.toon` | `contract-id | kind(api|schema|event|queue) | surface(path/route/topic) | shape-ref(file:line) | version/compat-rule | consumers | materiality-category | evidence | trust-label | grounding_type` |
| `deep/invariants.toon` | `invariant-id | statement | enforcement-site(file:line) | discriminator(how to prove it failed) | scope | materiality-category | evidence | trust-label | grounding_type` |
| test-oracles | deferred — no file or folder produced; do not initialize a stub |

Repo-lower facts cite durable source paths and record `grounding_type`. Raw source citations do not substitute for a repo-lower cell when promoting service-higher implementation claims.

`kb/<repo>/deep/` is the repo-local evidence FLOOR for incident-material contracts and invariants. Canonical incident-material predicate: a fact is incident-material iff it could affect one of these categories: incident routing | blast radius | failure discrimination | ownership/escalation | high-risk operational change. A repo is incident-material iff >=1 mined fact meets that trigger; a repo with zero qualifying facts is recorded `not-material` with searched scope. Each populated `deep/` row cites its `materiality-category`; partial coverage is `suspected ⚠️` plus a verification-queue entry, not a separate grade. Statically mined contracts/invariants are capped at `source-inferred/declared` unless a live signal justifies `verified/observed`. Apply `verification-and-evidence.md` before trust labeling: classify candidates, attach trust-label/grounding_type, honor the closed-world ban; a control-class invariant/contract is `verified/observed` only when traced to its enforcement checkpoint, never from declaration presence. Service-wide consequences promote into existing CORE homes only: API/runtime edges -> `topology/`; failure implications/discriminators -> `failure-knowledge/`; ownership/support -> `service/`; telemetry proof paths -> `observability/`. Do not create `core/contracts/` or `core/invariants/`.

## Required destination artifacts

### `topology/deployable-unit-coverage.md`

Audit-read matrix: each material unit/plane x the four survivor questions -> covering artifact or explicit gap + searched scope. A single overview cannot stand in for uncovered units/planes.

### `topology/per-deployable-units.md`

One row per material unit/plane: `unit | hosting model | deployment/runtime scope | scale/routing dimension | owner/team | on-call/escalation | evidence(kb/<repo>/...:line) | trust | grounding_type`.

### `topology/endpoints-ports-catalog.md`

One row per exposed surface: `unit | endpoint/route/surface | port | protocol/source kind | auth/control attribute | caller/dependency notes | evidence(kb/<repo>/...:line) | trust | grounding_type`.

### Promote-up record

For every promoted floor fact: `source_cell (kb/<repo>/...:line) -> destination_cell (CORE file:line) -> payload`, or a cited drop/gap: duplicate · stale · non-material · sensitive-unsafe · superseded · unknown with searched scope. Missing or mismatched destination payload is not promoted.

## Concept-model spec — `service/concept-model.md`

Table-first, ~10-20 concepts max. Columns: `concept | category | role | triage use | serves CORE area | aliases | evidence | trust | grounding_type`.

Categories (exact schema): domain-entity · unit-of-work · process-stage/lifecycle-state · identity/tenancy/partition-key · correlation/trace-key · deployment/topology · resource/capacity · policy/config · acronym.

Rules:
- **NAME is not a concept.** A component, repo, deployable, or plane name alone is not enough. A topology, tenancy, capacity, lifecycle, routing, or correlation dimension is a concept when it explains behavior, ownership, blast-radius, routing, failover, capacity, or diagnosis.
- Required categories identity/tenancy/partition-key, correlation/trace-key, and process-stage/lifecycle-state are populated or carry an audited `none found (material units + source classes searched)` gap.
- `deployment/topology` may not dominate.
- Inclusion requires >=2 of 4: appears in telemetry/log/query dimensions; drives routing/partitioning/ownership/blast-radius; encodes lifecycle/state/process order; is needed to map symptom to root cause/mitigation/owner.
- `triage use` is mandatory.
- `service/glossary.md` is flat term expansion; concept-model is the load-bearing triage index. Cross-link, do not duplicate.

## Observability artifacts

### `observability/source-catalog.md`

Mandatory even when no live verification is reachable.

Row schema:
`signal | source kind | table/metric/log name | join keys | dimensions | recipe/query pointer | discovery source | last live verification (date or NEVER) | trust | grounding_type | restricted? | exact verify-later action | CONSUMER WARNING`

Rules:
- Any source-inferred row requires `last live verification = NEVER` and a CONSUMER WARNING containing this clause with concrete values filled: `do not conclude absence or a block without live-checking <source> via <join key>`.
- The warning must name the source and join key; literal `<source>` or `<join key>` placeholders fail the gate.
- A design-time bootstrap with no live signal is `source-inferred` at best, even if the source name came from code.
- `recipe/query pointer` points to a recipe file/card, not copied query sprawl.
- `exact verify-later action` names the source class, join key, and expected proof to collect.

### `observability/restricted-sources.md`

Records sources that reject, are access-gated, or are scope-gated: `source | gate/rejection observed | scope tested | alternate reachable source? | consumer note`. Consumers must not generalize a gate on one source to all sources.

### `observability/canonical-signals.md`

Names the preferred signal routes for incident triage: `symptom family | canonical signal | source-catalog ref | join key | freshness | trust | grounding_type | non-dispositive alternatives | verify-later`. It is a routing index over source-catalog rows, not a second home for telemetry facts.

## Failure-knowledge spec — `failure-knowledge/`

Discriminator-first entries: `symptom | signature | discriminator | mechanism | likely owner | mitigation | evidence | trust | grounding_type | verify-later`.

Seed from hotspots, rulebook risks, audit findings, source catalog gaps, and sanitized overlays. Splitting a dense failure-knowledge README into per-family pages is justified only when a retrieval test shows `symptom -> signature -> owner` is faster; do not split for folder aesthetics.

## Index and lens-ready seam

`00-index/` is the task-neutral seam:
- `task-router` routes first-time and future consumers into the right CORE anchors.
- `core-map` resolves stable IDs/anchors to canonical homes.
- restored candidate `.toon` catalogs: `ownership.toon`, `incident-clusters.toon`. `.toon` is a restored candidate machine-readable index format, not proven.

**Anchor-ID stability invariant.** Stable IDs for CORE anchors and `core-map` entries survive moves/renames and are never recycled for a different fact. If a fact moves, update the resolver; if a fact splits, mint scoped new IDs and retain a supersession pointer.

## Incident telemetry-routing-card (produced now)

The incident lens is primary, so `00-index/telemetry-routing-card` is produced in this onboarding run as a route view over CORE observability. Row schema: `symptom family | first source-catalog ref | join key | canonical signal ref | discriminator | restricted-source note | empty-result warning | freshness/trust | method-gates | next owner/escalation ref`.

Method-gates are one-line pointers to the consuming incident-investigation operating rules:
- probe-before-proximate — run the cheap canonical source route before declaring a proximate/manual-kit outcome.
- empty-is-not-absent — a zero-row result is non-dispositive unless the canonical source, join key, scope, and freshness were live-checked.
- detection-vs-onset — separate alert/detection time from actual service onset before causal ordering.
- provenance-strength — scale verdict strength to evidence trust, grounding_type, freshness, and blocked sources.

Incident-primary preservation rule: the incident `telemetry-routing-card` is the only mandatory produced lens. Review and dev remain seam-only until a consuming workflow proves usage; this skill creates no review/dev lens files, taxonomies, or acceptance criteria.

Future review/dev router contract shape, when needed: `trigger | core_refs (ordered pointers into CORE) | evidence_required | freshness_required | stop_conditions`; pointer-only, no duplicated facts, stable anchor IDs only. Non-authoritative illustration only: review routes may start from changed surfaces and dev routes may start from extension points, but no ordering or handoff is emitted as KB doctrine by this skill.

## Overlays and contributions

`overlays/incidents/` contains 90-day incident knowledge as priors, not truth: aggregates + stable IDs + owner DLs only; no PII, raw titles, or copied restricted payloads. Overlay rows past their 90-day window roll off or are re-verified from a refreshing source change; stale priors must not silently persist.

`contributions/` is initialized as intake/accepted shape only. A separate curator agent writes contributions later under the single mutation contract in `kb-mutation.md`; onboarding is read-only with respect to write-back behavior.

## Freshness / provenance header

Each KB root (or `service/provenance.md`) includes:
- source repo names, branches, and SHAs scanned
- scan date
- `last live verification` date or `NEVER`
- independent-audit verdict summary
- open/escalated thread summary with exact verify-later action
- verification-queue pointer
- stale-risk markers
- overlay window, roll-off/reverification date, and sanitization statement when overlays exist

## UPDATE procedure

For first-time onboarding, initialize the layout and verification queue. For later changes, follow `kb-mutation.md`: select `incremental`, map old layout to current areas if needed, re-mine only changed surfaces, compare evidence strength, record `kb-mutation` status, update freshness, and emit the Clean Deliverable Packet.
