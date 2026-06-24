# Artifact Manifest

Manifest version: 1.0. Canonical home for the committed KB artifact tree, per-artifact schemas, row-ordering rules, and dependency edges. Consumers derive file structure and column schemas from here; `references/kb-layout.md` covers layout principles; `references/reproducibility-contract.md` covers run contracts, stable-ID algorithm, and decidable predicates.

## Status codes

| Code | Meaning |
|---|---|
| M | Mandatory — must exist; may contain a `none-found` gap note with searched scope |
| G | Capability-gated — created iff evidence capability is reachable; record capability-gap note when absent |
| P | Predicate-conditional mandatory — mandatory iff the named predicate evaluates to the specified outcome; a gap record in `00-index/evidence-ledger.toon` is required when evidence for the predicate is inconclusive |
| V | Variable-slot — created iff a promotion predicate passes; slot count varies per run |
| O | Optional-derived — produced when evidence warrants; not required for done gates |

## Full artifact tree

```
services/<service>/
  README.md                                 M
  service.yaml                              M
  00-index/
    task-router.md                          M
    telemetry-routing-card.md               M
    core-map.md                             M
    evidence-ledger.toon                    M
    ownership.toon                          M
    incident-clusters.toon                  M
    ai-asset-catalog.md                     G
  service/
    identity.md                             M
    ownership.md                            M
    access-escalation.md                    M
    concept-model.md                        M
    glossary.md                             M
    support-boundaries.md                   M
  topology/
    service-graph.md                        M
    dependencies.md                         M
    blast-radius.md                         M
    endpoints-ports-catalog.md              M
    deployable-unit-coverage.md             M
    per-deployable-units.md                 M
    data-flow-handoffs.md                   M
  observability/
    README.md                               M
    source-catalog.md                       M
    join-keys.md                            M
    canonical-signals.md                    M
    restricted-sources.md                   M
  failure-knowledge/
    README.md                               M
    candidate-signatures.md                 M
    <signature-family-id>.md                V  (one per promoted family; see promotion predicate §)
    secondary-signatures.md                 O
  overlays/incidents/                       G  (whole folder; requires reachable incident-history source)
    README.md                               M  (within folder)
    ownership-90d.md                        G
    ownership-90d.toon                      G
    incident-clusters-90d.md               G
    incident-clusters-90d.toon             G
    monitors-90d.md                         G
  kb/<repo>/                                M  (one set per service repo)
    README.md                               M
    entry-points.md                         M
    modules.md                              M
    concepts.md                             M
    ai-assets.md                            M
    deep/contracts.toon                     P  (P3=incident-material)
    deep/invariants.toon                    P  (P3=incident-material)
    deep/not-material.md                    P  (P3=non-material; sole deep/ artifact for non-material repos)
  contributions/
    README.md                               M
    INTAKE.md                               M
    intake/                                 O
    accepted/                               O
```

## Per-artifact schemas

### `README.md` (root, M)

Sections: service overview; 4-area mental model table (`Area | Folder | Answers`); incident decision tree (numbered steps); area index (links to all folders); freshness/provenance footer. Must contain the untrusted-distilled-context disclaimer.

### `service.yaml` (root, M)

YAML fields: `name`, `aliases[]`, `repos[]` (each entry: `name | path | url | branch | sha`). All fields mandatory; `sha` at scan date. This file is the committed service/repo source snapshot — it records repo names, branches, and SHAs but does not carry the full run-lock fields (live-evidence mode, capability map, overlay window, prior KB hash). Full provenance lock summary is in the `@meta` block of `00-index/evidence-ledger.toon`.

### `00-index/task-router.md` (M)

Schema: `symptom-family | CORE-area | file-anchor | notes`. One row per major symptom class; covers all four CORE areas. Built after all CORE areas are rendered.

**Required cross-link:** for stale-output / missing-data symptom families (a proximate cause bottoms out on a stale or missing data entity), route the responder to `topology/data-flow-handoffs.md` to walk upstream. Static pointer to the artifact only — never enumerate individual handoff rows (avoids re-render churn).

### `00-index/telemetry-routing-card.md` (M)

Schema: `symptom-family | first-source-catalog-ref | join-key | canonical-signal-ref | discriminator | restricted-source-note | empty-result-warning | freshness/trust | method-gates | next-owner/escalation-ref`.

Required method-gate pointer lines per row: `probe-before-proximate` · `empty-is-not-absent` · `detection-vs-onset` · `provenance-strength`. Produced during this onboarding run as a route view over CORE observability.

