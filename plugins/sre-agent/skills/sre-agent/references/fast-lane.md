# Intake fast-lane (wave-1 duplicate verification)

The full contract for the intake fast-lane named in `SKILL.md` § Intake fast-lane. At intake,
BEFORE Scout, the coordinator RECOGNIZES — cheaply, from orientation it already holds — that
the current incident is a member of a convergent recurrence FAMILY, then dispatches ONE bounded
specialist to answer a single question: **is this the SAME root cause as the family's previous
incidents?** The coordinator RECONCILES the answer: SAME → fast-track the disposition and post
it as a verified duplicate; DIFFERENT / ambiguous / can't-verify → ESCALATE to the full
deep-lane. This is a speed path, not a lower bar: every uncertainty fails open to the deep-lane,
and any post rests on a cited live observation from THIS incident, never on an imported prior
verdict.

This mirrors the two-wave review shape — a wave-1 independent derivation followed by a reconcile
that escalates on a divergence lead — applied to investigation: the wave-1 specialist DERIVES the
sameness answer on live evidence, and the coordinator reconciles SAME vs. escalate.

Honesty floor: [investigation-invariants.md](investigation-invariants.md). Operational
efficiency floor: [operational-discipline.md](operational-discipline.md). The wave-1 live check
runs the same mechanism-discriminator gate rigor as the deep-lane —
[grading-rubric.md](grading-rubric.md) § Mechanism-discriminator gate — scoped to one predicate.
Honesty wins over speed.

## The backbone: RECOGNIZE → VERIFY LIVE → RECONCILE (never import)

The agent may classify the RELATIONSHIP between incidents, but it never imports another
incident's cause, verdict, or mitigation as truth (the never-import rule in
[investigation-invariants.md](investigation-invariants.md)): the family's previous root cause is
only the HYPOTHESIS the wave-1 specialist RE-ESTABLISHES on THIS incident's live evidence. The
fast-lane is exactly that rule run fast:

1. **RECOGNIZE** — from the intake recurrence cluster, judge that this incident is a member of a
   convergent recurrence family. A relationship RECOGNITION the agent is allowed to do, read off
   the cluster it already holds at intake — routing, not a verdict.
2. **VERIFY LIVE (wave-1 specialist)** — one bounded specialist establishes the family's previous
   root cause as a CLAIM, extracts a falsifiable discriminator from it, and checks that
   discriminator against THIS incident's live evidence.
3. **RECONCILE (coordinator)** — SAME → fast-track + post as a verified duplicate; DIFFERENT /
   ambiguous / can't-verify → escalate to the full deep-lane.

The disposition is never imported. The cluster and the family's prior root cause are
claims-not-authority orientation; the live observation is the only thing that disposes the
incident. **The safety is the mandatory minimal live check + escalate-on-divergence, not
signal-independence** — the agent's OWN prior RCA in the family is a valid hypothesis to verify,
not a disqualifier.

## The recurrence family (the matcher)

The matcher is the **intake recurrence cluster**: the read-only incident-history recurrence pass
the run already holds at intake surfaces the sibling incidents matched to this incident's
recurrence identity over a recent window. Each row carries the identity axes — signal/error
signature, affected component/operation, entity/cohort, scope boundary, and owning team — plus
`Severity` and the sibling's `ClaimedRootCause`, `ClaimedMitigation`, `IsNoise`, and `Status`.
`Status` distinguishes active from mitigated/closed siblings, so the specialist can pick a
recently-resolved sibling to read the prior root cause from. Sibling
`ClaimedRootCause`/`ClaimedMitigation` are frequently EMPTY and `IsNoise` frequently false even
for a genuinely recurring artifact; membership rests on the shared recurrence IDENTITY across
siblings, and the previous root cause is READ LIVE from a representative sibling's discussion/RCA
or from this incident's own prior in-thread RCA (below), not from those structured fields. No
curated per-service registry file is consulted or required.

## Where this fires

