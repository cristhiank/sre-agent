# Workflow — deterministic phases

Canonical home for first-time/incremental flow, CORE-first ordering, stage guardrails, open-thread lifecycle, and Clean Deliverable Packet. Schemas and artifact statuses: `references/artifact-manifest.md`. Reproducibility contract, input lock, evidence ledger, stable IDs, and predicates: `references/reproducibility-contract.md`.

## Mode selection

Select up front:

- `first-time` — build the repo-lower floor and incident-primary CORE from scratch; lock inputs before scouting.
- `incremental` — repo SHAs advanced; follow `kb-mutation.md`, re-mine only changed surfaces, and preserve curated/promoted/higher-grade facts.

Both modes are read-only over source and live systems.

## Stage packets block progression

Each required stage emits a packet. Missing packet = missing stage = hard block. Do not proceed by saying the work was implicit, self-reviewed, or captured in final prose; either receive/cite/merge the packet or write an explicit degraded/abort note before stopping.

## CORE-first ordering

Build CORE areas before any lens view or `task-router` projection:

1. repo-lower floor: `kb/<repo>/entry-points.md`, `modules.md`, `concepts.md`, and incident-material `deep/` contracts/invariants per `kb-layout.md`.
2. CORE areas: `service/`, `topology/`, `observability/`, `failure-knowledge/`.
3. incident `telemetry-routing-card` produced now as a route view over CORE observability.
4. index/seam: `00-index/task-router`, `00-index/core-map`, restored candidate `.toon` catalogs.
5. overlays and contributions shape.
6. Clean Deliverable Packet and independent audit.

Incident routing-card is authored now because incident is the primary lens. Review/dev are lens-ready seams only and are not authored by this skill.

## Phase 0 — Input/run lock + capability map

Record the input/run lock before any scouting (schema: `references/reproducibility-contract.md`). Store as `_run-lock.yaml` in the transient run-root; never in the committed KB.

Create a CAPABILITY MAP: `capability | what it does | match to this onboarding | stage served | action-or-gap`. Record reachable evidence classes: repo inventory, code search, build/declaration surfaces, docs, read-only telemetry, incident history, monitor history, durable memory, and safe local commands.

A source is unavailable only after a confirmed probe: canonical invocation attempted, non-guessed target, and auth/setup failure distinguished from genuine absence. A failed live probe never licenses skipping reachable static mining.

Mine version-control history and build a code-graph/symbol index in the transient run-root outside the deliverable. Use it for discovery and verification; final KB citations resolve to durable source paths or repo-lower cells, not run-root artifacts. Hotspots seed `failure-knowledge/` candidates and verification queue entries.

## Phase 1 — Broad cheap inventory → normalized candidate records

Scouts enumerate sources and emit normalized candidate records (not classified conclusions). Every discovered fact is a candidate record with `claim-class`, `normalized-subject`, `evidence`, and `trust-label = suspected ⚠️` until rules are applied. Scouts do not classify; builders classify.

- Exhaustively enumerate declaration surfaces: build/package/workspace manifests, deployment/runtime configs, routing/scheduling artifacts, infrastructure declarations, ops automation, and docs that declare runnable/operated units.
- Classify materiality: runnable, deployable, externally reachable, scheduled, privileged, persistent, or operationally authoritative.
- Probe via entry points, public contracts, telemetry/log field names, and deployment manifests; avoid blind recursive summaries.
- Enumerate repos and SHAs under `repos/`; classify deployable-binary vs library.
- Enumerate exposed surfaces: HTTP, RPC, queues/topics, timers, workers, background processes, storage triggers, and ops-managed routes.
- Spot telemetry emission names, join keys, dimensions, query/recipe corpus, dashboards/recipes, state/config/secret surfaces, owners/escalation, and docs.
- For each telemetry source, mine its ATTRIBUTION SCHEMA: the identity/scope keys (failing-population binding), the per-component/per-operation fields (component/logger/category + operation/action, including keys inside any structured-properties field), and severity/exception fields — plus a recipe for per-component/per-operation composition (which component is active at a window) and full-window per-group counts, not only a total-by-path count. Request/inbound fields alone cannot attribute a resource/saturation symptom decoupled from demand.
- Extract domain vocabulary into repo-lower `concepts.md`: identity/tenancy/partition, correlation/trace, lifecycle/state, topology, routing, capacity, policy/config.
- Mine deep candidates from declaration surfaces: API/schema/event/queue declarations and validation/guard/enforcement code. Apply the incident-material predicate (P3) from `references/reproducibility-contract.md` before trust labeling; partial coverage is `suspected ⚠️` plus a verification-queue entry.
- Enumerate the guidance-asset corpus under `repos/` via a read-only local discovery capability; degrade to a bounded filename/dir glob if that capability is absent. Cover **two classes**: AI-guidance assets (agent docs, instruction files, skill packs, subagents, chatmodes, prompt files, shared agent reference docs) AND **incident-material human-written guidance** (troubleshooting guides / runbooks / known-issues / alert-response docs). For the human-guidance class apply the two-stage structural materiality filter + symptom→doc forcing function from `references/dispatch.md §Guidance-asset discovery+triage` so the sweep stays material, not a doc dump. Pointer-only — never copy doc content or values. Bound the sweep to `repos/`; record provenance per asset (repo + path + SHA). Capability-gated and degradable: record a capability-gap note plus `glob-fallback = partial-coverage` marker when the capability is unavailable. Only the specialized discovery *capability* is non-hard: if it is unavailable, a documented glob-fallback plus `none-found (searched scope)` is acceptable and not a hard fail. Producing the floor/catalog rows for whatever WAS discovered — with the required symptom/trigger phrase + the matching `00-index/telemetry-routing-card.md` (and/or `task-router.md`) cross-link for human-guidance rows — IS gated (done-gate + mechanical audit). Corpus definition: `references/verification-and-evidence.md §Guidance-asset (auxiliary)`.

