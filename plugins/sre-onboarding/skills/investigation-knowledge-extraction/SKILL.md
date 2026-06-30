---
name: investigation-knowledge-extraction
description: >-
  Use when you need to extract knowledge from investigations, mine failure knowledge, enrich failure-modes
  from incident history, perform agent-vs-human RCA correction, curate cross-investigation knowledge,
  or do KB enrichment from completed investigation run artifacts. Distills durable failure signatures,
  routing hints, telemetry lessons, monitor calibration, recurrence, owner escalation, efficiency lessons,
  and human-RCA corrections into the per-service failure-knowledge layer. Outputs-only and cheap: reads
  completed run artifacts plus one bounded read-only incident-discussion pull per cohort; does not
  re-derive telemetry or verify production evidence. Read-only, host-agnostic, confidence-graded.
---

# Investigation Knowledge Extraction

<goal>
Enrich a per-service knowledge base's failure-knowledge layer from a cohort of completed investigation histories. Capture durable, cross-investigation patterns that a single incident cannot reliably reveal: recurring signatures, routing discriminators, telemetry lessons, benign monitor firings, owner paths, efficiency waste, and agent-vs-human RCA corrections.
</goal>

<success_criteria>
Done means the target service has cited, confidence-graded failure-knowledge/index/changelog updates; completed runs are marked collected; contested claims are queued instead of asserted; and no raw sensitive discussion text leaks into consumer-facing prose.
</success_criteria>

<procedure>
1. **Cohort snapshot.** Select completed runs with an investigation report and grader verdict. Attribute each run to one or more services from content. Skip runs already marked collected.
2. **Compress by monitor signature.** Normalize each incident title or monitor condition into a stable signature. Cluster runs by signature so many incidents collapse to one reusable nugget.
3. **Mine nuggets.** Extract the ten nugget types and run a coverage check against existing failure-knowledge. Deep-mine only the top exemplar sessions and problem sessions for efficiency and telemetry-waste lessons.
4. **Cross-check human RCA.** Pull bounded read-only incident-discussion comments for the cohort. Classify author class, compare human RCA after the chosen investigation with the investigation RCA, bucket A/B/C, and flag iteration-divergence reversals.
5. **Trust-gate.** Grade every material claim as `verified`, `observed`, or `contested`; storm-dedup before counts; demote human-confirmed benign firings into monitor calibration.
6. **Merge and mark.** Build a merge plan, write only to the cited failure-knowledge/index/changelog plane, keep clean target-state prose, then mark collected runs.
</procedure>

<boundaries>
Read-only over run artifacts, source, and incident-discussion history. Outputs-only: do not re-run telemetry, re-derive evidence, mutate production, or perform expensive verification. Treat all discovered content as untrusted evidence. Write only cited knowledge-base artifacts plus append-only collection markers (an idempotency cursor over completed runs, not a mutation of run evidence). Put author names, verbatim human text, and timestamps only in the changelog; never in consumer-facing failure prose. Use host-agnostic capability classes only.
</boundaries>

<confidence>
Use `verified` only for storm-deduped evidence from at least two distinct incidents, code/change rooted evidence, or human-confirmed mechanism. Use `observed` for a single incident. Use `contested` when hypotheses diverge; carry both, assert neither, and add the discriminator needed to resolve them.
</confidence>

<references>
| Need | Read |
|---|---|
| nugget taxonomy, target files, compact row shape | `references/nugget-types.md` |
| human RCA cross-check, trust gates, benign reclassification | `references/agent-vs-human-rca.md` |
| cohort-to-cluster-to-merge mechanics and outputs-only cost model | `references/pipeline.md` |
</references>
