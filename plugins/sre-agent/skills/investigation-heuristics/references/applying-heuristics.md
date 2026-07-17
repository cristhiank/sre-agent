# Applying heuristics

The generalized learnings are **prior method**, not answers. Each entry is a phase-tagged interrogative
check with a scope and a falsifier. Apply it as a question against *this* incident's evidence.

## Anatomy of a heuristic

```
[phase] <one-line interrogative check>
  id: <stable-kebab-id>
  applies-when: <the signal/shape that makes this worth asking>
  does-not-apply-when: <the falsifier — the condition under which it does NOT apply; check it first>
```

- **`[phase]`** — when to ask it:
  - `[scout]` — early, while forming hypotheses and deciding ownership/routing.
  - `[grader]` — late, while pressure-testing a disposition before accepting it.
  - `[any]` — relevant throughout.
- **`id:`** — a short stable slug; use it in the usage breadcrumb so a consulted heuristic can be referenced across curation edits.
- **applies-when** — only spend the check when the current signal matches this scope. Skip the rest.
- **does-not-apply-when** (the falsifier) — the disconfirming condition. If it trips, the check does **not** apply here; do not use it.

## How to apply (the loop)

1. Filter to heuristics whose `[phase]` matches your current step and whose *applies-when* matches the
   signal. Ignore the others — a short list is meant to be scanned, not force-fit.
2. For each, ask the question against the incident's own evidence. **Check the falsifier first.**
3. If it holds, let it steer attention, a candidate hypothesis, a routing decision, or verdict
   skepticism — never the conclusion. If the falsifier (`does-not-apply-when`) trips or evidence
   disagrees, drop it and proceed on evidence (mark it `misleading` in the breadcrumb).
4. Record the one-line usage breadcrumb per consulted heuristic, keyed on its `id`.

## "None apply" is a valid, expected outcome

A novel incident need not match any prior pattern. Consulting the layer and finding nothing relevant is a
correct result — say so and investigate on evidence. Forcing a match is the failure mode this whole layer
is designed to avoid: prior method should reduce blind spots, not manufacture a wrong hypothesis.

## Anchoring guard

These checks are surfaced *before* evidence exists to contradict them, which is exactly when anchoring is
strongest. Three habits keep them honest:
- Treat every entry as a **question**, never a verdict — the interrogative phrasing is deliberate.
- Always evaluate the **falsifier** before acting on the check.
- Prefer checks that **widen** the search (ownership, alternative causes, disconfirmation) over checks
  that **narrow** to a single answer early.

## Worked example (illustrative shape only — not a real learning)

```
[scout] Does the paging component actually own the fault, or is it a propagator?
  id: propagator-routing
  applies-when: an orchestration/retry/gateway layer surfaces a failure produced further downstream
  does-not-apply-when: the paging component is itself the failing unit, or no distinct downstream error exists
```

Applied: if the incident's terminal error names a distinct downstream unit, ask whether routing should
follow that unit rather than the surfacing layer — then verify against ownership evidence before
reassigning. If the surfacing component is itself the failing unit (falsifier), the check does not apply.

> This example is a fabricated illustration of the *format*. The actual heuristics are supplied at runtime
> by the resolved knowledge root; this skill ships no real learnings.