## Phase 2 — Evidence normalization/reconciliation → ledger

- Assign stable IDs to all enumerated subjects using the algorithm in `references/reproducibility-contract.md`.
- Evaluate decidable predicates (P1–P7) against candidate records; update `status` and `predicate-inputs`.
- Confirm repo-inventory completeness.
- Enumerate material units and operational planes from declaration surfaces. Planes are first-class even with no app entry point.
- Reconcile every candidate: `promoted | non-material/duplicate/stale (cited) | left open (missing evidence + why)`.
- Model `repo != build artifact != deployable unit != runtime process != exposed surface`.
- Open the open-thread ledger; rank evidence quality.
- Produce the per-repo incident-material roll-up: `incident-material(yes/no) | trigger-category (if yes) | searched-scope`. Excluded repos get a `non-material` ledger row with searched scope.

### Open-thread ledger lifecycle

Each row: `thread | why-material | class | current-state | searched-scope | next-action | terminal-state`.

Terminal states:
- `promoted` — predicate applied/blocked, evidence cited, trust/grounding justified, stable ID assigned, destination recorded.
- `rejected` — investigated and unsupported; searched scope cited.
- `open:escalated` — missing evidence; surface/owner, exact verify-later action, and reachability reason named.

`open:escalated` is valid only when decisive evidence is unreachable in this run (P7 from `references/reproducibility-contract.md`). Try reachable static paths first. Naming unverified scope does not discharge a row.

## Phase 3 — Edge/dependency and seam discovery → records

- Enumerate exposed edges first, then search for references to those stable surfaces.
- Classify every candidate edge: observed-runtime-call | deploy/config reference | test-only | docs-only | generated/unused-candidate. Emit one ledger record per classified edge.
- Distinguish embedded-library from runtime service-to-service calls.
- Enumerate outbound, ops-managed, runtime-injected, out-of-app-repo, scheduled, and webhook/plugin edges as hard as inbound.
- Discover seams beyond direct calls: shared identity, ops-managed/out-of-app-repo edges, secret-injection, shared infrastructure, consequence-ownership, and artifact-mediated data-flow handoffs (a producer writes a durable entity — table, ingestion/state/terminal record, materialized view, blob, queue/topic message, checkpoint, generated config/script, precomputed dataset — that a decoupled consumer reads later, so a STALE-BUT-PRESENT entity silently degrades the consumer with no call-site error). Evaluate P8 to route artifact-mediated handoffs to `topology/data-flow-handoffs.md`; synchronous request/response edges and pure triggers stay in `service-graph.md`. If a change in A can break B, it is a seam even without direct call topology.
- Join via stable surface identifiers and deployment-manifest verification; never by name matching alone.

## Phase 4 — Artifact rendering from manifest

**Render from ledger, not from exploration.** Builders project `promoted` ledger rows into their declared `destination` using schemas from `references/artifact-manifest.md`. No KB artifact is drafted directly from exploration prose; all tables are filled from ledger records.

Build repo-lower first, then derive service-higher CORE:
- `service/`: identity, ownership, access/escalation, support boundaries, concept-model/glossary.
- `topology/`: service-graph, endpoints/ports catalog, dependencies, blast-radius, deployable-unit coverage matrix, per-deployable-units, data-flow-handoffs.
- `observability/`: source-catalog, join-keys, canonical-signals, restricted-sources.
- `failure-knowledge/`: discriminator-first signatures rendered by P6 three-way routing — promoted variable-slot files, `secondary-signatures.md`, and `candidate-signatures.md`.

Cross-layer grounding is mandatory for implementation claims. Facts discovered while drafting service-higher land in repo-lower first, then promote up. Narratives summarize and cite canonical ledger record IDs.

Also render AI-guidance asset products: per-repo `kb/<repo>/ai-assets.md` (inventory + triaged-lead rows, non-promotable, capped at `docs-only`/`suspected ⚠️`, sensitive values pointer-only) and `00-index/ai-asset-catalog` (pointer-only, multi-consumer tags, `why-included` basis tied to a named materiality test per P4, anti-authority header). Floor and catalog schemas: `references/artifact-manifest.md`; catalog-inclusion rule: `references/dispatch.md`.

