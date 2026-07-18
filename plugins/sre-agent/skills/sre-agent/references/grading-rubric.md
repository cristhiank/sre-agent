# Grading Rubric

The grader is the adversarial judge: it refuses proximate-as-root while an answerable upstream lead remains, and closes every material lead with cited evidence or an honest dead-end. Shared rules live in [investigation-invariants.md](investigation-invariants.md); this file says how to judge.

## Verdict classes

- **Confirmed / Likely-rooted**: cause and mechanism are verified against an authoritative source for that mechanism; timing fits, the decisive discriminator is checked and cited, and material alternatives are addressed.
- **Proximate-only**: the named cause is the failure mechanism itself, the upstream "why" — including the trigger/precondition "why" when the mechanism is conditioned on a dependency/precondition state — is unexplained, or a reachable mechanism discriminator remains unchecked. A verdict that bottoms out on a present-but-stale durable entity (the entity sub-block of the specialist claim-readiness ledger reading `entity_class=handoff`) with a reachable upstream producer and `upstream_probe_attempted=no` (`reachability_outcome=unattempted_open`) is Proximate-only, NOT Root — "pipeline-output-is-not-a-leaf"; the producer-run/eligibility lifecycle axis below carries the keyed enforcement.
- **Inconclusive-blocked**: the next discriminating evidence is unreachable with available evidence sources. A keyless primary signal is not exhaustion while a generic pivot — time window ∧ affected scope/entity ∧ operation/step — into another next-causal-layer source remains unspent within the structural refinement allowance.
- **Refuted**: observations contradict the theory.

## Symptom-vs-cause test

A theory that names only an error, missing resource, timeout, exception, or failed job is **Proximate-only** unless it explains the upstream mechanism: what made that condition occur.

## Target-alignment gate

The primary RCA target is the intake `rca_target`: the underlying measured/impacted failure. When real unresolved measured failures exist, the failure-cause lead is primary; alert/monitor mechanics are secondary or contributing unless the signal is pure noise or the underlying failures are already explained.

The report's primary finding must address the declared `rca_target` or carry the `5_grader` discovery receipt proving that target unreachable or clean. No third path.

Pure-noise/tuning-artifact closure is valid only with a receipt showing the underlying failure dimension was inspected and found clean or immaterial. Any scope expansion beyond `literal_trigger` must cite evidence that the same measured failure exists in the expanded scope.

## Mechanism-discriminator gate

A proposed root cause is not eligible for **Likely-rooted** or **Confirmed** until the claimed mechanism is verified against an authoritative source for that mechanism, not merely inferred from symptom telemetry.

Shared probe-fitness rule: a reused/adapted curated query that returns empty is `reached_empty` and never refutes a live lead until schema/freshness and probe fitness establish what the empty result can rule out; citation relocation validates code anchors, not telemetry schema. This applies wherever reused-guidance queries run, not only the change-arrival gate.

Before promotion, the grader must record all four:

1. The proposed causal mechanism, including the specific state it corrupts or the decision it changes, not just the failure signal.
2. At least one plausible rival mechanism that produces the same observed symptom.
3. A pre-declared, falsifiable discriminator predicate with the expected observation under each mechanism, favored and rival, stated before checking. When the Scout coverage-map disposition (`consulted:<capability> -> covered:<asset> | no-coverage`) names a covered asset for this hypothesis, the expected favored/rival values must be predicted from (or reconciled against) that asset, with the asset cited as their provenance; the grader content-verifies that the cited asset actually supports the stated expected values — a citation string alone does not pass, and a cited asset that does not support the values is treated as no provenance. `no-coverage` is admissible only when the Scout coverage map recorded no covering asset after consulting the named capability; it cannot be self-asserted against a coverage map that names one.
4. The checked concrete observed value of that discriminator, from an authoritative live source for the claim; orientation knowledge can never fill this record (see "Aim before you probe" in [investigation-invariants.md](investigation-invariants.md)).

The discriminator predicate and per-mechanism expected values must be pre-registered before the checked value is observed: carried as an `open-answerable: mechanism-unverified` obligation or specialist hypothesis from a prior round/turn, or pre-declared in Scout's discriminator table or a specialist's first-pass note (predicate + expected favored/rival stated and kept visibly separate before the value was observed), then checked. Scout's discriminator table (declared by an agent that never checks the value) is the strong form; a specialist's own first-pass pre-registration is the weaker form — accept it only when the predicate + expected favored/rival are kept visibly separate from the checked value AND were not shaped by any observed/symptom-correlated result; if the predicate, the expected values, and the checked value first appear together with no visible separation and no prior Scout/refinement artifact, treat it as post-hoc. A discriminator first stated with its own result is post-hoc and does not pass; keep **Proximate-only** and run one focused verification turn against the pre-registered discriminator.

Multi-arm defects: when a confirmed code/config defect has more than one supported failure arm that produces the same observed symptom class, pre-register expected values for EACH in-scope arm, not only the favored one. Limit arms to those supported by the mechanism source, matching the observed symptom class, and plausible for the incident window/entity within one refinement slot and the applicable probe cap. Refuting one arm does NOT refute the defect while another in-scope arm matches; positive in-incident evidence of the defect's mechanism firing counts as support even when not keyed to the exact failing id. For drop/suppress arms, state whether the missing id is the EXPECTED signature — absence of the id can be the arm's signature, not its refutation.

A symptom-consistent narrative does not pass: "telemetry is consistent with X" is not a discriminator unless it names distinct expected values per mechanism and a checked result.

Authoritative source means whatever fits the claimed mechanism: implementation behavior, configuration or rollout state, runtime/control-plane state, maintained operational documentation, or telemetry that observes the mechanism itself. Require implementation reading only when the mechanism depends on implementation behavior; runtime, infrastructure, configuration, or control-plane claims can be verified by authoritative state for that claim.

If the surviving mechanism depends on implementation/config behavior, the checked mechanism must include the producer code/config path: the code/config that creates the corrupt state or decision, not only the runtime/telemetry layer that propagates or consumes the symptom. Telemetry that only shows the symptom is not sufficient for a code/logic-rooted cause.

Boundedness: require one serious same-symptom rival, not exhaustive enumeration. Code/config reading is scoped to the minimal producer path indicated by the discriminator, not broad archaeology. If that producer code/config is unreachable after the applicable read/probe caps are spent, use **Inconclusive-blocked** with the discovery receipt and engineer next step, not **Likely-rooted** on the symptom story. If the scoped producer code/config does not confirm the defect, disconfirm or downgrade the hypothesis; do not keep spelunking.

If the discriminator is reachable and unchecked, mark the lead `open-answerable: mechanism-unverified`, keep the verdict **Proximate-only**, and require one focused verification turn. If the discriminator is unreachable with available evidence sources, use **Inconclusive-blocked** with an engineer next step.

Verified-enough means the mechanism source was checked, timing fits, the checked discriminator supports the theory, and material alternatives are refuted or bounded as non-material. Stop when remaining gaps would not reasonably change the user-facing RCA; do not loop solely to raise confidence.

Verified-enough covers actionability, not only verdict confidence. A reachable check that cannot raise the verdict CLASS is still an `actionability-refinement` obligation — NOT dismissable as `corroborative-only` — when ALL hold: the verdict ceiling is Proximate-only and the consuming component/path is unknown; the pivot tuple (time window ∧ affected scope/entity ∧ operation/step) is already in hand; a capability-map-named source can answer it within the single post-synthesis batch and an available max-five slot; and the result would SELECT a candidate component/path/operation or DATE an app-level changepoint that changes the Manual Investigation Kit's next concrete operator step — not merely raise confidence. This obligation gates the DELIVERABLE (Manual Kit / reply-back), never the verdict class: it is recorded in the settle receipt (`generic_pivot_ladder` / `remaining_gap` / `structural_cap_used`), never promoted through `verdict_ceiling_lead` or Gate B. Each qualifying lead's obligation occupies at most ONE parallel-sync slot (one specialist) WITHIN the single post-synthesis batch — it never adds another batch — and is subordinate to verified-enough above; once that slot is spent, denied, discovery-exhausted, or structurally cap-exhausted, settle even if more precision is theoretically available. `corroborative-only` may retire only a check that would raise confidence without changing the operator's next concrete step.

