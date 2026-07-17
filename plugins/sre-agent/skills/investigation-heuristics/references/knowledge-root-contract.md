# Knowledge-root contract

This skill is a **content-free consumer**. It ships no learnings; it reads them at runtime from a resolved
cross-service knowledge root. All actual learning content lives outside this (public) skill.

## Resolution order

1. `SRE_KNOWLEDGE_ROOT` environment variable (absolute path to the knowledge root) — authoritative.
2. A workspace `sre-knowledge/` directory, if present.
3. This skill's relative fallback.

Report the resolved source + path in one line, or report that no root was found. **A missing root is the
normal case** for anyone using this skill outside the SRE knowledge environment — degrade to a silent
no-op, never an error.

## Expected file

Under the resolved root: `learnings/sre-learnings.md` — a single compact file of phase-tagged interrogative
heuristics. Its first content line carries a contract-version marker:

```
<!-- sre-learnings contract-version: 1 -->
```

## Version handling

- Version **recognized** → consume the file.
- Version **missing or unrecognized** → emit one diagnostic line and skip (do not guess a shape). This is a
  producer/consumer drift signal, not normal public usage.

## Degradation matrix

| Condition | Behavior |
|---|---|
| Root not found (env unset, no fallback) | Silent no-op — contribute nothing, continue. |
| Root found, file missing/empty/unparseable | ONE visible diagnostic line, then continue. |
| Root found, `contract-version` unrecognized | ONE visible diagnostic line, then continue. |
| Root found, file valid | Consume and apply. |

## Boundary

This skill only READS the file. It never writes learnings, never runs the producer pipeline, and never
depends on the producer at runtime — a pure read-only data dependency across a versioned contract.
