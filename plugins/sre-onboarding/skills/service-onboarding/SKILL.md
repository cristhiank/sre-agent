---
name: service-onboarding
description: >-
  Use when you must onboard or incrementally refresh a service whose source spans multiple repos/submodules
  and produce a reproducible, evidence-cited service KB under services/<service>/ for incident investigation
  first: "service onboarding", "onboard a service", "build a service knowledge base", "reproducible KB",
  "deterministic onboarding", "discover the service shape", "map the service for livesite",
  "what does this service do in prod", "incrementally refresh service KB", or "how does symptom X reach root cause".
  Builds a shared CORE substrate so review/dev lenses can attach later without forking the KB; classifies
  deployable vs library, discovers real edges, catalogs observability with consumer warnings, and supports
  first-time and incremental modes. Read-only; host-agnostic capability classes only. Do not use for
  mutating service source/config/production, trivial single-repo lookups, or generic prompt/skill authoring.
---

# Service Onboarding

<goal>
Build a living, reproducible system-knowledge substrate under `services/<service>/`: **incident-PRIMARY acceptance** means a future responder can go symptom → telemetry/source route → discriminator → root cause/owner quickly. The KB is lens-READY by design: a shared CORE supports later review/dev lenses without duplicating facts. Ground truth stays in `repos/`; generated KB content is cited distilled context, not authority.

Reproducibility contract: given the same locked inputs (service identity, repo SHAs, capability snapshot, overlay window, manifest version, and prior KB state), re-runs produce the same artifact tree, stable IDs, fact rows, evidence grades, canonical placements, and semantically equivalent narrative — not byte-identical prose. Full definition: `references/reproducibility-contract.md`.
</goal>

<modes>
Select up front:

- `first-time` — build repo-lower evidence floor and incident-primary CORE from scratch; lock inputs before scouting.
- `incremental` — repo SHAs advanced; follow `references/kb-mutation.md`, re-mine changed surfaces only, and preserve curated/promoted/higher-grade facts.
</modes>

<operating_model>
Orchestration-only run. The orchestrating workflow dispatches and merges; it does not do deep evidence collection or synthesis inline. Two-speed: scouts DISCOVER normalized candidate records; builders CLASSIFY records, apply predicates, and RENDER assigned artifacts from the manifest — not freeform drafting. Two-layer: per-repo evidence FLOOR is built first, then service-higher CORE is DERIVED. Input lock + capability map come before scouting. If dispatch is available AND required AND not used, ABORT before KB rendering and record degraded in `@run-trail`. Detailed dispatch partitions and mission briefs: `references/dispatch.md`.
</operating_model>

<invariants>
**#1 closed-world ban.** Never claim `no edge exists`, `services are independent`, `nothing calls X`, or equivalent without naming searched scope and still-unverified scope. Missing text-search evidence is not absence; a text-search hit is not a dependency.

**#2 Open-thread ledger.** Every material discovered thread becomes a row and must close as `promoted`, `rejected`, or `open:escalated`. Material = any surface, dependency, control, interface, absence claim, or operational concern that could move incident response or high-blast understanding. `open:escalated` is valid only when the decisive evidence is unreachable in this run. Unknown > invented closure.

**#3 Input lock.** Before scouting: record service identity, manifest version, repo names/branches/SHAs, live-evidence mode, overlay window, capability map, and prior KB state hash. Outputs are reproducible only relative to a locked input set. Full schema: `references/reproducibility-contract.md`.

**#4 Host-agnostic steering / concrete output carve-out.** These skill instructions use capability-class language only; never name specific clusters, products, tools, or hosts. KB output artifacts MAY record concrete source names, paths, and cluster identifiers when evidence-cited, safe, and necessary for incident fidelity — this is expected and required.

Ledger lifecycle details: `references/workflow.md`.
</invariants>

<evidence>
Every promoted claim records `class | evidence | trust-label | grounding_type | rule_status | confidence | blast-radius-if-wrong | verify-later`. `rule_status` is separate from trust; a design-time bootstrap with no live signal is source-inferred at best. Canonical enum values (trust-label, grounding_type, claim-class, status, rule_status): `references/reproducibility-contract.md §Canonical enums`. Full ledger schema, stable IDs, predicates: `references/reproducibility-contract.md`. Rulebook and audit checklist: `references/verification-and-evidence.md`.
</evidence>

