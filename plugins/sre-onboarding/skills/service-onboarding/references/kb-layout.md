# KB Layout

Canonical home for layout principles: CORE-first ordering, the two-layer rule, cross-layer grounding, lens-ready seam, overlays/contributions, and the freshness update procedure. **Canonical artifact tree and per-file table schemas: `references/artifact-manifest.md`. Decidable predicates (concept inclusion, incident-material repo, AI-asset inclusion): `references/reproducibility-contract.md`.**

Two-layer rule: **repo-lower** `kb/<repo>/` is built first as the evidence floor; **service-higher** is derived from it and is the product. Every responder-useful service-higher implementation claim traces to a repo-lower cell unless `verification-and-evidence.md` explicitly allows a non-repo grounding type.

## CORE areas (incident-primary, lens-ready)

Preserve the four survivor questions while using these canonical areas:

| CORE area | Survivor question | Required content |
|---|---|---|
| `service/` | identity, ownership, access/escalation, support boundaries, concept-model/glossary | service identity, owner/on-call, escalation/access, support boundaries, concept model, glossary |
| `topology/` | where it runs + who calls whom | service-graph, endpoints/ports catalog, dependencies, blast-radius, deployable-unit coverage matrix, per-deployable-units |
| `observability/` | how to observe | source-catalog, join-keys, canonical-signals, restricted-sources |
| `failure-knowledge/` | what breaks and why | signatures and mechanisms, discriminator-first |

Completeness is outcome-based: collectively, without leaving the KB, a responder can route symptom → signal/source → owner → likely mechanism → next discriminator. Gaps use canonical ledger statuses (`open:escalated`, `rejected`, `non-material`, or `sensitive-unsafe`) with searched scope and verify-later action; never invent fields.

## Per-repo floor — `kb/<repo>/`

The repo-lower evidence floor is the authoritative base for all service-higher implementation claims. Repo-lower facts cite durable source paths and record `grounding_type`. Raw source citations do not substitute for a repo-lower cell when promoting service-higher implementation claims.

`kb/<repo>/ai-assets.md` is auxiliary, **non-promotable**: an asset row never satisfies cross-layer grounding for any implementation claim; any claim mined from an asset must re-resolve to repo-source or live evidence and pass the normal rulebook before promotion. Trust caps at `docs-only` or `suspected ⚠️`; sensitive operational metadata is recorded as pointer + sanitized purpose only.

`kb/<repo>/deep/` is the repo-local evidence floor for incident-material contracts and invariants. A repo is incident-material iff ≥1 mined fact meets the P3 trigger (full predicate: `references/reproducibility-contract.md`). Statically mined contracts/invariants are capped at `source-inferred/declared` unless a live signal justifies `verified/observed`. Service-wide consequences promote into existing CORE homes only: API/runtime edges → `topology/`; failure implications/discriminators → `failure-knowledge/`; ownership/support → `service/`; telemetry proof paths → `observability/`. Do not create `core/contracts/` or `core/invariants/`.

File schemas for all `kb/<repo>/` slots: `references/artifact-manifest.md`.

## Required topology artifacts — key audit-gate rules

`topology/deployable-unit-coverage.md`: Audit-read matrix. A single overview cannot stand in for uncovered units/planes.

`topology/per-deployable-units.md`: One row per material unit/plane. Schema: `references/artifact-manifest.md`.

`topology/endpoints-ports-catalog.md`: One row per exposed surface. Schema: `references/artifact-manifest.md`.

**Promote-up record:** For every promoted floor fact: `source_cell (kb/<repo>/...:line) → destination_cell (CORE file:line) → payload`, or a cited drop/gap using canonical ledger statuses: `duplicate · stale · non-material · sensitive-unsafe · superseded · open:escalated · rejected`. Missing or mismatched destination payload is not promoted.

## Concept-model principles — `service/concept-model.md`

Table-first; 10–20 concepts max. Schema and category enum: `references/artifact-manifest.md`. Inclusion predicate (P2): `references/reproducibility-contract.md`.

Rules:
- **NAME is not a concept.** A component, repo, deployable, or plane name alone is not enough. A topology, tenancy, capacity, lifecycle, routing, or correlation dimension is a concept when it explains behavior, ownership, blast-radius, routing, failover, capacity, or diagnosis.
- `deployment/topology` may not dominate.
- `triage use` is mandatory.
- `service/glossary.md` is flat term expansion; concept-model is the load-bearing triage index. Cross-link, do not duplicate.

## Observability — key audit-gate rules

