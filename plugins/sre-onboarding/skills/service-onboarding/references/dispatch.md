# Dispatch — deterministic two-speed coordination

Canonical home for orchestration-only coordination, scout/builder/auditor partitioning, evidence-capable workers, and onboarding-vs-curator non-conflict pointer. Artifact schemas and statuses: `references/artifact-manifest.md`. Partition inputs derived from enumerated repos, deployable units, evidence classes, and artifact groups in the manifest.

The orchestrating workflow is intake-and-dispatch only: it dispatches discovery and synthesis, keeps inline work to intake/capability-map/inventory/merge/handoff, and never launders unsupported claims.

## Capability tiers

| Tier | Role | Used for |
|---|---|---|
| fast discovery model | scout | cheap candidate discovery, inventory, surface enumeration, telemetry-name spotting, cross-reference discovery |
| reasoning synthesis model | builder | classification, rule passage, edge promotion, CORE synthesis, cited KB writing |
| adversarial reviewer | auditor | independent completeness audit; hunts missing/falsely closed threads and samples packets |

Scout outputs are candidates only. A builder classifies and applies rules before KB inclusion.

## Dispatch preflight

Before dispatching, record a dispatch-decision into the `@run-trail` block of `00-index/evidence-ledger.toon`:

`dispatch-available | dispatch-required | used | packet-evidence | degraded-reason`

- `dispatch-available`: `yes` if an awaitable dispatch mechanism is confirmed reachable; `no` only after a bounded probe fails.
- `dispatch-required`: `yes` for any run with ≥1 scout or builder partition (all first-time and incremental runs).
- `used`: `yes` when dispatched workers returned merged packets; `no` otherwise.
- `packet-evidence`: packet IDs or hashes of received worker outputs; `none` when `used=no`.
- `degraded-reason`: reason dispatch was not used when `used=no`; `n/a` when `used=yes`.

**ABORT rule:** if `dispatch-available=yes AND dispatch-required=yes AND used=no`, ABORT before any KB rendering; set `dispatch-mode=degraded` in `@run-trail`. Inline synthesis cannot satisfy the done gate when dispatch was available and required. Restart with dispatch enabled.

**Degraded path:** if `dispatch-available=no` after a confirmed probe, set `dispatch-mode=degraded`, document the gap, and proceed minimally — never pretend dispatch happened.

A required stage is complete only after output is received, cited, and merged. A handle or running status is not completion. Missing required packet blocks progression; write an explicit degraded/abort note instead of silently continuing.

## Partitions

Partition keys are derived from enumerated sources, not guessed. Before dispatch, read the input lock (`_run-lock.yaml`), list all repos in `service.yaml`, enumerate material units from Phase 1 candidate records, and assign each scout exactly one partition key. No two scouts receive overlapping partition keys.

### Scout partitions

Dispatch one scout per material unit or operational plane, derived from the enumerated repos and material units in the input lock:
- deployable unit (one per enumerated material unit)
- runtime plane
- data plane
- control plane
- dependency plane
- telemetry/ops plane
- `service/` ownership/access plane when distinct
- `topology/` edge/blast-radius plane
- `observability/` source and join-key plane
- `failure-knowledge/` hotspot/signature plane
- Guidance-asset discovery (capability-gated): the guidance-asset corpus under `repos/` — AI-guidance assets AND incident-material human-written guidance (troubleshooting guides / runbooks / known-issues / alert-response docs) — via a read-only local discovery capability or a bounded filename/dir glob fallback
- incremental re-mine partition for changed surfaces in `incremental` mode (derived from changed-surface closure table in `references/reproducibility-contract.md`)

Partition so scouts do not overlap reads. A plane with no app entry point still gets a scout if operationally material.

### Builder partitions