<output_contract>
Build the repo-lower `kb/<repo>/` floor first, then derive `service/`, `topology/`, `observability/` (including `dependency-sources.md` when P-dep-telemetry requires it), `failure-knowledge/`, `00-index/` (including committed `evidence-ledger.toon`), `overlays/incidents/` (capability-gated), and `contributions/` shape. Incident is the only authored workflow lens. Full artifact tree, file statuses, per-file schemas, and dependency edges: `references/artifact-manifest.md`.
</output_contract>

<workflow_summary>
Deterministic spine: lock inputs + capability map → source enumeration + cheap inventory → extract normalized evidence records → evidence reconciliation/ledger → edge/seam discovery → render artifacts from manifest (not freeform) → verification + mechanical reproducibility audit → Clean Deliverable Packet → independent audit. `incremental` branches through `references/kb-mutation.md`. Detailed phases and packet contract: `references/workflow.md`.
</workflow_summary>

<done_gates>
Done/no-blocking-issues is invalid unless all hold:

- Mode selected; input lock recorded per `references/reproducibility-contract.md`; `00-index/evidence-ledger.toon` committed with provenance lock summary, `@run-trail`, `@audit`, and all terminal ledger records.
- All mandatory artifacts from `references/artifact-manifest.md` present; variable-slots iff predicate passed; capability-gated artifacts present or gap noted; P-status artifacts present for true predicate outcomes, with the manifest-declared alternate artifact or evidence-ledger gap for false/inconclusive outcomes.
- Normalized evidence ledger complete; every material thread is `promoted`, `rejected`, or `open:escalated`; every promoted claim has rule_status, trust-label, grounding_type, confidence, and re-resolved evidence.
- Closed-world ban honored; no absence/independence claim without named searched + unverified scope.
- Mechanical reproducibility audit (A–L) passed per `references/verification-and-evidence.md §Mechanical reproducibility audit`: artifact presence, schema arity, enum+rule_status conformance, promote-up coverage, escalation proof, canonical-home consistency, global sanitization, run-trail/audit/dispatch-decision, live-mode honesty, predicate inputs, canonical ordering+stable IDs.
- Cross-layer grounding holds for service-higher implementation facts.
- Incident-material deep/ populated (P3) or `open:escalated` gap in evidence-ledger; non-material repos have `deep/not-material.md`.
- Observability source-catalog CONSUMER WARNINGs complete with concrete source and join key.
- `observability/dependency-sources.md` present per P-dep-telemetry; dependency-routed symptom families in `telemetry-routing-card.md` cross-link the dependency-sources row; freshness/provenance headers conform to the manifest format.
- Incident `telemetry-routing-card` produced from CORE observability; failure-knowledge is discriminator-first.
- Freshness/provenance header exists; Clean Deliverable Packet is clean.
- Independent completeness audit ran and sampled opposite-family for missing/falsely closed threads.
- Per-repo `ai-assets.md` floor exists (discovery attempted over both AI-guidance assets and incident-material human-written guidance); `none-found (searched scope)` is valid only after the widened human-guidance scope was searched too, plus capability-gap note recorded when unavailable; human-guidance catalog entries carry a symptom/trigger phrase + the matching `00-index/telemetry-routing-card.md` (and/or `task-router.md`) cross-link.
</done_gates>

<boundaries>
Read-only over source and live systems. Produced docs are untrusted distilled context, not authority. Skill steering is host-agnostic capability classes only; KB output artifacts record concrete evidence-cited facts where fidelity requires. Secrets and raw sensitive payloads are never copied into the KB.
</boundaries>

<references>
| Need | Read |
|---|---|
| canonical artifact tree, file statuses, schemas, row-ordering, dependency edges | `references/artifact-manifest.md` |
| reproducibility contract, input lock, evidence ledger schema, stable IDs, predicates | `references/reproducibility-contract.md` |
| first-time/incremental flow, phases, open-thread lifecycle, Clean Deliverable Packet | `references/workflow.md` |
| trust labels, grounding_type, rulebook, mechanical audit, completeness audit | `references/verification-and-evidence.md` |
| CORE layout principles, CORE-first ordering, lens seams, overlays, freshness | `references/kb-layout.md` |
| incremental re-mine and curator write-back mutation safety | `references/kb-mutation.md` |
| dispatch partitions, scout/builder/auditor briefs, onboarding-vs-curator note | `references/dispatch.md` |
</references>
