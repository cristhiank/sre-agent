# Reproducibility Contract

Canonical home for the operational definition of a reproducible KB, the input/run lock schema, normalized evidence ledger schema, stable-ID algorithm, canonical ordering, decidable predicates, render-from-ledger contract, and incremental changed-surface closure rules.

## Operational definition

A KB run is **reproducible** when: given the same locked inputs (§ Input/run lock), the same capability reachability, and the same evidence snapshot/window, a re-run produces:

1. The same artifact tree (manifest-defined files present/absent per status codes and predicate outcomes).
2. The same stable IDs assigned to the same subjects.
3. The same fact rows with equivalent content, evidence citations, trust grades, and canonical placements.
4. Semantically equivalent narrative (same claims, same caveats) — not byte-identical prose.
5. The same gap/open-thread set, and the same drop-reason for each dropped candidate.

**Not required for reproducibility:** prose phrasing, whitespace, section ordering within prose blocks, or the exact wording of verification-queue actions.

**Source-derived facts are reproducible.** Live telemetry and incident overlay evidence is reproducible only if snapshotted or if the live-evidence mode is recorded as `attempted` or `disabled` (not pretended to be deterministic). Record the mode honestly.

## Input/run lock schema

Record before scouting begins. Store as `_run-lock.yaml` in the transient run-root (never in the committed KB). The committed provenance header summarizes these fields.

| Field | Type | Notes |
|---|---|---|
| `service-identity` | string | canonical service slug; matches `service.yaml:name` |
| `manifest-version` | string | artifact-manifest version this run targets (e.g., `1.1`) |
| `source-roots[]` | path list | local roots where repos are checked out |
| `repo-names[]` | string list | names matching `service.yaml:repos[].name` |
| `repo-branches[]` | string list | one per repo, same order |
| `repo-SHAs[]` | string list | one per repo at scan date; must be exact, not approximate |
| `live-evidence-mode` | enum | `disabled` / `attempted` / `snapshot` |
| `overlay-window` | string | e.g., `90d` or `none`; must be explicit |
| `capability-map[]` | record list | `capability \| reachable(yes/no/partial) \| probe-result` |
| `prior-kb-state-hash` | string | SHA of prior committed KB root, or `none` for first-time |
| `scan-date` | ISO date | date the run was initiated |

A run is only reproducible relative to its locked input set. Changing any field (even adding one new repo) produces a new distinct run; do not compare outputs across different input locks.

## Live-capability vocabulary

Canonical states for `capability-map-summary` entries in `00-index/evidence-ledger.toon @meta` and for any live/capability state recorded in KB artifacts. Any capability state not in this table is a FAIL in mechanical audit item I.

| State | Meaning | Probe required? |
|---|---|---|
| `disabled-by-scope` | capability deliberately excluded from this run; no probe attempted | no |
| `attempted-unreachable` | a bounded probe ran and failed; record capability name, canonical target, and auth-vs-absence result | yes — record probe-result |
| `reachable-snapshot` | probe succeeded; evidence captured at scan date | yes |

**Forbid bare `unreachable`** without a recorded bounded probe. Use `attempted-unreachable` + probe-result for any failed live probe. Use `disabled-by-scope` when excluded by design.

## Normalized evidence ledger schema

The ledger is the central data structure. Builders fill artifact schemas from ledger rows; they never draft KB pages directly from exploration.

**Transient vs committed ledger.** During the run, `_ledger.toon` in the transient run-root is the full working ledger. After Phase 5, a redacted subset is written to `00-index/evidence-ledger.toon` in the committed KB — the committed replay surface. The done gate requires the committed ledger to exist and be current. Incremental runs MUST read `00-index/evidence-ledger.toon` from the prior committed KB to establish prior record statuses; they must not read the prior transient run-root artifact (which may not exist). No secrets, raw PII, or restricted payloads in the committed ledger.

**Row schema:**

`record-id | claim-class | normalized-subject | predicate-inputs | evidence | trust-label | grounding_type | rule_status | confidence | blast-radius-if-wrong | destination | status | verify-later`