Live-verdict provenance floor: before emitting `Likely-rooted` or `Confirmed`, apply the mechanism-discriminator gate and the failing-unit enumeration gate (when population/aggregate evidence is material) as a pre-promotion checklist, plus the disqualifier that an unverified change/version is actionability-only, never causal verification, so it cannot stand as the cause (this disqualifies; it does not require producing provenance to promote). A rooted verdict rests on a verified mechanism and checked discriminator against an authoritative source — quality, not count: a purely temporal or correlational association, an unkeyed convenience sample, or a named-but-unverified change treated as the cause cannot satisfy that bar, and stacking several such thin threads does not either. Follow the reachability disposition above (reachable-unchecked → `Proximate-only`; unreachable decisive mechanism check → blocked path), and do not over-hunt once verified-enough (above) is met. Record the result in the grader's `Confidence reducer / verdict cap` field — REQUIRED for every `Likely-rooted`/`Confirmed`, with `status=none` admissible only via the positive attestation that the discriminator is none of those thin threads.

Change-arrival gate (a VERDICT GATE, not a softener): a named change — a CODE introduction (commit/PR/build) OR an INFRA/CONTROL-PLANE introduction event (cert/secret/DNS/config/flag rotation or push) — cannot rank above `last-touch, not proven introduction` until its ARRIVAL/effect in the FAILING PROD SCOPE at/before onset is resolved. Apply the mapping as a crisp UNORDERED four-way — there is NO standalone "unverified ⇒ soften" license; the reachable-unfetched case MUST probe before any cap:
- `arrival=DISPROVEN` ⇒ **REFUTE** it as the cause: status `closed-refuted`, cap `correlation-not-causal`; it survives only as a ruled-out note and MUST NOT stand as a capped-but-named prime suspect. DISPROVEN criteria are symmetric — CODE: the serving release build predates the change's merge into the SERVING ref, or the change lives in a non-serving branch (no backport/cherry-pick into the serving branch present). INFRA/control-plane: the rotation/change's authoritative scope EXCLUDES the failing region/unit, OR the artifact served at onset (cert/config/flag value) is the PRE-change value (the rotation/push landed AFTER onset).
- `arrival=UNVERIFIED` **with REACHABLE provenance** (merge-target ref, release-build cut time, deploy/rollout record — or, for infra, the control-plane change-event/rotation log — normally reachable via a source-control/release-history or infra-state capability) ⇒ an OPEN `unattempted_open` lead: it is the `verdict_ceiling_lead` and MUST-dispatch ONE bounded arrival probe per Gate B. NO settle, NO soften, NO capped-but-named suspect while reachable-unfetched (prohibited).
- `arrival=UNVERIFIED` **with `denied` / `discovery_exhausted` / genuinely-unreachable** (the bounded probe was spent and returned a cited terminal) ⇒ cap that lead at **Proximate-only** with a `change-mapping-unverified` reducer — absence of arrival evidence is NOT proof of non-arrival, so this CAPS, never REFUTES, a genuine deploy (false-negative guard).
- `arrival=VERIFIED` (positive evidence it reached the failing prod scope at/before onset) ⇒ eligible to support `Likely-rooted`/`Confirmed` if the rest of the gate passes.

`reached_empty` terminal (a FIT probe — scope + window + source receipt — reaches the authoritative serving-build/deploy or control-plane record and finds no arrival): it RESOLVES to `DISPROVEN` when that authoritative record EXCLUDES the change (the serving-build manifest does not contain it / the served artifact is the pre-change value); otherwise it is a cited cap terminal like `denied`/`discovery_exhausted` (Proximate cap) — NEVER a frozen open lead. This resolution applies to the ARRIVAL question against an authoritative, COMPLETE serving-build/deploy/control-plane manifest only; the DISPROVEN branch requires that manifest to positively EXCLUDE the change (proof-of-absence), and the non-excluding branch is a `denied`/`discovery_exhausted` cap, not `reached_empty`. It does not extend the general `reachability_outcome` enum (see below / artifact-contracts.md), where a bare `reached_empty` still cannot cap. Arrival is resolved against merge into the SERVING ref, not the original merge target: `DISPROVEN` requires that no backport/cherry-pick of the change into the serving branch is present, and a hypothetical/unruled backport does NOT downgrade an established serving-build-predates-merge finding to `UNVERIFIED` — a backport must be POSITIVELY evidenced to move a lead off `DISPROVEN`. This applies symmetrically to code and infra/control-plane introductions: the introduction-provenance machinery is source-control-shaped, so an infra/control-plane rotation/push is an equally valid, equally arrival-gated introduction event with its own first-class path. Record the resolved value in the `Confidence reducer / verdict cap` field.

IMPORTANT: no asserted runtime STATE (node/process death, schema-missing, regression-introduced, etc.) without an authoritative NON-SYMPTOM source (runtime/control-plane/implementation source + observed value); the symptom metric itself — metric-zero, threshold breach, single sample, or temporal correlation — proves a MEASUREMENT, not a runtime state; absent a non-symptom source, mark status=correlation-not-causal (measurement-only) and cap at Proximate-only.

<bad-example>
Observed: the success-rate metric reads zero for a scale unit.
Wrong: "Root cause: the node/process is dead."
Why wrong: metric-zero is the symptom measurement; node/process death is a runtime state that needs host/control-plane evidence — none was checked.
Correct: "throughput measured zero; runtime state unverified" -> cap at Proximate-only and name the control-plane/host check that would confirm.
</bad-example>

When a **Confirmed** or **Likely-rooted** verdict is tied to a verified code/config/schema/artifact/service-owned location, emit an actionability-only introduction provenance obligation: closest introducing change or labeled last-touching change for the implicated path/symbol — or, for an infra/control-plane cause, the closest introducing rotation/push event (cert/secret/DNS/config/flag) for the implicated control-plane surface. This is post-verdict actionability only, may reuse the producer-code citation from this gate, and does not gate the verdict.

### Discriminator ladder (typed, pre-registered rungs)

The discriminator above is not a single predicate but a typed, layered **ladder** along the failing signal's OWN lineage — consumer → producer → upstream.

**Trigger/precondition axis.** When the surviving mechanism is self-sufficient on the consumer side (a missing guard, an absent defensive branch, an unset timeout) but is *conditioned on* a precondition/trigger state S — a dependency mode, elevated latency, resource pressure, a degraded upstream — i.e. "X failed because precondition P was in state S", the ladder MUST carry a **precondition/trigger rung** along this axis, not only the consumer→producer→upstream failure-mechanism axis. Its `claim` = "**what drove P into state S** for the failing population/window" (not merely that P was in state S); its pre-registered `favored_value`/`rival_values` name the **competing trigger drivers** (e.g. resource pressure vs a dependency-side change/deploy vs an upstream feed/state shift), so re-confirming that the dependency was slow/degraded does NOT close the rung — only resolving *which driver* put P in state S does. Carry `evidence_shape`, `observed_identity`, and `dispositive=true` while that "why" could change the verdict, owner, or cap. A consumer-side guard/defensive-code ABSENCE (or any self-sufficient consumer mechanism) does NOT settle `Likely-rooted`/`Confirmed` while this rung is `open`. When the rung is `unattempted_open`/agent-answerable, Gate B makes it a **MUST-dispatch** bounded follow-up — it does not cap; a final `Proximate-only` is valid only after that bounded follow-up is spent, OR the rung resolves to `denied`/`discovery_exhausted` with a cited receipt — exactly the upstream-"why" pivot, on the trigger/precondition axis. A specialist may add at most ONE child precondition rung when its own claim depends on an unexplained material precondition; deeper why-chains are pre-registered by Scout or the Grader's bounded follow-up, never recursed inside a specialist.

