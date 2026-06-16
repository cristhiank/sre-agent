# Workflow — cheap -> expensive phases

Size budget: ~210 lines. Canonical home for first-time/incremental flow, CORE-first ordering, stage guardrails, open-thread lifecycle, and Clean Deliverable Packet.

## Mode selection

Select up front:

- `first-time` — build the repo-lower floor and incident-primary CORE from scratch.
- `incremental` — repo SHAs advanced; follow `kb-mutation.md`, re-mine only changed surfaces, and preserve curated/promoted/higher-grade facts.

Both modes are read-only over source and live systems.

## Capability map + reachability gate

Before scouting, create a CAPABILITY MAP: `capability | what it does | match to this onboarding | stage served | action-or-gap`. Record reachable evidence classes: repo inventory, code search, build/declaration surfaces, docs, read-only telemetry, incident history, monitor history, durable memory, and safe local commands.

A source is unavailable only after a confirmed probe: canonical invocation attempted, non-guessed target, and auth/setup failure distinguished from genuine absence. A failed live probe never licenses skipping reachable static mining.

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

## Phase 0 — Mechanical substrate

Mine version-control history and build a code-graph/symbol index in the transient run-root outside the deliverable. Use it for discovery and verification; final KB citations resolve to durable source paths or repo-lower cells, not run-root artifacts.

Hotspots seed `failure-knowledge/` candidates and verification queue entries.

## Phase 1 — Broad cheap inventory

- Exhaustively enumerate declaration surfaces: build/package/workspace manifests, deployment/runtime configs, routing/scheduling artifacts, infrastructure declarations, ops automation, and docs that declare runnable/operated units.
- Classify materiality: runnable, deployable, externally reachable, scheduled, privileged, persistent, or operationally authoritative.
- Probe via entry points, public contracts, telemetry/log field names, and deployment manifests; avoid blind recursive summaries.
- Enumerate repos and SHAs under `repos/`; classify deployable-binary vs library.
- Enumerate exposed surfaces: HTTP, RPC, queues/topics, timers, workers, background processes, storage triggers, and ops-managed routes.
- Spot telemetry emission names, join keys, dimensions, query/recipe corpus, dashboards/recipes, state/config/secret surfaces, owners/escalation, and docs.
- Extract domain vocabulary into repo-lower `concepts.md`: identity/tenancy/partition, correlation/trace, lifecycle/state, topology, routing, capacity, policy/config.
- Mine deep candidates from declaration surfaces: API/schema/event/queue declarations and validation/guard/enforcement code. Apply the `kb-layout.md` incident-material predicate and `verification-and-evidence.md` rulebook before trust labeling; deep rows use trust-label + grounding_type, and partial coverage is `suspected ⚠️` plus a verification-queue entry.
- Enumerate the AI-guidance asset corpus (agent docs, instruction files, skill packs, subagents, chatmodes, prompt files, shared agent reference docs) under `repos/` via a read-only local AI-guidance-asset discovery capability; degrade to a bounded filename/dir glob if that capability is absent. Bound the sweep to `repos/` and to known asset filenames/dirs; record provenance per asset (repo + path + SHA). Capability-gated and degradable: if the capability is unavailable, record a capability-gap note plus a `glob-fallback = partial-coverage` marker and continue core onboarding. This sweep is never a hard done-gate.

## Phase 2 — Evidence normalization and inventory reconciliation

- Confirm repo-inventory completeness.
- Enumerate material units and operational planes from declaration surfaces. Planes are first-class even with no app entry point.
- Reconcile every candidate: represented | non-material/duplicate/inactive (cited) | left open (missing evidence + why).
- Model `repo != build artifact != deployable unit != runtime process != exposed surface`.
- Rank evidence quality and open the open-thread ledger.
- Produce the per-repo incident-material roll-up: `incident-material(yes/no) | trigger-category (if yes) | searched-scope`. Excluded repos get a `not-material` rejected-style ledger row with searched scope.

### Open-thread ledger lifecycle

Each row: `thread | why-material | class | current state | searched scope | next action | terminal state`.

Terminal states:
- `promoted` — rule applied/blocked, evidence cited, trust/grounding justified, destination recorded.
- `rejected` — investigated and unsupported; searched scope cited.
- `open:escalated` — missing evidence, surface/owner, exact verify-later action, and reachability reason named.

`open:escalated` is valid only when decisive evidence is unreachable in this run. Try reachable static paths first. Naming unverified scope does not discharge a row.

