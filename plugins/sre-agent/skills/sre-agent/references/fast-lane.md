# Intake fast-lane (recognized-recurrence confirm-and-dispose)

The full contract for the intake fast-lane named in `SKILL.md` § Intake fast-lane. This is
the coordinator SELF-GOVERNING the lane at intake with judgment, steered by this prompt: it
reasons over the cheap orientation it already holds — the captured recurrence identity and
the curated known-recurrence registry as claims-not-authority evidence — and steers itself
to a bounded confirm-and-dispose when it recognizes an already-curated, low-consequence
recurrence, instead of always paying the full Scout→Grader investigation. It is a speed
path, not a lower bar: any uncertainty fails open to the full deep-lane, and the auto-post
rests on a cited live confirming observation.

Honesty floor: [investigation-invariants.md](investigation-invariants.md). Operational
efficiency floor: [operational-discipline.md](operational-discipline.md). The probe runs
the same mechanism-discriminator gate rigor as the deep-lane —
[grading-rubric.md](grading-rubric.md) § Mechanism-discriminator gate — scoped to one
predicate. Honesty wins over speed.

## Where this fires

End of `1_intake`, AFTER recurrence-identity capture + the completed CAPABILITY MAP, BEFORE
Scout dispatch. The deep-lane (`2_scout` → `5_grader` → `6_report` → `7_knowledge`), the
Grader gates, the duplicate/verdict-determinism gate, and the poster contract are all
unchanged; the fast-lane only ADDS a pre-Scout path and REUSES the poster.

## The lane decision (coordinator self-governance at intake)

This is the coordinator self-governing the lane with judgment, steered by this prompt — not
a mechanical rule that bypasses reasoning. Reading the registry is an intake-artifact read
(action class (a)); the registry entries are curated orientation EVIDENCE the coordinator
reasons over — claims, not authority, bounded by service/component/signature, the same
status and honesty floor as the existing failure-knowledge KB grounding and Scout's
known-issue consultation — never a lookup table that disposes an incident on its own.

The coordinator reasons over the cheap orientation it already holds — the captured
recurrence identity plus these curated entries — and judges whether this is a recognized,
low-consequence recurrence worth a bounded confirming probe instead of the full
investigation. Steer to the fast-lane only when the coordinator can affirm ALL of:

1. **Curated recurrence recognized.** The captured recurrence identity (signal/error
   signature, affected operation/component, `monitor_id`, affected entity/`cohort`,
   `scope_boundary`, owning service/team) matches the declared `match_predicate` of exactly
   ONE curated/promoted entry. **Per-axis match rule:** EVERY populated sub-value of the
   entry's `match_predicate` must align with the captured recurrence identity; ANY
   partial-axis mismatch — or an axis the entry requires that the identity does not satisfy
   — → ESCALATE. A single matching sub-value (e.g. the signal signature alone) is NOT a
   match. A clean single recognition across all required axes is required: no credible
   entry, two-or-more plausible entries, or an identity too thin to recognize confidently →
   ESCALATE (never guess a match). _Worked example:_ the entry's `signal_signature` matches
   but its `cohort`/`scope_boundary` differ from (or are absent in) the captured identity →
   the axes disagree → ESCALATE, even though the signature lined up.
2. **Severity in band.** Sev ∈ {3,4} AND within the matched entry's `sev_applicability`.
3. **Blast radius in band.** The incident's scope is within the entry's
   `blast_radius_bound` (e.g. single component / one cohort). Wider scope → ESCALATE.
4. **Falsifiable discriminator present.** The entry carries a falsifiable `discriminator`
   (the probe predicate) AND a `disposition` template. An entry lacking a discriminator
   never drives the fast-lane — it remains a deep-lane known-issue-first orientation lead
   (`SKILL.md` § Six-stage flow, stage 3 Specialists, and
   [grading-rubric.md](grading-rubric.md) § Known-issue acceleration settle).

The coordinator reasons over CURATED/promoted entries ONLY — discovered by description
through the CAPABILITY MAP as service knowledge. Never run-local `7_knowledge` candidates
and never sibling run directories (the same isolation rule as Scout and
[run-store.md](run-store.md) § Reading prior runs). It records the decision in `run.md` as
`fast-lane: admitted <registry-id>` or `deep-lane: <reason>`. The decision lives in the
coordinator's judgment; when in doubt it fails open to the full deep-lane.

## The confirming probe (probe-before-block — the recall floor)

On admission, dispatch ONE bounded `full-evidence` probe specialist as an awaited
dispatch (`SKILL.md` § Execution model), economical/mid class, with
`capabilities_to_invoke` = the matched entry's named `evidence_source`:

- It checks ONLY the entry's `discriminator` predicate against THIS incident's LIVE
  evidence, applying the same mechanism-discriminator gate rigor as the deep-lane, scoped
  to that one predicate.