## Phase 5 — Verification/enrichment + mechanical audit

Apply `verification-and-evidence.md` rulebook. Re-resolve citations. Verify telemetry live when reachable; otherwise create source-catalog rows with `last live verification = NEVER`, trust `source-inferred/declared`, exact verify-later action, and a CONSUMER WARNING whose anti-absence clause names the source and join key.

Fold in sanitized 90-day `overlays/incidents/` priors when reachable. Priors influence suspicion and routing; they do not redefine CORE truth. Priors older than the window roll off unless re-verified by a refreshing source change.

**Mechanical reproducibility audit** (runs before done, distinct from the independent completeness audit): execute decidable checklist A–I from `references/verification-and-evidence.md §Mechanical reproducibility audit`. All items must PASS before proceeding.

Write concise, cited, versioned CORE artifacts, the produced-now incident `telemetry-routing-card`, and index/seam files. Initialize `contributions/` shape only. Avoid empty ceremonial folders; incident-material repo `deep/` contracts/invariants must be populated per P3, or gap records (`open:escalated`) recorded in `00-index/evidence-ledger.toon` when evidence is unreachable.

After rendering all artifacts, write `00-index/evidence-ledger.toon`: copy all terminal ledger records (promoted, rejected, non-material, stale, open:escalated, sensitive-unsafe, superseded, duplicate), filling all schema fields, redacting secrets and restricted payloads. Set the `@meta` provenance lock summary block. Write the `@run-trail` block (dispatch-mode + one row per executed stage: `stage | worker-role | packet-id-or-hash | searched-scope | merge-status | outcome`) and the `@audit` block (auditor identity/capability, non-builder attestation, sampled artifacts, findings, closure). **Both `@run-trail` and `@audit` must be committed to this file BEFORE any transient scratch or `_work/` directories are deleted.** This committed artifact is required for incremental determinism.

## Phase 6 has been merged into Phases 4 and 5

KB generation (former Phase 6) is now integral to Phase 4 (render from manifest) and Phase 5 (verify + audit). There is no separate generation step; rendering happens directly from the ledger.

## Clean Deliverable Packet (hard gate before done)

Deliverable root is `services/<service>/`. Before final, write a packet the independent audit can sample. It contains:

**Pre-condition (hard gate):** commit `@run-trail` and `@audit` blocks into `00-index/evidence-ledger.toon` BEFORE deleting any transient scratch files or `_work/` directories. Deleting the only audit/dispatch evidence before it is committed to the evidence ledger is a HARD FAIL.

1. Literal file-tree listing of the deliverable root.
2. Allowlist scan result: every entry under the deliverable root is a durable KB artifact or is marked `found` and remediated. Novel scratch names such as codegraph-like, symbol-index-like, or run-root-like directories fail even if they are not dot-prefixed.
3. Fast-path denylist scan result, one line per entry: `_work/`, `_onboarding/`, `scouts/`, `.onboarding/`, any dot-prefixed scratch dir, loose `ledger.md`, loose `audit.md` -> `found` or `clean`.
4. Remediation record for every `found`: moved/deleted before final, with destination if moved.
5. Confirmation that no KB file links into the transient run-root.
6. Freshness/provenance location in the committed KB.

If any non-durable deliverable-root entry remains, the run is not done. It may not be waived as a gap.

## Closure checkpoint

No trusted KB until:
- every required stage packet exists or an explicit degraded/abort note stops the run
- input lock recorded and all fields complete
- every material thread is terminal
- every promoted claim has rule_status, trust label, grounding_type, confidence, evidence, and stable ID
- source-catalog warnings contain the required anti-absence clause with source/join key filled, and restricted-sources gates pass
- incident telemetry-routing-card is produced from CORE observability
- incident-material repos have populated `deep/` contracts/invariants (P3), or `open:escalated` gap records in committed `00-index/evidence-ledger.toon` with verify-later action
- `00-index/evidence-ledger.toon` committed with provenance lock summary, `@run-trail`, `@audit`, and all terminal records
- mechanical reproducibility audit passed (Phase 5)
- Clean Deliverable Packet is clean after remediation
- independent completeness audit runs and persists findings
- Guidance-asset corpus (AI-guidance + incident-material human-written guidance) is discovered + cataloged, or per-repo `none-found (searched scope)` plus capability-gap/partial-coverage note recorded; `none-found` is valid only after the widened human-guidance scope was searched too, and human-guidance catalog entries carry a symptom/trigger phrase + the matching `00-index/telemetry-routing-card.md` (and/or `task-router.md`) cross-link

## Anti-naive guardrails

- No encyclopedias, but no responder-useful omissions. Use progressive disclosure.
- Text-search hit is not a dependency; missing hit is not no dependency.
- Look-alike signals/limits/keys can be distinct mechanisms.
- Auth/control is not a graph node by default, but auth/identity failures are first-class failure modes.
- Human docs are lead evidence, not authority.
- Be clever, not brute-force: entry points -> exposed edges -> stable cross-references -> classified claims.
