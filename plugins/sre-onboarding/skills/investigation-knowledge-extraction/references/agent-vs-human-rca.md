# Agent-vs-Human RCA Cross-Check

<purpose>
Correct wrong, incomplete, or over-confident investigation RCAs by comparing completed investigation outputs with later human RCA statements from the incident discussion. This is the highest-value enrichment step and the only step that reads outside the run artifacts.
</purpose>

<inputs>
- Completed cohort manifest with chosen investigation timestamp, report path, verdict, services, and collection state.
- One bounded, read-only incident-discussion extract for the cohort.
- Optional multi-investigation history for detecting verdict changes.
</inputs>

<author_classification>
Classify discussion entries as `bot`, `ai-assisted`, or `human` using author and text heuristics. Treat classification as evidence, not authority. A human RCA candidate must be substantive and contain a mechanism statement; manually reject routing-only comments, acknowledgements, pasted logs, or query dumps that accidentally match mechanism words.
</author_classification>

<bucket_logic>
For each incident with a human RCA candidate:

- **A — investigation ahead.** Investigation mechanism is stronger or earlier than the human note. Do not demote; use the human note only as optional confirmation if it agrees.
- **B — human diverges.** Human mechanism conflicts with or materially completes the investigation mechanism. Treat the human RCA as the gold source for curation. Record the wrong mechanism, correct mechanism, and discriminator that separates them.
- **C — agree.** Investigation and human RCA describe the same mechanism. Promote trust when the source is independent and substantive.

Also flag **iteration-divergence**: multiple investigations for one incident whose grader verdict changed. A changed verdict is an over-confidence reversal signal even without a human RCA.
</bucket_logic>

<trust_gating>
- `verified`: storm-deduped evidence from at least two distinct incidents, code/change rooted evidence, or human-confirmed mechanism.
- `observed`: one incident with coherent report evidence.
- `contested`: credible conflicting mechanisms remain. Carry both hypotheses, assert neither, and write a verification-queue item with the missing discriminator.

Storm-dedup before quoting counts: repeated firings from the same underlying state within a short window count once. Trust for indexes must be no stronger than the weakest load-bearing claim in the target prose.
</trust_gating>

<benign_reclassification>
When a human source confirms a firing is expected or benign, remove or demote it from failure clusters and write a monitor-calibration record instead. Examples of benign classes: test or synthetic tenant, conditional scheduling skip, intentionally disabled path, sparse/low-volume variance, or expected preview behavior. Recalculate recurrence and trust after the demotion.
</benign_reclassification>

<clean_target_state>
Knowledge-base prose states the corrected mechanism and discriminator neutrally. Put human author names, verbatim discussion text, and timestamps only in the changelog/provenance entry. Do not leave migration breadcrumbs such as "previously thought" in consumer-facing failure prose; the target should read as if curated correctly from the start.
</clean_target_state>