- It STATES the entry's `expected_favored` and `expected_rival` values BEFORE reading
  (pre-registration), then records the observed value, the gate status, and a cited
  `OBS###`.
- **Budget:** ≤ ~2 targeted reads / a tight time cap; never a broad sweep, never a second
  hypothesis. If the discriminator cannot be evaluated within budget, that is `blocked`,
  not a soft pass.
- **Scope corroboration (within the same budget):** before CONFIRMED can license a
  disposition, the probe must corroborate the incident's ACTUAL scope from the live
  evidence it reads and confirm it falls within the entry's `blast_radius_bound` (e.g.
  SU-local, not multi-SU). The intake-captured scope is pre-Scout and non-authoritative,
  so it is never trusted on its own; if the live evidence shows a wider scope than the
  bound (or the scope cannot be corroborated within budget), that is a DISAGREES/blocked
  outcome → ESCALATE.
- **Returns CONFIRMED** — the live observation matches the entry's `expected_favored`
  signature, so the known disposition holds for THIS incident, carried by a cited OBS — or
  **DISAGREES / inconclusive / blocked** — the live pattern differs (matches the rival or
  neither pre-registered value), the blast radius is wider than the entry's bound, a new
  dimension/cohort appears, or the discriminator could not be evaluated.

A checked value matching neither `expected_favored` nor `expected_rival` is post-hoc and
does NOT confirm (the CP-3 pre-named-value discipline in
[grading-rubric.md](grading-rubric.md)) → ESCALATE.

## Disposition (auto-post all confirmed matches, observation-cited)

On CONFIRMED only:

- Compose the disposition from the entry's `disposition` template: cause + mechanism,
  `known recurrence — registry ref <id>`, **the probe's confirming OBS as the cited
  evidence**, and the entry's engineer next-step. Honesty floor preserved: the post cites
  the live confirming observation, NEVER "registry says so".
- The fast-lane disposition does **not** classify `canonical`/`duplicate-of`. That linkage
  is exclusively a Scout (sibling discovery) + Grader (clock-ordering) product, and the
  one-predicate probe confirms only the discriminator, never a sibling relation — so the
  coordinator NEVER infers or posts a cross-incident duplicate relation. A sibling/duplicate
  linkage may be carried ONLY when it is already host-supplied — the incident system's own
  duplicate linkage present in the intake bundle, or the matched entry's `provenance` — and
  it is rendered labeled `incident-system-linked`, explicitly not a derived verdict.