Build in CORE-first order:
1. repo-lower `kb/<repo>/` floor
2. per-repo deep builder for incident-material contracts/invariants
3. `service/`
4. `topology/`
5. `observability/`
6. `failure-knowledge/`
7. incident `telemetry-routing-card`
8. `00-index/` and overlays/contributions shape
9. Clean Deliverable Packet

No write overlap: two builders never write the same file.

### Auditor partition

The auditor runs after the ledger, KB, source-catalog, telemetry-routing-card, promote-up records, mutation/migration records when applicable, and Clean Deliverable Packet exist.

## Evidence-capable dispatch

Any worker that must query, fetch, authenticate against, inspect, or validate a source gets an evidence-capable worker, not reasoning-only. Unknown evidence need counts as evidence-using unless split.

## Observable run trail

Each stage emits a minimal packet to the transient run-root outside the deliverable:
- inventory
- capability-map
- per-unit/per-plane scout
- per-repo deep builder
- per-area builder
- mutation packet for `incremental`
- old-layout migration map when applicable
- audit
- Clean Deliverable Packet
- final KB summary

Packets record searched scope, evidence classes used, gaps, claims promoted, and mutation statuses when applicable. The committed KB promotes only durable provenance summaries, never links to run-root packets. A missing required packet blocks the next stage.

## Mission-brief shapes

**Scout:** Inventory material unit/plane U: owning repo(s), SHA/branch, deployable-binary vs library, entry points, exposed surfaces, telemetry names, source-catalog candidates, state/config/secret surfaces, concept candidates, open threads. For each telemetry source, also discover the correlation/trace identifier it structurally carries and the principal/identity dimensions that distinguish the service-under-onboarding from callers. For each dependency, discover server-side telemetry coordinate candidates and caller-side emission candidates. Return **normalized candidate records** (one per discovered fact, with `claim-class`, `normalized-subject`, `evidence`, `trust-label = suspected ⚠️`); do not classify or assign stable IDs. Classify nothing.

**Deep builder:** Mine API/schema/event/queue declarations and validation/guard/enforcement code; evaluate P3 (incident-material predicate); produce the per-repo incident-material roll-up; and write `deep/contracts.toon` / `deep/invariants.toon` only for incident-material repos, or record `open:escalated` gap rows in `00-index/evidence-ledger.toon` when evidence is unreachable. Assign stable IDs (`repo.<slug>`, `contract-id`, `invariant-id`) per `references/reproducibility-contract.md` before writing rows. Apply the `verification-and-evidence.md` rulebook before trust labeling: classify candidates, attach trust-label/grounding_type, honor the closed-world ban; a control-class invariant/contract is `verified/observed` only when traced to its enforcement checkpoint, never from declaration presence.

**Builder:** From repo-lower evidence and scout candidate records: assign stable IDs, evaluate applicable predicates (P1–P6, P8, and P-dep-telemetry where assigned), update ledger statuses, and **render assigned artifact tables from the ledger using schemas from `references/artifact-manifest.md`**. Fill table rows from promoted ledger records. Record promote-up entries or cited drops/gaps. Write only assigned files. Honor closed-world ban; no direct page drafting from exploration.

**Observability builder:** From observability-plane and dependency-plane candidate records: evaluate P5 and P-dep-telemetry, render `observability/source-catalog.md`, `observability/dependency-sources.md`, `observability/join-keys.md`, and routing views from the ledger. For each dependency, reconcile caller-side emission candidates, server-side telemetry coordinate candidates, access/eligibility notes, and cross-boundary join keys without restating the topology edge. Concrete identifiers land only in KB output when evidence-cited, safe, and necessary.

**Incremental builder:** Follow `kb-mutation.md`; derive re-mine scope from the changed-surface closure table in `references/reproducibility-contract.md`; produce old-layout migration map when needed; preserve curated/promoted/higher-grade facts; re-render affected artifacts from updated ledger rows (not patch-in-place); status touched facts; emit mutation records.