**Producer-run / eligibility lifecycle axis.** When the symptom is non-advancing producer output — no-emission / missing-data / sparse-metric / SLA-not-met-by-absence (a sample is *absent* rather than failing), OR a durable data entity that is *stale-but-present* (its freshness anchor behind a cited expected-freshness source for the failing window, per the entity sub-block of the specialist claim-readiness ledger) — the ladder MUST carry a **producer-run/eligibility lifecycle rung** along this axis, instantiated and dispositioned BEFORE any monitor-definition cap. Its `claim` = "what was the producer's run/eligibility lifecycle state for the failing target+window?", classified best-effort into one of: `not-eligible/expected-no-emission` | `eligible-but-not-attempted` | `attempted-but-failed/suppressed` | `produced-but-not-collected` | `produced-and-collected/monitor-artifact` — each carrying its evidence basis and a caveat, with `unknown/insufficient` kept as an honest class. Resolve it from PRODUCER-side reachable evidence first: run/scheduler telemetry; presence-vs-absence of the producer's own records across comparable in-scope targets; the source-code/config eligibility gate; control-plane/feature-flag state; and — for a DERIVED-metric alert (negation/difference/weighted sum) — arithmetic reconstruction of the alerted scalar from the raw authoritative producer signal. The **monitor null/missing-bucket policy is signal-validity evidence only**: it CANNOT be the decisive unreachable cap or the decisive Manual-Kit predicate while ANY producer-side eligibility/run evidence (config, scheduler/run telemetry, control-plane state, source path, or derived-metric reconstruction) remains reachable — demote the monitor-definition export to a NON-BLOCKING confirmation / fix-authoring kit item. The honest `blocked-unreachable` receipt is preserved ONLY for genuinely out-of-band producer state (a deeper authoritative store after the reachable producer evidence is spent); "Dead-end honestly" MUST NOT be invoked for the producer-eligibility question while it is still answerable (it is `unattempted_open`, MUST-dispatch per Gate B). This axis is DISTINCT from the Trigger/precondition axis but composes: the no-emission lifecycle rung fires FIRST; once a gate/eligibility-false state is found, the precondition/trigger rung then asks "what drove eligibility/precondition false?". For the stale-but-present branch the ceiling is keyed on that entity sub-block: `entity_class=handoff` with `reachability_outcome=unattempted_open` (a reachable upstream producer, `upstream_probe_attempted=no`) ⇒ refuse Root and emit a refinement obligation for the upstream-producer probe ("pipeline-output-is-not-a-leaf"); only `denied` or `discovery_exhausted` with the cited receipt licenses the proximate ceiling on that entity. When `freshness_expectation_source=indeterminate` and unresolved, the stale classification is unproven — treat the lead as an open obligation to establish the expectation, NOT a confirmed handoff, so no upstream walk is forced on an unproven-stale entity.

**Onset-signature / cause-class axis.** When onset is BROAD-SYNCHRONOUS — abrupt, ~simultaneous across many independent units/regions/scale-units, and sustained — AND app-code regression is excluded for the failing path, the ladder MUST carry an onset-signature rung that PROMOTES a shared-infrastructure / control-plane cause class (certificate/secret/DNS/config/flag rotation, shared dependency, infra topology change) as a LEADING hypothesis: a per-unit code/deploy change MUST NOT outrank it on merge-time correlation alone. `~simultaneous` = onset spread << the service's known rolling-deploy/ring cadence; when that cadence is unknown, treat synchronicity as UNVERIFIED and carry deploy AND rotation as CO-LEADING rather than demoting deploy. `App-code excluded` means `arrival=DISPROVEN` for EVERY named code change on the failing path, not an assumption. Staggered/rolling onset instead supports a rollout/deploy cause class; synchronous onset supports a shared-trigger/rotation cause class. Its `favored_value`/`rival_values` name the competing cause classes (shared-trigger/rotation vs per-unit rollout/deploy vs app-code regression). This is a cause-CLASS inference, but onset shape + code-exclusion ALONE is ELIMINATION-ONLY — it makes the shared-trigger/rotation class a LEADING hypothesis (feeding the infra/control-plane edge lens), NEVER a concluded class, because a broad-synchronous code-excluded onset is EQUALLY consistent with a provider/downstream outage, a shared-dependency throttle, capacity/quota exhaustion, or poison traffic. The cause CLASS CONCLUDES only when in-hand onset is COMBINED WITH a POSITIVE authoritative control-plane observation (a rotation/push/change event) whose scope matches the failing population. The class-vs-exact-artifact split still holds: once positively evidenced, the cause CLASS ("a rotation, not a code change") is REACHABLE and MUST NOT be parked `blocked-unreachable`; only identifying the EXACT rotated artifact (needs an out-of-band secret/config-store or infra control-plane capability) is the out-of-band Manual-Kit item.

Each rung is pre-registered exactly like the gate's discriminator (orientation-proposed, grader content-verified — a citation string alone does not pass), and carries:

- `claim` — the causal assertion this rung tests.
- `favored_value` + `rival_values[]` — the pre-declared per-mechanism discriminator values (favored vs each serious same-symptom rival), stated before the value is observed.
- `evidence_shape` — the acceptable probe/observation form.
- `dispositive` — whether either outcome of this rung could change the verdict, owner, or verdict cap. Orientation proposes it; the grader content-verifies BOTH values: a `dispositive=false` requires a cited bounded-as-non-material rationale (the same bar as a bounded-as-non-material refutation in this gate), so the hard rung cannot be silently demoted to dodge Gate B. If either outcome could reverse or cap the verdict, it is dispositive.
- `observed_identity{scenario, signal, owner, correlation_key}` — projected from an authoritative probe; each field non-null, or recorded `n/a-not-exposed` with cited justification where the capability does not expose it.
- `reachability_outcome` — the typed enum already defined in `artifact-contracts.md` §`5_grader/` (`reached | reached_empty | denied | discovery_exhausted | unattempted_open`), reused on the rung as-is, not redefined.
- `status` — `open | closed_confirmed | closed_refuted`.

Orientation arms a rung's favored/rival/dispositive (per "Aim before you probe" in [investigation-invariants.md](investigation-invariants.md)); it never closes a rung — only an authoritative live probe that passes Gate A moves `status` off `open`.

**Gate A — population-scoped identity match (bidirectional).** A rung reaches `closed_confirmed` OR `closed_refuted` only if its `observed_identity.correlation_key` matches the symptom's failing population AND `scenario`/`signal`/`owner` are projected non-null (or recorded `n/a-not-exposed` with cited justification where the capability does not expose them). Aggregate or healthy-population evidence cannot close a rung in either direction — it leaves `status: open`. This enforces the bidirectional "Bind to the authoritative failing population" and "Identity-match, not name-match" invariants in [investigation-invariants.md](investigation-invariants.md). This is the rung-level identity-projection specialization of the Failing-unit enumeration gate below: that gate establishes the population / key-compatible-join standard; Gate A additionally requires the projected identity fields (scenario/signal/owner/correlation_key) before a rung can close in either direction. A disposition that records the failing population itself **absent / unavailable / empty** is the same population-scoped close and is bound the same way: it is valid only when the probe's `observed_identity` is projected and its `correlation_key` matches the *enumerated* failing population. A negative / zero-row / "unavailable" result from a probe scoped to an adjacent, unimpacted, or unprojected cohort does not describe the failing population — for the failing-population lead this is `probe_fitness fit=no` / no FIT probe issued for that population, so the disposition stays `open-answerable` (re-probe fit-for-population) and never `reached_empty`, a closed rung, or an absence that can settle the lead.

<bad-example>
Rung claim: an upstream producer error is the cause.
Wrong (false-confirm): set `status: closed_confirmed` because a co-occurring error's column name *contains* the failing dataset's name, with `observed_identity.scenario` left unprojected — a lexical name-match.
Why wrong: `correlation_key` was never matched to the failing population and `scenario` is null; the label overlap ranks the lead, it cannot close the rung. Gate A blocks: identity unprojected.
Correct: project `observed_identity` from an authoritative probe carrying the scenario/correlation key; close only if it matches the failing population — else keep `status: open` and dispatch the projection as a follow-up.
</bad-example>

<bad-example>
Rung claim (the true producer rung): the per-unit producer mechanism is NOT the cause.
Wrong (false-refute): set `status: closed_refuted` on a service-wide invocation count (the healthy majority) showing the producer succeeding at full volume.
Why wrong: the aggregate is the healthy population, not the failing cohort; a per-unit mechanism cannot be ruled OUT on the healthy majority. Gate A blocks: evidence not scoped to the failing units' `correlation_key`.
Correct: scope the refutation probe to the enumerated failing cohort's keys; `closed_refuted` only if the mechanism is absent THERE — else `status: open`.
</bad-example>

