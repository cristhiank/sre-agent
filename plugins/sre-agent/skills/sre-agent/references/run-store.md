# Run Store

The investigator persists each run as markdown under one folder. Shared honesty rules live
in [investigation-invariants.md](investigation-invariants.md); stage expectations live in
[artifact-contracts.md](artifact-contracts.md).

## Location and id

`<run-root>/<run-id>/`. Resolve `run-root` in this priority order, then resolve to an
absolute path before dispatch:
1. An explicit run-root supplied by the coordinator or caller (e.g. in the task prompt) — use it verbatim.
2. **If `SRE_AGENT_WORK_ROOT` is set in the environment, the run-root is exactly `<SRE_AGENT_WORK_ROOT>/.investigations/`.** Do not search for or infer a `.investigations/` directory anywhere else (an ancestor or the workhome). This keeps concurrent and automated runs isolated under their own work root.
3. Otherwise, default to `.investigations/` at the working root.

`run-id`: `inv-<source><incidentId>-<UTCyyyymmddThhmmssZ>`.
`<source>` is a short incident host tag; the UTC suffix disambiguates reruns.

## Reading prior runs (iteration mode only)

By default a run neither reads nor reuses any other run; each run is self-contained.
Reading prior run dirs is permitted ONLY when the task explicitly authorizes it for a
new-information iteration (see [followup.md](followup.md)), and then only within these
limits:
- **Scope:** run dirs directly under THIS run's already-resolved run-root. Never
  traverse to an ancestor, the workhome, a different work root, or another incident's
  run-root.
- **Resolution is unchanged:** never scan for, infer, or switch run-root to "find" prior
  runs; read only within the run-root resolved by the priority order above.
- **Read-only, never resume:** never write into, append to, or modify a prior run dir;
  always mint a fresh `run-id` and write only there.
- **Carry-forward is by reference:** cite a prior run's observation as
  `<priorRunId>/OBS###` (bare `OBS###` is unique only within one run); prior observations
  and verdicts are claims, re-verified per the honesty floor, not imported as truth.

## Layout

```text
<run-id>/
  run.md                         # index: identity, scope, status, pointers, gaps
  1_intake/                      # captured incident claims and pointers
  2_scout/                       # surfaces, hypotheses, discriminating questions, gaps
  3_evidence/                    # merged observations, index, timeline
    observations/                # factual observations keyed by OBS###
  4_specialists/<name>/          # that specialist's theory notes and local artifacts
  5_grader/                      # ranking, verdict rationale, optional follow-up
  6_report/                      # bounded investigation report (+ live post when authorized)
  7_knowledge/                   # durable knowledge candidates (only when triage finds novelty)
```

Do not rename the stage directories.

## `run.md` and observations

`run.md` is the compact index/status page: incident identity, bounded scope, stage
completion, artifact pointers, open questions, and gaps.

`run.md` also carries a compact `model_tiering` record — the single canonical schema
home for tiering. Recorded per dispatch: role · chosen tier · the role's default tier ·
the model actually used · whether the preferred tier was available · escalation reason
when above the role default (claim-gated reasoning | synthesis | high-ambiguity) ·
fallback reason + selector (harness-limited | coordinator-choice | model-unavailable |
override) when they differ. This makes silent fallback and above-default escalation
visible; it does not imply the coordinator can use a tier the host does not expose.

The coordinator tracks open questions and gaps there in whatever compact form is clear.
A CAPABILITY MAP lives in `run.md` or `1_intake/capability-map.md`; per capability,
record capability, match, stage, and action-or-gap. Keep it small.
Stable `OBS###` ids are the citation key. Once assigned, do not reuse or silently rewrite
them; material claims in later stages cite those ids.

## Write isolation

Each subagent writes only its own stage directory. The coordinator owns `run.md` and the
merged `3_evidence/` observations, index, and timeline.
If a stage is partial, record the limitation in `run.md` and keep the gap visible rather
than fabricating downstream certainty.