| Column | Values / Notes |
|---|---|
| `record-id` | stable ID (see § Stable ID algorithm) |
| `claim-class` | see `§Canonical enums` |
| `normalized-subject` | canonical slug for the fact's subject (service, unit, repo, edge, signal, concept) |
| `predicate-inputs` | which decidable predicate(s) were evaluated; comma-separated predicate names |
| `evidence` | `file:line` or typed anchor; re-resolved after generation |
| `trust-label` | see `§Canonical enums` |
| `grounding_type` | see `§Canonical enums` |
| `rule_status` | see `§Canonical enums` |
| `confidence` | `high · medium · low · unknown` |
| `blast-radius-if-wrong` | free text: what breaks or who is affected if this claim is incorrect; `unknown` when scope is unclear |
| `destination` | canonical artifact path + stable-ID anchor where this row will be rendered |
| `status` | see `§Canonical enums` |
| `verify-later` | exact action: source class, join key, expected proof; or `none` |

**Status / drop-reason taxonomy:**

| Status | Meaning |
|---|---|
| `promoted` | rule applied/blocked, evidence cited, trust/grounding justified, destination recorded |
| `duplicate` | same claim already promoted to same destination with equal or stronger evidence |
| `stale` | source anchor disappeared or unverified past freshness threshold |
| `non-material` | investigated; does not meet any incident-materiality trigger |
| `sensitive-unsafe` | contains secrets, raw PII, or restricted payload; blocked from KB |
| `superseded` | stronger evidence for same claim was found in same run |
| `open:escalated` | decisive evidence unreachable in this run; exact gap and action named; requires proof fields `reachable-static-probes-tried`, `attempted-source-classes`, `why-unreachable`; invalid if same-subject reachable static evidence exists |
| `rejected` | investigated and unsupported; searched scope cited |

## Canonical enums

This section is the single canonical home for KB enum value sets. All other files that need to reference these values MUST point here (`references/reproducibility-contract.md §Canonical enums`); they must not restate the value lists.

| Enum | Canonical values |
|---|---|
| `claim-class` | `control/auth · config/secret · edge/dependency · failure-mode · concept · observability · ownership/escalation · overlay · ai-asset` |
| `trust-label` | `verified/observed · source-inferred/declared · docs-only · suspected ⚠️` |
| `grounding_type` | `repo-source · live-telemetry · incident-overlay · monitor-history · docs-only · manual-curated` |
| `status` | `promoted · duplicate · stale · non-material · sensitive-unsafe · superseded · open:escalated · rejected` |
| `rule_status` | `not-applied · applied · blocked` |

## Stable ID algorithm

IDs are assigned to subjects before rendering. Assignment is deterministic: derived from a normalized source key, not from row order or discovery sequence. Collision suffix appended from the first 6 chars of the SHA-1 of the full source key — not an incrementing integer.

**ID prefixes and source keys:**

| ID pattern | Subject | Source key |
|---|---|---|
| `svc.<slug>` | service identity anchor | normalized service name |
| `repo.<slug>` | per-repo floor root | repo name (normalized) |
| `topo.unit.<unit-slug>` | deployable unit or operational plane | normalized unit name from deployment manifest |
| `topo.edge.<caller>.<callee>` | topology edge | `<caller-unit-slug>.<callee-unit-slug>.<kind>` |
| `topo.handoff.<producer-slug>.<entity-slug>.<consumer-slug>` | artifact-mediated handoff edge | `<producer-unit-slug>.<entity-slug>.<consumer-unit-slug>` |
| `obs.source.<source-slug>` | observability source / telemetry table | normalized source name from discovery source |
| `obs.signal.<symptom>.<source>` | canonical signal | `<symptom-family-slug>.<source-slug>` |
| `fk.<symptom-family>.<mechanism>` | failure-knowledge signature family | `<symptom-slug>.<mechanism-slug>` |
| `asset.<repo>.<path-hash>` | AI-guidance asset | `<repo-slug>.<first-6-of-path-sha>` |