**Gate B — promotion on closed dispositive.** `Likely-rooted`/`Confirmed` is settle-able only when every `dispositive` rung is closed (`closed_confirmed` or `closed_refuted`). An open `dispositive` rung whose `reachability_outcome` is `unattempted_open` is AGENT-ANSWERABLE: it MUST be dispatched as a bounded refinement obligation (per the Bounded refinement decision below) — it cannot cap the verdict and cannot be punted to a human. Only `denied` or `discovery_exhausted` on an open dispositive rung licenses a `Proximate-only`/blocked cap. A material attribution/mechanism/scope-binding gap the grader states in its OWN reasoning or verdict narrative IS such an open `dispositive` rung: it MUST be materialized as one (or as an explicit `mechanism-unverified` cap) and cannot coexist with a settled `Confirmed` — prose doubt in the narrative caps the verdict, it never merely decorates it. This composes with the reachability gate and the `verdict_ceiling_lead` receipt (see the Lead ledger and Bounded refinement decision below); it does not restate them.

**Gate C — anti-anchoring.** A prior AI or human conclusion enters the discriminator set only as an `open` rival rung tagged `provenance=inherited`, closable solely by independent re-derivation through Gate A. The hypothesis space is seeded from the capability/pipeline map (consumer → producer → upstream lineage), never from the prior claim's frame. This composes with the recurrence "claims, not truth" invariant in [investigation-invariants.md](investigation-invariants.md) and §`6_report/` collaborate-mode; it is not duplicated here. A discovered known/ongoing-issue asset (a failure-mode signature, TSG, runbook, AI asset, or historical RCA, from any read-only knowledge capability) is exactly such an inherited rung: it may order the hypotheses and arm a discriminator, but it is NOT an incident-history sibling and never triggers the `canonical`/`duplicate-of` classification on its own (that lane is reserved for siblings sharing the recurrence identity). When a known-issue rung's discriminator passes Gate A on this incident's failing population, the verdict may settle without an extra evidence batch through the normal mechanism-discriminator and failing-unit gates — the asset never lowers the verdict bar and never licenses a clean, benign, or by-design close on its own.

**Gate E — post-onset actor eligibility.** A change whose SERVING-ref merge/arrival timestamp (the same serving-ref resolution the change-arrival gate uses — NOT the feature-branch date; a responder cherry-pick/backport into the serving branch after onset carries an earlier feature-branch date but a post-onset serving arrival, and Gate E fires on that serving arrival) is AT/AFTER incident onset, or that is a revert/rollback/mitigation/failover of a suspected change, is a REMEDIATION action, not an introduction — CAUSALLY INELIGIBLE as evidence of what introduced the incident, REGARDLESS of arrival (a revert DOES reach prod, so it passes the change-arrival gate cleanly; this is a separate temporal+actor filter). It may corroborate WHICH artifact responders suspected, but it is NOT causal corroboration of the root cause and never lifts the verdict. When merge–onset separation is within the onset-estimate uncertainty, treat timing as UNVERIFIED (dispatch to tighten onset), not automatically ineligible. Gate E bars the revert ARTIFACT as an introduction; it does NOT bar a targeted rollback's OBSERVED EFFECT on the symptom from serving as a discriminator for the reverted cause class. Where Gate C guards a prior CONCLUSION entering as fact, Gate E guards a responder ACTION entering as causal evidence.

## Failing-unit enumeration gate

After target-alignment and signal-validity establish that real underlying failures exist, and the cause claim rests on an aggregate signal (a rate, count, ratio, or threshold breach), one population standard governs both promoting and refuting a rival. Promote only when the theory accounts for the authoritative failing population — the concrete failing units, or a justified keyed cohort, keyed to the producer; a count or time coincidence without a key-compatible join is salience, not proof. A refutation resting on a population join or empty result counts only from a fit-for-population probe (an empty join with incompatible ID-space/scope, or a non-authoritative source, is a probe defect, not refutation, per "Empty is not absent" in [investigation-invariants.md](investigation-invariants.md)); discriminator-predicate or bounded-as-non-material refutations (per the mechanism-discriminator gate) are unaffected. Exact failed-ID joins are strongest but not the only valid form — any key-compatible join to the producing population (keyed cohort, per-failure records, or direct mechanism telemetry) also qualifies. If units were enumerable but not inspected, mark the lead `open-answerable`, keep **Proximate-only**, and require one focused enumeration pass — which precedes the producer code/config verification the mechanism-discriminator gate requires. The gate does not fire on a pure measurement/threshold artifact, when the mechanism is already verified from a direct non-aggregate source that is itself the authoritative failing population (not a coincident side-population), or when fit-for-purpose enumeration was attempted and the units/keys are genuinely unreachable — the last becomes `blocked-unreachable` with the discovery receipt.

The counted entity and key semantics must match. A request/trace/correlation key may
enumerate that activity class; it cannot establish a session/user/tenant/resource count
without an authoritative mapping. Blank/null values are excluded before distinct counts,
or population coverage remains `unknown`; an empty string counted as one distinct value
is a probe defect, not one affected entity.

When the RCA target is an aggregate/population signal and real failures exist, failing-unit enumeration is not complete merely by binding keys to the producer. When the affected units/cohorts are reachable, run ONE bounded **population-decomposition pass** before settling at "alert valid / aggregate failing": derive the failing units or a justified keyed cohort; select and rank 1-3 operator-resolvable exemplars by measured contribution to the alert, then actionability; deeply trace only the highest-ranked reachable exemplar to its next-causal-layer discriminator; and run a cheap **homogeneity check** over the failing population. Remaining exemplars use already-cited evidence; do not add probes merely to deepen every entry. Each exemplar records a unit class (never assume a person), sanitized key or durable source pointer, contribution basis, next-causal clue (deep result for the primary; cited result or typed unknown for others), and owner/action effect; a cohort states its grouping basis. If contribution is unavailable, type it and rank by actionability rather than inventing a score. Type missing fields `not-emitted | redacted | query-capped | unjoined | unavailable`; when a load-bearing field remains missing, emit exactly one executable next check for the ranked set: the check most likely to change cause, owner, or action. Classify population coverage as `shared-pattern-supported` / `multiple-cohorts` / `per-entity-heterogeneous` / `unknown-insufficient`, with the sample basis and a generalization caveat; never project the primary exemplar across the population unless the homogeneity check supports it. This is depth on one exemplar plus a bounded operator shortlist, NOT exhaustive enumeration; stop after the ranked set and homogeneity guard in the single refinement allowance. A skipped-but-reachable decomposition is `open-answerable`, not a settled aggregate.

## Cross-source reconciliation

When a material claim rests on a quantity that two or more sources should measure over the same population and window, and they disagree beyond expected sampling/scope tolerance, the disagreement is itself a lead, not a footnote. Reconcile it within the pass — scope, sampling, scaling, dedup, ID-space, or time-fit. If it stays unreconciled, name the authoritative source for the claim, carry the gap as an explicit caveat, record the cap in the `Confidence reducer / verdict cap` field, and escalate to a refinement obligation only when it meets the `open-answerable` bar. Do not silently adopt one figure or average them. This is horizontal (two in-hand sources disagree), distinct from the cross-source PIVOT in the Bounded refinement decision (escalating a *blocked* lead via in-hand keys to an unprobed next-causal layer). It composes with, and does not replace, the failing-unit enumeration gate (which binds an aggregate down to its concrete units).

Likewise, a common symptom across independently owned sources does not establish one
shared mechanism. A shared-cause attribution requires authoritative closure for each
source or a verified common boundary that directly explains each source's observation;
otherwise report per-source findings and open closures, and cap the unified attribution.

## Cross-incident classification and verdict determinism

When the scout's recurrence check reports a recent or concurrent incident sharing this incident's recurrence identity (the identity defined in the recurrence invariant — see [investigation-invariants.md](investigation-invariants.md)), the grader classifies the current incident as `canonical` or `duplicate-of <incident>` rather than judging it in isolation, and records the relationship so the report cross-links it. Selection is over the sibling set the scout actually observed (a run sees only siblings already created within its recurrence window), so scope the claim as canonical-as-observed: prefer an external incident-management parent/duplicate link when the scout recorded one (recurrence output carries any parent/duplicate link the incident-history exposes); otherwise order by one clock chosen for the whole comparison — authoritative trigger-fire time if every ordered sibling exposes it, else declared start if every ordered sibling exposes it, else created/opened time — and the canonical is the earliest sibling on that clock, tie-broken by stable incident id. Compare only siblings that expose the chosen clock; a sibling lacking it, or whose time is too coarse to order against the others, is a related/unordered match rather than silently ordered, and if no shared clock exists across the observed siblings, record them as related with no time-canonical. The current run is `duplicate-of <canonical>` unless it is itself the canonical. This classifies the relationship only; it never imports the sibling's cause, verdict, or mitigation as truth, and each run still grades its own evidence independently.

