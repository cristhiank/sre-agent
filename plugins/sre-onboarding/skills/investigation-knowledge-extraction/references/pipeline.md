# Pipeline

<cost_model>
Outputs-only by construction. Read completed run artifacts, compact event logs, current knowledge-base files, and one bounded read-only incident-discussion extract per cohort. Do not re-run telemetry, replay investigations, expand live evidence, or perform expensive verification. The compression step makes cost scale with signature count, not raw incident count.
</cost_model>

<phases>
1. **Snapshot.** Build a frozen cohort from completed runs: report present, grader verdict present, service attribution from content, and no collected marker. Emit a manifest, cursor, and inventory.
2. **Normalize signatures.** Strip deployment/noise flags, normalize volatile ids/timestamps/numbers/scale labels, preserve workflow/controller identity, and cluster by the resulting monitor signature. Record bound monitor/rule ids, useful telemetry tokens, mechanism lexicon hits, verdict mix, owners, services, and sample incidents.
3. **Mine deterministic nuggets.** Produce one compact nugget row per cluster per owning service. Add coverage status against existing failure-knowledge. Covered rows become evidence/recurrence updates; gaps and partials require discriminator narrative.
4. **Deep-mine selected sessions.** From session metrics, choose top exemplars and problem sessions using outcome first and efficiency second. Extract convergence/waste signals: redundant calls, repeated failed searches, missing context reuse, useful versus wasted telemetry paths, and cost/outcome deltas.
5. **Cross-check human RCA.** Run the bounded incident-discussion pull once for the cohort, classify author class, identify post-investigation human RCA candidates, bucket A/B/C, and add iteration-divergence reversals.
6. **Plan merge.** Map every candidate to a target file and action: new failure-knowledge file, update existing file, thematic calibration/config/routing file, index update, changelog entry, or verification queue. Carry merge-gate flags such as single-incident observed-only, hypothesis-only, blocked upstream, cross-service edge label, and verify-later.
7. **Write curated KB.** Apply only approved target-state writes to `failure-knowledge/`, `00-index/`, relevant observability routing surfaces, and the changelog. Preserve existing higher-trust curated facts. Use clean prose, stable ids, citations, trust grades, and no raw sensitive payloads.
8. **Mark collected.** After successful KB writes, add collection markers to the chosen completed runs so future cohorts skip them.
</phases>

<merge_rules>
- Index trust equals the least-confident material claim it points at.
- Cross-service edges must be labeled `dependency`, `look-alike`, or `causal-in-incident:<id>`.
- If proximate evidence is strong but upstream cause is blocked, write `proximate: X; upstream: pending` and keep root-cause trust low.
- Hypothesis-only content becomes a triage discriminator or verification item, not durable root cause.
- Every contested item needs a discriminator, missing evidence, owner, and next read-only verification path.
</merge_rules>

<outputs>
Expected outputs are file-first: cohort manifest/cursor, signature-cluster catalog, nugget catalog, selected deep-mine notes, human-RCA review, merge plan, knowledge-base patches, changelog provenance, verification queue, and collection markers. Return paths and compact summaries rather than large raw extracts.
</outputs>
