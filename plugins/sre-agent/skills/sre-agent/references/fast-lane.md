# Intake fast-lane (recognized-recurrence confirm-and-dispose)

The full contract for the intake fast-lane named in `SKILL.md` § Intake fast-lane. This is
the coordinator SELF-GOVERNING the lane at intake with judgment, steered by this prompt: it
recognizes — cheaply, from orientation it already holds — that the current incident is a
member of a known recurrence FAMILY, runs ONE bounded live probe to RE-ESTABLISH the
family's disposition on this incident's own evidence, and disposes; instead of always paying
the full Scout→Grader investigation. It is a speed path, not a lower bar: any uncertainty
fails open to the full deep-lane, and any post rests on a cited live confirming observation,
never on an imported prior verdict.

Honesty floor: [investigation-invariants.md](investigation-invariants.md). Operational
efficiency floor: [operational-discipline.md](operational-discipline.md). The probe runs
the same mechanism-discriminator gate rigor as the deep-lane —
[grading-rubric.md](grading-rubric.md) § Mechanism-discriminator gate — scoped to one
predicate. Honesty wins over speed.

## The backbone: SAMENESS → ARM → PROBE → RE-ESTABLISH (never import)

The agent may classify the relationship between incidents, but it never imports another
incident's cause, verdict, or mitigation as truth (the never-import rule in
[investigation-invariants.md](investigation-invariants.md)): prior knowledge ARMS the
authoritative probe but can NEVER substitute for it. The fast-lane is exactly that rule run
fast:

1. **SAMENESS** — recognize cheaply that this incident is a member of a known recurrence
   family. This is a relationship RECOGNITION the agent is allowed to do, read off the
   incident-history recurrence cluster the agent already holds at intake.
2. **ARM** — use the family's convergent prior CLAIMED disposition only to arm a bounded
   live probe with a falsifiable discriminator: what to check, the expected known-artifact
   value vs. the expected real-incident value, on a named live evidence source.
3. **PROBE** — run the discriminator against THIS incident's live evidence.
4. **RE-ESTABLISH** — the probe independently re-establishes the disposition on live
   evidence. The prior told us WHAT TO CHECK; live evidence gives THE ANSWER.

The disposition is never imported. The cluster and the family's claims are claims-not-
authority orientation; the live observation is the only thing that disposes the incident.

## What "all previous investigations" means here (the matcher)

The matcher is NOT a curated per-service file. It is the **intake recurrence cluster**: the
read-only incident-history recurrence pass the run already holds at intake surfaces the
sibling incidents matched to this incident's recurrence identity over a recent window. Each
row carries the fields the admission rule needs: the **identity axes** — signal/signature,
affected component/operation, entity/cohort, scope boundary, and owning team — plus
`Severity` (for the Sev/scope band), and the sibling's `ClaimedRootCause`,
`ClaimedMitigation`, `IsNoise`, and `Status`. Operationally: `IsNoise=true` siblings are
independent corroboration of a benign/known disposition (admission rule 2); `Status`
distinguishes active vs. mitigated/closed siblings so a live-and-diverging sibling is not
mistaken for a settled one. Because the fast-lane and deep-lane post their conclusions back
into the incident-management system, that history INCLUDES the agent's own prior verdicts — so
the cluster is "all previous investigations" as recorded in IcM, not a hand-maintained list;
the independence discount in admission rule 2 keeps the agent's own echoed posts from
self-reinforcing. No curated per-service registry file is consulted or required.

## Where this fires

End of `1_intake`, AFTER recurrence-identity capture, the intake recurrence cluster, and the
completed CAPABILITY MAP, BEFORE Scout dispatch. The deep-lane (`2_scout` → `5_grader` →
`6_report` → `7_knowledge`), the Grader gates, the duplicate/verdict-determinism gate, and
the poster contract are all unchanged; the fast-lane only ADDS a pre-Scout path and REUSES
the poster.

## Tier model (the discriminator source — sameness is always the cluster)

Sameness ALWAYS comes from the intake recurrence cluster (above). The DISCRIMINATOR that arms
the probe comes from the highest-trust source available:

- **Tier 1 — promoted ledger entry.** An identity-keyed entry in the recurrence ledger (§
  Recurrence ledger) that matches on EVERY populated recurrence-identity axis (a partial-axis
  match does NOT select a Tier-1 entry — fall to Tier 2 or ESCALATE) carries a vetted,
  falsifiable `discriminator` with `expected_favored` (the known-artifact signature) and
  `expected_rival` (what a real incident looks like) on a named live `evidence_source`, plus a
  disposition. On CONFIRMED → AUTO-POST via the existing `Known-recurrence` poster band, under
  the same authorization/idempotency rules and post-mode parity as deep-lane posting.