Verdict determinism: the grader applies the same verdict gate to every incident sharing the recurrence identity, so independent runs on equivalent evidence converge rather than diverge — convergence is an emergent property of applying the gate, not an instruction to match or import a sibling's verdict. When the trigger is an alert/monitor/SLO, **Confirmed** REQUIRES that the alerted condition be independently reconstructed from authoritative raw/service-owned signals — its numerator, denominator, and scope, over the incident's declared window from intake — AND rooted to a verified producer per the mechanism-discriminator gate; this reconstruction is required whether or not the authoritative trigger definition (numerator, denominator, window, threshold) is reachable — an unreachable definition is only the most common occasion for it, never the sole condition, and a reachable definition still does not license **Confirmed** without the reconstruction. Until it exists, this rule sets only the MAXIMUM verdict that trigger-definition evidence allows — cap at **Likely-rooted**; the mechanism-discriminator and failing-unit enumeration gates can still lower it, and target-alignment still governs whether the underlying failure is real. When the incident-context capability decoded the alert's own embedded telemetry/monitor query deep-link, that decoded query is the cheapest authoritative reproduction path and is the one to run; once the alerted condition is reconstructed the unreachable trigger definition is no longer load-bearing and the report states the trigger-definition gap as immaterial. Does not qualify (cap stays at **Likely-rooted** or lower): incident narrative or alert summary only; aggregate or proxy telemetry that does not reconstruct the numerator/denominator/window/scope of the alerted condition; raw failures without threshold or scope equivalence; or a producer not verified by the mechanism gate.

The consistency engine is the discriminator ladder applied across siblings:

- **CP-1 — signature binds to a canonical ladder.** A symptom binds by signature (capability-class symptom + lineage shape) to a canonical discriminator ladder: the KB signature template (see `artifact-contracts.md` §`7_knowledge/`), or — for a novel fault family — bootstrapped from the service's own pipeline/capability map. Similar incidents therefore load the SAME rungs with the SAME pre-registered favored/rival values, so two siblings discriminate the same lineage the same way.
- **CP-3 — advance only on a pre-named value.** A rung's verdict advances only when the checked value equals a pre-registered `favored_value` or one of its `rival_values`; a checked value matching neither is post-hoc and does not advance the rung (per the pre-registration discipline in the mechanism-discriminator gate).
- **CP-4 — siblings reconcile or split.** Siblings of one signature reconcile to ONE canonical cause, OR split (two distinct cohorts = two ladders — the count/population-discrepancy path), OR separate a confirmed mechanism from a demonstrated human-only upstream residual. Convergence is on process + evidence-bar, never on a pre-closed conclusion: each run still independently passes Gate A on its own failing population, so a wrong canonical template is repaired or deprecated in §`7_knowledge/` (Gate D) — the CP-4 split/residual outcome — rather than re-served to the next sibling.

Cross-incident attribution tiers (for a cross-signal cascade candidate; same-signal siblings stay under the canonical/duplicate-of classification above): attribute only as strongly as the evidence supports — `related` (overlapping time/resource only; no causal-direction claim), `cascade-candidate` (overlapping window PLUS either a shared failing resource/key or a directly dependent component on a known dependency edge, with the related incident starting first; title/team or bare co-residency alone is insufficient), or `likely-downstream-of <incident>` (a cascade-candidate AND this incident's own evidence matches a downstream failure mode documented in service dependency/topology knowledge, a runbook, or verified prior-incident evidence — not inferred from the related incident's title — AND no stronger local cause survives). Absent that local match, cap at `cascade-candidate` and cross-link rather than attribute or route, under the same no-import rule above.

<bad-example>
Observed: an upstream storage/capacity-saturation alert on a shared resource precedes a downstream service's query-success-rate drop on that same resource.
Wrong: grade the downstream incident "root cause: upstream resource exhausted" by importing the upstream incident.
Why wrong: the upstream incident is a hypothesis prior; this incident was not independently checked for the expected downstream signature (e.g. allocation/load failures keyed to the saturated resource), and a stronger local cause may survive.
Correct: surface it as a cascade-candidate, test this incident's own evidence for the downstream signature, and only then attribute "likely downstream of <incident>" + route to the upstream owner — else cross-link as related.
</bad-example>

## Adversarial pass

During initial synthesis, ask: "what would make this theory false or incomplete?" and
"why did this condition exist?" Re-derive defeat modes from the goal + evidence; do not
rubber-stamp the specialists' conclusion. The final independent check is the bounded
consequence audit below, not another broad grading pass.

## Bounded consequence audit

After initial synthesis and the optional single post-synthesis batch, perform one final
reasoning-only audit. It replaces a broad second re-grade.

1. Read only the provisional ranking, `claim-integrity.toon`, returned obligation deltas,
   and the cited `OBS###` rows needed for the selected claims. Do not reopen Scout or
   Specialist narratives, fetch evidence, or emit another obligation.
2. Run a deterministic semantic scan over every consequence-bearing row: state plane and
   time basis; population entity/key fit and empty policy; observed-versus-claimed
   protocol layer; negative-evidence rival symmetry; and per-source attribution closure.
3. Build the independently re-derived set in two parts. First, independently determine
   from the provisional ranking and cited observations whether change-arrival,
   onset-signature-to-cause-class, or Gate E post-onset actor eligibility was triggered;
   do not trust a synthesis-time trigger flag or row omission. ALWAYS include every
   triggered consequence-bearing claim in this PINNED anti-miscausation set. A missing
   expected claim-integrity row is itself an audit failure and blocks that claim from
   publication. The pinned set is exempt from the top-two ranking and is never ranked
   out. Then select at most TWO additional
   claims whose failure would most change the verdict, owner, mitigation, operator
   action, or incident-state/recovery wording. Independently derive every selected
   claim from its cited evidence and rival predictions, ignoring the prior prose
   conclusion. When neither the pinned set nor another claim is present, the semantic
   scan alone is the audit.
4. Write `5_grader/consequence-audit.toon`, finalize the affected Material Claim Integrity
   `publish` dispositions, and update only the ranking fields those dispositions change.

Output cap: `~3KB`, no tools. If the audit cannot finish, mark unaudited
consequence claims `qualified` or `blocked`, retain the gap, and proceed to Report. A
failed audit never creates another evidence batch.

## Independent residual discovery

Before evaluating this gate, read the prior `5_grader/ranking.md` and
`5_grader/refinement-obligations.md`. If an obligation with
`origin: independent-residual-discovery` already exists while the receipt is absent or
`unconsumed`, recover by writing `consumed` with outcome `recovered:<id>` and do not
emit another obligation.

On the first Grader pass for which ALL four conditions hold, perform one independent residual-discovery attempt:

1. A real underlying failure and its concrete failing population or justified keyed cohort are bound.
2. The causal origin, producer, or trigger remains open because current support stops at a measurement, consumer symptom, propagator, or ownership handoff.
3. No Scout, core, edge, or pursuit obligation already owns a discriminator capable of closing that gap.
4. No prior `5_grader/ranking.md` exists (the first Grader pass), or its receipt says
   `Residual discovery: unconsumed`. After the recovery check above, an existing
   ranking without the receipt and without a residual-obligation witness is malformed
   and does not satisfy this condition.

Derive candidates from the run `rca_target`, the bound failing population/cohort, and merged `OBS###` observations — not from Scout hypothesis names, worker names, or inherited conclusions. Select at most ONE highest-material omitted hypothesis class whose resolution could change the verdict, causal owner, or user-facing action. Normalize it conservatively to the existing canonical hypothesis-class key and dedupe it against the lead and obligation ledgers: attach to an existing matching lead that lacks the closing discriminator rather than creating a duplicate. Monitor-only/side-signal evidence and cross-ownership handoffs do not independently trigger residual discovery when an existing signal, dependency, or pivot obligation already owns the discriminator.

