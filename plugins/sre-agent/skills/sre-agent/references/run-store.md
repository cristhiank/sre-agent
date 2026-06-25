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
home for tiering. Recorded per dispatch: role · chosen capability class · the role's
default class · the advertised models seen at resolution + the resolved model actually
used and the basis (`latest-in-advertised-list` | `host-default-fallback` |
`capability-step-down`) · context-window tier
(`standard` | `large-context` | `no-override-exposed`) · whether the preferred tier was
available · escalation reason when above the role default (claim-gated reasoning |
synthesis | high-ambiguity) · fallback reason + selector (harness-limited |
coordinator-choice | model-unavailable | override) when they differ. For each
Specialist dispatch it ALSO records the two-layer resolution inputs: the skill's
declared `default_class` and `minimum_allowed_class` (from its `model_affinity`
contract), the Scout `model_demand` tag, and the resolved `effective_class` with
its basis (`skill-default` | `scout-escalated` | `coordinator-escalated` |
`grader-escalated` | `contract-fault-failsafe`), so an over- or under-tiered
specialist and any fail-safe-to-heavy contract fault are visible. The model is
selected by capability class and newest-stable-generation-within-one-family from the
dispatch tool's advertised list, never a hardcoded name; resolve once and set it on the
actual dispatch call. This makes silent fallback, above-default escalation, and
stale-generation or standard-window subagent picks visible; it does not imply the
coordinator can use a tier the host does not expose.

<bad-example>
The dispatch tool advertises `claude-sonnet-4.6` and `claude-sonnet-4.5`; the coordinator dispatches Scout on `claude-sonnet-4.5`.
Wrong: 4.5 is not the newest generation in the fast/economical class that the list advertises — selecting it is name-from-memory anchoring, not list resolution.
Correct: resolve from the advertised list and pick the newest stable Sonnet it advertises (`claude-sonnet-4.6` for THIS list; pick a newer Sonnet if one is advertised later), record `basis=latest-in-advertised-list`. Likewise, a heavy-read Scout left on the standard window when the host exposes a large-context tier is a defaulted-window miss — set the large-context parameter on that dispatch, not just in the record.
</bad-example>

<bad-example>
A specialist skill declares `model_affinity: {default_class: mid, minimum_allowed_class: mid}`, the Scout tags its hypothesis `model_demand: high` (contradictory multi-source evidence), but the coordinator dispatches it on the mid class to save cost.
Wrong: `model_demand: high` projects to the reasoning-heavy class and is upward-only, so `effective_class = max(mid, reasoning-heavy)` resolves to reasoning-heavy; mid is below the resolved class. Equally wrong is treating a missing, malformed, stale, or invalid `model_affinity` as license to pick any class — that case fails safe to reasoning-heavy.
Correct: resolve `effective_class = max(default_class, model_demand→class, coordinator_risk_override)`, never below `minimum_allowed_class` or below `default_class`, and record the basis (here `scout-escalated`).
</bad-example>

The coordinator tracks open questions and gaps there in whatever compact form is clear. When a known-issue acceleration path is taken or considered (per the Grader's known-issue decision rule in [grading-rubric.md](grading-rubric.md)), `run.md` records a compact line: the leading candidate's source asset/capability, the dispositive `OBS###` id(s) that closed its discriminator, and the settle-or-fail-open reason — so the acceleration is auditable and a wrong shortcut is visible.
A CAPABILITY MAP lives in `run.md` or `1_intake/capability-map.md`; per capability,
record capability, match, stage, and action-or-gap. Keep it small.
Stable `OBS###` ids are the citation key. Once assigned, do not reuse or silently rewrite
them; material claims in later stages cite those ids.

## Write isolation

Each subagent writes only its own stage directory. The coordinator owns `run.md` and the
merged `3_evidence/` observations, index, and timeline.
If a stage is partial, record the limitation in `run.md` and keep the gap visible rather
than fabricating downstream certainty.