- **Tier 2 — cluster-derived (no full-axis ledger entry).** No promoted ledger entry matches
  on every identity axis, but the convergent siblings' `ClaimedRootCause`/`IsNoise` CLAIMS let
  the coordinator DERIVE a falsifiable discriminator from them. On CONFIRMED → compose the
  disposition to the REPORT-ONLY / DRAFT finalize ONLY (the `6_report` draft + evidence kit) —
  NEVER a live incident-system mutation on a self-derived discriminator, neither standalone NOR
  collaborator/additive. This conservatism is locked.
- **Else → ESCALATE** to the full deep-lane.

## SAMENESS / admission rule (a real recurrence-family match, or fail open)

Take the fast-lane ONLY when ALL of the following hold; otherwise ESCALATE (fail open):

1. **Convergent recurrence cluster.** The intake recurrence cluster shows MULTIPLE siblings
   sharing the captured RECURRENCE IDENTITY — signal/error signature + affected
   operation/component + entity/cohort + scope. Title OR owning-team alone is INSUFFICIENT
   (the cascade/relationship floor in [grading-rubric.md](grading-rubric.md) § Cross-incident
   classification): require the signature PLUS ≥1 more identity axis converging across
   multiple siblings. A thin or ambiguous cluster → ESCALATE.
2. **Convergent prior disposition (independence-discounted).** The cluster's
   `ClaimedRootCause`/`IsNoise` claims converge on ONE recognizable disposition (e.g. a benign
   measurement/monitor artifact). The convergence must rest on at least one INDEPENDENT signal
   — a human-entered `ClaimedRootCause`/`ClaimedMitigation`, the `IsNoise` flag, or a prior
   DEEP-LANE verdict — NOT solely on the agent's OWN prior fast-lane/auto-posts, which do not
   count as independent corroboration (they are the agent's echoed conclusions, not a fresh
   confirmation). If the only convergence is the agent's own prior posts → ESCALATE (treat as
   not-yet-independently-confirmed). Divergent, empty, or contradictory claims → ESCALATE.
3. **Derivable falsifiable discriminator.** A discriminator is available — from a Tier-1
   ledger entry OR derivable from the convergent claims — with `expected_favored` (the known
   signature) vs. `expected_rival` (what a REAL incident looks like) on a named live
   `evidence_source`. No falsifiable discriminator formable → ESCALATE.
4. **Severity and scope in band.** Sev ∈ {3,4} AND the incident's scope is consistent with
   the family's typical bound.

This is a relationship RECOGNITION for routing. It does NOT set `canonical` / `duplicate-of`
— that classification remains a Scout (sibling discovery) + Grader (clock-ordering) product
([grading-rubric.md](grading-rubric.md) § Cross-incident classification), and the fast-lane
NEVER auto-posts a duplicate-of / canonical claim. The coordinator records the decision in
`run.md` as `fast-lane: admitted (tier=<1|2>)` or `deep-lane: <reason>`. The decision lives
in the coordinator's judgment; when in doubt it fails open to the full deep-lane.

## The confirming probe (probe-before-block — the recall floor that re-establishes)

On admission, dispatch ONE bounded `full-evidence` probe specialist as an awaited dispatch
(`SKILL.md` § Execution model), economical/mid class, with `capabilities_to_invoke` = the
discriminator's named `evidence_source`:

- It STATES the discriminator's `expected_favored` and `expected_rival` values BEFORE reading
  (pre-registration), then checks ONLY the discriminator against THIS incident's LIVE
  evidence, applying the same mechanism-discriminator gate rigor as the deep-lane scoped to
  that one predicate, and records the observed value, the gate status, and a cited `OBS###`.
- **Budget:** ≤ ~2 targeted reads / a tight time cap; never a broad sweep, never a second
  hypothesis. If the discriminator cannot be evaluated within budget, that is `blocked`, not
  a soft pass.
- **Scope corroboration (within the same budget):** before CONFIRMED can license a
  disposition, the probe corroborates the incident's ACTUAL scope from the live evidence it
  reads and confirms it falls within the family's bound. The intake-captured scope is
  pre-Scout and non-authoritative, so it is never trusted on its own; live evidence showing a
  wider scope than the family bound (or scope that cannot be corroborated within budget) is a
  DISAGREES/blocked outcome → ESCALATE.
- **Returns CONFIRMED** — the live evidence matches `expected_favored`, so the family's
  disposition holds for THIS incident, carried by a cited OBS — or **DISAGREES / inconclusive
  / blocked** — the live pattern matches the rival (or neither pre-registered value), the
  scope is wider than the bound, a new dimension/cohort appears, or the discriminator could
  not be evaluated.