**Required cross-link:** for stale-output / missing-data symptom families (a proximate cause bottoms out on a stale or missing data entity), carry a static pointer to `topology/data-flow-handoffs.md` for the upstream-walk view — point at the artifact, never enumerate individual handoffs (avoids re-render churn).

### `00-index/core-map.md` (M)

Schema: `stable-id | kind | canonical-home | aliases | superseded-by | notes`. Covers all stable IDs from all CORE areas. Sorted by `stable-id` ascending. ID algorithm: `references/reproducibility-contract.md`.

### `00-index/evidence-ledger.toon` (M)

Committed replay surface for all terminal ledger records from the most recent run. Required for incremental determinism: subsequent runs read this artifact to establish prior record statuses and avoid overwriting higher-grade facts.

**`@meta` block (provenance lock summary — all replay-relevant fields):** `manifest-version | scan-date | live-evidence-mode | overlay-window | prior-kb-state-hash | capability-map-summary`. `capability-map-summary` embeds each capability's reachability result using the canonical vocabulary from `references/reproducibility-contract.md §Live-capability vocabulary` (`disabled-by-scope | attempted-unreachable | reachable-snapshot` + probe-result summary) rather than pointing to a transient file. This block is the authoritative committed provenance lock; `service.yaml` carries repo/SHA only.

**`@run-trail` block (mandatory):** records how the run was orchestrated. Contains:
- **dispatch-decision sub-record (one per run):** `dispatch-available | dispatch-required | used | packet-evidence | degraded-reason`. `single-pass` or `degraded` dispatch-mode is valid ONLY when `dispatch-available=no` or `dispatch-required=no`; if `dispatch-available=yes AND dispatch-required=yes AND used=no`, the run must ABORT.
- **stage rows (one per executed stage):** `stage | worker-role | packet-id-or-hash | searched-scope | merge-status | outcome`.
- **dispatch-mode summary:** `dispatched | single-pass | degraded` — derived from the dispatch-decision record.
Must be committed to this file BEFORE any transient scratch or `_work/` directories are deleted.

**`@audit` block (mandatory):** records the independent audit result. Fields: `auditor-identity/capability | non-builder-attestation(yes/no) | sampled-artifacts | findings | closure`. Auditor must differ from any builder or orchestrator recorded in `@run-trail`. Must be committed to this file BEFORE any transient scratch or `_work/` directories are deleted.

**`@schema` block + data rows:** all terminal records from the normalized evidence ledger. Same schema as `references/reproducibility-contract.md`, redacted of secrets and raw restricted payloads. Records with `status = promoted`, `rejected`, `non-material`, `stale`, `open:escalated`, `sensitive-unsafe`, `superseded`, or `duplicate` are all included. Transient `_ledger.toon` in the run-root is the full working copy; this committed artifact is the redacted, finalized subset.

**`@promote-up` block:** records cross-layer promote-up events. Schema: one row per event: `source_cell | destination_cell | payload`. Each row documents a service-higher implementation claim's floor-cell origin. Referenced by audit items D and F.

Gap records for `deep/` predicate outcomes (`open:escalated` or `non-material`) live here, not as schema rows in `contracts.toon`/`invariants.toon`.

### `00-index/ownership.toon` (M)

TOON format: `@meta` block + `@schema` + data rows. Machine-readable, non-authoritative ("restored candidate" marker required). Populated from overlays when available; otherwise from `service/` ownership evidence with lower trust grade.

### `00-index/incident-clusters.toon` (M)

TOON format: `@meta` block + `@schema` + data rows. Machine-readable, non-authoritative ("restored candidate" marker required). Populated from incident overlays and/or promoted failure-knowledge signatures when available; otherwise write `none-found (searched scope: <overlays-source + failure-knowledge>)` — do not fall back to ownership evidence.

### `00-index/ai-asset-catalog.md` (G)

Anti-authority header required (verbatim): `docs-only lead, not authority; re-resolve before use.`

Schema: `asset-ref(kb/<repo>/ai-assets.md anchor) | consumers(tags: incident|review|dev) | one-line-purpose | why-included(named materiality test) | trust(docs-only|suspected ⚠️) | freshness(repo SHA)`.