When one eligible class survives, pre-register exactly ONE discriminator predicate with
expected favored and rival values, then persist ONE standard scoped refinement
obligation using the **Bounded refinement decision** shape below plus
`origin: independent-residual-discovery`; retain the lead in the existing
`open-answerable` state. Only after that obligation is durable, write
`Residual discovery: consumed` with outcome `obligation:<id>`. If no eligible class
survives, write `consumed` with outcome `no-eligible-class` and emit no obligation. The
Grader owns both writes and cannot fetch or dispatch.

The attempt is single-shot. Carry `consumed` and its outcome forward on every regrade.
If the gate does not open, retain the prior receipt; initialize an absent receipt as
`unconsumed` / `not-triggered` only when no prior `ranking.md` exists. If a prior
ranking exists without the receipt and no
`origin: independent-residual-discovery` obligation exists, write `consumed` /
`malformed-prior-receipt`, record that explicit Grader-state gap, and do not emit a
residual obligation. An existing recovery-witness obligation always takes the recovery
branch above. A residual obligation or its returned result cannot activate residual
discovery again, broaden the class, spawn a sibling/child residual, or create an
additional pursuit round.

## Lead ledger: no silent null-close

List each material lead or hypothesis from scout + specialists and close each as:

- `closed-supported` with cited `OBS###` evidence,
- `closed-refuted` with cited `OBS###` evidence,
- `open-answerable` with the missing discriminator and reachable evidence shape,
- `blocked-unreachable` with an explicit access/source limitation.

A lead whose hypothesis the Scout coverage map marks orientation-covered is not gate-ready unless its discriminator's expected values cite the covering asset as provenance AND the grader has content-verified the asset supports them (per record #3 of the mechanism-discriminator gate); a covered lead that advanced on own-observations without that cited, content-verified provenance bounces to `open-answerable`. A `no-coverage` hypothesis (the coverage map found no covering asset) still settles honestly. A hypothesis advancing via own-observations toward a rooted verdict with NO coverage-map disposition at all — Scout omitted it, or it emerged after Scout in a follow-up — is not gate-ready either: it requires a coverage-map disposition for THAT hypothesis (`consulted:<capability> -> covered:<asset> | no-coverage`: a cited, content-verified best-fit orientation asset OR an explicit `no-coverage` from the discovered-capability coverage scan), so an absent disposition bounces the lead to `open-answerable` until the scan produces one. Exception: when the run carries run-level `curated_guidance_reuse=bypassed:high-severity-triage`, the absent per-lead coverage-map disposition caused by that whole-shelf bypass is disposition-satisfying and does NOT bounce or cap the lead; the live checked value required by mechanism-discriminator gate record #4 is still required. A covered orientation asset never closes a lead and never fills the checked value — it only predicts the expectation the authoritative live probe confirms. Observability-only telemetry: carry `curated_guidance_reuse` — the per-lead specialist reuse mode `reused | adapted | rejected-because:<reason> | not-applicable`, plus the run-level `bypassed:high-severity-triage` stamped once per run (never per-lead); receipt semantics are canonical in `artifact-contracts.md` § Reusable guidance receipt. For `rejected-because:<reason>` or `not-applicable` on a `covered:<asset>` lead, the Grader records a soft observability note if the receipt names no inspected asset element; this adds no penalty and no new bounce.

A causal lead targets the declared `rca_target` / actual failure cause. It may be closed
`blocked-unreachable` only with the discovery receipt defined in
`artifact-contracts.md` §`5_grader/`, proving target discovery was exhausted from
sourced candidates, not asserted. Name the `verdict_ceiling_lead` — the open causal lead
targeting `rca_target` whose block would downgrade the verdict — at settlement (co-equal
leads each carry the receipt). The grader MAY NOT write `blocked-unreachable`, and MAY NOT
cap the verdict at Proximate-only-due-to-block, unless that lead's
`reachability_outcome` is `denied` (cited verbatim returned error/sentinel string + probe
call_id) OR `discovery_exhausted` (cited capability-map absence, required bounded
discovery sequence completed) — per `artifact-contracts.md` §`5_grader/`; there is no unattempted-but-blocked
state. A `reachability_outcome` of `unattempted_open` or `reached_empty` cannot cap (the sole exception is the arrival-gate resolution against an authoritative, complete serving-build/deploy/control-plane manifest per § Mechanism-discriminator gate → Change-arrival gate, where a fit `reached_empty` that positively EXCLUDES the change resolves to `DISPROVEN`): the
grader spends one bounded follow-up (cross-source pivot or next-layer probe — including a
probe in a source/cluster already held in-run, reuse-before-probe applying) or keeps the
lead Inconclusive-pending — open-answerable pending that bounded follow-up, a non-terminal
pursuit state, NOT the settled `Inconclusive-blocked` verdict — never `blocked`. Bounded discovery sequence: (1) observability/service-
knowledge lookup, (2) schema/catalog discovery against sourced candidates, (3) one
correct/authed probe against a discovered diagnostic source. No invented or guessed
targets; no receipt means the causal lead remains `open-answerable` or needs another
focused follow-up.

No material lead is silently dropped.

## Bounded refinement decision

Mark a lead `open-answerable` when it targets the upstream **or
precondition/trigger** "why" or mechanism verification, a reachable evidence path
exists, the result could materially change the verdict, and it fits one focused
specialist slot. Emit a bounded refinement obligation carrying: obligation and lead
ids; pre-registered predicate and favored/rival values; acceptable evidence shape;
in-hand keys; prior OBS ids with reuse/freshness mode; stop condition; dependency ids or
an independence reason; and allowed `answered|invalid-premise|unanswerable` outcomes
with their diagnostic payload.

Rank independent obligations by consequence and select at most five into the single
awaited parallel-sync post-synthesis batch. Every overflow obligation remains
`open-answerable` with reason `fanout-cap`, activation evidence, and its next
discriminator; no additional sub-batch exists. Obligations are independent only when
none depends on another's result for keys, failing-unit scope, source selection,
predicate validity, materiality, or live-source safety. Combine obligations that share
the same missing discriminator/source, and serialize only on a genuine dependency or
source-safety constraint.

A refinement specialist checks only its assigned discriminator plus the minimal
premise/freshness validation needed to decide answerability. Reuse settled/static OBS
only when they exactly cover the predicate, source, keys/failing population, incident
time, and scope; otherwise perform a fresh narrow read, always doing so for volatile live
values or a cross-source pivot. An `invalid-premise|unanswerable` return never closes a
material lead; the Grader reconciles it in the lead ledger.

A reachable path includes the access invariant's cross-source pivot: use in-hand
correlation/identity keys or, when the primary signal is keyless, the **generic pivot
ladder (time window ∧ affected scope/entity ∧ operation/step)** against a
capability-map-named next-causal-layer source (telemetry, logs, traces, config, code, or
control-plane state). That pivot is a required first call within the selected slot.
Only a spent fit-for-purpose probe, or the absence of any named source that consumes
those dimensions, permits settlement. For a no-emission/SLA-by-absence symptom,
producer-run or eligibility evidence is such a reachable path and remains
`open-answerable` (MUST-dispatch when a max-five slot and the single refinement wave are
available) before any monitor-definition cap.

All post-synthesis obligations share ONE awaited batch. Rank the complete set by
consequence and dispatch at most five; every overflow obligation remains
`open-answerable` with reason `fanout-cap`, activation evidence,
and its next discriminator. Once the batch is consumed, any newly
exposed answerable lead remains `open-answerable` with reason `wave-cap`; it does
not create a second batch. Dispatch selected attempts only after synthesis has settled
and only when they are material, reachable, non-duplicative, and within the max-five
slot cap. If none qualifies, record `post_initial_batch=not-dispatched` and proceed to
the bounded consequence audit; `MUST-dispatch` elsewhere means "when the single
refinement wave and a max-five slot are available."

IMPORTANT: do not settle Inconclusive-blocked or a final Proximate-only — and do not emit `dispatch: no` — while the discovery receipt's cross-source-pivot, its `generic_pivot_ladder`, and `in_hand_branches_dispositioned` (and, when the trigger is signal-shaped, `signal_validity`) remain unspent within the single refinement allowance — each completes only as a probed source + observed result, or a terminal value cited to its source/absence (not a soft "unavailable"; for `signal_validity`, the routed capability's disposition is the completing result); see [artifact-contracts.md](artifact-contracts.md) §`5_grader/`. These passes share that one refinement wave and consume at most one focused pass each. When no reachable in-hand-keyed path and no capability-map-named source consuming the generic ladder's dimensions remain, or the applicable wave/probe cap is exhausted, that terminal IS the receipt — settle and name the engineer next step; do not loop.

