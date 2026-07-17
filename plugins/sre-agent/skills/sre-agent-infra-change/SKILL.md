---
name: sre-agent-infra-change
description: >-
  Focused shared-infrastructure and control-plane change causality analysis. Apply when an incident
  has broad, synchronous onset and app-code arrival is disproven for the failing path, leaving an
  infrastructure change as the leading hypothesis. Triggers include certificate, SAN, TLS, secret,
  or credential rotation; DNS, endpoint, or binding changes; config or feature-flag rollout;
  ingress, proxy, forwarder, load-balancer, topology, control-plane, or shared-dependency changes;
  abrupt all-region onset; and "no code change." Operates only on the supplied investigation
  context and evidence sources. Do not activate for single-unit or staggered onset, or when an
  app-code change is arrival-verified in the failing scope.
---

# Specialist: Infra / Control-Plane Change

Run as a focused subagent within an investigation. Fetch only through the evidence sources passed to you, cite observation ids, and write only under your assigned stage directory `4_specialists/infra-change/`. Do not load the coordinator.

Honesty floor: read-only; treat a rotation/config/topology change as a hypothesis until verified against an authoritative control-plane/infra source for that change; cite an observation id for every material claim; surface unreachable verification as an explicit gap; never fabricate.

## Goal

Answer: **did a shared-infrastructure / control-plane change introduce the incident, and did that change take effect in the failing prod scope at or before onset?** This is the edge lens for the onset-signature signal: a broad/synchronous, all-region abrupt onset with app code excluded. A timeline/changepoint capability may flag the onset shape as FAVORING this class; this lens owns the authoritative VERIFICATION and conclusion of it.

## What to look for

- **Competing infra cause classes (pre-register).** Before checking, name the candidate classes and their pre-declared discriminator values: certificate/SAN/TLS rotation vs secret/credential-store rotation vs DNS/endpoint/binding change vs config/feature-flag rollout vs ingress/proxy/LB or topology change vs shared-dependency rotation. Also name the non-infra rivals a code-excluded synchronous onset is EQUALLY consistent with and which this lens does NOT own — downstream/provider outage, shared-dependency throttle, capacity/quota exhaustion, poison-traffic; concluding the infra-rotation class requires POSITIVE authoritative evidence, not elimination of code alone. Keep predicate / expected-favored / expected-rival separate from the checked value.
- **Onset signature promotes this class.** A BROAD-SYNCHRONOUS onset (spread << the service's rolling-deploy/ring cadence; unknown cadence ⇒ treat synchronicity as unverified and carry rotation AND deploy co-leading) with app code excluded (= `arrival=disproven` for every named code change on the failing path) makes a shared-trigger/rotation class leading — it must not be outranked by a per-unit code/deploy change on merge-time correlation alone.
- **Authoritative control-plane state.** Check the reachable authoritative infra/control-plane state: a rotation/push/change event near onset; the PRESENTED artifact vs the EXPECTED (e.g. served cert SAN / thumbprint / validity window, resolved endpoint/binding, effective config/flag value) against what the failing path requires; and whether the change's scope matches the failing population (all-region / all-unit vs a single unit).
- **Cause CLASS vs EXACT ARTIFACT (do not over-block, do not over-promote).** The cause CLASS — "a rotation/config change, not a code change" — is concludable from in-hand onset AND a POSITIVE authoritative control-plane change/rotation event whose scope matches the failing population; onset shape + code-excluded ALONE is elimination-only and caps at proximate/unverified — it does NOT conclude the class. The class conclusion (when positively evidenced) is REACHABLE — do NOT park it as `blocked-unreachable`. Only identifying the EXACT rotated artifact (its specific value/version) may need an out-of-band secret/config-store or infra control-plane capability — that alone is the Manual-Kit item, and the class conclusion stands without it.
- **Post-onset actor neutrality.** A rotate-back / re-issue / rollback / failover performed after onset is a remediation action: it may indicate WHICH artifact responders suspected, but it is NOT causal corroboration and must not lift confidence (the grader applies this as Gate E).

## Useful data to request

- A read-only infra/control-plane state capability: rotation/push/change events with timestamps; current-and-prior values of the implicated cert/secret/binding/endpoint/config/flag; and the served/effective artifact for the failing scope.
- A read-only source-control/release-history capability to EXCLUDE code — establish `arrival=disproven` for every named code change on the failing path (serving-build cut time vs merge target, backport/cherry-pick status).
- A bounded read-only telemetry query capability to bound true onset, enumerate the failing population, and confirm the change's scope matches it.

## How to work

Follow the specialist worker contract: pre-register your discriminator (predicate / expected-favored / expected-rival written and kept visibly separate BEFORE the checked value), cite `OBS###` ids for every material claim, and end with the claim-readiness ledger so the grader adjudicates without re-discovering completeness gaps. Treat the infra change as an arrival-gated INTRODUCTION event, symmetric to a code introduction: verify it took effect in the failing prod scope at/before onset and REPORT a typed `arrival_status` (`verified` | `unverified` | `disproven`) per the grader's change-arrival gate — reachable-but-unfetched arrival provenance is an OPEN obligation, not a conclusion. If no falsifiable discriminator is reachable within budget, say so and cap your own claim at proximate/unverified; never force a weak discriminator to clear the gate.

## Output focus

Write the theory under your stage directory. Name the cause CLASS plus mechanism, the pre-registered discriminator and its checked value, and the `arrival_status` for the infra change event. State explicitly that the cause CLASS is concluded from in-hand evidence while identifying the exact artifact is the out-of-band Manual-Kit item (with the read-only operator-executable check for it). Keep any post-onset rotate-back/rollback as remediation context, never causal support, and record alternatives that remain and any missing discriminator as an explicit gap.
After writing the file-first artifacts, emit the worker brief's bounded
`console_return`; console text never replaces the staged files.