**Human-guidance rows** additionally require a **symptom/trigger** field — carried either as a symptom-led `one-line-purpose` (lead with `Symptom: <non-generic trigger>` …) or an explicit column, matching the existing table style — and MUST be cross-linked from `00-index/telemetry-routing-card.md` (and/or `task-router.md`) under the matching symptom family so the runtime orientation scan can route symptom→doc.

Pointer-only; inherits grounding via `asset-ref` pointer to the floor row. Non-normative: no ordering/precedence, no "use this first", no `evidence_required`, no `stop_conditions`. Grouping by primary consumer tag is presentation-only and encodes no precedence.

### `service/identity.md` (M)

Sections: service name + aliases; what it does; primary users / tenant model; product area; key capacity/scale dimensions; known historical names. Every claim cited.

### `service/ownership.md` (M)

Sections: teams (name, scope, DL/contact); on-call rotation; incident-response primary; escalation paths. Grounding required per entry.

### `service/access-escalation.md` (M)

Sections: access levels; elevation procedures; break-glass paths; required approvals; JIT/approval system pointer. Source-cited.

### `service/concept-model.md` (M)

Table-first; 10–20 concepts max.

Schema: `concept | category | role | triage-use | serves-CORE-area | aliases | evidence | trust | grounding_type`.

Category enum (exact, use these values): `domain-entity · unit-of-work · process-stage/lifecycle-state · identity/tenancy/partition-key · correlation/trace-key · deployment/topology · resource/capacity · policy/config · acronym`.

Inclusion predicate (≥2 of 4 must hold): (a) appears in telemetry/log/query dimensions; (b) drives routing/partitioning/ownership/blast-radius; (c) encodes lifecycle/state/process order; (d) needed to map symptom to root cause/mitigation/owner. Full predicate definition: `references/reproducibility-contract.md`.

Required categories `identity/tenancy/partition-key`, `correlation/trace-key`, and `process-stage/lifecycle-state` are populated or carry audited `none found (material units + source classes searched)`.

`deployment/topology` may not dominate. `triage-use` column is mandatory.

### `service/glossary.md` (M)

Flat term → definition. Cross-link to concept-model; do not duplicate load-bearing concept rows.

### `service/support-boundaries.md` (M)

Sections: in-scope; out-of-scope; SLA/commitments; handoff points; downstream consumer notes.

### `topology/service-graph.md` (M)

Node/edge table. Edge `kind` enum: `observed-runtime-call | deploy/config-reference | test-only | docs-only | generated/unused-candidate`. Auth/identity as edge attributes. Closed-world ban applies; absence claims must name searched scope.

### `topology/dependencies.md` (M)

Schema: `dependency | kind(runtime|library|infra|ops-managed) | direction(inbound|outbound) | version/compat | evidence | trust | grounding_type`.

### `topology/blast-radius.md` (M)

Schema: `failure-scope | affected-units | affected-users/tenants | downstream-cascades | mitigation-boundary | evidence | trust | grounding_type`. Derived from service-graph + dependencies.

### `topology/endpoints-ports-catalog.md` (M)

Schema: `unit | endpoint/route/surface | port | protocol/source-kind | auth/control-attribute | caller/dependency-notes | evidence(kb/<repo>/...:line) | trust | grounding_type`. One row per exposed surface.

### `topology/deployable-unit-coverage.md` (M)

Audit matrix: `unit | service/ | topology/ | observability/ | failure-knowledge/ | gap+searched-scope`. One row per material unit/plane. A single overview cannot substitute for uncovered units/planes.

### `topology/per-deployable-units.md` (M)

Schema: `unit | hosting-model | deployment/runtime-scope | scale/routing-dimension | owner/team | on-call/escalation | evidence(kb/<repo>/...:line) | trust | grounding_type`. One row per material unit/plane.

### `topology/data-flow-handoffs.md` (M)

Entity-indexed, symptom-agnostic artifact-mediated data-flow handoff view: enables walking UPSTREAM when a proximate cause bottoms out on a stale/missing data entity.

Schema: `handoff-id | producer | consumer | entity/artifact | freshness-anchor | upstream-owner | fk-xref | guidance-xref | traversal-probe | evidence(kb/<repo>/...:line) | trust | grounding_type`. One row per producer→entity→consumer handoff.

**Hard-required cells (promoted row):** producer, consumer, entity/artifact, traversal-probe, evidence, trust, grounding_type. **Degradable cells:** freshness-anchor, upstream-owner → `unknown (searched scope)`; fk-xref, guidance-xref → `none`.

