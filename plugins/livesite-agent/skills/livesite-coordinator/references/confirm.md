# Stage 2 — Confirm

Stage 2 is DISPATCHED. The coordinator selects candidates via the confirm budget and
must-confirm rules (see Confirm budget and candidate selection below), then launches one
confirm subagent per selected candidate in parallel; unselected candidates receive
disposition `unverified-lead`. Each subagent owns only its own `stage2/<candidate-id>/`
directory. Verdicts — not Stage 1 candidates — are where conclusions live.

## Dispatch contract

**Inputs per subagent (injected by the coordinator at dispatch time):**
- Full candidate record from `stage1/candidates.toon` (including `evidence_locator`).
- The **ready-to-run `repull_recipe`** from `evidence_locator` — the exact capability
  invocation the subagent executes verbatim, substituting only the window/time value.
  The recipe is window-parameterized and serves both the fresh re-pull and the chronic
  long-window first-seen check. The subagent does NOT re-derive capability syntax.
- Any capability-invocation gotchas the host environment exposes (e.g., required auth
  context, pagination limits, mandatory parameter formats) — injected by the coordinator
  so the subagent executes rather than rediscovers.
- Capability handles for the bounded read-only telemetry log-query capability and the
  metrics time-series capability available in the host environment.
- `window_anchor_utc` from `run-state.toon` (for consistent UTC-anchored re-pulls).
- Output path: `stage2/<candidate-id>/verdict.toon`.

**Output per subagent:**
- `stage2/<candidate-id>/verdict.toon` — written by the subagent; read by the coordinator
  during merge.

## Confirm budget and candidate selection

**Budget** (`confirm_budget`): the cap on the number of non-must-confirm candidates
confirmed per run. Configurable per service in `service.toml` as `confirm_budget`
(tunable default: ~5; adjust per service volume and alert SLA). MUST-CONFIRM candidates
(Pass 1 below) are confirmed OUTSIDE this cap — the budget does not suppress a
must-confirm, even when the must-confirm count alone exceeds `confirm_budget`. Selection
proceeds in the following order:

**Pass 1 — MUST-CONFIRM class (OUTSIDE/ABOVE the budget cap; always same-run):**
A candidate is must-confirm if ANY of the following hold:
- Direction is `new` or `spike` AND blast radius is wide (broad scale-unit or version
  spread) AND `data_quality: ok`.
- Multi-source or cross-plane corroboration: `related_candidates` contains an entry on a
  different plane or component (both independently signal the same burst window).
- Extreme burst factor (a configurable floor, well above the normal burst threshold from
  `probes.toon`).
Must-confirm candidates are ALWAYS dispatched the same run, regardless of the budget count.
The budget cap is applied only to what remains after must-confirm.

**Pass 2 — Reserve aging slots within the budget (oldest-deferred leads first):**
From the `confirm_budget` slots, first reserve a bounded count (configurable in
`service.toml` as `confirm_budget_aging_slots`; default: 1–2) for the candidates with
the highest `runs_deferred` from prior runs, draining oldest-deferred leads first. This
bounds miss latency: any persisting deferred lead is caught within a bounded number of
runs. If no deferred leads exist, these slots are freed to Pass 3.

**Pass 3 — Fill remaining budget slots by `confirm_priority` (deterministic ranking):**
Fill remaining budget slots by descending `confirm_priority` (see
`references/funnel-triage.md`). This blend front-loads:
- High burst factor (primary weight).
- Direction: `new` and `spike` weighted above `drop`.
- Broad blast radius.
- `data_quality: ok` (penalize thin or partial-coverage candidates).
This ranking is DETERMINISTIC and recorded; it must NOT degrade to raw `triage_order` alone.

**Deferred candidates → `unverified-lead`:**
Candidates not selected by any pass are deferred for this run. The coordinator:
- Records each with disposition `unverified-lead` in `final/manifest.toon`.
- Appends or refreshes the deferred lead in `observations.toon` with `first_seen` and
  `runs_deferred` (see `references/memory-trust.md`).
