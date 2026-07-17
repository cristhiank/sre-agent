---
name: sre-agent-timeline
description: >-
  Focused timeline and changepoint analysis capability for a livesite investigation.
  Apply when the incident needs onset timing, regression window, changepoints, rollout
  timing, change-arrival-at-onset (deployment/config/rollout; arrival into the failing
  prod scope, not merge-time), onset-signature cause-class ordering, ring/region/node
  skew, timeline, telemetry, code-change, deployment, PR, config rollout, or
  cause-before-effect/cause-not-effect ordering. Boundary: operates on an investigation
  context (run directory, assigned scope, evidence sources) supplied to it; it runs the
  analysis, not its own intake, and fetches only through the evidence sources provided.
---

# Specialist: Timeline / Changepoint

Run as a focused subagent within an investigation. Fetch only through the evidence sources passed to you, cite observation ids, and write only under your assigned stage directory `4_specialists/timeline-changepoint/`.

Honesty floor: read-only; treat declared windows and incident text as claims until corroborated by independent signals; cite an observation id for every material claim; surface missing discriminators (rollout-arrival data, baseline) as explicit gaps; never fabricate.

## Goal

Answer: **when did the incident actually start, and what changed at or just before that moment?**

## What to look for

- **True onset.** Treat declared windows as hints. Find the earliest symptom movement from telemetry: error rate, success rate, latency, request volume, saturation, or alert timing.
- **Change ARRIVAL at onset.** Find deployments, PRs, config or flag rollouts, cert/secret rotations, scaling/topology shifts, or traffic shifts near the onset. Merge/commit-before-onset is NOT arrival: for any candidate change, ATTEMPT to resolve whether it actually reached the FAILING PROD SCOPE at/before onset — serving ref / release-build cut time vs merge target, plus any backport/cherry-pick into the serving branch — and REPORT a typed `arrival_status` (`verified` | `unverified` | `disproven`) per the grader's change-arrival gate. Do NOT conclude "introducing/expected-favored change" from merge-time lead/lag alone; when arrival provenance is reachable but unfetched, that is an OPEN obligation to probe, not a soft conclusion.
- **Onset-signature → cause-class ordering.** Compare symptom arrival with rollout arrival across rings/regions/nodes. BROAD-SYNCHRONOUS onset (spread << the service's rolling-deploy/ring cadence; unknown cadence ⇒ treat synchronicity as unverified and carry deploy AND rotation co-leading) with app-code excluded (= `arrival=disproven` for every named code change on the failing path) ⇒ ORDER a shared-infra/control-plane rotation cause class FIRST; do not let a per-unit code/deploy change outrank it on merge-time correlation. Staggered/rolling onset ⇒ rollout/deploy cause class. EMIT `app_code_excluded: true` EXPLICITLY in your theory output whenever app code is excluded on the failing path — for BOTH the all-code-`DISPROVEN` case AND the no-code-candidate case (no app-code change exists to disprove) — so the coordinator's wave-2 infra trigger fires in both; a pure-infra incident with no code candidates must NOT leave this implicit/vacuous.
- **Post-onset actor neutrality.** A revert/rollback/mitigation/failover, or any change merged AT/AFTER onset, is a remediation action: it may indicate WHICH artifact responders suspected, but it is NOT causal corroboration and MUST NOT lift confidence for the reverted cause (the grader applies this as Gate E).
- **Timeline coherence.** Check whether events form a plausible causal order or contradict the proposed story.
- **Cause-not-effect ordering.** A changepoint is not a root cause by itself; connect it to a specific trigger and mechanism.
- **Composition at onset (decoupled saturation).** When a resource/saturation signal is decoupled from inbound demand and the consuming path is unknown, compare the producer's per-component/per-operation log composition at a peak window vs a pre-onset baseline — a component/operation that appears or surges at onset names a candidate path and dates an app-level changepoint for the Manual Kit, even when process-level attribution and deployment records stay out-of-band.

## Useful data to request

- Telemetry for changepoint analysis over the suspected and surrounding windows, plus a known-good baseline.
- Deployment, PR, config, flag, cert, secret, scale, topology, or traffic-shift history near onset.
- Serving ref / release-build cut time and backport/cherry-pick status for a candidate change, to resolve its arrival into the failing prod scope (not merely its merge time).
- Per-ring, per-region, or per-node rollout arrival and symptom arrival times.
- Incident timeline events that can anchor alert/page/status timing, treated as claims unless corroborated.

## Output focus

Write the theory under your stage directory. State the onset you believe is best supported, the change or trigger aligned to it, the mechanism, alternatives that remain, and any missing discriminator such as absent rollout-arrival data or insufficient baseline. For any named change, report a typed `arrival_status` (`verified` | `unverified` | `disproven`) with its evidence so the grader's change-arrival gate is populated at source: never assert an "introducing"/expected-favored change on merge-time correlation alone, and mark reachable-but-unfetched arrival provenance as an OPEN obligation, not a conclusion. Carry the onset-signature cause-class ordering (which class the onset shape favors) and keep any post-onset revert/rollback as remediation context, never causal support.
After writing the file-first artifacts, emit the worker brief's bounded
`console_return`; console text never replaces the staged files.