**What this is NOT** — `data-flow-handoffs.md` is DISTINCT from three look-alikes; it never duplicates them:
- **vs `service-graph.md`** — service-graph holds SYNCHRONOUS request/response edges (failure = immediate call error). data-flow-handoffs holds ASYNC artifact-mediated edges (failure = silent staleness). A handoff is NOT re-listed as a service-graph edge; service-graph may carry a pointer, never a duplicate row.
- **vs `cross-service.md` / `(EXTERNAL; cross-service: <slug>; service-kb: ../../<slug>/)` marker** — that is KB FEDERATION (which sibling owns the far end). data-flow-handoffs is intra-service DATA FLOW (producer→entity→consumer). A handoff that crosses a service edge cites the federation pointer for the far side and owns only its side.
- **vs `failure-knowledge` discriminators** — failure-knowledge is SYMPTOM-indexed (found only if you already suspect the symptom). data-flow-handoffs is ENTITY-indexed / symptom-agnostic (found by walking upstream from a stale entity). They cross-reference via `fk-xref`; mechanisms are never duplicated.

**Gating:** P8 (`references/reproducibility-contract.md`). **Promoted-only render:** only `promoted` P8 rows appear here; non-promoted edges (one-sided `open:escalated`, hypothesis `suspected ⚠️`, searched) stay in the evidence ledger, never inline. An all-reject service renders a single ledger-derived `none-found (searched scope)` note. Status M for every service; emptiness is evidenced, never skipped.

**Cross-service composition:** a handoff crossing a sibling-service edge cites the federation pointer for the far side and owns only its own side; do not duplicate provider internals.

**Stable ID:** `topo.handoff.<producer-slug>.<entity-slug>.<consumer-slug>` — slug rules in `references/reproducibility-contract.md`. Enums (`trust`, `grounding_type`, `status`): `references/reproducibility-contract.md §Canonical enums`.

### `observability/README.md` (M)

Sections: primary source classes; join-key quick reference; query-authoring notes. Anti-absence caveat required: sources may be access-gated; see `restricted-sources.md`.

### `observability/source-catalog.md` (M)

Mandatory even when no live verification is reachable.

Schema: `signal | source-kind | table/metric/log-name | join-keys | dimensions | recipe/query-pointer | discovery-source | last-live-verification(date or NEVER) | trust | grounding_type | restricted? | exact-verify-later-action | CONSUMER WARNING`.

Hard rules (gating done):
- Any source-inferred row: `last-live-verification = NEVER` + CONSUMER WARNING containing `do not conclude absence or a block without live-checking <source> via <join key>` with concrete source and join key filled (literal placeholders fail).
- `recipe/query-pointer` → path to recipe file; no copied query sprawl in this catalog.
- `exact-verify-later-action` names source class, join key, and expected proof to collect.

### `observability/join-keys.md` (M)

Schema: `service | key-name | type | example-value-shape | cross-service-join | evidence | trust | grounding_type`.

### `observability/canonical-signals.md` (M)

Schema: `symptom-family | canonical-signal | source-catalog-ref | join-key | freshness | trust | grounding_type | non-dispositive-alternatives | verify-later`. Routing index over source-catalog rows; not a second home for telemetry facts.

### `observability/restricted-sources.md` (M)

Schema: `source | gate/rejection-observed | scope-tested | alternate-reachable-source? | consumer-note`. Consumers must not generalize a gate on one source to all sources.

### `failure-knowledge/README.md` (M)

Index table: `family-id | file | symptom-summary | status(verified|candidate|secondary)`. Discriminator-first note. Rendered after all signature files exist.

### `failure-knowledge/<signature-family-id>.md` (V — variable-slot)

**P6 three-way routing:** promoted families go to `<signature-family-id>.md`; secondary families go to `secondary-signatures.md`; remaining non-rejected families go to `candidate-signatures.md`. The evidence ledger records the routing decision through `destination`; `failure-knowledge/README.md` summarizes the status.

**Stable ID:** `fk.<symptom-family>.<mechanism>` — slug rules in `references/reproducibility-contract.md`. IDs are assigned before rendering; promotion decisions are recorded in the evidence ledger.

Schema: `symptom | signature | discriminator | mechanism | likely-owner | mitigation | evidence | trust | grounding_type | verify-later`. `discriminator` column is mandatory and must be actionable. Promotion/candidate/secondary status is recorded in `failure-knowledge/README.md` and the evidence ledger, not as an extra column in signature files.

