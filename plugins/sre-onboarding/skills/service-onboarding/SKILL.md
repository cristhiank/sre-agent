---
name: service-onboarding
description: >-
  Use when you must onboard or incrementally refresh a service whose source spans multiple repos/submodules
  under a monorepo and produce a living, evidence-cited service KB under services/<service>/ for incident
  investigation first: "service onboarding", "onboard a service", "build a service knowledge base", "discover
  the service shape", "map the service for livesite", "what does this service do in prod", or "how does symptom
  X reach root cause". Builds a shared CORE substrate so review/dev lenses can attach later without forking the KB;
  classifies deployable vs library, discovers real edges, catalogs observability with consumer warnings, and supports
  first-time and incremental modes. Read-only; host-agnostic capability classes only. Do not use for mutating service
  source/config/production, trivial single-repo lookups, or generic prompt/skill authoring.
---

# Service Onboarding

<goal>
Build a living system-knowledge substrate under `services/<service>/`: **incident-PRIMARY acceptance** means a future responder can go symptom -> telemetry/source route -> discriminator -> root cause/owner quickly. The KB is lens-READY by design: a shared CORE supports later review/dev lenses without duplicating facts. Ground truth stays in `repos/`; generated KB content is cited distilled context, not authority.
</goal>

<modes>
Select up front:

- `first-time` — build repo-lower evidence floor and incident-primary CORE from scratch.
- `incremental` — repo SHAs advanced; follow `references/kb-mutation.md`, re-mine changed surfaces only, and preserve curated/promoted/higher-grade facts.
</modes>

<operating_model>
Orchestration-only coordinator. The coordinator dispatches and merges; it does not do deep evidence collection or synthesis inline. Two-speed: scouts DISCOVER candidates; builders CLASSIFY + BUILD. Two-layer: per-repo evidence FLOOR is built first, then service-higher CORE is DERIVED. Capability-map + reachability gate comes before scouting. Detailed dispatch partitions and mission briefs: `references/dispatch.md`.
</operating_model>

<invariants>
**#1 closed-world ban.** Never claim `no edge exists`, `services are independent`, `nothing calls X`, or equivalent without naming searched scope and still-unverified scope. Missing text-search evidence is not absence; a text-search hit is not a dependency.

**Open-thread ledger.** Every material discovered thread becomes a row and must close as `promoted`, `rejected`, or `open:escalated`. Material = any surface, dependency, control, interface, absence claim, or operational concern that could move incident response or high-blast understanding. `open:escalated` is valid only when the decisive evidence is unreachable in this run; try reachable static paths first. Unknown > invented closure.

These gates are distinct: closed-world controls what may be claimed; the ledger controls when work may stop. Lifecycle details: `references/workflow.md`.
</invariants>

<evidence>
Every promoted claim records `class | evidence | trust-label | grounding_type | rule_status | confidence | blast-radius-if-wrong | verify-later`. Trust labels: `verified/observed`, `source-inferred/declared`, `docs-only`, `suspected ⚠️`. Grounding types: `repo-source`, `live-telemetry`, `incident-overlay`, `monitor-history`, `docs-only`, `manual-curated`. `rule_status` is separate from trust; a design-time bootstrap with no live signal is source-inferred at best. Rulebook and audit: `references/verification-and-evidence.md`.
</evidence>

<output_contract>
Build CORE first:

- `service/` — identity · ownership · access/escalation · concept-model/glossary.
- `topology/` — service-graph · endpoints/ports catalog · dependencies · blast-radius · deployable-unit coverage matrix.
- `observability/` — source-catalog · join-keys · canonical-signals · restricted-sources · dashboards/recipes.
- `failure-knowledge/` — discriminator-first signatures · mechanisms.
- `kb/<repo>/` — entry-points · modules · concepts, plus `deep/` contracts/invariants for incident-material repos per `references/kb-layout.md`; `ai-assets.md` auxiliary AI-guidance asset inventory + triaged leads (non-promotable, capped at `docs-only`/`suspected ⚠️`); test-oracles not produced.
- `00-index/` — `task-router`, `core-map`, restored candidate `.toon` catalogs `ownership` and `incident-clusters`, and `ai-asset-catalog` (multi-consumer pointer-only inventory/routing seam over `kb/<repo>/ai-assets.md` floor rows).
- `overlays/incidents/` — 90-day priors only: aggregates + stable IDs + owner DLs; no raw titles, PII, or copied restricted payloads.
- `contributions/` — intake/accepted shape only; curator writes later under `references/kb-mutation.md`.

