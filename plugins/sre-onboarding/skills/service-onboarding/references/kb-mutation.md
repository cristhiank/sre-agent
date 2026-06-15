# KB Mutation

Size budget: ~160 lines. Canonical home for incremental re-mine and curator write-back safety. Other files point here instead of restating the preservation rule.

## Purpose

The KB is living. It changes through two triggers that share one mutation-safety contract:

1. `incremental` — repo SHAs advanced; diff them and re-mine only changed surfaces.
2. write-back — a separate curator agent proposes a learned contribution. This skill initializes `contributions/` shape and states the contract; it does not perform curator writes.

## Shared preservation rule

Never overwrite a curated, promoted, owner-verified, or higher-grade fact unless the run produces stronger evidence and a change/contribution record. Lower-grade evidence never replaces higher-grade evidence. Every touched fact receives a `kb-mutation` status:

- `preserved`
- `superseded(stronger-evidence+record)`
- `re-graded`
- `new`
- `stale`
- `removed`

This is the single canonical non-conflict rule.

## Evidence-strength comparison

One definition of stronger: lower-grade evidence never replaces higher-grade evidence, full stop. Trust grade is the hard floor. Applicability, `grounding_type`, freshness, and review adjudicate only among same-grade candidates or candidates whose trust grade is higher than the current fact.

Compare same-or-higher-grade candidates in this order, scoped to the same claim:
1. evidence applicability to the exact scope
2. `grounding_type` fit for the claim class (`repo-source` mandatory for implementation claims)
3. rule_status: `applied`/`blocked` beats `not-applied`
4. freshness and re-resolved anchors
5. owner/curator review where required

Trust labels rank for all facts, including repo `deep/` contracts/invariants: `verified/observed` > `source-inferred/declared` > `docs-only` > `suspected ⚠️`. A later live check that strengthens trust without replacing the fact is `re-graded`, not `superseded`.

A newer scan with weaker evidence preserves the older stronger fact and appends stale-risk or verification-queue notes.

## Stale and deleted facts

If a source code path, manifest, query corpus entry, or typed anchor disappears, do not silently drop the fact. Mark it `removed` with record: old anchor, new scan scope, deletion evidence, affected canonical home, downstream links updated, and verify-later action if runtime truth may persist outside source.

If a fact is not contradicted but its source is old or unverified, mark `stale`, update freshness, and queue re-verification.

## `incremental` procedure

1. Read current provenance: source SHAs, scan date, verification queue, contribution records, and prior layout version.
2. If the KB predates this layout, produce an old-layout migration map before mutating; do not half-migrate.
3. Diff repo SHAs and declaration surfaces.
4. Re-mine only changed surfaces plus any dependent surfaces needed to classify edges, observability, failure-knowledge, and concept impacts.
5. Apply evidence-strength comparison.
6. Update canonical homes, cross-links, `00-index/core-map`, verification queue, freshness, and overlays if their source window changed.
7. Emit mutation records, migration map if applicable, and the Clean Deliverable Packet.

## Old-layout migration

Map old -> new before incremental mutation. Literal runs must not orphan old-home artifacts or leave split-brain roots.

| Old artifact/home | New canonical home |
|---|---|
| root `concept-model.md` | `service/concept-model.md` |
| root `glossary.md` | `service/glossary.md` |
| root or glue `request-tracing.md` | `observability/` route recipes and join-key docs; flow semantics may cross-link to a future `flows/` anchor, but observability remains the canonical signal home |
| `state-and-config/` or equivalent | split by fact: ownership/access to `service/`, deployment/runtime/config topology to `topology/`, telemetry/config signals to `observability/`, failure signatures to `failure-knowledge/`, repo-local facts to `kb/<repo>/` |
| old `infra/` | `topology/` for runtime/deployment rows; `service/` for ownership, escalation, access, support boundaries |
| old `service-graph/` | `topology/` |
| old `observability/` | `observability/` with source-catalog, join-keys, canonical-signals, restricted-sources, dashboards/recipes |
| old `failure-modes/` | `failure-knowledge/` |
| old `incident-knowledge/` | `overlays/incidents/` |
| old per-repo READMEs | `kb/<repo>/entry-points.md`, `modules.md`, `concepts.md`; `deep/` follows the incident-material populated-or-explicit-`unknown` rule in `kb-layout.md` |

Migration records cite old path, new canonical home, mutation status, stable anchor/core-map update, and unresolved gaps. The independent audit checks this map and fails orphaned old-home artifacts.

## Clean-deliverable across mutation

Incremental runs must emit the same Clean Deliverable Packet as first-time runs. The prior `_work/*-delta` leak is a failure mode: mutation scratch never lands in the deliverable. The allowlist/denylist and remediation requirements live in `workflow.md`; do not duplicate them here.

## Contribution record

Minimal record fields:

`id | source(sre-investigation|code-review|dev|onboarding|human) | source_ref | change-type | target(core/incident-lens/overlay/repo-deep) | statement | grounding_type | evidence_grade | anchors[] | placement.canonical_home | dedupe | contradiction+resolution | status`

Optional but encouraged: confidence, freshness policy, sensitivity, owner/reviewer, links to affected `00-index/core-map` entries.

## No-laundering gate

Trust comes from `grounding_type`, evidence grade, anchors, rule passage, and review. It never comes from the `source` tag. A contribution from an investigation, review, dev task, onboarding run, or human is still only a candidate until grounded and placed.

## Tiered ceremony

High-risk facts require review before promotion: contracts, auth/control, PII/secrets, ownership/escalation, canonical telemetry sources, and routing failure-mechanisms.

Low-risk pointers, local conventions, and non-authoritative route hints may promote with lower ceremony, but still carry an explicit grade, anchors, and status.

## Sensitivity gate

Block promotion of secrets, raw PII, restricted samples, and copied sensitive payloads. Store references, stable IDs, hashes, or redacted non-reconstructable descriptors instead of copied payloads.

## Initialized shape

Onboarding may create:

- `contributions/README.md` — states curator-owned lifecycle and this contract
- `contributions/intake/`
- `contributions/accepted/`

Onboarding must not write learned curator facts except as onboarding-generated baseline records governed by this file.