A checked value matching NEITHER `expected_favored` NOR `expected_rival` is post-hoc and does
NOT confirm (the CP-3 pre-named-value discipline in [grading-rubric.md](grading-rubric.md)) →
ESCALATE.

## Disposition (by tier — re-established, never imported)

On CONFIRMED only. The post always states the LIVE observation; it never says "IcM says so"
or imports the prior verdict.

- **CONFIRMED + Tier 1 → AUTO-POST.** Compose the disposition from the ledger entry — cause +
  mechanism, "known recurrence — ledger ref `<id>`", **the probe's confirming OBS as the
  cited evidence**, and the entry's engineer next-step. Render the named `Known-recurrence`
  disposition ([subagents/poster.md](subagents/poster.md) § Post-body structure (Facts
  header) / § Verdict policy) and post via the existing poster path
  ([subagents/poster.md](subagents/poster.md) § Live incident-system posting) under the SAME
  rules as deep-lane posting: live posting requires BOTH explicit brief authorization AND a
  non-gated incident-posting capability, passes the capability-owned idempotency/audit marker
  and AI-generated disclosure, and fails closed on an unverified duplicate scan. With no
  authorization or no non-gated capability the run is report-only — that is not a gap.
- **CONFIRMED + Tier 2 → REPORT-ONLY / DRAFT finalize.** Compose the SAME disposition shape,
  cite the live confirming OBS, and write it to the `6_report` report-only finalize (the
  concise draft + evidence kit) as the `report-only (tier-2 self-derived; human confirmation
  required)` outcome — a recognized-recurrence DRAFT for human confirmation. `DRAFT` is that
  report-only finalize path, NOT a live mode. A Tier-2 disposition performs NO incident-system
  mutation — neither a standalone post NOR a collaborator/additive contribution — regardless of
  brief authorization or capability mode; a self-derived discriminator never licenses a live
  post. The draft still credits any in-flight human work in its body, but it never mutates the
  incident.
- **Post mode parity (Tier-1 live post only).** For the Tier-1 LIVE post, the existing Poster
  post mode ([subagents/poster.md](subagents/poster.md) § Verdict policy / `artifact-contracts.md`
  §`6_report/` Post mode) still governs: when the thread already carries a human conclusion,
  contribute as a collaborator rather than overwrite it. The Tier-2 finalize is report-only
  regardless of post mode.
- **No duplicate-of / canonical.** The fast-lane disposition does NOT classify `canonical` /
  `duplicate-of`. That linkage is exclusively a Scout (sibling discovery) + Grader
  (clock-ordering) product, and the one-predicate probe confirms only the discriminator,
  never a sibling relation — so the coordinator NEVER infers or posts a cross-incident
  duplicate relation. A sibling/duplicate linkage may be carried ONLY when it is already
  host-supplied — the incident system's own duplicate linkage present in the intake bundle,
  or the ledger entry's `provenance` — rendered labeled `incident-system-linked`, explicitly
  not a derived verdict.
- **Always observable.** Emit an observable fast-lane verdict and a `6_report` so the run is
  auditable (reuse the `6_report` stage and the Poster's report contract; render the
  `Known-recurrence` disposition as a short Known / ongoing issue note stating the tested
  recurrence-family match + this incident's live discriminator result, NOT a re-derived RCA
  and NOT a Grader verdict band). Record a FAST-LANE receipt + the `model_tiering` Tier Record
  in `run.md` ([run-store.md](run-store.md) § Run-state digest).

### FAST-LANE receipt (in `run.md`)

One compact block so the path is auditable:

```text
fast_lane:
  tier: <1 | 2>
  discriminator_source: <ledger | derived>
  family: <recurrence-identity summary · cluster size · convergent disposition>
  decision: admitted (sev=<n>, scope=<bound>, convergent cluster)
  probe: <OBS###> · expected_favored=<...> · observed=<...> · gate=<pass|fail|blocked>
  outcome: confirmed | escalated-<reason>
  disposition: <posted-live(tier-1) | collaborator-live(tier-1) | draft-report-only(tier-2) | report-only(<reason>)> · incident_system_linked=<id|none>
```

The `disposition` tokens `posted-live(tier-1)`, `collaborator-live(tier-1)`, and
`draft-report-only(tier-2)` are the fast-lane-specific disposition values — additive to, not a
rename of, the deep-lane's `posted`; `report-only(<reason>)` stays the generic report-only token.

## Escalation triggers (fail open — never dispose on doubt)