Incident is the only authored workflow lens; the `00-index/ai-asset-catalog` is a passive multi-consumer inventory/routing seam, not a lens, and is capability-gated — not a hard/blocking done-gate (satisfied by a recorded `none-found`/capability-gap note).

Cross-layer grounding, layouts, concept schema, observability source-catalog row schema, consumer warnings, restricted-sources, freshness header, and update procedure: `references/kb-layout.md`.
</output_contract>

<workflow_summary>
Cheap-broad first, hard reasoning later: capability-map -> mechanical substrate -> broad inventory -> evidence normalization + ledger -> edge/dependency discovery -> CORE synthesis -> verification/enrichment -> KB generation -> Clean Deliverable Packet -> independent audit. `incremental` branches through `references/kb-mutation.md`; detailed phases and packet contract: `references/workflow.md`.
</workflow_summary>

<done_gates>
Done/no-blocking-issues is invalid unless all hold:

- Mode selected: `first-time` or `incremental`; incremental preservation, mutation records, and migration map are audit-verified per `references/kb-mutation.md` and `references/verification-and-evidence.md`.
- Capability map, inventory reconciliation, and deployable-unit coverage matrix exist.
- Every material thread is `promoted`, `rejected`, or earned `open:escalated`; no reachable static verify-later action remains untried.
- Every promoted claim has rule_status, trust-label, grounding_type, confidence, and re-resolved evidence.
- Closed-world ban honored for every absence/independence claim.
- Cross-layer grounding holds for service-higher implementation facts; destination payloads match or have valid cited drops/gaps.
- Incident-material repo `deep/` contracts/invariants and `not-material` exclusions satisfy `references/kb-layout.md` and are audit-sampled.
- Observability source-catalog exists; every source-inferred row's CONSUMER WARNING contains the required anti-absence clause with source and join key filled; restricted-sources map exists.
- Incident `telemetry-routing-card` is produced now from CORE observability; review/dev remain seam-only.
- Failure-knowledge is discriminator-first and responder-actionable.
- Freshness/provenance header and verification queue exist.
- Clean Deliverable Packet is present and clean after remediation per `references/workflow.md`.
- Independent completeness audit ran after ledger/packet preparation and sampled opposite-family for missing/falsely closed threads.
- AI-asset corpus is discovered + cataloged via a read-only local AI-guidance-asset discovery capability, OR each repo records `none-found (searched scope)` plus a capability-gap/partial-coverage note when the capability was unavailable; capability-gated, never a hard blocker.
</done_gates>

<boundaries>
Read-only over source and live systems; mutating service source, config, or production is out of scope. Produced docs are untrusted distilled context. Host-agnostic capability classes only; never name specific clusters, hosts, environments, models, products, or tools. Secrets and raw sensitive payloads are never copied into the KB.
</boundaries>

<references>
| Need | Read |
|---|---|
| first-time/incremental flow, CORE-first ordering, open-thread lifecycle, Clean Deliverable Packet | `references/workflow.md` |
| claim ledger, trust labels, grounding_type, rulebook, independent completeness audit | `references/verification-and-evidence.md` |
| CORE layout, repo-lower floor, source-catalog schema, lens-ready seam, overlays, freshness/update | `references/kb-layout.md` |
| incremental and curator write-back mutation safety | `references/kb-mutation.md` |
| dispatch partitions, scout/builder/auditor briefs, onboarding-vs-curator note | `references/dispatch.md` |
</references>