## Phase 3 — Edge/dependency and seam discovery

- Enumerate exposed edges first, then search for references to those stable surfaces.
- Classify every candidate edge: observed-runtime-call | deploy/config reference | test-only | docs-only | generated/unused-candidate.
- Distinguish embedded-library from runtime service-to-service calls.
- Enumerate outbound, ops-managed, runtime-injected, out-of-app-repo, scheduled, and webhook/plugin edges as hard as inbound.
- Discover seams beyond direct calls: shared identity, ops-managed/out-of-app-repo edges, secret-injection, shared infrastructure, and consequence-ownership. If a change in A can break B, it is a seam even without direct call topology.
- Join via stable surface identifiers and deployment-manifest verification; never by name matching alone.

## Phase 4 — Operational and CORE synthesis

Build repo-lower first, then derive service-higher CORE:
- `service/`: identity, ownership, access/escalation, support boundaries, concept-model/glossary.
- `topology/`: service-graph, endpoints/ports catalog, dependencies, blast-radius, deployable-unit coverage matrix, seam/blast-radius notes from shared identity, ops-managed/out-of-app-repo edges, secret-injection, shared infrastructure, and consequence-ownership.
- `observability/`: source-catalog, join-keys, canonical-signals, restricted-sources, dashboards/recipes.
- `failure-knowledge/`: discriminator-first signatures and mechanisms.

Cross-layer grounding is mandatory for implementation claims. Facts discovered while drafting service-higher land in repo-lower first, then promote up.

## Phase 5 — Verification and enrichment

Apply `verification-and-evidence.md` rulebook. Re-resolve citations. Verify telemetry live when reachable; otherwise create source-catalog rows with `last live verification = NEVER`, trust `source-inferred/declared`, exact verify-later action, and a CONSUMER WARNING whose anti-absence clause names the source and join key.

Fold in sanitized 90-day `overlays/incidents/` priors when reachable. Priors influence suspicion and routing; they do not redefine CORE truth. Priors older than the window roll off unless re-verified by a refreshing source change.

## Phase 6 — KB generation

Write concise, cited, versioned CORE artifacts, the produced-now incident `telemetry-routing-card`, and index/seam files. Initialize `contributions/` shape only. Avoid empty ceremonial folders; incident-material repo `deep/` contracts/invariants must be populated or explicit `unknown` with searched scope + verify-later action.

Also write the AI-guidance asset products: per-repo `kb/<repo>/ai-assets.md` (inventory + triaged-lead split rows, non-promotable, capped at `docs-only`/`suspected ⚠️`, sensitive values pointer-only) and `00-index/ai-asset-catalog` (pointer-only, multi-consumer tags, `why-included` basis tied to a named materiality test, anti-authority header, freshness=repo SHA). Cross-link incident-relevant catalog entries FROM `00-index/telemetry-routing-card` and `failure-knowledge/` as SECONDARY "human-authored playbook lead" pointers — pointer only, never a new canonical fact. Floor and catalog schemas: `kb-layout.md`; catalog-inclusion rule: `dispatch.md`.

## Clean Deliverable Packet (hard gate before done)

Deliverable root is `services/<service>/`. Before final, write a packet the independent audit can sample. It contains:

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
- every material thread is terminal
- every promoted claim has rule_status, trust label, grounding_type, confidence, and evidence
- source-catalog warnings contain the required anti-absence clause with source/join key filled, and restricted-sources gates pass
- incident telemetry-routing-card is produced from CORE observability
- incident-material repos have populated or explicit-`unknown` `deep/` contracts/invariants with searched scope + verify-later action
- deployable-unit coverage matrix is present
- Clean Deliverable Packet is clean after remediation
- independent completeness audit runs and persists findings, including incremental mutation/migration evidence when mode is `incremental`
- AI-asset corpus is discovered + cataloged via the AI-guidance-asset discovery capability, OR per-repo `none-found (searched scope)` plus a capability-gap/partial-coverage note recorded when the capability was unavailable; capability-gated, not a hard blocker

## Anti-naive guardrails

- No encyclopedias, but no responder-useful omissions. Use progressive disclosure.
- Text-search hit is not a dependency; missing hit is not no dependency.
- Look-alike signals/limits/keys can be distinct mechanisms.
- Auth/control is not a graph node by default, but auth/identity failures are first-class failure modes.
- Human docs are lead evidence, not authority.
- Be clever, not brute-force: entry points -> exposed edges -> stable cross-references -> classified claims.