## Time-dependent held-branch closure

A branch has a **time-dependent reactivation predicate** when its disposition depends
on a live value that can change after the branch's prior authoritative evidence window,
such as recurrence, still-firing, or still-degraded. A static refutation or close has no
freshness obligation.

During synthesis, the Grader identifies every held or conditionally closed branch with
such a predicate and emits exactly one bounded fresh-read obligation through § Bounded
refinement decision. The coordinator dispatches the selected obligation; the Grader
remains fetch-free. Obligations sharing the same source, signature, and window may share
one read, but every branch keeps its own receipt. The read:

- evaluates this branch's own signature, never a proxy or sibling;
- covers without a gap from the end of the branch's prior authoritative evidence window
  through the report-time target; and
- establishes source coverage, including ingestion or watermark fitness, for that
  window. An empty result without proven coverage is not a negative.

Map the fresh-read result into the lead ledger:

- `fired` -> reopen the branch `open-answerable` with reason `predicate-fired`, cite the
  fresh evidence, and carry its separate owner/action consequence plus next
  discriminator into Report;
- `not-fired` with proven window coverage -> retain the branch's prior evidence-based
  disposition and record the predicate discharged;
- empty, unavailable, or ingestion-lagged coverage -> keep it `open-answerable` with
  reason `predicate-unproven`; and
- no fitting slot before the protected Report tail -> keep it `open-answerable` with
  reason `predicate-unclosed`.

A sibling branch's result never discharges this predicate. The contract adds no second
evidence batch and changes no verdict bar by itself; ordinary consequence/materiality
rules determine whether the reopened branch changes the primary verdict. Final
settlement records the per-branch receipt from
[artifact-contracts.md](artifact-contracts.md) §`5_grader/`; it never converts an
unproven or skipped predicate into closure.

## Known-issue acceleration settle

When the run took the known-issue-first staged path (the coordinator narrowed the first wave to a discovered known/ongoing-issue candidate's discriminator), the Grader may settle and decline the deferred fanout ONLY when that candidate's checked discriminator passes the full mechanism-discriminator and failing-unit enumeration gates on THIS incident's authoritative live evidence AND its serious same-symptom rival is refuted or bounded non-material on live evidence — the same bar as any rooted verdict, reached without an extra evidence batch, never lowered, and never a clean/benign/by-design close on the asset alone. It fails open to the deferred normal fanout on any divergence: the live signal exceeds the known issue's expected scope/severity/duration envelope, the rival is not refuted, the discriminator is unreachable, or the match is ambiguous (including no pipeline-derived rival named). The Grader records a one-line acceleration receipt — candidate source asset/capability · checked-discriminator OBS id(s) · failing-unit gate status · rival status · incident-scope/severity/duration match · decision (settle | fail-open-normal-fanout) · reason — so the path is auditable (the report and `run.md` render it). On settle, the Grader resolves every `deferred-by-known-issue-first` lead — either bounding it non-material or `closed-refuted` on the known issue's failing-population evidence with a one-line rationale (the verified known-issue mechanism plus the refuted serious rival explains the measured failing population, so a co-symptom hypothesis over that same population is bounded-as-non-material per the mechanism-discriminator gate), or, on fail-open, dispatching it in the normal one-per-hypothesis wave; no deferred lead is silently closed. A known-issue match never sets the `canonical`/`duplicate-of` classification; that lane stays the recurrence-identity sibling set.

<bad-example>
Lead: failures with in-hand correlation ids to a provider or next-causal-layer telemetry source whose schema was already discovered this run.
Wrong: settle "Inconclusive-blocked — provider telemetry unavailable."
Why wrong: the source was reachable (its schema was just read); "unavailable" was asserted, not probed — an assumed-block. Its typed `reachability_outcome` is `unattempted_open` (no probe issued), not `denied`, so it cannot cap the verdict.
Correct: run the failing-unit join against the source, cite the observed result, then settle at the verdict the evidence supports.
</bad-example>

### blocked-unreachable

> **Reachability floor (unconditional):** a `blocked-unreachable` cap or `dispatch: no` is valid only after every decisive-scope candidate — the primary AND every capability-class-fit or scope-derived same-class alternate whose evidence could change the verdict — is CLOSED by exactly one of: (a) a phase-current, read-only probe of THAT candidate returning a terminal denied / missing-source / schema-absent error, cited by call_id + verbatim result; or (c) a gated cross-class structural non-fit — the candidate's capability class from the taxonomy `{metrics-query | log/trace-grep | GUI-only-reader | elevated-entitlement-read | control-plane/config}` (established from a schema/catalog probe call_id or explicit read-shape citation of the candidate's own interface/schema) plus a stated reason why that class structurally cannot carry the decisive-scope data model; the class + citation are recorded in `alternate_capability_pivot`; (c) is NEVER valid between same-class candidates. A currently reached catalog/runtime surface lacking the decisive field does not by itself establish (a) or (c), and neither does the absence of a supplied owner reader, operational source, or command. While a bounded source-finding or discriminator probe remains available, retain one consolidated `unattempted_open` obligation to locate and probe a fit source, then dispatch it; this does not reclassify a genuinely-empty fit probe (`reached_empty`), which discharges the pivot obligation. A fit alternate that already `reached`/`reached_empty` DISCHARGES the pivot obligation — settle on that evidence; do not re-subject it to (a)/(c) (see also `natural-ceiling` below). A **discovery artifact** (scout output, derived map, or enumeration generated this run), a KB/docs note, a documentation gap, or absence-from-a-map may AIM and RANK probes but NEVER closes a candidate. Consolidate to the weakest/most-open outcome across candidates; if any decisive candidate is unprobed, malformed, uncertain, cap-skipped, or only scout/doc-excluded, emit `unattempted_open` (a scoped follow-up), not `blocked-unreachable`. Probe first, classify after.
>
> **Terminology:** *CAPABILITY MAP* (and any scout enumeration) — **agent-built every run** ⇒ a **discovery artifact**; it aims and ranks probes, NEVER closes a candidate. Mere absence or silence from any map or enumeration is never the required explicit negative. *Discovery artifact* — scout output, derived map, or enumeration generated during this run; routes and ranks, NEVER closes. KB/docs notes are orientation — neither class. This agent has no provided/static authoritative capability inventory; there is no inventory-based closer. Full field definitions: `artifact-contracts.md` §`5_grader/`.
>
> **Cap-finalization falsifier check (required before emitting `blocked-unreachable`, or a `natural-ceiling`/reachability-based cap on a next-causal-layer `verdict_ceiling_lead`):** name every decisive-scope alternate with (a) its phase-current probe call_id + verbatim returned error, OR (c) a gated cross-class structural non-fit tag — capability class from the taxonomy (per floor above, established from a schema/catalog probe call_id or read-shape citation) + stated reason the class cannot carry the decisive-scope data model. When the trigger is a `natural-ceiling`/reachability cap on a next-causal-layer `verdict_ceiling_lead` reached through a `reached`/`reached_empty` primary, ALSO discharge (v) the VERTICAL obligation: a cited spent `generic_pivot_ladder` probe of that layer's OWN instrumentation (schema-attributable to it + call_id distinct from the primary's already-`reached` call_id, per that field's distinctness rule) or a cited schema/catalog discovery-exhaustion of it — re-citing the primary probe, or a same-producer-schema requery, does not satisfy (v). If any named alternate has neither (a) nor a valid gated (c), or (v) is required and unmet, the cap is invalid — `reachability_outcome` projects to `unattempted_open`; dispatch a scoped follow-up.

