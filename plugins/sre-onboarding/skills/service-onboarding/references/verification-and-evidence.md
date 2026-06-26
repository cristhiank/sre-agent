# Verification & Evidence

Canonical home for claim fields, trust labels, `grounding_type`, rule_status/trust-label separation, rulebook, mechanical reproducibility audit, and independent completeness audit. Evidence ledger row schema, stable ID algorithm, and status/drop-reason taxonomy: `references/reproducibility-contract.md`.

Found != claimed. A fact is promoted only when the right rule is applied or explicitly blocked, evidence is cited, and trust/grounding are labeled.

## Evidence ledger and drop-reason taxonomy

The normalized evidence ledger is the canonical data structure for all promoted claims. Schema and full status/drop-reason taxonomy (`promoted · duplicate · stale · non-material · sensitive-unsafe · superseded · open:escalated · rejected`): `references/reproducibility-contract.md`. Builders fill artifact schemas from ledger rows; never draft KB pages directly from exploration.

## Claim-ledger fields

Every promoted claim upgrades the evidence line every doc already writes:

`class | evidence(file:line or typed anchor) | trust-label | grounding_type | rule_status | confidence | blast-radius-if-wrong | verify-later`

Claim classes: `references/reproducibility-contract.md §Canonical enums`.

## Trust labels

Canonical value set: `references/reproducibility-contract.md §Canonical enums`.

| Label | Meaning |
|---|---|
| `verified/observed` | live runtime or telemetry signal observed for the claim scope |
| `source-inferred/declared` | read from repo source, manifests, config, or generated declarations; not live-run |
| `docs-only` | from human docs only |
| `suspected ⚠️` | unconfirmed; include how to verify |

A design-time bootstrap with no live signal reaches `source-inferred/declared` at best; never `verified/observed`. `unknown > invented`.

## `grounding_type` axis

`grounding_type` is separate from trust. Canonical values: `references/reproducibility-contract.md §Canonical enums`.

Implementation claims require `repo-source` grounding. Service recurrence, ownership/escalation, live schema, monitor history, and incident overlay claims may carry non-repo grounding; they still require typed evidence or they fail. Do not force repo-cell-or-fail where the claim is not an implementation claim. AI-guidance asset rows in the floor `kb/<repo>/ai-assets.md` carry `grounding_type` `docs-only` or `manual-curated`. The `00-index/ai-asset-catalog` is pointer-only and inherits grounding via the `asset-ref` pointer to the floor row; it carries no grounding column of its own.

## Two axes — `rule_status` ⊥ `trust-label`

Canonical values for both axes: `references/reproducibility-contract.md §Canonical enums`.

- `rule_status`: did the class rule run against reachable evidence?
- `trust-label`: what evidence strength supports the claim?

Promotion rule: evidence may rise above suspected/presence-only only after `rule_status=applied` or `rule_status=blocked` with the block recorded. `applied` does not imply a live signal. Example: a cold bootstrap can be `rule_status=applied`, `trust-label=source-inferred/declared`, `grounding_type=repo-source`.

## Collision map — contract/invariant artifact types

Contracts and invariants are artifact types, not new claim classes. Reuse the existing rulebook rows:

- contract -> edge/dependency | control/auth | config/secret | observability, depending on surface.
- invariant -> failure-mode | control/auth | config/secret | concept, depending on what it constrains.

An invariant enforcement site is verified under the existing control/auth presence-is-not-enforcement rule when it is a control. A contract is verified under edge/dependency or config/secret when its surface is a runtime edge, declaration, or configured value. Deep facts use the existing `trust-label` + `grounding_type`; statically mined contracts/invariants are capped at `source-inferred/declared` unless a live signal justifies `verified/observed`. There is no separate `partial` grade: partial coverage is `suspected ⚠️` plus a verification-queue entry.

## Rulebook rules

**control/auth — presence is not enforcement.** Trace the control to where it is checked on the relevant path. Captured, configured, logged, or registered-but-unbound controls stay suspected.

**config/secret — read values, never echo secrets.** Read values and scan committed history/surfaces. A committed secret is a failure-mode; record location + type only, never the value or reconstructable fragment.

**edge/dependency — closed-world ban.** Enumerate inbound, outbound, ops-managed, runtime-injected, and out-of-app-repo edges. Any absence claim names searched scope and still-unverified scope.

