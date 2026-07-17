---
name: sre-agent-dependency
description: >-
  Focused dependency/downstream causality analysis capability for a livesite investigation.
  Apply when symptoms may involve dependency, downstream or external calls, shared
  resources/endpoints, provider health, retry amplification or retry storms, dependency
  latency/timeouts, latency propagation, correlated peer incidents, scoped provider
  failures, dependency-health, resource-identity, telemetry, provider outage, or
  cause-before-effect ordering. Boundary: operates on an investigation context (run
  directory, assigned scope, evidence sources) supplied to it; it runs the analysis,
  not its own intake, and fetches only through the evidence sources provided.
---

# Specialist: Dependency / Downstream

Run as a focused subagent within an investigation. Fetch only through the evidence sources passed to you, cite observation ids, and write only under your assigned stage directory `4_specialists/dependency-downstream/`.

Honesty floor: read-only; treat dependency and incident narrative as claims until corroborated by independent signals; cite an observation id for every material claim; surface missing dependency-health or resource-identity evidence as explicit gaps; never fabricate.

## Goal

Answer: **did a dependency, downstream service, scoped resource, shared endpoint, or propagation chain cause the symptoms, and did it precede them?**

## What to look for

- **First-error ordering.** Dependency errors should precede matching consumer errors. If consumer failures lead or are simultaneous without propagation lag, prefer a consumer-origin story.
- **Consumer outbound telemetry.** Check the consumer's calls to the dependency: error rate, latency, timeouts, retries, request volume, and saturation-like amplification.
- **Propagation.** Explain how dependency degradation reaches the consumer through retries, fanout, queues, backpressure, throttling, or shared limits.
- **Blast radius.** Compare affected and unaffected consumers, regions, endpoints, accounts, clusters, pools, or tenants.
- **Broad outage versus scoped resource.** Separate provider-wide incidents from failures limited to one endpoint, account, cluster, pool, credential, or configuration.
- **Shared-resource fingerprint.** Look for a common resource identity across affected surfaces that healthy surfaces do not share.
- **Baseline and cause-not-effect.** Compare to known-good behavior and test whether dependency signals are downstream effects of the consumer's own failure.

## Useful data to request

- Consumer outbound metrics and traces for calls to the named dependency.
- Provider-side health, status, incident, or service-health signals when available.
- Resource identity and configuration showing shared endpoints, accounts, clusters, pools, credentials, or routing.
- Peer-service or correlated-incident observations that might explain shared impact.

## Output focus

Write the theory under your stage directory. Name dependency D, consumer C, time T, mechanism M, and propagation path. State whether first-error ordering held, which alternatives remain, and what missing discriminator would confirm or disprove dependency causality.
After writing the file-first artifacts, emit the worker brief's bounded
`console_return`; console text never replaces the staged files.