### `failure-knowledge/candidate-signatures.md` (M)

Same column schema as promoted files. Never empty: initialize with `none-identified (searched scope: <...>)` if no candidates found. Candidate status is recorded in `failure-knowledge/README.md` and the evidence ledger.

### `failure-knowledge/secondary-signatures.md` (O)

Same schema. Produced only for P6-secondary families (one reusable key plus actionable discriminator, but not enough evidence for promotion). Secondary status is recorded in `failure-knowledge/README.md` and the evidence ledger.

### `overlays/incidents/` files (G)

90-day window; aggregates + stable IDs + owner DLs only. No PII, raw incident titles, or copied restricted payloads. Sanitization statement required in each file's header. Stale rows past the window roll off or are re-verified from a refreshing source change.

`ownership-90d.toon` and `incident-clusters-90d.toon`: TOON format with `@meta` + `@schema` + data rows; machine-readable.

### `kb/<repo>/README.md` (M)

Sections: repo name + SHA at scan; deployable-binary vs library classification; materiality verdict; key source paths. Freshness pointer to root provenance.

### `kb/<repo>/entry-points.md` (M)

Schema: `surface | kind(main|host|startup|endpoint|entrypoint) | path(file:line) | deployable-binary-or-library | evidence | trust`. All exposed surfaces enumerated.

### `kb/<repo>/modules.md` (M)

Schema: `module | responsibility | composition-root(path or N/A) | load-bearing? | evidence | trust | grounding_type`.

### `kb/<repo>/concepts.md` (M)

Repo-local concept candidates feeding `service/concept-model.md`. Tag column: `promoted | local-only | pending`.

### `kb/<repo>/ai-assets.md` (M)

**Inventory row schema:** `path | kind(illustrative enum: agent_doc|instruction|skill|subagent|chatmode|prompt|shared-ref|human-guidance|runbook|troubleshooting-guide|known-issues|alert-response) | repo | title`.

**Triaged-lead row schema:** `asset-ref | declared-purpose(sanitized) | consumer-relevance(incident|review|dev, multi-tag) | maps-to(unit/CORE-area or unmapped) | trust(docs-only|suspected ⚠️) | grounding(docs-only|manual-curated) | freshness(repo SHA) | verify-later`.

**Human-guidance rows** additionally carry a **symptom/trigger** field — as a symptom-led `declared-purpose` or an explicit `symptom/trigger` column matching the existing table style — holding the non-generic symptom→this-doc phrase; the matching `00-index/telemetry-routing-card.md` (and/or `task-router.md`) cross-link is recorded so the catalog entry can route symptom→doc.

Non-promotable; **pointer-only — never copy doc content or values**. Sensitive values (cluster URLs, subscription IDs, GUIDs): pointer + sanitized purpose only; raw values not copied. Empty repo: write `none-found (searched scope: <globs/dirs incl. widened human-guidance Stage A directories>)` — honest only after the widened human-guidance scope was searched too.

### `kb/<repo>/deep/contracts.toon` (P — mandatory iff P3=incident-material)

TOON schema: `contract-id | kind(api|schema|event|queue) | surface(path/route/topic) | shape-ref(file:line) | version/compat-rule | consumers | materiality-category | evidence | trust-label | grounding_type`.

When evidence for P3 is inconclusive, record an `open:escalated` row in `00-index/evidence-ledger.toon` rather than inserting gap rows in this schema.

### `kb/<repo>/deep/invariants.toon` (P — mandatory iff P3=incident-material)

TOON schema: `invariant-id | statement | enforcement-site(file:line) | discriminator(how to prove it failed) | scope | materiality-category | evidence | trust-label | grounding_type`.

When evidence for P3 is inconclusive, record an `open:escalated` row in `00-index/evidence-ledger.toon`.

### `kb/<repo>/deep/not-material.md` (P — mandatory iff P3=non-material)

Single record: `repo | searched-scope | materiality-verdict=not-material | reason | ledger-row-ref`. This is the sole deep/ artifact for non-material repos.

### `contributions/` (M shape)

`README.md`: curator-owned lifecycle and mutation contract pointer. `INTAKE.md`: contribution intake template. `intake/` and `accepted/` directories initialized as empty; curator populates later.

---

## Freshness / provenance header

Required in: root `README.md`, each `kb/<repo>/README.md`, `observability/source-catalog.md`. Optional pointer at `service/provenance.md`.

