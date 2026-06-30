# Nugget Types

<contract>
Mine compact, provenance-carrying nuggets. Each row cites source incident ids, chosen investigation id, grader verdict, trust grade, and target knowledge-base file. Use a compact pipe-delimited or equivalent file-first format; keep long narrative in the target Markdown file.
</contract>

<types>
| Type | Captures | Primary target |
|---|---|---|
| `failure_signature` | symptom → normalized signature → discriminator → mechanism → owner → mitigation → telemetry → trust | `failure-knowledge/*.md` |
| `investigation_routing` | when observed signal X appears, route to owner/task/look-alike Y | `00-index/task-router.md` |
| `config_precondition` | latent non-failure rules that alter behavior or visibility | `failure-knowledge/` and `config-gotchas.md` |
| `telemetry_useful` | signal/source, what it discriminates, and how to bind it | `observability/source-catalog` and `00-index/telemetry-routing-card.md` |
| `telemetry_wasted` | dead-end queries, missing columns, unreachable facets, or non-localizing paths | `observability/restricted-sources` and telemetry routing |
| `monitor_calibration` | expected or noisy firings, preview/low-denominator/min-sample issues, benign patterns | `monitor-calibration.md` |
| `owner_escalation` | team/component/scenario routing learned from incidents | `00-index/ownership.toon` |
| `recurrence_cluster` | recurring signatures, frequency, seasonality, and storm-deduped counts | `00-index/incident-clusters.toon` |
| `investigation_efficiency` | why runs converged or wasted work, from run events and outcomes | `overlays/` and reviewer feedback |
| `agent_vs_human_rca` | investigation RCA compared with later human RCA; wrong/incomplete/over-confident corrections | `failure-knowledge/*.md`, `monitor-calibration.md`, trust promotion/demotion |
</types>

<row_shape>
Recommended compact columns: `id|type|service|title|statement|discriminator|telemetry|owner|action|evidence_incidents|verdict|trust|refs|kb_target`. Escape delimiters or keep prose fields one-line. Split multi-values with a stable list separator. Stable ids are service-prefixed kebab slugs derived from the normalized signature and strongest binding evidence.
</row_shape>

<coverage>
For every signature cluster, classify current KB coverage as `covered`, `partial`, or `gap` by matching stable evidence tokens, useful telemetry names, and signature keywords. Covered clusters become recurrence/evidence updates. Partial and gap clusters require a narrative discriminator before merge. Hypothesis-only or blocked root-cause rows are triage discriminators, not durable failure signatures.
</coverage>