End of `1_intake`, AFTER recurrence-identity capture, the intake recurrence cluster, and the
completed CAPABILITY MAP, BEFORE Scout dispatch. The deep-lane (`2_scout` → `5_grader` →
`6_report` → `7_knowledge`), the Grader gates, the duplicate/verdict-determinism gate, and the
poster contract are all unchanged; the fast-lane only ADDS a pre-Scout wave-1 dispatch and REUSES
the poster.

## RECOGNIZE / admission rule (a real recurrence-family match, or fail open)

Dispatch the wave-1 specialist ONLY when ALL of the following hold; otherwise ESCALATE (fail
open) straight to Scout:

1. **Convergent recurrence cluster.** The intake recurrence cluster shows MULTIPLE siblings
   sharing the captured RECURRENCE IDENTITY — signal/error signature + affected
   operation/component + entity/cohort + scope. Title OR owning-team alone is INSUFFICIENT (the
   cascade/relationship floor in [grading-rubric.md](grading-rubric.md) § Cross-incident
   classification): require the signature PLUS ≥1 more identity axis converging across multiple
   siblings. A thin or ambiguous cluster → ESCALATE.
2. **A prior conclusion to verify.** At least one family member carries a readable prior root
   cause — a recently-resolved sibling's discussion/RCA, or this incident's own prior agent RCA
   in-thread — from which a falsifiable discriminator can be formed. The prior conclusion MAY be
   the agent's OWN earlier post: that is a valid hypothesis to re-verify live, not a
   disqualifier. If no family member carries any readable prior conclusion → ESCALATE.
3. **Severity and scope in band.** Sev ∈ {3,4} AND the incident's intake-captured scope is
   consistent with the family's typical bound (the wave-1 check re-confirms scope on live
   evidence below).

This is a relationship RECOGNITION for routing. It does NOT set `canonical` / `duplicate-of` —
that classification remains a Scout (sibling discovery) + Grader (clock-ordering) product
([grading-rubric.md](grading-rubric.md) § Cross-incident classification). The coordinator records
the decision in `run.md` as `fast-lane: recognized` or `deep-lane: <reason>`. The decision lives
in the coordinator's judgment; when in doubt it fails open to the full deep-lane.

## The wave-1 duplicate-verification specialist (the dispatch)

On recognition, dispatch ONE bounded `full-evidence` specialist as an awaited dispatch
(`SKILL.md` § Execution model), economical/mid class, whose entire brief is to answer: **"is THIS
incident the SAME root cause as the family's previous incidents?"** It runs three moves within a
tight budget:

**a. Establish the family's previous root cause (as a CLAIM).** Because sibling
`ClaimedRootCause`/`IsNoise` fields are typically empty, the specialist reads the prior root cause
from a representative recently-resolved sibling's discussion/RCA via the read-only
incident-context/history capability (e.g. an incident-context fetch that INCLUDES a sibling's
descriptions/discussion), OR from this incident's own prior agent RCA in-thread. It treats what it
reads as a CLAIM, never authority. From that claim it extracts the prior root cause plus a
FALSIFIABLE discriminator: `expected_favored` = the known root-cause signature as it would appear
on THIS incident's live evidence, and `expected_rival` = what a genuinely DIFFERENT / real root
cause would look like on the same live evidence. If NO falsifiable discriminator can be formed from
any sibling or the own prior RCA, the specialist returns **can't-verify** (→ escalate).

**b. Fast live check (the minimal check).** The specialist STATES `expected_favored` and
`expected_rival` BEFORE reading (pre-registration), then checks that ONE predicate against THIS
incident's LIVE evidence with the deep-lane mechanism-discriminator gate rigor scoped to the single
predicate, recording the observed value, the gate status, and a cited `OBS###`. To return SAME the
live evidence must BOTH **(i) match `expected_favored` AND (ii) refute the rival** — a genuinely
different / real root cause. Within the same budget it also corroborates the incident's ACTUAL
scope from the live evidence and confirms it falls within the family bound (the intake-captured
scope is pre-Scout and not trusted alone). **Budget:** ≤ ~2 targeted reads / a tight time cap;
never a broad sweep, never a second hypothesis. If the predicate cannot be evaluated within budget,
that is a block, not a soft pass.