**Slug rules:** lowercase alphanumeric + hyphens only; strip punctuation; collapse whitespace to hyphen; max 48 chars; truncate at last hyphen boundary. Example: `"Query Engine Web App"` → `query-engine-web-app`.

**ID stability invariant:** once assigned, a stable ID is never recycled for a different subject. If a subject moves, update `00-index/core-map.md`. If a subject splits, mint scoped child IDs (`fk.parent.mechanism-a`, `fk.parent.mechanism-b`) and add a supersession pointer in core-map.

## Canonical ordering

Apply in priority order within any artifact table:

1. **Manifest order** — follow the artifact's defined section sequence from `references/artifact-manifest.md`.
2. **Category enum** — rows within a section sort by the canonical enum order for that column.
3. **Stable ID** — within same category, sort by stable ID lexically ascending.
4. **Evidence priority** — tie-break: `verified/observed` > `source-inferred/declared` > `docs-only` > `suspected ⚠️`.
5. **Lexical path** — final tie-break: source path lexical ascending.

## Decidable predicates

Predicates are evaluated against evidence before any artifact is rendered. Record each predicate evaluation as `predicate-inputs` in the ledger row.

### P1 — Deployable-unit enumeration

A unit is enumerated iff ≥1 of: (a) has a deployment/runtime manifest (container spec, service config, scheduler config); (b) is explicitly exposed as a runnable surface; (c) is referenced as a distinct operational entity in ownership or on-call data. Operational planes (data, control, telemetry) are first-class units even with no app entry point.

### P2 — Concept inclusion

A concept is included in `service/concept-model.md` iff ≥2 of 4 hold: (a) appears in telemetry/log/query dimensions; (b) drives routing/partitioning/ownership/blast-radius; (c) encodes lifecycle/state/process order; (d) is needed to map symptom to root cause/mitigation/owner. Component/repo/deployable names alone do not qualify.

### P3 — Incident-material repo

A repo is incident-material iff ≥1 mined fact meets a trigger: incident-routing | blast-radius | failure-discrimination | ownership/escalation | high-risk operational change. A repo with zero qualifying facts is `not-material` with searched scope. Each `deep/` row cites its `materiality-category`.

### P4 — AI-asset catalog inclusion

An asset is included in `00-index/ai-asset-catalog.md` iff it maps to ≥1 named test from the routing set: `incident-routing | ownership/escalation | observability | failure-discrimination | review-guidance`. Dev-relevant assets are included ONLY when they map to one of the four incident-flavored tests; `review-guidance` alone does not include a dev-relevant asset. Everything else stays floor-only (`kb/<repo>/ai-assets.md`).

### P5 — Source-catalog inclusion

An observability source is included in `observability/source-catalog.md` iff: (a) a telemetry emission name, table name, metric name, or log stream was discovered in repo source, manifests, or docs; or (b) a live probe confirmed the source is reachable. A mention of a source name in a doc is sufficient for a `docs-only` row; it is not sufficient for a `source-inferred/declared` row without a code/config discovery.

### P6 — Failure-signature three-way routing

Route each non-rejected signature family deterministically:

1. `promoted` → dedicated `failure-knowledge/<id>.md` iff ≥1 holds: (a) traced to a confirmed incident record with discriminator evidence; (b) ≥2 independent source signals (for example, telemetry + code, or telemetry + overlay); (c) independently reviewed with grounded evidence.
2. `secondary` → `failure-knowledge/secondary-signatures.md` iff it does not meet promotion but has exactly one reusable key (recurrence OR cross-source mechanism) and an actionable discriminator.
3. `candidate` → `failure-knowledge/candidate-signatures.md` for all other non-rejected patterns with searched scope and verify-later action.

The routing decision is recorded in `failure-knowledge/README.md` and in the evidence ledger through `destination` and `predicate-inputs`; it is not an extra status column in signature files.

### P7 — Open/escalated validity