<bad-example>
Primary source in a capability class is reached; a same-class alternate is absent from a scout-built enumeration; a KB note marks it "disabled"; no probe of the alternate is issued.
Wrong: cite the scout enumeration / KB note as a scope-exclusion -> set `alternate_capability_pivot: none-named` or `discovery_exhausted` -> emit `blocked-unreachable`.
Why wrong: the scout enumeration is a discovery artifact -- routes and ranks, NEVER closes a candidate. The KB note is orientation. Neither is a phase-current probe result (no cited returned error) nor a gated cross-class structural non-fit. The alternate is `unattempted_open`; consolidated `reachability_outcome` cannot cap.
Correct: probe the same-class alternate to its own terminal `denied`/`discovery_exhausted` before any cap. Expected: alternate = `unattempted_open`, cap not allowed, scoped follow-up dispatched. Citing scout output that did not list a candidate is not a valid scope/match exclusion.
</bad-example>

Mark a lead `blocked-unreachable` when the discriminator requires evidence this agent cannot reach — per the reachability floor above, every decisive-scope candidate must be CLOSED before the cap is valid. The typed `reachability_outcome` (per `artifact-contracts.md` §`5_grader/`) is the single gate; a genuinely-empty fit probe is `reached_empty` and never licenses the block. Then stop and name the engineer's suggested next step; do not spin on weaker proxies, and do not re-probe beyond the applicable probe cap. When the block is a present-but-redacted field, the cited returned redaction sentinel (not genuine emptiness — see "Redacted is not absent" in [investigation-invariants.md](investigation-invariants.md)) IS the `denied` outcome (the probe reached the source but the decisive field is access-limited; cite {probe call_id, verbatim sentinel string}); still label the receipt `present-but-redacted` with the field name and the richer-access resolver (UI or elevation), so the report renders the content-exists framing and the kit's resolution path instead of "no evidence" — redaction maps to `denied` only for gating, never collapsing into a "no evidence" render.

`natural-ceiling` is a synonym for a properly-received block, not a distinct license to cap: it names a verdict ceiling reached because the next-causal-layer evidence is genuinely out-of-band, and is admissible ONLY when the decisive `verdict_ceiling_lead` `reachability_outcome` has resolved to `denied` or `discovery_exhausted` — per the reachability floor above, that outcome is the WEAKEST/MOST-OPEN across all decisive-scope candidates; any `unattempted_open` fit alternate forces it to `unattempted_open`, making `natural-ceiling` UNAVAILABLE. When `verdict_ceiling_lead` names a next-causal-layer cause reached through a `reached`/`reached_empty` primary, `natural-ceiling`/`discovery_exhausted` also requires a cited spent vertical `generic_pivot_ladder` probe of that layer's own telemetry (per that field's distinctness rule) or a cited schema/catalog discovery-exhaustion of it; producer-side telemetry, KB out-of-band framing, or capability-map silence alone leaves `reachability_outcome=unattempted_open` and makes the cap invalid. A named candidate leaves the candidate set ONLY by floor condition (a) its own cited returned probe error, or (c) a gated cross-class structural non-fit (capability class from the taxonomy + stated reason; NEVER between same-class candidates) — never by a KB note, discovery-artifact scope absence, scout enumeration omission, or same-class non-fit assertion. The pivot obligation is discharged as soon as ONE fit alternate `reached`/`reached_empty` (that resolves the lead); stop probing once one reaches — "all fit candidates `denied`/`discovery_exhausted`" governs only licensing the cap. If the single refinement allowance or applicable probe cap is exhausted while any fit alternate remains `unattempted_open`, settle **Proximate-only with the gap visible** — never `natural-ceiling`/`blocked-unreachable`.

<bad-example>
Observed: primary producer telemetry is `reached` and names a next-causal-layer dependency/engine as the ceiling cause; that dependency's OWN telemetry (its logs/exported metrics, reachable by following the failing units' correlation/job/output-path ids — possibly a different namespace within a source already reached) was never probed; a KB note calls the dependency "out-of-band"; the capability map does not list it.
Wrong: set `reachability_outcome=discovery_exhausted`/`natural-ceiling` citing the KB note + map silence, or re-citing the producer-side probe as the vertical pivot.
Why wrong: the KB note is orientation and the map omission is a discovery artifact — neither closes; no cited spent probe schema-attributable to the dependency's OWN instrumentation (distinct from the primary call_id) ⇒ `unattempted_open`, cap invalid.
Correct: run the vertical `generic_pivot_ladder` probe of the dependency's own telemetry, cite it, then settle at the supported verdict — Proximate-only with the gap visible if the bounded probe exhausts.
</bad-example>

<bad-example>
Observed: the primary metrics-query source for a restricted/sovereign scale unit returns `fetch failed`; a capability-map-named log/trace-grep capability that reaches that same scope was never invoked; a KB note says the scope is sovereign/not-available.
Wrong: set `reachability_outcome=denied` from the primary probe and cap `natural-ceiling`/`blocked-unreachable`, citing the KB note.
Why wrong: the map-named alternate is a fit candidate left `unattempted_open`, so the consolidated ceiling outcome is `unattempted_open` (cannot cap); the KB note is orientation, not a `denied` receipt.
Correct: probe the alternate to its own terminal `denied`/`discovery_exhausted` before any cap; if it `reached`/`reached_empty`, settle on that evidence.
</bad-example>

A documentation gap never satisfies `blocked-unreachable` (see the access invariant in [investigation-invariants.md](investigation-invariants.md)): an unattempted or never-targeted evidence source — including the service's own telemetry source when only the incident-record source was probed — is `open-answerable`, not blocked. A discovery-artifact-listed alternate read capability that could reach the decisive scope (its omission from the CAPABILITY MAP does not close it) but was not invoked likewise stays `unattempted_open`, and a KB/docs scope-access-gating note is orientation, not a `denied` receipt — per the `natural-ceiling` gate above. The block is honest only once the `verdict_ceiling_lead` `reachability_outcome` resolves to `denied` (a fit probe returned a terminal access error/sentinel) or `discovery_exhausted` (the bounded discovery sequence completed without finding a probeable source, cited per `artifact-contracts.md` §`5_grader/`); a genuinely-empty fit probe is `reached_empty`, not a block. Completing that sequence, not an endless search for another source, is terminal. When the service exposes a coverage map, the bounded discovery preceding a `blocked-unreachable` settlement MUST include consulting the coverage map's best-fit orientation asset — recording the asset and the probeable source it named (then probed), or that no covering asset exists. The docs-only orientation asset never satisfies the block itself (it is documentation, per the access invariant above) and never closes a lead or promotes a verdict; it only surfaces the probeable source the block still requires probing. This forbids skipping a named where-to-look lead before punting, without ever letting a docs-only asset close the lead.

An unread incident discussion thread is likewise not a block: a causal lead cannot close `blocked-unreachable` until the incident's own discussion thread — the discussion-thread summary captured at intake and carried by scout (human comments, transfers, prior RCA/mitigation, linked change/rollout notes) — was read or recorded genuinely unavailable; an unread thread is not "unavailable" (apply the access invariant in [investigation-invariants.md](investigation-invariants.md)), and skipping it is a probe defect, not a true block. A human-stated cause or mitigation read from the thread is still a claim, not authority: it requires the mechanism-discriminator gate's authoritative corroboration before it can close, confirm, or downgrade a lead.

## Over-rejection calibration

Be adversarial but fair: stop at a grounded-enough verdict when the cause precedes the symptom, the mechanism source was checked, the decisive discriminator supports the theory, evidence is cited from reachable non-narrative observations, major alternatives are refuted or bounded as non-material, and remaining gaps would not reasonably change the user-facing RCA.

Structural allowance: one post-synthesis awaited evidence batch, followed by the bounded consequence
audit. No second or third follow-up batch.

## Report binding

Report wording matches the verdict class. Reserve root-cause, likely, or probable language for **Likely-rooted** or **Confirmed**. A **Proximate-only** or **Inconclusive-blocked** result uses the plain external label with the unresolved "why" and the engineer suggested next step. Never give a clean all-clear while material leads are open. When the unresolved "why" or a verdict cap rests on a decisive discriminator that needs a human-only or out-of-band capability, the report carries `OCE next checks`, projected from the internal Manual Investigation Kit (canonical shape in `artifact-contracts.md` §`6_report/`); the first check is the one that would lift the cap.

For deep-lane runs, Report also binds to the final Material Claim Integrity Receipt.
`blocked` rows are not rendered as facts; `qualified` rows use no stronger wording than
the receipt allows. A Manual Investigation Kit branch is decisive only when its receipt
records distinct rival predictions and `discriminates=yes`.