**c. Return.** The specialist returns **SAME** — the discriminator gate PASSED (the live evidence
matched `expected_favored` AND refuted the rival) and scope is in bound, carried by a cited OBS —
or **DIFFERENT / ambiguous / can't-verify** — the live evidence matches the rival, matches NEITHER
pre-named value (post-hoc, the CP-3 discipline in [grading-rubric.md](grading-rubric.md)), shows
scope wider than the bound, no falsifiable discriminator could be formed, or the check was blocked
(gate `fail` or `blocked`). SAME is licensed ONLY by gate `pass`; a `same` reconcile result with a
gate that is not `pass` is a self-contradictory receipt and is treated as DIFFERENT → escalate.

## RECONCILE (coordinator, action (f))

**SAME → FAST-TRACK + AUTO-POST.** Reconcile SAME ONLY when the receipt carries BOTH
`wave1_result: same` AND the wave-1 discriminator `gate: pass` with a cited live OBS — an
internally consistent verified-duplicate receipt. Compose the disposition: this incident is the
SAME recurring issue as the family (a duplicate of the known recurrence family), stating cause +
mechanism and citing the wave-1 confirming OBS (honesty floor: the disposition rests on THIS
incident's live check, never "a sibling said so"), plus the engineer next-step. AUTO-POST via the
existing poster path ([subagents/poster.md](subagents/poster.md) § Live incident-system posting) as
a **collaborator / additive duplicate-reference** contribution — an additive "same as the known
recurrence family, verified live" note, NOT a standalone re-derived RCA — under the SAME rules as
deep-lane posting: live posting requires BOTH explicit brief authorization AND a non-gated
incident-posting capability, passes the capability-owned idempotency/audit marker and AI-generated
disclosure, and fails closed on an unverified duplicate scan. Apply **post-mode parity**: when the
thread already carries a human root cause / mitigation / progress, contribute additively/
respectfully rather than overwrite it (disposition token `collaborator-duplicate-live`); otherwise
post the additive duplicate-reference note directly (token `posted-duplicate-live`). With no
authorization or no non-gated capability the run is report-only — that is not a gap. Render the
named `Known-recurrence` disposition ([subagents/poster.md](subagents/poster.md) § Post-body
structure / § Verdict policy), emit an observable fast-lane verdict + a `6_report`, and record the
FAST-LANE receipt + the `model_tiering` Tier Record in `run.md` ([run-store.md](run-store.md) §
Run-state digest).

The disposition NEVER sets `canonical` / `duplicate-of` and NEVER imports a sibling's verdict: its
duplicate assertion rests only on the recurrence-identity sibling relationship it VERIFIED live —
reconciling with [grading-rubric.md](grading-rubric.md) § Cross-incident classification, where
`canonical`/`duplicate-of` is exclusively a Scout + Grader clock-ordering product. A
sibling/duplicate linkage may be carried ONLY when it is already host-supplied — the incident
system's own duplicate linkage present in the intake bundle — rendered labeled
`incident-system-linked`, explicitly not a derived verdict.

**DIFFERENT / ambiguous / can't-verify → ESCALATE.** Run the normal deep-lane (Scout →
Specialists → Grader → Report), NO verdict bar lowered. The recognition + the wave-1 specialist's
OBS carry forward as intake context — a pre-registered discriminator check the deep-lane reuses.
Never post a duplicate on doubt. **Catch-all:** any wave-1 return that is not an internally
consistent `wave1_result: same` + `gate: pass` — including a missing/unparseable result or gate, or
any unexpected value — escalates to the deep-lane (defensive fail-open).

### FAST-LANE receipt (in `run.md`)

One compact block so the path is auditable:

```text
fast_lane:
  family: <recurrence-identity summary · cluster size>
  recognized: sev=<n> · scope=<bound> · convergent cluster
  prior_root_cause: <one-line claim> · source=<sibling id | own-prior-RCA-in-thread>
  discriminator: expected_favored=<...> · expected_rival=<...>
  wave1_result: same | different | ambiguous | cant-verify
  wave1_check: <OBS###> · observed=<...> · gate=<pass | fail | blocked>
  family_validated_marker: <seen: ts=<UTC> | absent> · fast_lanes_since_deep_validation=<n>
  disposition: posted-duplicate-live | collaborator-duplicate-live | report-only(<reason>) · incident_system_linked=<id | none>
  outcome: fast-tracked | escalated-<reason>
```