**observability — source names are not live proof.** Source-catalog rows need discovery source, last live verification date or NEVER, trust, `grounding_type`, and consumer warning when source-inferred. That warning must contain `do not conclude absence or a block without live-checking <source> via <join key>` with concrete source and join key filled; placeholders fail. The row must also capture the source's attribution schema — identity/scope keys, per-component/per-operation fields (including keys inside any structured-properties field), correlation/trace keys structurally carried by the source, and a recipe for per-component/per-operation composition and full-window per-group counts — so a resource/saturation symptom decoupled from demand can be attributed; a source that CAN carry per-component/per-operation attribution but captured only request/inbound dimensions is under-mined, not done; a promoted row that omits a structurally carried correlation/trace key is under-mined, not done; if `service/concept-model.md` category `identity/tenancy/partition-key` does not resolve which concrete principal denotes the service-under-onboarding versus callers, affected observability rows are under-mined, not done. A source that structurally emits only request/inbound fields records that limit with rationale and is not thereby under-mined.

**failure-mode — required synthesis.** Build discriminator-first signatures/mechanisms from hotspots, rulebook risks, source-catalog gaps, overlays, and audit findings.

**citation — re-resolve.** Every `file:line` or typed anchor is re-resolved after generation. Unresolved evidence is fixed, downgraded to suspected, or moved to verification queue.

**Guidance-asset (auxiliary) — non-promotable lead.** Guidance assets are docs-only/suspected leads. The corpus is two classes: **AI-guidance assets** (agent docs, instruction files, skill packs, subagents, chatmodes, prompt files, shared agent reference docs) AND **incident-material human-written guidance** (troubleshooting guides / runbooks / known-issues / alert-response docs). This rule is the single canonical home of the corpus definition; other files reference it. A guidance-asset row never satisfies cross-layer grounding and never promotes an implementation claim; re-resolve to repo-source or live evidence first and apply the relevant rule before promotion. Stamp freshness to repo SHA and downgrade on drift. These are **pointer-only — the catalog records a pointer + symptom/trigger, never copied doc content or values.** Sensitive operational metadata (cluster URLs, subscription/principal/team IDs, GUIDs) is always recorded as pointer + sanitized purpose only; concrete non-sensitive source names may appear only when evidence-cited and necessary.

## Closed-world examples

Bad: `Service A has no downstream dependencies.`

Good: `No outbound calls found in repos X/Y and deployment manifests A/B; unverified: runtime service discovery, ops-managed routes, external scheduler jobs.`

Bad: `Telemetry table returned zero rows, so event did not happen.`

Good: `0 rows from source S with filter F/window W; non-dispositive because source is source-inferred and canonical source not live-checked via join key J.`

## Mechanical reproducibility audit

Runs first, before done; distinct from the independent completeness audit. Each item has a scope, procedure, and explicit PASS/FAIL condition. All items (A–L) must PASS before the done gate clears. Canonical enum values referenced below: `references/reproducibility-contract.md §Canonical enums`.

### A. Artifact presence & scope