**Guidance-asset discovery+triage:** Enumerate the guidance-asset corpus under `repos/` (corpus definition: `references/verification-and-evidence.md §Guidance-asset (auxiliary)`) via a read-only local discovery capability (degrade to bounded filename/dir glob if absent). Discover **two classes**: (a) AI-guidance assets (agent/instruction/skill/subagent/chatmode/prompt/shared-ref filenames), AND (b) **incident-material human-written guidance** (troubleshooting guides / runbooks / known-issues / alert-response docs). For class (b) apply the **two-stage structural materiality filter** so the sweep does not dump every doc: **Stage A — directory/type signal:** troubleshooting-guide / runbook / known-issues / alert-response directories are IN; design / onboarding / architecture / API-reference narrative is OUT. **Stage B — content-shape:** include only if the doc carries an actionable discriminator — a decision table, a threshold, a symptom→cause→action chain, an escalation/on-call path, or an eligibility/config rule. **Forcing function:** include a human-guidance doc only if you can write a non-generic symptom→this-doc routing line; if the best you can do is "explains X", exclude it. Pointer-only — never copy doc content or values. Then for all discovered assets: classify `kind` (per canonical illustrative enum in `references/artifact-manifest.md`) and `consumer-relevance` (tags `incident | review | dev`, multi-tag); map to material units / CORE areas where evidenced; grade `docs-only` or `suspected ⚠️`; write every discovered asset to the `kb/<repo>/ai-assets.md` floor (inventory + triaged-lead rows, non-promotable, sensitive values pointer-only). `none-found (searched scope)` is honest only after the widened human-guidance scope (Stage A/B directories) was searched too.

**Catalog-inclusion rule** (decided here, not in the catalog): an asset is included in `00-index/ai-asset-catalog` only when it maps to ≥1 named materiality test from the routing set `incident-routing | ownership/escalation | observability | failure-discrimination | review-guidance`. Dev-relevant assets are included ONLY when they map to one of the four incident-flavored tests (`incident-routing | ownership/escalation | observability | failure-discrimination`); `review-guidance` alone does not include a dev-relevant asset. Everything else stays floor-only. Each catalog row records the named materiality test as its `why-included` basis, carries the anti-authority marker, and inherits grounding via the `asset-ref` pointer to the floor row.

**Human-guidance catalog rows (recall fix, required):** a human-guidance asset's catalog row MUST carry a **symptom/trigger routing phrase** (the same non-generic phrase that passed the forcing function) AND MUST be **cross-linked from `00-index/telemetry-routing-card.md` (and/or `task-router.md`)** under the matching symptom family, so the runtime orientation/coverage-map scan can route symptom→doc instead of walking past it. A human-guidance entry without a symptom/trigger phrase and the matching 00-index cross-link is not done. (AI-agent assets keep the consumer/materiality-test rule above unchanged.) Optional (advisory, thin): when a cataloged human-guidance doc cites a code symbol the code-mining also captured, you MAY add a one-line `drift-candidate` note — advisory only, never copying values and never adjudicating which source is correct.

**Auditor:** Runs the **mechanical reproducibility audit** (decidable checklist A–L: `references/verification-and-evidence.md §Mechanical reproducibility audit`) AND the **independent completeness audit** (adversarial sampling of inventory, ledger, CORE areas, deep contracts/invariants, source-catalog warning content, restricted-sources, telemetry-routing-card, promote-up records, mutation records when applicable, old-layout migration map, and Clean Deliverable Packet). Records findings and closure decision. Writes the `@audit` block in `00-index/evidence-ledger.toon` (auditor identity/capability, non-builder-attestation, sampled-artifacts, findings, closure). Does not approve work it performed.

## Onboarding vs curator

Onboarding may regenerate baselines, including in `incremental` mode, but must preserve curated/promoted/higher-grade facts per `kb-mutation.md`. Curator write-back is a separate curation workflow; onboarding only initializes the `contributions/` shape and obeys the shared mutation contract.