Deferral is never a silent drop; every deferred candidate is traceable.

## What each confirm subagent does

1. **Re-pull fresh evidence** — execute the ready-to-run `repull_recipe` from the
   candidate's `evidence_locator` verbatim (substituting only the time value for the
   fresh window). Do NOT reuse Stage 1 cached probe output; do NOT re-derive capability
   invocation syntax; always re-pull independently using the injected recipe.

2. **Evaluate fresh evidence — cheap early exit** — before running the expensive chronic
   pull, check whether the fresh re-pull re-confirms the burst:
   - **Signal cleared or normalized** (rate returned to baseline or below the burst
     threshold by re-pull time): emit `recovered` or `refuted` directly from fresh
     evidence alone. **Do NOT run the chronic pull.** This is the cheap early exit;
     it avoids the long-window pull when the signal has already resolved.
   - **Fresh re-confirms the burst** (rate still elevated): proceed to step 3. The
     chronic check is MANDATORY before any `confirmed` verdict.

3. **Chronic-vs-fresh check (MANDATORY before `confirmed`)** — execute the `repull_recipe`
   verbatim with the long historical window (see Chronic-vs-fresh rule below for default
   window selection). The recipe is window-parameterized; substitute only the window
   value — do NOT re-derive capability invocation syntax for the chronic pull. See the
   chronic rule below. This check is required before any `confirmed` verdict; skipping
   it is a Stage 2 defect.

4. **Evaluate verdict** — based on fresh evidence and (when reached) the chronic-vs-fresh
   result, assign:
   - `confirmed` — fresh evidence corroborates the candidate; burst is real, recent, and
     non-chronic. **Requires the chronic check (step 3).**
   - `refuted` — fresh evidence contradicts; signal is absent, noise, or an artifact.
   - `recovered` — signal was real but has resolved since Stage 1 (rate returned to
     baseline by the time of the re-pull). May be reached from step 2 without the chronic
     pull.
   - `inconclusive` — evidence is insufficient (thin baseline, partial coverage,
     chronic contamination, or ambiguous).

5. **Write `verdict.toon`** — record the verdict, supporting evidence, data quality
   assessment, and chronic-vs-fresh result. Set `chronic_check: skipped-early-exit` when
   the cheap early exit fired (step 2 cleared the signal without a chronic pull).

## Chronic-vs-fresh rule