**Scope:** all manifest-declared artifacts (`references/artifact-manifest.md`) + `kb/**/deep/*.toon`.  
**Procedure:** for every M/G/P/V artifact, check presence and predicate outcome against ledger evidence. Record the count of files scanned and the explicit file-glob scope.  
**PASS:** every M artifact FILE is present (a `none-found`/gap note is content within that file about what was found — it is never a substitute for the file's existence); every P artifact is present iff its predicate outcome is true (gap record in `00-index/evidence-ledger.toon` required for inconclusive predicates), including `observability/dependency-sources.md` present iff P-dep-telemetry has any `promoted` or `open:escalated` outcome and absent only when all dependencies have terminal `rejected` ledger records carrying `none-found (searched scope)`; every G artifact present or has a capability-gap note; every V artifact present for each passing promotion predicate; no extra transient or scratch artifacts in deliverable root; `kb/**/deep/*.toon` included in audit scope; file count and glob scope reported.
**FAIL:** any M artifact FILE absent; any P artifact present for a false/inconclusive predicate without a gap record; `observability/dependency-sources.md` absent when P-dep-telemetry has a `promoted` or `open:escalated` outcome, or present when all dependencies have terminal `rejected` ledger records carrying `none-found (searched scope)`; any G artifact absent without a capability-gap note; any V artifact missing for a true predicate; any transient/scratch artifact in the deliverable root; deep/*.toon excluded from audit scope.

### B. Schema arity

**Scope:** all artifacts incl `kb/**/deep/*.toon` — every markdown table and every TOON `@data` row.  
**Procedure:** count cells in each markdown table row vs header column count; count fields in each TOON `@data` row vs `@schema` column count.  
**PASS:** every row's cell/field count equals the header/schema count.  
**FAIL:** any row with wrong arity (too few or too many). Report `file:line` of each violation. A column-shifted row (e.g., merged cell pushing data right) is a FAIL.

### C. Enum conformance

**Scope:** all artifacts incl `kb/**/deep/*.toon`.  
**Procedure:** for every `trust-label`, `grounding_type`, `claim-class`, `status`, and `rule_status` cell in any table or TOON row, check membership in the canonical set defined in `references/reproducibility-contract.md §Canonical enums`. Layer words (`floor`, `core`, `gap`, `floor-promotion`) used as `grounding_type` values and any `*-doc` or `code+*` compound tokens are not in the canonical set.  
**PASS:** every enum-column value is a member of its canonical set.  
**FAIL:** any non-canonical value. Report `file:line` and the unknown value.

### D. Cross-layer grounding + promote-up coverage

**Scope:** service-higher artifacts (`service/`, `topology/`, `observability/`, `failure-knowledge/`) and `@promote-up` records in the ledger.  
**Procedure:** for every service-higher implementation row: (1) verify it cites a `kb/<repo>/…` floor cell; (2) verify there is a matching `@promote-up` entry (`source_cell -> destination_cell -> payload`) in the ledger, OR a cited drop/gap in `00-index/evidence-ledger.toon`. Count source implementation rows vs matching promote-up+gap total.  
**PASS:** every service-higher implementation row cites a floor cell; promote-up count + gap count equals source implementation row count (no silent under-coverage).  
**FAIL:** any service-higher implementation row citing raw `docs/`, `sources/`, or a non-floor path directly; any row with neither a matching `@promote-up` nor a cited drop/gap; counts do not match.

### E. Escalation earned

**Scope:** `00-index/evidence-ledger.toon` — every row with `status = open:escalated`.  
**Procedure:** for each `open:escalated` row: (1) verify proof fields `reachable-static-probes-tried`, `attempted-source-classes`, and `why-unreachable` are present and non-empty; (2) verify no same-subject static evidence is both reachable AND dispositive — i.e., sufficient to settle (promote or reject) the specific claim without additional live evidence. Non-implementation claims (live-evidence recurrence, ownership, monitor history) are judged against whether the reachable static evidence actually settles THAT specific claim, not just whether static evidence on the subject exists.  
**PASS:** every `open:escalated` row has all three proof fields; no `open:escalated` row has same-subject dispositive static evidence reachable in-repo.  
**FAIL:** any `open:escalated` row missing a proof field; any `open:escalated` row for which same-subject static evidence is reachable AND dispositive for that specific claim (must be promoted or rejected instead).

### F. Single-canonical-home consistency

**Scope:** all KB artifacts — evidence ledger, deep rows, topology tables, endpoints catalog, README open-thread summary, `@promote-up` records.  
**Procedure:** group all fact occurrences by `record-id`/normalized subject/surface across the full artifact set. For each group with multiple occurrences: verify exactly one is the canonical home record; verify all others are pointer references (`see <canonical-home>:<stable-id>`) with MATCHING status/trust/grounding. Verify README open-thread summary matches the ledger's terminal `open:escalated` set exactly.  
**PASS:** each fact has exactly one canonical home; all cross-references are pointers with matching fields; README open-thread summary equals the ledger's terminal `open:escalated` set.  
**FAIL:** any fact with two or more non-pointer occurrences; any derived copy with differing status/trust/grounding; README open-thread summary differs from the ledger's `open:escalated` set.

### G. Global sanitization

**Scope:** EVERY committed KB artifact under `services/<service>/` — not only `kb/<repo>/ai-assets.md` and overlays.  
**Procedure:** scan all files for sensitive operational metadata — application/client/tenant/subscription IDs, GUID principals, secrets/tokens, chat/collaboration/portal/webhook URLs, raw operational endpoint URLs. For each match: verify it is a pointer + sanitized purpose only. Non-sensitive concrete source names (telemetry table names, log stream names, join key names) remain allowed when evidence-cited and necessary for incident fidelity.  
**PASS:** no raw sensitive operational metadata value in any committed KB artifact; every flagged pattern is a pointer + sanitized purpose.  
**FAIL:** any raw application/client/tenant/subscription ID, GUID principal, secret/token, or chat/collaboration/portal/webhook URL in any committed artifact. Report `file:line`.

### H. Run-trail & audit present

**Scope:** `00-index/evidence-ledger.toon`.  
**Procedure:** verify (1) `@run-trail` block exists with a dispatch-decision sub-record containing all 5 fields (`dispatch-available | dispatch-required | used | packet-evidence | degraded-reason`), a `dispatch-mode` value, and ≥1 stage row (`stage | worker-role | packet-id-or-hash | searched-scope | merge-status | outcome`); (2) `@audit` block exists with all required fields (`auditor-identity/capability | non-builder-attestation | sampled-artifacts | findings | closure`); (3) auditor identity differs from builder/orchestrator identities in `@run-trail`; (4) `non-builder-attestation=yes`; (5) `sampled-artifacts`, `findings`, and `closure` are non-empty; (6) dispatch-decision does NOT show `dispatch-available=yes AND dispatch-required=yes AND used=no` — this combination is a HARD FAIL regardless of the `dispatch-mode` value recorded.  
**PASS:** both blocks exist and are well-formed; dispatch-decision sub-record present with all 5 fields; no ABORT-condition (`dispatch-available=yes AND dispatch-required=yes AND used=no`) present; auditor ≠ builder/orchestrator; all required sub-fields are non-empty.  
**FAIL:** either block absent; dispatch-decision sub-record missing or any of its 5 fields absent; dispatch-decision shows `dispatch-available=yes AND dispatch-required=yes AND used=no`; any `@audit` field missing or empty; auditor matches a builder or orchestrator; `non-builder-attestation=no`. Self-graded audit does not count.

### I. Live-mode honesty

**Scope:** `00-index/evidence-ledger.toon @meta` capability-map-summary; all KB artifacts using capability/live state words.  
**Procedure:** for every capability-map-summary entry and every live/capability state reference in KB artifacts: verify the value is from the canonical vocabulary (`references/reproducibility-contract.md §Live-capability vocabulary`): `disabled-by-scope | attempted-unreachable | reachable-snapshot`. For any `attempted-unreachable` entry: verify a probe-result is recorded (capability name, canonical target, auth-vs-absence result).  
**PASS:** every capability state uses canonical vocabulary; every `attempted-unreachable` entry has a recorded bounded probe result.  
**FAIL:** bare `unreachable` or `unknown` without probe proof; any non-canonical state word; any `attempted-unreachable` entry without a probe-result record.

### J. Predicate inputs recorded

**Scope:** `00-index/evidence-ledger.toon` — all rows with `status = promoted`.  
**Procedure:** for every promoted row, verify the `predicate-inputs` field is non-empty and names at least one predicate from `references/reproducibility-contract.md §Decidable predicates`.  
**PASS:** every promoted row has a non-empty `predicate-inputs` field naming at least one predicate.  
**FAIL:** any promoted row with an empty or absent `predicate-inputs` field.

### K. Canonical ordering & stable-ID format

**Scope:** all artifact tables and `00-index/evidence-ledger.toon` data rows.  
**Procedure:** (1) verify each artifact table is sorted per the canonical ordering rules from `references/reproducibility-contract.md §Canonical ordering` (manifest order → category enum → stable ID → evidence priority → lexical path); (2) verify every stable ID matches a recognized prefix+slug pattern from `references/reproducibility-contract.md §Stable ID algorithm` (`svc.` / `repo.` / `topo.unit.` / `topo.edge.` / `obs.source.` / `obs.signal.` / `fk.` / `asset.`) followed by a lowercase-alphanumeric-hyphen slug; (3) verify no stable ID is a bare integer or appears to derive from row-position order.  
**PASS:** all tables sorted per canonical ordering; all stable IDs match a recognized prefix+slug pattern; no integer-sequence IDs present.  
**FAIL:** any table with rows out of canonical order; any stable ID that is a bare integer or lacks a recognized prefix; any ID that appears to derive from row order rather than subject normalization.

### L. Freshness-header format

**Scope:** root `README.md`, each `kb/<repo>/README.md`, `observability/source-catalog.md`, and any artifact that includes a freshness/provenance header.
**Procedure:** verify each header is one italicized line matching `references/artifact-manifest.md §Freshness / provenance header` exactly: `_Freshness: scan date=<...>; live-verification posture=<verified-live|source-inferred|NEVER>; repo table=<repo | branch | SHA rows or pointer>; last live verification=<date|NEVER>; independent-audit verdict=<...>; open/escalated thread summary=<...>; verification-queue pointer=<...>; stale-risk markers=<...>; overlay window + roll-off date + sanitization statement=<...|N/A>_`. Check the fixed field order, the leading `_Freshness: ` and trailing `_` wrapper, and the `live-verification posture` enum.
**PASS:** every scoped header uses the fixed field order, wrapper, and one of `verified-live`, `source-inferred`, or `NEVER` for `live-verification posture`.
**FAIL:** any scoped header missing the `_Freshness: … _` wrapper, using fields out of order or renamed fields, omitting a required field, or using a non-enum `live-verification posture`.

---

## Independent completeness audit

The audit is mandatory before done. It runs after the open-thread ledger exists and is performed by an independent, opposite-family review pass that hunts for missing or falsely closed material threads.

The audit samples:
- final KB
- breadth trail and stage packets
- open-thread ledger and `00-index/evidence-ledger.toon` (committed)
- claim ledger entries
- repo incident-material roll-up records, including `not-material` exclusions with searched scope
- repo `deep/` contracts/invariants for incident-material repos
- promote-up records
- `kb-mutation` mutation records when mode is `incremental`
- old-layout migration map when an existing KB predates this layout
- Clean Deliverable Packet
- observability source-catalog consumer-warning content and restricted-sources map
- per-repo `kb/<repo>/ai-assets.md` floor and `00-index/ai-asset-catalog`

Audit checks:
1. Inventory first: omitted material unit/plane invalidates the rest.
2. Ledger: every material thread is `promoted`, `rejected`, or `open:escalated`; no reachable static action remains untried for an escalation.
3. Rule/trust/grounding: promoted claims have `rule_status`, `trust-label`, and `grounding_type`; implementation claims have repo-source grounding.
4. Payload grounding: each service-higher promoted implementation fact traces to a repo-lower cell and matching destination payload, or has a valid cited drop/gap.
5. Concept-model: mostly names, missing required categories without searched scope, or unpromoted load-bearing vocabulary fails.
6. Observability: source-inferred rows include NEVER live verification, exact verify-later action, and CONSUMER WARNING content containing the anti-absence clause with concrete source and join key; restricted-source gates are scoped.
7. Clean deliverable: packet contains deliverable-root file tree, allowlist scan, fast-path denylist scan, remediation record, and is clean after remediation.
8. Incremental packet: an `incremental` run produced mutation records and the Clean Deliverable Packet; self-asserted preservation without audit-sampled records fails.
9. Migration completeness: old-layout migration map is complete; no orphaned old-home artifact remains, including root concept/glossary/tracing/state/config/failure/topology artifacts.
10. Repo deep: samples included repos for populated `deep/` contracts/invariants (P3=incident-material) or `open:escalated`/`non-material` gap records in `00-index/evidence-ledger.toon`; samples excluded repos for a `deep/not-material.md` record; any populated `deep/` row with no materiality-category is over-extraction. Test-oracles are not produced.
11. Incident route: telemetry-routing-card resolves to CORE observability rows and stable anchors.
12. Guidance-asset corpus: catalog is pointer-only and non-normative — no ordering/precedence, no "use this first", no `evidence_required`, no `stop_conditions`, no acceptance criteria; every included catalog row carries a `why-included` basis tied to a named materiality test from the routing set `incident-routing | ownership/escalation | observability | failure-discrimination | review-guidance` and the anti-authority marker; the dev-tag inclusion threshold is honored — a dev-tagged asset is included only when it maps to one of the four incident-flavored tests (`review-guidance` alone does not include a dev-relevant asset); floor `ai-assets.md` is non-promotable (no row used as cross-layer grounding for an implementation claim); empty repos record `none-found (searched scope)`, honest only after the widened human-guidance scope was searched too; a capability-gap + `glob-fallback = partial-coverage` note is recorded when the discovery capability was unavailable; every human-guidance row carries a symptom/trigger phrase AND the matching `00-index/telemetry-routing-card.md` (and/or `task-router.md`) cross-link (a human-guidance row missing either fails).

Valid audit output records auditor identity/pass, inspected artifacts, findings, and closure decision. Missing or self-graded audit does not count.