ANY of the following runs the **normal deep-lane** (Scout → Specialists → Grader → Report),
unchanged in depth and recall:

- **no convergent cluster** (no recurrence family — title/team-only or thin/ambiguous match);
- **divergent prior claims** (the cluster's claims do not converge on one disposition);
- **no falsifiable discriminator** formable from a ledger entry or the convergent claims;
- probe **DISAGREES** (live pattern matches the rival, or neither pre-registered value);
- probe **inconclusive** (discriminator evaluated but ambiguous);
- probe **blocked** (discriminator could not be evaluated within budget / capability);
- **out-of-bound** Sev or scope (intake-captured Sev or scope outside the family bound);
- **live scope wider than the family bound** (the probe's live evidence shows the actual
  incident scope exceeds the bound, or scope could not be corroborated within budget);
- **probe-capability-unavailable** (the discriminator's `evidence_source` is not reachable).

On escalation, the cluster recognition + the probe OBS carry forward as intake context — the
probe becomes a pre-registered discriminator check the deep-lane reuses — and NO verdict bar
is lowered. This is the existing intake "fail open on any uncertain signal" discipline.

## Recall safeguards (the floor under any post)

1. Sameness is read off the intake recurrence cluster — a real convergent recurrence family,
   never a single sibling guess and never run-local `7_knowledge` candidates or sibling run
   directories (the same isolation rule as Scout and [run-store.md](run-store.md) § Reading
   prior runs).
2. The confirming probe is MANDATORY, runs the same discriminator-gate rigor as the
   deep-lane, and is observation-cited.
3. ANY uncertainty escalates (the trigger list above). Never dispose on doubt.
4. A live mutation auto-posts ONLY on Tier 1 (a promoted, vetted discriminator); a Tier 2
   self-derived discriminator NEVER auto-posts and NEVER contributes a live collaborator post —
   it always takes the report-only/DRAFT finalize.
5. The disposition is RE-ESTABLISHED by the probe on live evidence and cites the live
   confirming OBS; the prior is never imported and the honesty/confidence floor is preserved.
6. **Independence discount.** The agent's own prior fast-lane/auto-posts in the cluster are
   NOT independent corroboration (admission rule 2); convergence must rest on a human claim,
   the `IsNoise` flag, or a prior deep-lane verdict, else ESCALATE — so the matcher cannot
   self-reinforce on its own echoed conclusions. This distinction assumes the agent's own
   contribution rides as a posted NOTE/claim (a labeled known-recurrence disposition or
   collaborator note), distinguishable from an independent human-entered `ClaimedRootCause`/
   `ClaimedMitigation` or the incident system's `IsNoise` flag — the agent never writes those
   independent fields.
7. **Periodic forced re-validation (drift breaker).** An always-CONFIRMED discriminator is
   invisible to escalate-then-mismatch drift telemetry, so on a self-governed sampling cadence
   — roughly every Nth recognized member of a family, or when the family's last deep-lane
   re-validation is stale — the coordinator deliberately routes a recognized recurrence to the
   FULL deep-lane even when the probe would confirm. This is judgment-based sampling, not a
   hardcoded counter. The re-validation is recorded in telemetry; a deep-lane result that
   diverges from the family's claimed disposition emits the `recurrence-drift → re-curate`
   telemetry flag — the offline extraction/promotion pipeline consumes that flag to hold or
   demote the ledger entry. The agent never writes, promotes, or demotes a ledger entry itself.
8. Telemetry flags escalate-and-find-new as recurrence drift → re-curate (below).

These compose with — and never override — the coordinator orchestration-only contract, the
redacted-is-not-absent floor, the duplicate/verdict-determinism gate, the model-tiering
fail-safe-to-reasoning-heavy rule, and the single-turn awaited execution model in `SKILL.md`.

## Telemetry (recurrence-family learning)

Record per fast-lane run: `tier`, `discriminator_source`, the family identity, probe outcome
(`confirmed` | `escalated-<reason>`), disposition, and Sev. A family/ledger entry that
repeatedly confirms accrues curation confidence and is a Tier-2 → Tier-1 promotion candidate
for the extraction pipeline. A discriminator whose probe **escalates and the deep-lane then
finds a different/new cause** is flagged `recurrence-drift → re-curate` — a real regression
hiding in a "known" family — so a perpetually mis-disposing discriminator is caught rather
than trusted. This is the inverse of zero-yield depth telemetry: the escalate-and-find-new
signal is the recall floor's audit.

Because drift is flagged only on escalate-then-mismatch, a discriminator that always reads
CONFIRMED would otherwise be invisible to this signal — the in-agent drift breaker
(§ Recall safeguards #7) addresses this by routing a sampled fraction of always-CONFIRMED
recognized recurrences to the full deep-lane; record each such forced re-validation and its
converge/diverge outcome in telemetry. The curation/extraction process should ALSO periodically
human-sample CONFIRMED fast-lane dispositions as a second layer.

## Recurrence ledger (Tier 1 — identity-keyed, never self-promoted)

The Tier-1 ledger is **identity-keyed** by recurrence identity, NOT a per-service file. Keying
by service path is what caused the family-misplacement failure (an incident owned by one team
whose entry sat under another service never matched); identity-keying matches the family
wherever the owning team sits.

The agent NEVER writes or self-promotes a Tier-1 entry. The ledger is fed by the existing
**offline extraction/promotion pipeline** (an operational pipeline outside this repo) that
mines confirmed investigations and emits nuggets carrying `discriminator | verdict | trust`,
with `trust=verified` reached at ≥2 confirming incidents. The agent's OWN repeatedly-confirmed
recurrences therefore GRADUATE Tier-2 → Tier-1 over time via that human/pipeline promotion —
never by the agent applying its own entry.

Each ledger entry:

```text
id: <stable id>
recurrence_identity:             # the identity key the cluster matches on (NOT a service path)
  signal_signature: <exact/regex — monitor/alert/error signature>
  component_operation: <affected operation/component>
  entity_cohort: <affected entity/cohort axis>
  scope_boundary: <scope boundary, e.g. su-local | cohort:X | single-component>
  owning_team: <owning service/team>
sev_applicability: [3,4]         # the Sev band this entry disposes; subset of {3,4}
scope_bound: <single-component | cohort:X | su-local | ...>
discriminator:                   # the falsifiable probe predicate
  predicate: <one falsifiable check distinguishing the known artifact from a real incident>
  expected_favored: <the known-artifact signature the probe should observe>
  expected_rival: <what a REAL incident would look like on the same evidence>
  evidence_source: <named live source: telemetry cluster.table or monitor definition>
verdict: <the convergent known disposition — e.g. benign measurement/monitor artifact>
disposition:                     # the auto-post template
  cause_mechanism: <plain cause + mechanism, "known recurrence — ledger ref <id>">
  engineer_next_step: <owner-routed action / mitigation / tuning ask>
trust: verified                  # promoted by the extraction pipeline at >=2 confirming incidents
evidence_icms: [<incident ids that confirmed this family>]
provenance: <curated-by · review/pipeline ref · first-seen incident(s) · recurrence count/window>
```

The `recurrence_identity` fields map straight onto the recurrence identity captured at
`1_intake`, so the coordinator recognizes a Tier-1 match axis-by-axis against the cluster and
the captured identity. An entry whose `discriminator` is missing or not falsifiable cannot
drive the fast-lane.

## Ledger growth (never self-promoted)

The fast-lane never adds its own entries. When the DEEP-LANE confirms a recurrence carrying a
falsifiable discriminator + bounds, the `7_knowledge` Knowledge Curator proposes an UN-APPLIED
identity-keyed `recurrence-ledger-delta` ([subagents/knowledge.md](subagents/knowledge.md)) —
the recurrence-identity match fields + discriminator + verdict + `evidence_icms`, written
run-local as a suggestion only. Human/pipeline curation and promotion (the extraction pipeline
above) is what makes an entry Tier-1 fast-lane-eligible.

## Illustrative example (inline — not a curated entry)

A recurring monitor artifact in the `Monitor-DropInTBAVolume` / `VolumeDropped25Percent`
family: the monitor's threshold rule fires on a 25% task-volume drop, but the recurring
Sev4 cluster's convergent prior claim is that these are an **absence-of-data sentinel**, not a
real decline.

- **Discriminator.** Read BOTH the windowed aggregate AND the raw sample series for the
  alerted metric over the incident window.
  - `expected_favored` (known artifact): `Sum(metric, 1440m) == 0` with **ZERO samples** in
    the window — the metric stopped emitting, so the "drop" is a measurement gap, not a real
    volume decline.
  - `expected_rival` (a REAL incident): a **non-zero, declining sample series** — samples are
    present and the value genuinely fell — which is a real decline → ESCALATE to the
    deep-lane.
- The probe reads the windowed `Sum` AND the raw sample `Count`; a zero-sample window confirms
  the artifact, a present-but-declining series matches the rival, and any value matching
  neither (e.g. samples present at a flat-but-nonzero level) is post-hoc → ESCALATE.

This is an illustration of the discriminator shape, not a live per-team ledger entry: the
real entry is identity-keyed and promoted by the extraction pipeline.