Execute the `repull_recipe` with a long historical window. **Use the SHORTER lookback by
default:** the smaller of `baseline_window × 3` and the service's configured short
chronic window (from `service.toml`; if absent, `baseline_window × 3` is the default).
Use the LONGER window (e.g., the service's configured long chronic window) only when the
candidate's burst depth, escalation history, or signal age requires deeper history.
Record the window used in `verdict.toon`.
If the signal appears continuously throughout that long window:
- The incident has been ongoing longer than the baseline window.
- The recomputed burst deflates: the "recent" period is no longer anomalous relative to the
  full historical window.
- **Do NOT confirm as a fresh regression.** Emit `inconclusive` or `recovered` with a note
  that the baseline is contaminated. Record in `verdict.toon`: recommend anchoring to a
  clean pre-incident baseline (a window fully before incident start) or to an absolute floor
  (e.g. signal was absent for N days before the incident window).

## Thin baseline rule

If the candidate carries `data_quality: thin_baseline`, or if the re-pull yields fewer data
points than the minimum materiality floor → emit `inconclusive`. Do not confirm or refute
on thin evidence. Record the gap explicitly in `verdict.toon`.

## verdict.toon schema

```
candidate_id:      <string; matches stage1/candidates.toon>
verdict:           confirmed | refuted | recovered | inconclusive
reason:            <if inconclusive: tooling-blocked | thin-baseline | chronic-contamination | ambiguous | partial-coverage>
confidence:        high | medium | low
fresh_evidence:    <one-line summary of what the re-pull showed>
chronic_check:     present | absent | skipped-early-exit
chronic_window:    <window used for the chronic-vs-fresh pull; "n/a" when chronic_check: skipped-early-exit>
data_quality:      ok | thin_baseline | empty-ambiguous | partial_coverage
why:               <evidence-graded rationale for this verdict>
baseline_note:     <if chronic: contamination description + recommended anchor>
```

## Parallelism and batching

All confirm subagents for a given run are dispatched as a single awaited parallel batch.
The coordinator does not advance to the merge step until every verdict file is written and
collected.

Honor the host concurrency limit: when the candidate count exceeds the limit, dispatch in
awaited batches of at most N candidates, collecting each batch fully before the next.
Do not dribble dispatches one-at-a-time; do not use background/detached dispatch.

## Subagent write isolation

Each confirm subagent writes ONLY to `stage2/<its-candidate-id>/verdict.toon` and
(optionally) `stage2/<its-candidate-id>/_scratch/` (transient working files only — not
contract output; not read by any other component).
No subagent reads or writes another candidate's directory, `run-state.toon`, `final/`,
`observations.toon`, or `cache/`. The coordinator reads all verdict files after the batch
completes; it does not read them during execution.

**Exception — coordinator tooling-blocked fallback:** if a confirm subagent fails after
the retry (see Confirm-subagent failure handling below), the COORDINATOR may write
`stage2/<candidate-id>/verdict.toon` with `verdict: inconclusive` and
`reason: tooling-blocked`. This is the ONLY case the coordinator writes inside a
`stage2/` directory; all other `stage2/` writes are exclusively the subagent's.

## Confirm-subagent failure handling

If a dispatched confirm subagent fails (errors out, returns no verdict file, or crashes):

1. **RETRY-DISPATCH once** — the coordinator dispatches a fresh subagent for the same
   candidate with the same inputs.
2. **If the retry also fails** — the coordinator:
   a. Writes `stage2/<candidate-id>/verdict.toon` with `verdict: inconclusive`,
      `reason: tooling-blocked`, and `why` noting the failure mode and retry count.
      (This is the single documented exception where the coordinator writes inside a
      `stage2/` directory — see Subagent write isolation above.)
   b. Records the tooling-blocked candidate as a caveat entry in `final/manifest.toon`
      (candidate id, `tooling-blocked` reason, disposition `inconclusive`).
      Stage-2 dispatch failures do NOT go into `stage1/receipts.toon`; that file is
      Stage 1's suppression and capability-degraded audit trail.
3. **The coordinator MUST NOT run the confirm evidence pull inline.** Executing the
   confirm re-pull inline breaches the dispatch boundary. Confirm is analyze-evidence
   work and MUST remain dispatched regardless of subagent failure. A coordinator that
   runs confirm pulls inline after subagent failure is committing an unacknowledged
   boundary breach, regardless of whether the result appears correct.

## Alert gate — verdict → disposition

Applied by the coordinator during the merge step. Confirm subagents emit verdicts only;
they do not apply this gate.

**Disposition scope (read before the table):** the table below shows **candidate-level disposition** — which file(s) a candidate's entry is promoted into. It does NOT govern which run files are written. BOTH `final/manifest.toon` AND `final/alert-draft.toon` are ALWAYS written for every run (terminal obligation — see Finalize-before-exit obligation below and SKILL.md). When no candidate is alert-worthy, `final/alert-draft.toon` is still written as a clean-sweep / no-alert receipt carrying the coverage caveat, unconfirmed-candidates caveat, and any degraded caveat — it is never skipped.

| Verdict | `data_quality` | `confidence` | Candidate disposition (not file-write) |
|---|---|---|---|
| `confirmed` | `ok` | `medium` or `high` | alert list in `final/alert-draft.toon` + recorded in `final/manifest.toon` |
| `confirmed` | `ok` | `low` | `final/manifest.toon` ONLY (not promoted to alert list) |
| `confirmed` | `thin_baseline` / `empty-ambiguous` / `partial_coverage` | any | `final/manifest.toon` ONLY (not promoted to alert list) |
| `refuted` | any | any | `final/manifest.toon` ONLY (not promoted to alert list) |
| `recovered` | any | any | `final/manifest.toon` ONLY (not promoted to alert list) |
| `inconclusive` | any | any | `final/manifest.toon` ONLY (not promoted to alert list) |

**Degraded/blocked — never a silent clean pass:** if any required signal-plane capability
was degraded or the run was `run-blocked`, the gate does NOT emit a clean "no issues"
alert draft even if zero candidates cleared the gate. Instead:
- `final/manifest.toon` records the degraded/blocked state and all capability gaps.
- `final/alert-draft.toon` carries an explicit caveat section listing the unavailable
  signal planes so a reviewer knows the sweep was partial.

A clean "no fresh regression in the COVERED surface" pass is valid ONLY when ALL of the
following hold:
1. ALL required signal-plane capabilities were available and returning data.
2. ALL candidates were `refuted`, `recovered`, or `inconclusive`.
3. Watch-tier coverage is adequate: no namespace is `coverage_status: never` or
   `coverage_status: overdue` in `cache/coverage.toon`.
4. Both `final/manifest.toon` and `final/alert-draft.toon` carry the count and list of
   candidates deferred as `unverified-lead` this run (unconfirmed-candidates caveat —
   always required; cross-references SKILL.md). If the deferred backlog exceeds the
   bounded service-configurable threshold, the result is DEGRADED: cap to
   inconclusive/partial — it cannot be a clean or confident pass.

If condition 3 fails (any namespace is `never`-covered or `overdue`): the result is capped
to "no fresh regression in the COVERED surface" — NOT a clean full-surface pass. Both
`final/manifest.toon` and `final/alert-draft.toon` MUST carry the mandatory coverage
caveat: namespaces FULL-swept this run, namespaces currently WATCHED (watch-set active),
NEVER-covered namespaces, the oldest `resweep_due` age, and the `pinned_critical` set.
A regression in a never-covered or overdue namespace cannot be ruled out; state this
explicitly. Never emit a conclusion implying full-surface coverage when coverage is
partial. This is a direct corollary of the honesty floor: "never claim no-anomaly when
a capability was unavailable for part of the sweep" — partial namespace coverage is a
coverage gap, and the same rule applies.

**Finalize-before-exit obligation:** A run is COMPLETE only when BOTH `final/manifest.toon`
AND `final/alert-draft.toon` have been written for the run-id this invocation created.
Writing both is the terminal obligation of every run — clean sweep, alert, or degraded
alike. If the run cannot complete the merge (blocked/degraded), it MUST still write
`final/manifest.toon` recording the incomplete/degraded state rather than exiting with
no `final/`; exiting without a `final/` for the current run-id is a run defect.

## Model-tier floor for confirm dispatch

Confirm subagents require `minimum_allowed_class: mid`. The economical class is BELOW
this floor and MUST NOT be used for confirm dispatches — driving the read-only telemetry
capability and evaluating evidence needs mid-tier reliability. Escalation to reasoning-heavy
is allowed when a candidate requires causal or contradiction reasoning. The coordinator MUST
select the model class per the `minimum_allowed_class: mid` floor and set it EXPLICITLY WHEN
THE DISPATCH MECHANISM EXPOSES A MODEL/CLASS SELECTOR. WHEN the host dispatch mechanism does
NOT expose an explicit class/model selector, the coordinator MUST (a) use the strongest/default
dispatch tier the host provides (which must be at least the mid floor — never an
economical/cheapest tier if one is selectable), and (b) RECORD in `run-state.toon` the
resolution basis (e.g. `"host exposes no explicit class selector; used host default dispatch
tier, treated as >= mid floor"`). A missing or ambiguous class resolution when a selector IS
available fails safe upward (never down to economical). Record the chosen class and
resolution basis in `run-state.toon` for each confirm dispatch.