`wave1_result` is the specialist's returned answer and `wave1_check.gate` is the mechanism-
discriminator gate status — two independent facts: a live post is licensed only when BOTH are
`same` and `pass`. `disposition` is the reconcile outcome; `collaborator-duplicate-live` when the
thread already carries human root cause / progress (post-mode additive), `posted-duplicate-live`
otherwise (both are live, additive duplicate-reference posts) — additive to, not a rename of, the
deep-lane's `posted`; `report-only(<reason>)` stays the generic report-only token.
`family_validated_marker` and `fast_lanes_since_deep_validation` carry the drift-backstop state
(§ Recall safeguards #7).

## Escalation triggers (fail open — never dispose on doubt)

ANY of the following runs the **normal deep-lane** (Scout → Specialists → Grader → Report),
unchanged in depth and recall:

- **no convergent cluster** (no recurrence family — title/team-only or thin/ambiguous match);
- **no readable prior conclusion** (no sibling discussion/RCA and no own prior in-thread RCA
  carrying a root cause to verify);
- **no falsifiable discriminator** formable from any sibling or the own prior RCA;
- wave-1 **DIFFERENT** (live evidence matches the rival, or neither pre-registered value);
- wave-1 **ambiguous** (predicate evaluated but inconclusive);
- wave-1 **can't-verify / blocked** (predicate not evaluable within budget, or the prior-RCA read
  was blocked);
- **gate not `pass` / internally inconsistent receipt** (`wave1_result: same` without a wave-1
  `gate: pass`, or a missing/unparseable result or gate — the defensive catch-all above);
- **out-of-bound** Sev or scope (intake-captured Sev or scope outside the family bound);
- **live scope wider than the family bound** (the wave-1 live evidence shows the actual incident
  scope exceeds the bound, or scope could not be corroborated within budget);
- **verification-capability-unavailable** (the live evidence source, or the incident-context read
  for the prior root cause, is not reachable);