- Emit an observable fast-lane verdict and a `6_report` so the run is auditable (reuse the
  `6_report` stage and the Poster's report contract; render the named `Known-recurrence`
  disposition — [subagents/poster.md](subagents/poster.md) § Post-body structure (Facts
  header) / § Verdict policy
  — as a short Known / ongoing issue note stating the tested known-issue match + this
  incident's live discriminator result, NOT a re-derived RCA and NOT a Grader verdict band).
- Apply the existing Poster **post mode** ([subagents/poster.md](subagents/poster.md) § Verdict
  policy / `artifact-contracts.md` §`6_report/` Post mode): when the incident's discussion
  thread already carries a human root cause, mitigation, or material progress, contribute as a
  collaborator (additive/credit, or a respectful evidence-cited contradiction) rather than a
  standalone known-recurrence note — never silently overwrite an in-flight human conclusion.
- Post via the existing poster path ([subagents/poster.md](subagents/poster.md) § Live
  incident-system posting) under the SAME rules as deep-lane posting: live posting
  requires BOTH explicit brief authorization AND a non-gated incident-posting capability,
  passes the capability-owned idempotency/audit marker and AI-generated disclosure, and
  fails closed on an unverified duplicate scan. With no authorization or no non-gated
  capability the run is report-only — that is not a gap.
- Record a FAST-LANE receipt + the `model_tiering` Tier Record in `run.md`
  ([run-store.md](run-store.md) § Run-state digest).

### FAST-LANE receipt (in `run.md`)

One compact line so the path is auditable:

```text
fast_lane:
  registry_id: <entry id>
  decision: admitted (sev=<n>, blast=<scope>, single recognized match)
  probe: <OBS###> · expected_favored=<...> · observed=<...> · gate=<pass|fail|blocked>
  outcome: confirmed | escalated-<reason>
  disposition: <posted | report-only(<reason>)> · incident_system_linked=<id|none>
```

## Escalation triggers (fail open — never dispose on doubt)

ANY of the following runs the **normal deep-lane** (Scout → Specialists → Grader → Report),
unchanged in depth and recall:

- probe **DISAGREES** (live pattern matches the rival, or neither pre-registered value);
- probe **inconclusive** (discriminator evaluated but ambiguous);
- probe **blocked** (discriminator could not be evaluated within budget / capability);
- **out-of-bound** Sev or blast-radius (the intake-captured Sev or scope is outside the
  entry's bound);
- **live scope wider than the entry's `blast_radius_bound`** (the confirming probe's live
  evidence shows the actual incident scope exceeds the bound, or scope could not be
  corroborated within budget);
- **ambiguous/multiple match** in the lane decision (more than one credible entry, or no
  confident single recognition);
- **probe-capability-unavailable** (the entry's `evidence_source` is not reachable).

On escalation, the lane decision + the probe OBS carry forward as intake context — the
probe becomes a pre-registered discriminator check the deep-lane reuses — and NO verdict
bar is lowered. This is the existing intake "fail open on any uncertain signal" discipline.

## Recall safeguards (the floor under auto-post)

1. Steered ONLY when the coordinator recognizes a curated/promoted registry entry
   (human-reviewed) — never a raw sibling guess or a run-local `7_knowledge` candidate.
2. The confirming probe is MANDATORY, runs the same discriminator-gate rigor as the
   deep-lane, and is observation-cited.
3. ANY uncertainty escalates (the trigger list above). Never dispose on doubt.
4. An entry must carry a falsifiable discriminator + Sev/blast bounds, or it cannot drive
   the fast-lane.
5. The auto-post cites the live confirming OBS; the honesty/confidence floor is preserved.
6. Telemetry flags escalate-and-find-new as registry drift → re-curate (below).

These compose with — and never override — the coordinator orchestration-only contract,
the redacted-is-not-absent floor, the duplicate/verdict-determinism gate, the
model-tiering fail-safe-to-reasoning-heavy rule, and the single-turn awaited execution
model in `SKILL.md`.

## Telemetry (recurrence-family learning)

Record per fast-lane run: `registry_id`, probe outcome (`confirmed` |
`escalated-<reason>`), disposition, and Sev. An entry that repeatedly confirms accrues
curation confidence. An entry whose probe **escalates and the deep-lane then finds a
different/new cause** is flagged `registry-drift → re-curate` — a real regression hiding in
a "known" family — so a perpetually mis-disposing entry is caught rather than trusted. This
is the inverse of zero-yield depth telemetry: the escalate-and-find-new signal is the
recall floor's audit.

Because drift is flagged only on escalate-then-mismatch, a discriminator that always reads
CONFIRMED is invisible to this signal — so the curated-KB process should periodically
human-sample CONFIRMED fast-lane dispositions to catch a discriminator that has silently
gone stale. This is a curation recommendation, not an in-agent mechanism.

## Registry growth (never self-promoted)

The fast-lane never adds its own entries. When the DEEP-LANE confirms a NEW recurring
known/benign disposition that carries a falsifiable discriminator + bounds, the `7_knowledge`
Knowledge Curator proposes an UN-APPLIED `registry-entry-delta`
([subagents/knowledge.md](subagents/knowledge.md)) — a suggestion only, written run-local.
Human curation/promotion into the service KB is what makes an entry fast-lane-eligible.

## Known-recurrence registry schema

A curated, per-service KB collection, read via the CAPABILITY MAP as service knowledge.
Per-service registries live with the service KB under
`services/<service>/failure-knowledge/known-recurrences/` (create the collection if
absent; match the service KB's existing failure-knowledge entry style). Curated/promoted
only (human-reviewed); never auto-applied, never written by a run.

Each entry:

```text
id: <stable id>
match_predicate:                 # structured fields the coordinator reasons over to recognize a recurrence
  signal_signature: <exact/regex — monitor/alert/error signature>
  component_operation: <affected operation/component>
  monitor_id: <stable monitor/alert id>
  cohort: <affected entity/cohort axis>
  scope_boundary: <scope boundary, e.g. su-local | cohort:X | single-component>
  owning_service: <service/team>
sev_applicability: [3,4]         # the Sev band this entry disposes; subset of {3,4}
blast_radius_bound: <single-component | cohort:X | su-local | ...>
discriminator:                   # the falsifiable probe predicate
  predicate: <one falsifiable check distinguishing the known artifact from a real incident>
  expected_favored: <the known-artifact signature the probe should observe>
  expected_rival: <what a REAL incident would look like on the same evidence>
  evidence_source: <named live source: geneva-dgrep/kusto cluster.table or monitor def>
disposition:                     # the auto-post template
  cause_mechanism: <plain cause + mechanism, "known recurrence — registry ref <id>">
  engineer_next_step: <owner-routed action / mitigation / tuning ask>
provenance: <curated-by · review ref · first-seen incident(s) · recurrence count/window>
```

Fields map straight onto the recurrence identity captured at `1_intake` (signal
signature, operation/component, monitor id, entity/cohort, scope boundary, owning
service), so the coordinator can recognize a match axis-by-axis against the identity it
already captured — EVERY populated `match_predicate` sub-value must align, any partial-axis
mismatch escalates (admission rule 1) — judgment over curated evidence, not a lookup. An
entry whose `discriminator` is missing or not falsifiable is a deep-lane orientation lead
only — it never drives the fast-lane.