Full source-catalog row schema and consumer-warning rules: `references/artifact-manifest.md`. Source-catalog is mandatory even when no live verification is reachable.

`observability/restricted-sources.md`: consumers must not generalize a gate on one source to all sources.

`observability/canonical-signals.md`: routing index over source-catalog rows; not a second home for telemetry facts.

## Failure-knowledge — key principles

Discriminator-first entries; `discriminator` column mandatory and must be actionable. Full schema and promotion predicate (P6): `references/artifact-manifest.md` and `references/reproducibility-contract.md`.

Splitting a dense `failure-knowledge/README.md` into per-family pages is justified only when a retrieval test shows `symptom → signature → owner` is faster; do not split for folder aesthetics.

## Index and lens-ready seam

`00-index/` is the task-neutral seam:
- `task-router` routes first-time and future consumers into the right CORE anchors.
- `core-map` resolves stable IDs/anchors to canonical homes.
- `evidence-ledger.toon` — committed replay surface (provenance lock summary + all terminal records). Required for incremental determinism.
- restored candidate `.toon` catalogs: `ownership.toon` (from overlays or service/ ownership), `incident-clusters.toon` (from overlays/failure-knowledge — does NOT fall back to ownership). `.toon` is a restored candidate machine-readable index format, not proven.
- `ai-asset-catalog` — multi-consumer pointer-only inventory/routing seam over per-repo `kb/<repo>/ai-assets.md` rows.

**Normativity:** `ai-asset-catalog` is intentionally "dumb" — pointer-only, non-procedural, non-normative, non-synthesizing. MUST NOT encode ordering/precedence, "use this first", `evidence_required`, `stop_conditions`, or acceptance criteria. Entries MAY be grouped by primary-consumer tag for scannability; grouping is presentation-only.

**Anti-authority header** (verbatim in every ai-asset-catalog): `docs-only lead, not authority; re-resolve before use.`

**Catalog inclusion rule** (which discovered assets a triage worker writes into the catalog): see `references/dispatch.md`. The catalog itself never decides inclusion.

**Anchor-ID stability invariant.** Stable IDs for CORE anchors and `core-map` entries survive moves/renames and are never recycled for a different fact. If a fact moves, update the resolver; if a fact splits, mint scoped new IDs and retain a supersession pointer. Full ID algorithm: `references/reproducibility-contract.md`.

## Incident telemetry-routing-card (produced now)

The incident lens is primary, so `00-index/telemetry-routing-card` is produced in this onboarding run as a route view over CORE observability. Row schema: `references/artifact-manifest.md`.

Method-gates (required pointer lines per row):
- probe-before-proximate — run the cheap canonical source route before declaring a proximate/manual-kit outcome.
- empty-is-not-absent — a zero-row result is non-dispositive unless the canonical source, join key, scope, and freshness were live-checked.
- detection-vs-onset — separate alert/detection time from actual service onset before causal ordering.
- provenance-strength — scale verdict strength to evidence trust, grounding_type, freshness, and blocked sources.

**Incident-primary preservation rule:** the incident `telemetry-routing-card` is the only mandatory produced lens. Review and dev remain seam-only; this skill creates no review/dev lens files, taxonomies, or acceptance criteria. The passive multi-consumer `00-index/ai-asset-catalog` is an inventory/routing seam, not a lens.

## Overlays and contributions

`overlays/incidents/` contains 90-day incident knowledge as priors, not truth: aggregates + stable IDs + owner DLs only; no PII, raw titles, or copied restricted payloads. Overlay rows past their 90-day window roll off or are re-verified from a refreshing source change; stale priors must not silently persist. Full file schemas: `references/artifact-manifest.md`.

`contributions/` is initialized as intake/accepted shape only. A separate curation workflow writes contributions later under the single mutation contract in `kb-mutation.md`; onboarding is read-only with respect to write-back behavior.

## Freshness / provenance header

Required fields and placement: `references/artifact-manifest.md`. Summary: each KB root includes source repo names/branches/SHAs, scan date, last live verification date or NEVER, audit verdict, open/escalated thread summary with exact verify-later actions, verification-queue pointer, stale-risk markers, and overlay window/roll-off/sanitization statement when overlays exist.

## UPDATE procedure

For first-time onboarding, initialize the layout and verification queue. For later changes, follow `kb-mutation.md`: select `incremental`, lock inputs (including prior KB state hash and old repo SHAs), map old layout to current areas if needed, re-mine only changed surfaces per the changed-surface closure table, compare evidence strength, record `kb-mutation` status, update freshness, and emit the Clean Deliverable Packet.
