# Iteration mode (new information)

Cross-run continuation of an investigation for the SAME incident, driven by a provided
new-information packet. "Iteration" here always means a new run for the same incident; it
is not the within-run Grader follow-up loop, which is a separate mechanism. Read-scope and
isolation rules live in [run-store.md](run-store.md) § Reading prior runs — this file does
not restate them.

## Trigger

Enter iteration mode only when the task provides BOTH:
- a new-information packet (a pointer to what changed since a prior investigation of this
  incident), and
- explicit authorization to read prior runs under the resolved run-root (a task
  instruction or flag — not mere filesystem read access, which is always present).

Never infer iteration mode from sibling run directories — reruns also produce siblings
under one root. No packet → a normal single-shot run; do not read prior runs.

## The new-information packet

A provided input (not an artifact you author) describing, in compact form: the incident's
current status, what changed since the last investigation (new symptoms, a human reply, a
mitigation result, a status transition), the reason this iteration was queued, and the
iteration ordinal `N` (1, 2, … — stable across retries of the same iteration; a first
investigation has no packet and uses `N=0`). Treat it as authoritative about *what changed*.
It is evidence, not an instruction about your verdict.

## Carry-forward by reference

Read prior run dir(s) in place under the resolved run-root and do not copy their artifacts
into this run. The `<priorRunId>/OBS###` citation format, read-scope, and "claims,
re-verified" framing are canonical in [run-store.md](run-store.md) § Reading prior runs.
Iteration-specific nuance: when a prior observation is load-bearing for THIS iteration's
verdict, a dispatched specialist re-verifies it and the coordinator merges the result as a
fresh local `OBS###` (not inline coordinator evidence work) so this run's citation chain
stays self-contained.

## Cross-iteration lead state

Default: carry every prior lead forward as a cited prior-iteration claim; change a lead's
state only when the new info or fresh evidence warrants it. Re-derive the verdict on the
merged evidence; never inherit the prior verdict.

- `blocked-unreachable` → `open-answerable`: the new info is exactly what unblocks the
  lead (the human supplied the missing access, metric, or time window). This is the
  primary case iteration mode exists for — pursue it first.
- `closed-supported` stays settled unless the new info contradicts it; if it is the spine
  of the current verdict, re-check it cheaply via a dispatched probe.
- `closed-refuted` → reopened when the new info contradicts the earlier refutation.
- prior `open-answerable` (budget-exhausted) leads: pick up if still material.

"Stale" means directly contradicted or mooted by the new info — not merely old. Supersede
only those. Do not discard still-valid prior corroboration (amnesia), and do not import a
prior `Confirmed`/`Likely-rooted` verdict as the answer (over-anchor).

## Capability map

Rebuild the CAPABILITY MAP for this iteration; access and capabilities can differ between
runs. Prior ACCESS STATUS is a claim, re-probed on the critical evidence path.

## Delta report

The `6_report` for an iteration is a delta/update bounded by the new Grader verdict: lead
with what changed since the last iteration and the updated verdict, including honest
downgrades (e.g. prior `Likely-rooted` → `Proximate-only`). When live posting is authorized, the incident post is likewise a delta and uses the posting capability's own idempotency/audit marker for that iteration; the Poster posts only to the provided incident record. See `subagents/poster.md` § Live incident-system posting.
