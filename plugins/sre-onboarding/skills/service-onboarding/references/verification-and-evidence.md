# Verification & Evidence

Size budget: ~170 lines. Canonical home for claim fields, trust labels, `grounding_type`, rule_status/evidence_trust separation, rulebook, and independent completeness audit.

Found != claimed. A fact is promoted only when the right rule is applied or explicitly blocked, evidence is cited, and trust/grounding are labeled.

## Claim-ledger fields

Every promoted claim upgrades the evidence line every doc already writes:

`class | evidence(file:line or typed anchor) | trust-label | grounding_type | rule_status | confidence | blast-radius-if-wrong | verify-later`

Classes: control/auth · config/secret · edge/dependency · failure-mode · concept · observability · ownership/escalation · overlay.

## Trust labels

| Label | Meaning |
|---|---|
| `verified/observed` | live runtime or telemetry signal observed for the claim scope |
| `source-inferred/declared` | read from repo source, manifests, config, or generated declarations; not live-run |
| `docs-only` | from human docs only |
| `suspected ⚠️` | unconfirmed; include how to verify |

A design-time bootstrap with no live signal reaches `source-inferred/declared` at best; never `verified/observed`. `unknown > invented`.

## `grounding_type` axis

`grounding_type` is separate from trust:

- `repo-source`
- `live-telemetry`
- `incident-overlay`
- `monitor-history`
- `docs-only`
- `manual-curated`

Implementation claims require `repo-source` grounding. Service recurrence, ownership/escalation, live schema, monitor history, and incident overlay claims may carry non-repo grounding; they still require typed evidence or they fail. Do not force repo-cell-or-fail where the claim is not an implementation claim. AI-guidance asset rows in the floor `kb/<repo>/ai-assets.md` carry `grounding_type` `docs-only` or `manual-curated`. The `00-index/ai-asset-catalog` is pointer-only and inherits grounding via the `asset-ref` pointer to the floor row; it carries no grounding column of its own.

## Two axes — `rule_status` ⊥ `evidence_trust`

- `rule_status`: `not-applied | applied | blocked`. Did the class rule run against reachable evidence?
- `evidence_trust`: `verified/observed | source-inferred/declared | docs-only | suspected ⚠️`.

Promotion rule: evidence may rise above suspected/presence-only only after `rule_status=applied` or `rule_status=blocked` with the block recorded. `applied` does not imply a live signal. Example: a cold bootstrap can be `rule_status=applied`, `evidence_trust=source-inferred/declared`, `grounding_type=repo-source`.

## Collision map — contract/invariant artifact types

Contracts and invariants are artifact types, not new claim classes. Reuse the existing rulebook rows:

- contract -> edge/dependency | control/auth | config/secret | observability, depending on surface.
- invariant -> failure-mode | control/auth | config/secret | concept, depending on what it constrains.

An invariant enforcement site is verified under the existing control/auth presence-is-not-enforcement rule when it is a control. A contract is verified under edge/dependency or config/secret when its surface is a runtime edge, declaration, or configured value. Deep facts use the existing `trust-label` + `grounding_type`; statically mined contracts/invariants are capped at `source-inferred/declared` unless a live signal justifies `verified/observed`. There is no separate `partial` grade: partial coverage is `suspected ⚠️` plus a verification-queue entry.

## Rulebook rules

**control/auth — presence is not enforcement.** Trace the control to where it is checked on the relevant path. Captured, configured, logged, or registered-but-unbound controls stay suspected.

**config/secret — read values, never echo secrets.** Read values and scan committed history/surfaces. A committed secret is a failure-mode; record location + type only, never the value or reconstructable fragment.

**edge/dependency — closed-world ban.** Enumerate inbound, outbound, ops-managed, runtime-injected, and out-of-app-repo edges. Any absence claim names searched scope and still-unverified scope.

**observability — source names are not live proof.** Source-catalog rows need discovery source, last live verification date or NEVER, trust, `grounding_type`, and consumer warning when source-inferred. That warning must contain `do not conclude absence or a block without live-checking <source> via <join key>` with concrete source and join key filled; placeholders fail.