Required fields:
- `scan date`
- `live-verification posture` (verified-live | source-inferred | NEVER)
- repo table: `repo | branch | SHA`
- `last live verification` (date or NEVER)
- `independent-audit verdict` (summary line)
- `open/escalated thread summary` with exact verify-later actions
- `verification-queue pointer`
- `stale-risk markers` (if any)
- `overlay window + roll-off date + sanitization statement` (when overlays exist)

## Row ordering rules

Within any artifact table, apply in priority order:

1. **Manifest order** — follow the artifact's defined section sequence from this document.
2. **Category enum** — rows within a section sort by the canonical enum order for that column (e.g., concept-model category enum; signature status: verified > secondary > candidate).
3. **Stable ID** — within same category, sort by stable ID ascending (lexical). ID algorithm: `references/reproducibility-contract.md`.
4. **Evidence priority** — tie-break: `verified/observed` > `source-inferred/declared` > `docs-only` > `suspected ⚠️`.
5. **Lexical path** — final tie-break: source path lexical ascending.

## Canonical placement / pointer-not-duplicate

- A fact lives in exactly one canonical home. Cross-references are pointers (`see <canonical-home>:<stable-id>`), never copies.
- Service-higher implementation facts trace to repo-lower cells (cross-layer grounding); CORE artifacts are distilled destinations, not duplicates.
- `00-index/ai-asset-catalog` is pointer-only over `kb/<repo>/ai-assets.md` floor rows; it carries no additional grounding.
- `observability/canonical-signals.md` is a routing index over `observability/source-catalog.md`; it adds routing metadata but introduces no new telemetry facts.
- **Anchor-ID stability invariant.** Stable IDs survive moves/renames; never recycled for a different fact. If a fact moves, update `00-index/core-map.md`; if a fact splits, mint scoped new IDs and retain a supersession pointer.

## Artifact dependency edges

| Upstream artifact(s) | Downstream artifact(s) | Dependency kind |
|---|---|---|
| normalized evidence ledger records | all artifact schemas | render-from-ledger: builders fill table rows from ledger; no direct page drafting from exploration |
| finalized terminal ledger records | `00-index/evidence-ledger.toon` | all terminal records written to committed replay surface at end of Phase 5 |
| `00-index/evidence-ledger.toon` (prior run) | incremental re-mine scope | prior statuses determine preservation vs re-grade for changed surfaces |
| `kb/<repo>/entry-points.md` + `modules.md` | `topology/per-deployable-units.md`, `topology/endpoints-ports-catalog.md` | floor → service-higher promote-up |
| `kb/<repo>/deep/contracts.toon` + `invariants.toon` | `topology/` + `failure-knowledge/` (pointer entries) | floor → CORE promote-up |
| `kb/<repo>/concepts.md` (all repos) | `service/concept-model.md` | floor concept candidates → service-higher promotion |
| `topology/per-deployable-units.md` | `topology/deployable-unit-coverage.md` | unit enumeration seeds coverage matrix |
| `topology/service-graph.md` + `topology/dependencies.md` | `topology/blast-radius.md` | graph data grounds blast-radius analysis |
| `kb/<repo>/deep/contracts.toon` + `kb/<repo>/concepts.md` + `failure-knowledge` discriminators + `observability/join-keys.md` | `topology/data-flow-handoffs.md` | floor+CORE → artifact-mediated handoff lineage promote-up (P8) |
| `observability/source-catalog.md` | `observability/canonical-signals.md` | canonical signals is a routing index over source-catalog rows |
| `observability/source-catalog.md` + `observability/join-keys.md` | `00-index/telemetry-routing-card.md` | CORE observability grounds the incident routing card |
| `kb/<repo>/ai-assets.md` (all repos) | `00-index/ai-asset-catalog.md` | floor rows aggregate into multi-consumer catalog (pointer-only) |
| `overlays/incidents/ownership-90d.*` | `00-index/ownership.toon` | overlay grounds machine-readable ownership catalog |
| `overlays/incidents/incident-clusters-90d.*` | `00-index/incident-clusters.toon` | overlay grounds machine-readable cluster catalog |
| all CORE areas + `00-index/telemetry-routing-card.md` | `00-index/task-router.md` | all areas contribute symptom-routing anchors |
| all CORE areas | `00-index/core-map.md` | all stable IDs registered here |
| `failure-knowledge/<id>.md` (promoted, V) | `00-index/incident-clusters.toon` | promoted signatures seed machine catalog |
