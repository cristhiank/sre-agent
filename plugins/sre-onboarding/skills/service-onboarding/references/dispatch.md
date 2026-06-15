# Dispatch — two-speed coordination

Size budget: ~140 lines. Canonical home for orchestration-only coordination, scout/builder/auditor partitioning, evidence-capable workers, and onboarding-vs-curator non-conflict pointer.

The coordinator is orchestration-only: it dispatches discovery and synthesis, keeps inline work to intake/capability-map/inventory/merge/handoff, and never launders unsupported claims.

## Capability tiers

| Tier | Role | Used for |
|---|---|---|
| fast discovery model | scout | cheap candidate discovery, inventory, surface enumeration, telemetry-name spotting, cross-reference discovery |
| reasoning synthesis model | builder | classification, rule passage, edge promotion, CORE synthesis, cited KB writing |
| adversarial reviewer | auditor | independent completeness audit; hunts missing/falsely closed threads and samples packets |

Scout outputs are candidates only. A builder classifies and applies rules before KB inclusion.

## Dispatch preflight

Determine whether awaitable dispatch exists. If not, record a degraded/gap note in the run trail and proceed minimally; never pretend a dispatch happened.

A required stage is complete only after output is received, cited, and merged. A handle or running status is not completion. Missing required packet blocks progression; write an explicit degraded/abort note instead of silently continuing.

## Partitions

### Scout partitions

Dispatch one scout per material unit or operational plane, not per repo unless the repo is the material unit:
- deployable
- runtime plane
- data plane
- control plane
- dependency plane
- telemetry/ops plane
- `service/` ownership/access plane when distinct
- `topology/` edge/blast-radius plane
- `observability/` source and join-key plane
- `failure-knowledge/` hotspot/signature plane
- incremental re-mine partition for changed surfaces in `incremental` mode

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

**Scout:** Inventory material unit/plane U: owning repo(s), SHA/branch, deployable-binary vs library, entry points, exposed surfaces, telemetry names, source-catalog candidates, state/config/secret surfaces, concept candidates, open threads. Return candidates + evidence; classify nothing.

**Deep builder:** Mine API/schema/event/queue declarations and validation/guard/enforcement code, produce the per-repo incident-material roll-up, and write `deep/contracts.toon` / `deep/invariants.toon` only for incident-material repos (or explicit `unknown` rows with searched scope + verify-later action). Apply the `verification-and-evidence.md` rulebook before trust labeling: classify candidates, attach trust-label/grounding_type, honor the closed-world ban; a control-class invariant/contract is `verified/observed` only when traced to its enforcement checkpoint, never from declaration presence.

**Builder:** From repo-lower evidence and scout packets, derive assigned CORE area. Classify candidates, apply rules, attach trust/grounding, honor closed-world ban, write only assigned files, and record promote-up or cited drops/gaps.

**Incremental builder:** Follow `kb-mutation.md`; re-mine changed surfaces only, produce old-layout migration map when needed, preserve curated/promoted/higher-grade facts, status touched facts, and emit mutation records.

**Auditor:** Independently sample inventory, breadth trail, ledger, CORE areas, repo `deep/` contracts/invariants, source-catalog warning content, restricted-sources, telemetry-routing-card, promote-up records, `kb-mutation` records, old-layout migration map when applicable, and Clean Deliverable Packet. Record findings and closure decision.

## Onboarding vs curator

Onboarding may regenerate baselines, including in `incremental` mode, but must preserve curated/promoted/higher-grade facts per `kb-mutation.md`. Curator write-back is a separate agent behavior; onboarding only initializes the `contributions/` shape and obeys the shared mutation contract.