- **periodic re-validation floor tripped** (§ Recall safeguards #7) — no in-window
  `family-validated` marker for this recurrence identity (staleness), OR the consecutive-fast-lane
  cap (N=10) reached: a deliberate, computable route to the deep-lane even when the wave-1 check
  would say SAME.

On escalation, the cluster recognition + the wave-1 OBS carry forward as intake context — the
check becomes a pre-registered discriminator the deep-lane reuses — and NO verdict bar is lowered.
This is the existing intake "fail open on any uncertain signal" discipline.

## Recall safeguards (the floor under any post)

1. Recognition is read off the intake recurrence cluster — a real convergent recurrence family,
   never a single sibling guess and never run-local `7_knowledge` candidates or sibling run
   directories (the same isolation rule as Scout and [run-store.md](run-store.md) § Reading prior
   runs).
2. The wave-1 live check is **MANDATORY before any post**, runs the same discriminator-gate rigor
   as the deep-lane, and is observation-cited: there is NO post without `wave1_result: same` AND a
   wave-1 `gate: pass` AND a cited live OBS.
3. ANY uncertainty escalates (the trigger list above). Never dispose on doubt.
4. The disposition is RE-ESTABLISHED by the specialist on THIS incident's live evidence and cites
   the live confirming OBS; the family's prior root cause is never imported, and the duplicate
   assertion rests on the recurrence-identity sibling relationship verified live, NOT on an
   imported sibling verdict. The honesty/confidence floor is preserved.
5. SAME requires the discriminator `gate: pass` — BOTH matching `expected_favored` AND refuting
   the rival; a checked value matching neither pre-named value is post-hoc, and a `same` result
   whose gate is not `pass` is a self-contradictory receipt → ESCALATE.
6. The post is a **collaborator / additive duplicate-reference**, never a standalone re-derived
   RCA, and never sets `canonical`/`duplicate-of` beyond the recurrence-identity relationship it
   verified.
7. **Periodic forced re-validation (drift guard — computable floor).** The backstop is the sole
   structural guard against a subtly-wrong prior root cause perpetuating as an auto-posted
   duplicate, so its firing is FLOORED and COMPUTABLE from state the run already holds at intake —
   the IcM recurrence cluster + discussion threads — never coordinator judgment alone, never a
   ledger, an offline pipeline, or sibling run-dir reads. The coordinator forces the FULL deep-lane
   (instead of the wave-1 fast-lane) when EITHER floor trips:
   - **(a) Marker staleness.** No `family-validated` marker (below) for this recurrence identity is
     visible in the intake recurrence cluster / thread within a bounded recency window — a
     conservative default of **~30 days**. A family with no fresh deep-lane validation in-window
     MUST take the deep-lane.
   - **(b) Consecutive-fast-lane cap.** After **N = 10** consecutive fast-lane SAME dispositions
     for the family with no intervening deep-lane validation (counted from the markers visible in
     the cluster/thread — the fast-lane's own additive posts are labeled and countable, and the
     absence of a newer `family-validated` marker means no deep-lane ran since), the next family
     member MUST take the deep-lane. Record the running count as
     `fast_lanes_since_deep_validation` in the receipt.

   The floor is judgment-INFORMED, not judgment-ONLY: the coordinator MAY deep-validate more often,
   but N and the recency window guarantee the family cannot go unbounded-many fast-lanes or
   unbounded-time without a fresh deep-lane re-derivation. A deep-lane re-validation whose result
   DIVERGES from the family's assumed root cause records a `recurrence-drift` knowledge candidate
   (`7_knowledge`, [subagents/knowledge.md](subagents/knowledge.md)) for human attention AND — since
   there is no ledger — surfaces the corrected disposition in that run's IcM post, so the family's
   future intake reads see it.

   **The `family-validated` marker (the ledger-free state).** When a DEEP-LANE run validates a
   recurrence family's root cause, its IcM post carries a machine-readable `family-validated` marker
   — a stable tag + the recurrence-identity + a UTC timestamp — as a watermark line the poster
   emits ([subagents/poster.md](subagents/poster.md) § Verdict policy). ONLY a deep-lane disposition
   writes this marker; a fast-lane duplicate post NEVER does. Because the marker rides in IcM, the
   NEXT sibling's intake recurrence cluster / thread read surfaces it — the coordinator computes both
   floors above from those markers with no separate ledger and no sibling run-dir reads.

These compose with — and never override — the coordinator orchestration-only contract, the
redacted-is-not-absent floor, the duplicate/verdict-determinism gate, the model-tiering
fail-safe-to-reasoning-heavy rule, and the single-turn awaited execution model in `SKILL.md`.

## Illustrative example (the discriminator shape)

A recurring monitor artifact in the `Monitor-DropInTBAVolume` / `VolumeDropped25Percent` family:
the monitor's threshold rule fires on a 25% task-volume drop, but the family's convergent prior
root cause — read live from a recently-resolved sibling's RCA or this incident's own prior in-thread
RCA — is an **absence-of-data sentinel**, not a real decline. The wave-1 specialist re-establishes
that on THIS incident's live evidence:

- **Discriminator.** Read BOTH the windowed aggregate AND the raw sample series for the alerted
  metric over the incident window.
  - `expected_favored` (known root cause — absence-of-data sentinel): `Sum(metric, 1440m) == 0`
    with **ZERO samples** in the window — the metric stopped emitting, so the "drop" is a
    measurement gap, not a real volume decline.
  - `expected_rival` (a genuinely DIFFERENT / real root cause): a **non-zero, declining sample
    series** — samples are present and the value genuinely fell — which is a real decline →
    DIFFERENT → escalate to the deep-lane.
- The specialist reads the windowed `Sum` AND the raw sample `Count` live; a zero-sample window
  confirms the artifact (SAME), a present-but-declining series matches the rival (DIFFERENT), and
  any value matching neither (e.g. samples present at a flat-but-nonzero level) is post-hoc →
  ESCALATE.

This illustrates the discriminator shape and the read-both-signals live check; the real prior root
cause is always read live from the family, never hardcoded.