`open:escalated` is valid iff: (a) the decisive evidence is genuinely unreachable in this run (not just inconvenient); (b) the exact gap is named; (c) a concrete verify-later action is recorded; (d) reachable static paths were tried first; (e) three required proof fields are present: `reachable-static-probes-tried` (list of static probe paths attempted), `attempted-source-classes` (classes of evidence attempted), `why-unreachable` (exact reason decisive evidence is not reachable). `open:escalated` is invalid if same-subject reachable static evidence exists — such rows must be promoted or rejected instead. Naming unverified scope without trying reachable paths does not discharge a row.

### P8 — Artifact-mediated data-flow handoff

Routes an edge to `topology/data-flow-handoffs.md` iff ALL hold:
- (a) **durable producer write** — the producer emits a named durable entity/artifact (table, ingestion/state/terminal record, materialized view, blob, queue/topic message, checkpoint, generated config/script, precomputed dataset), not a synchronous response payload;
- (b) **decoupled consumer read** — a distinct consumer (different unit/repo/scheduled job/process) reads it asynchronously / later;
- (c) **silent-staleness risk** — the consumer reads the entity for DATA/STATE such that a STALE-BUT-PRESENT entity silently produces degraded output with no call-site error. This is DISTINCT from a pure TRIGGER/signal whose ABSENCE halts the consumer observably (no run). Decidability: ✅ a durable blob / ingestion-or-terminal record / materialized view / pipeline-metadata record / precomputed dataset read for data (stale-but-present degrades silently) qualifies; ❌ a completion-queue/event that only TRIGGERS the next stage (absence = observable non-run), a read-through cache resolved within the request, a row written and read inside the SAME request, or a config flag resolved synchronously do NOT (they stay in `service-graph.md`); borderline (a completion message that ALSO carries consumed state, or a compute-enqueue) qualifies ONLY if a stale-but-present payload silently degrades output — static proxy: qualifies iff the cited consumer read consumes the payload's DATA fields into its output with no freshness/validity guard at the read site; if the cited read only acks/dispatches on arrival, reject; if neither is citable, reject with searched scope. A freshness anchor exists or is required to detect the staleness;
- (d) **two-sided evidence ≥ docs-only** — both the producing write and the consuming read are cited; cross-repo name-match alone is insufficient.

Outcomes (decidable, reproducible — mirrors P1–P7):
- (a)–(d) hold and a concrete `traversal-probe` is statically derivable → `promoted` row in `topology/data-flow-handoffs.md`.
- Only one side evidenced (producer XOR consumer) → `open:escalated` under P7 discipline: name the missing side as the gap and carry P7's three proof fields. If P7's "invalid if same-subject reachable static evidence exists" clause fires (the missing side was reachable but unsearched), PROMOTE or REJECT instead — never leave it escalated. Not a promoted row; not rendered in the artifact.
- (a)–(d) hold but no concrete `traversal-probe` is statically derivable → `open:escalated` under the same P7 discipline (named gap: `traversal-probe`; searched scope; verify-later). Not a promoted row. `traversal-probe` stays hard-required; never invent a probe to satisfy the cell.
- No edge passes P8 → the file carries a ledger-derived `none-found (searched scope)` note citing the rejected/searched edges. The file is mandatory (M); emptiness is evidenced, never skipped.

`claim-class` reuses existing `edge/dependency` (no enum churn); destination + `predicate-inputs=P8` distinguish artifact-mediated handoffs from synchronous service-graph edges.

### P-dep-telemetry — Dependency telemetry catalog

A dependency D requires a `observability/dependency-sources.md` row iff (a) D is a promoted row in `topology/dependencies.md` AND (b) ≥1 telemetry coordinate for D — {a server-side table/db/cluster name, a caller-side metric/log emission, or a documented telemetry coordinate} — was discovered in repo source / manifests / docs / dashboards.

Outcomes (decidable, reproducible):
- Both caller-side and server-side coordinates discovered → `promoted` row in `observability/dependency-sources.md`.
- Exactly one side discovered → `open:escalated` ledger row under P7 discipline: name the missing side (`caller-side emission` or `server-side coordinate`) plus P7's three proof fields (`reachable-static-probes-tried`, `attempted-source-classes`, `why-unreachable`). This degrades on cross-owner-unreachable; it does not block the KB.
- No coordinate after searched scope → terminal ledger record with status `rejected` and note `none-found (searched scope)`; no `dependency-sources.md` row.