**failure-mode — required synthesis.** Build discriminator-first signatures/mechanisms from hotspots, rulebook risks, source-catalog gaps, overlays, and audit findings.

**citation — re-resolve.** Every `file:line` or typed anchor is re-resolved after generation. Unresolved evidence is fixed, downgraded to suspected, or moved to verification queue.

**AI-asset (auxiliary) — non-promotable lead.** AI-guidance assets (agent docs, instruction files, skill packs, subagents, chatmodes, prompt files, shared agent reference docs) are docs-only/suspected leads. An `ai-assets.md` row never satisfies cross-layer grounding and never promotes an implementation claim; re-resolve to repo-source or live evidence first and apply the relevant rule before promotion. Stamp freshness to repo SHA and downgrade on drift. Sensitive operational metadata (cluster URLs, subscription/principal/team IDs, GUIDs) is recorded as pointer + sanitized purpose only; raw values only if already allowed in a canonical KB home.

## Closed-world examples

Bad: `Service A has no downstream dependencies.`

Good: `No outbound calls found in repos X/Y and deployment manifests A/B; unverified: runtime service discovery, ops-managed routes, external scheduler jobs.`

Bad: `Telemetry table returned zero rows, so event did not happen.`

Good: `0 rows from source S with filter F/window W; non-dispositive because source is source-inferred and canonical source not live-checked via join key J.`

## Independent completeness audit

The audit is mandatory before done. It runs after the open-thread ledger exists and is performed by an independent, opposite-family review pass that hunts for missing or falsely closed material threads.

The audit samples:
- final KB
- breadth trail and stage packets
- open-thread ledger
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
3. Rule/trust/grounding: promoted claims have `rule_status`, `evidence_trust`, and `grounding_type`; implementation claims have repo-source grounding.
4. Payload grounding: each service-higher promoted implementation fact traces to a repo-lower cell and matching destination payload, or has a valid cited drop/gap.
5. Concept-model: mostly names, missing required categories without searched scope, or unpromoted load-bearing vocabulary fails.
6. Observability: source-inferred rows include NEVER live verification, exact verify-later action, and CONSUMER WARNING content containing the anti-absence clause with concrete source and join key; restricted-source gates are scoped.
7. Clean deliverable: packet contains deliverable-root file tree, allowlist scan, fast-path denylist scan, remediation record, and is clean after remediation.
8. Incremental packet: an `incremental` run produced mutation records and the Clean Deliverable Packet; self-asserted preservation without audit-sampled records fails.
9. Migration completeness: old-layout migration map is complete; no orphaned old-home artifact remains, including root concept/glossary/tracing/state/config/failure/topology artifacts.
10. Repo deep: samples included repos for populated or explicit-`unknown` contracts/invariants with searched scope + verify-later action, and excluded repos for a `not-material` record with searched scope; any populated `deep/` row with no materiality-category is over-extraction. Test-oracles are not produced.
11. Incident route: telemetry-routing-card resolves to CORE observability rows and stable anchors.
12. AI-asset corpus: catalog is pointer-only and non-normative — no ordering/precedence, no "use this first", no `evidence_required`, no `stop_conditions`, no acceptance criteria; every included catalog row carries a `why-included` basis tied to a named materiality test from the routing set `incident-routing | ownership/escalation | observability | failure-discrimination | review-guidance` and the anti-authority marker; the dev-tag inclusion threshold is honored — a dev-tagged asset is included only when it maps to one of the four incident-flavored tests (`review-guidance` alone does not include a dev-relevant asset); floor `ai-assets.md` is non-promotable (no row used as cross-layer grounding for an implementation claim); empty repos record `none-found (searched scope)`; a capability-gap + `glob-fallback = partial-coverage` note is recorded when the AI-guidance-asset discovery capability was unavailable.

Valid audit output records auditor identity/pass, inspected artifacts, findings, and closure decision. Missing or self-graded audit does not count.