The artifact is predicate-conditional mandatory: present iff at least one dependency reaches `promoted` or `open:escalated` under this predicate; absent only when all dependencies have terminal `rejected` ledger records carrying `none-found (searched scope)`. Emptiness must be evidenced.

## Render-from-ledger contract

1. **No direct page drafting from exploration.** Scouts produce normalized candidate records. Builders classify records, evaluate predicates, and assign ledger statuses. Only then do builders render artifact tables by projecting `promoted` rows into their declared `destination` using the manifest schema.
2. **Narratives summarize records and cite canonical IDs.** Any prose section in a KB artifact derives from promoted ledger rows; each claim cites the `record-id` or equivalent typed anchor.
3. **Gaps are ledger-derived.** A `none-found` gap note means no record with a matching subject and class reached `promoted` status; it must cite the searched scope from the `rejected` or `open:escalated` rows, not from inference.
4. **Idempotency test.** A re-render from the same locked ledger must produce the same artifact content. If re-rendering changes an artifact, a ledger row changed — that is a mutation event requiring a new ledger version, not silent re-rendering.

## Incremental changed-surface closure

When `mode = incremental`, the input lock includes the prior KB state hash and old repo SHAs. The closure rule determines which records must be re-mined.

### Changed-surface → affected record classes → destination artifacts

| Changed surface | Re-mine record classes | Destination artifacts to re-render |
|---|---|---|
| repo source (entry-points, modules, contracts, invariants) | `edge/dependency · control/auth · observability · concept · failure-mode` | `kb/<repo>/` floor; `topology/`; `topology/data-flow-handoffs.md`; `failure-knowledge/`; `observability/source-catalog.md`; `observability/dependency-sources.md` (+ matching `00-index/telemetry-routing-card.md` / `task-router.md` symptom cross-link) |
| deployment/runtime manifests | `edge/dependency · config/secret · ownership/escalation` | `topology/per-deployable-units.md`; `topology/service-graph.md`; `topology/endpoints-ports-catalog.md`; `topology/data-flow-handoffs.md`; `observability/dependency-sources.md` (+ matching `00-index/telemetry-routing-card.md` / `task-router.md` symptom cross-link) |
| telemetry/observability config | `observability` | `observability/source-catalog.md`; `observability/dependency-sources.md`; `observability/canonical-signals.md`; `observability/join-keys.md` |
| ownership/escalation config | `ownership/escalation` | `service/ownership.md`; `service/access-escalation.md`; `00-index/ownership.toon` |
| incident overlay source (new window) | `overlay` | `overlays/incidents/`; `00-index/incident-clusters.toon` |
| concept/glossary sources | `concept` | `service/concept-model.md`; `service/glossary.md`; `kb/<repo>/concepts.md` |
| docs (human docs only) | `failure-mode · concept · observability` (suspected ⚠️ cap) | `failure-knowledge/`; `service/`; `observability/` incl `observability/dependency-sources.md` (with `docs-only` trust cap) |
| AI-guidance assets | `ai-asset` | `kb/<repo>/ai-assets.md`; `00-index/ai-asset-catalog.md` |
| incident-material human guidance (troubleshooting guides / runbooks / known-issues / alert-response) — re-render only, stays docs-only/non-promotable | `guidance-asset` (pointer-only; docs-only/suspected ⚠️ cap; no promotion) | `kb/<repo>/ai-assets.md`; `00-index/ai-asset-catalog.md` (+ matching `00-index/telemetry-routing-card.md` / `task-router.md` symptom cross-link) |

**Preservation contract:** higher-grade ledger records are preserved unless the new run produces stronger evidence (see `references/kb-mutation.md` for evidence-strength comparison). Re-graded and superseded records carry a mutation status; they are never silently replaced.

**Final KB clean target-state:** no migration breadcrumbs, no stale transitional notes, no `_delta` or `_old` artifacts in the committed deliverable. The delivered KB must read as a current-state document grounded in the locked inputs of this run.
