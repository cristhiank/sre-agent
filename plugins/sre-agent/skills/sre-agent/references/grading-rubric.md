# Grading Rubric

The grader is the adversarial judge: it refuses proximate-as-root while an answerable upstream lead remains, and closes every material lead with cited evidence or an honest dead-end. Shared rules live in [investigation-invariants.md](investigation-invariants.md); this file says how to judge.

## Verdict classes

- **Confirmed / Likely-rooted**: cause and mechanism are verified against an authoritative source for that mechanism; timing fits, the decisive discriminator is checked and cited, and material alternatives are addressed.
- **Proximate-only**: the named cause is the failure mechanism itself, the upstream "why" is unexplained, or a reachable mechanism discriminator remains unchecked.
- **Inconclusive-blocked**: the next discriminating evidence is unreachable with available evidence sources. A keyless primary signal is not exhaustion while a generic pivot — time window ∧ affected scope/entity ∧ operation/step — into another next-causal-layer source remains unspent within budget.
- **Refuted**: observations contradict the theory.

## Symptom-vs-cause test

A theory that names only an error, missing resource, timeout, exception, or failed job is **Proximate-only** unless it explains the upstream mechanism: what made that condition occur.

## Target-alignment gate

The primary RCA target is the intake `rca_target`: the underlying measured/impacted failure. When real unresolved measured failures exist, the failure-cause lead is primary; alert/monitor mechanics are secondary or contributing unless the signal is pure noise or the underlying failures are already explained.

The report's primary finding must address the declared `rca_target` or carry the `5_grader` discovery receipt proving that target unreachable or clean. No third path.

Pure-noise/tuning-artifact closure is valid only with a receipt showing the underlying failure dimension was inspected and found clean or immaterial. Any scope expansion beyond `literal_trigger` must cite evidence that the same measured failure exists in the expanded scope.

## Mechanism-discriminator gate

A proposed root cause is not eligible for **Likely-rooted** or **Confirmed** until the claimed mechanism is verified against an authoritative source for that mechanism, not merely inferred from symptom telemetry.

Before promotion, the grader must record all four:

1. The proposed causal mechanism, including the specific state it corrupts or the decision it changes, not just the failure signal.
2. At least one plausible rival mechanism that produces the same observed symptom.
3. A pre-declared, falsifiable discriminator predicate with the expected observation under each mechanism, favored and rival, stated before checking.
4. The checked concrete observed value of that discriminator.

The discriminator predicate and per-mechanism expected values must be pre-registered before the checked value is observed: carried as an `open-answerable: mechanism-unverified` obligation or specialist hypothesis from a prior round/turn, or pre-declared in Scout's discriminator table or a specialist's first-pass note (predicate + expected favored/rival stated and kept visibly separate before the value was observed), then checked. Scout's discriminator table (declared by an agent that never checks the value) is the strong form; a specialist's own first-pass pre-registration is the weaker form — accept it only when the predicate + expected favored/rival are kept visibly separate from the checked value AND were not shaped by any observed/symptom-correlated result; if the predicate, the expected values, and the checked value first appear together with no visible separation and no prior Scout/refinement artifact, treat it as post-hoc. A discriminator first stated with its own result is post-hoc and does not pass; keep **Proximate-only** and run one focused verification turn against the pre-registered discriminator.

Multi-arm defects: when a confirmed code/config defect has more than one supported failure arm that produces the same observed symptom class, pre-register expected values for EACH in-scope arm, not only the favored one. Limit arms to those supported by the mechanism source, matching the observed symptom class, and plausible for the incident window/entity (reachable within budget). Refuting one arm does NOT refute the defect while another in-scope arm matches; positive in-incident evidence of the defect's mechanism firing counts as support even when not keyed to the exact failing id. For drop/suppress arms, state whether the missing id is the EXPECTED signature — absence of the id can be the arm's signature, not its refutation.

A symptom-consistent narrative does not pass: "telemetry is consistent with X" is not a discriminator unless it names distinct expected values per mechanism and a checked result.

Authoritative source means whatever fits the claimed mechanism: implementation behavior, configuration or rollout state, runtime/control-plane state, maintained operational documentation, or telemetry that observes the mechanism itself. Require implementation reading only when the mechanism depends on implementation behavior; runtime, infrastructure, configuration, or control-plane claims can be verified by authoritative state for that claim.

If the surviving mechanism depends on implementation/config behavior, the checked mechanism must include the producer code/config path: the code/config that creates the corrupt state or decision, not only the runtime/telemetry layer that propagates or consumes the symptom. Telemetry that only shows the symptom is not sufficient for a code/logic-rooted cause.

Boundedness: require one serious same-symptom rival, not exhaustive enumeration. Code/config reading is scoped to the minimal producer path indicated by the discriminator, not broad archaeology. If that producer code/config is unreachable within budget, use **Inconclusive-blocked** with the discovery receipt and engineer next step, not **Likely-rooted** on the symptom story. If the scoped producer code/config does not confirm the defect, disconfirm or downgrade the hypothesis; do not keep spelunking.

If the discriminator is reachable and unchecked, mark the lead `open-answerable: mechanism-unverified`, keep the verdict **Proximate-only**, and require one focused verification turn. If the discriminator is unreachable with available evidence sources, use **Inconclusive-blocked** with an engineer next step.

Verified-enough means the mechanism source was checked, timing fits, the checked discriminator supports the theory, and material alternatives are refuted or bounded as non-material. Stop when remaining gaps would not reasonably change the user-facing RCA; do not loop solely to raise confidence.

Live-verdict provenance floor: before emitting `Likely-rooted` or `Confirmed`, apply the mechanism-discriminator gate and the failing-unit enumeration gate (when population/aggregate evidence is material) as a pre-promotion checklist, plus the disqualifier that an unverified change/version is actionability-only, never causal verification, so it cannot stand as the cause (this disqualifies; it does not require producing provenance to promote). A rooted verdict rests on a verified mechanism and checked discriminator against an authoritative source — quality, not count: a purely temporal or correlational association, an unkeyed convenience sample, or a named-but-unverified change treated as the cause cannot satisfy that bar, and stacking several such thin threads does not either. Follow the reachability disposition above (reachable-unchecked → `Proximate-only`; unreachable decisive mechanism check → blocked path), and do not over-hunt once verified-enough (above) is met. Record the result in the grader's `Confidence reducer / verdict cap` field — REQUIRED for every `Likely-rooted`/`Confirmed`, with `status=none` admissible only via the positive attestation that the discriminator is none of those thin threads.

IMPORTANT: no asserted runtime STATE (node/process death, schema-missing, regression-introduced, etc.) without an authoritative NON-SYMPTOM source (runtime/control-plane/implementation source + observed value); the symptom metric itself — metric-zero, threshold breach, single sample, or temporal correlation — proves a MEASUREMENT, not a runtime state; absent a non-symptom source, mark status=correlation-not-causal (measurement-only) and cap at Proximate-only.

<bad-example>
Observed: the success-rate metric reads zero for a scale unit.
Wrong: "Root cause: the node/process is dead."
Why wrong: metric-zero is the symptom measurement; node/process death is a runtime state that needs host/control-plane evidence — none was checked.
Correct: "throughput measured zero; runtime state unverified" -> cap at Proximate-only and name the control-plane/host check that would confirm.
</bad-example>

When a **Confirmed** or **Likely-rooted** verdict is tied to a verified code/config/schema/artifact/service-owned location, emit an actionability-only introduction provenance obligation: closest introducing change or labeled last-touching change for the implicated path/symbol. This is post-verdict actionability only, may reuse the producer-code citation from this gate, and does not gate the verdict.

## Failing-unit enumeration gate

After target-alignment and signal-validity establish that real underlying failures exist, and the cause claim rests on an aggregate signal (a rate, count, ratio, or threshold breach), one population standard governs both promoting and refuting a rival. Promote only when the theory accounts for the authoritative failing population — the concrete failing units, or a justified keyed cohort, keyed to the producer; a count or time coincidence without a key-compatible join is salience, not proof. A refutation resting on a population join or empty result counts only from a fit-for-population probe (an empty join with incompatible ID-space/scope, or a non-authoritative source, is a probe defect, not refutation, per "Empty is not absent" in [investigation-invariants.md](investigation-invariants.md)); discriminator-predicate or bounded-as-non-material refutations (per the mechanism-discriminator gate) are unaffected. Exact failed-ID joins are strongest but not the only valid form — any key-compatible join to the producing population (keyed cohort, per-failure records, or direct mechanism telemetry) also qualifies. If units were enumerable but not inspected, mark the lead `open-answerable`, keep **Proximate-only**, and require one focused enumeration pass — which precedes the producer code/config verification the mechanism-discriminator gate requires. The gate does not fire on a pure measurement/threshold artifact, when the mechanism is already verified from a direct non-aggregate source that is itself the authoritative failing population (not a coincident side-population), or when fit-for-purpose enumeration was attempted and the units/keys are genuinely unreachable — the last becomes `blocked-unreachable` with the discovery receipt.

## Cross-source reconciliation

When a material claim rests on a quantity that two or more sources should measure over the same population and window, and they disagree beyond expected sampling/scope tolerance, the disagreement is itself a lead, not a footnote. Reconcile it within the pass — scope, sampling, scaling, dedup, ID-space, or time-fit. If it stays unreconciled, name the authoritative source for the claim, carry the gap as an explicit caveat, record the cap in the `Confidence reducer / verdict cap` field, and escalate to a follow-up obligation only when it meets the `open-answerable` bar. Do not silently adopt one figure or average them. This is horizontal (two in-hand sources disagree), distinct from the cross-source PIVOT in the Loop decision (escalating a *blocked* lead via in-hand keys to an unprobed next-causal layer). It composes with, and does not replace, the failing-unit enumeration gate (which binds an aggregate down to its concrete units).

## Cross-incident classification and verdict determinism

When the scout's recurrence check reports a recent or concurrent incident sharing this incident's recurrence identity (the identity defined in the recurrence invariant — see [investigation-invariants.md](investigation-invariants.md)), the grader classifies the current incident as `canonical` or `duplicate-of <incident>` rather than judging it in isolation, and records the relationship so the report cross-links it. Selection is over the sibling set the scout actually observed (a run sees only siblings already created within its recurrence window), so scope the claim as canonical-as-observed: prefer an external incident-management parent/duplicate link when the scout recorded one (recurrence output carries any parent/duplicate link the incident-history exposes); otherwise order by one clock chosen for the whole comparison — authoritative trigger-fire time if every ordered sibling exposes it, else declared start if every ordered sibling exposes it, else created/opened time — and the canonical is the earliest sibling on that clock, tie-broken by stable incident id. Compare only siblings that expose the chosen clock; a sibling lacking it, or whose time is too coarse to order against the others, is a related/unordered match rather than silently ordered, and if no shared clock exists across the observed siblings, record them as related with no time-canonical. The current run is `duplicate-of <canonical>` unless it is itself the canonical. This classifies the relationship only; it never imports the sibling's cause, verdict, or mitigation as truth, and each run still grades its own evidence independently.

Verdict determinism: the grader applies the same verdict gate to every incident sharing the recurrence identity, so independent runs on equivalent evidence converge rather than diverge — convergence is an emergent property of applying the gate, not an instruction to match or import a sibling's verdict. When the trigger is an alert/monitor/SLO and the authoritative definition of what it measures (numerator, denominator, window, threshold) is the decisive source for whether the alerted condition is real, and that definition is unreachable, this rule sets only the MAXIMUM verdict that trigger-definition evidence allows — cap at **Likely-rooted**; the mechanism-discriminator and failing-unit enumeration gates can still lower it, and target-alignment still governs whether the underlying failure is real. The cap lifts to **Confirmed** only when the alerted condition is independently reconstructed from authoritative raw/service-owned signals — its numerator, denominator, and scope, over the incident's declared window from intake — AND rooted to a verified producer per the mechanism-discriminator gate, so the unreachable trigger definition is no longer load-bearing; the report then states the trigger-definition gap as immaterial. Does not qualify (cap stays at **Likely-rooted** or lower): incident narrative or alert summary only; aggregate or proxy telemetry that does not reconstruct the numerator/denominator/window/scope of the alerted condition; raw failures without threshold or scope equivalence; or a producer not verified by the mechanism gate.

Cross-incident attribution tiers (for a cross-signal cascade candidate; same-signal siblings stay under the canonical/duplicate-of classification above): attribute only as strongly as the evidence supports — `related` (overlapping time/resource only; no causal-direction claim), `cascade-candidate` (overlapping window PLUS either a shared failing resource/key or a directly dependent component on a known dependency edge, with the related incident starting first; title/team or bare co-residency alone is insufficient), or `likely-downstream-of <incident>` (a cascade-candidate AND this incident's own evidence matches a downstream failure mode documented in service dependency/topology knowledge, a runbook, or verified prior-incident evidence — not inferred from the related incident's title — AND no stronger local cause survives). Absent that local match, cap at `cascade-candidate` and cross-link rather than attribute or route, under the same no-import rule above.

<bad-example>
Observed: an upstream storage/capacity-saturation alert on a shared resource precedes a downstream service's query-success-rate drop on that same resource.
Wrong: grade the downstream incident "root cause: upstream resource exhausted" by importing the upstream incident.
Why wrong: the upstream incident is a hypothesis prior; this incident was not independently checked for the expected downstream signature (e.g. allocation/load failures keyed to the saturated resource), and a stronger local cause may survive.
Correct: surface it as a cascade-candidate, test this incident's own evidence for the downstream signature, and only then attribute "likely downstream of <incident>" + route to the upstream owner — else cross-link as related.
</bad-example>

## Adversarial pass

Each grading round, independently ask: "what would make this theory false or incomplete?" and "why did this condition exist?" Re-derive defeat modes from the goal + evidence; do not rubber-stamp the specialists' conclusion.

## Lead ledger: no silent null-close

List each material lead or hypothesis from scout + specialists and close each as:

- `closed-supported` with cited `OBS###` evidence,
- `closed-refuted` with cited `OBS###` evidence,
- `open-answerable` with the missing discriminator and reachable evidence shape,
- `blocked-unreachable` with an explicit access/source limitation.

When the service exposes a curated guidance catalog, a lead that advanced via own-observations is not gate-ready unless its claim-readiness-ledger `curated-asset consulted` field is populated — a guidance-asset id/name, or `none-exists` with the probed source it would have named; a blank field bounces the lead to `open-answerable` rather than any settled class. A docs-only guidance asset never closes a lead or promotes a verdict; an honest `none-exists` still settles.

A causal lead targets the declared `rca_target` / actual failure cause. It may be closed
`blocked-unreachable` only with the discovery receipt defined in
`artifact-contracts.md` §`5_grader/`, proving target discovery was exhausted from
sourced candidates, not asserted. Bounded discovery budget: (1) observability/service-
knowledge lookup, (2) schema/catalog discovery against sourced candidates, (3) one
correct/authed probe against a discovered diagnostic source. No invented or guessed
targets; no receipt means the causal lead remains `open-answerable` or needs another
focused follow-up.

No material lead is silently dropped.

## Loop decision

Mark a lead `open-answerable` when it targets the upstream "why" or mechanism verification, a reachable evidence path exists, the result could materially change the verdict, and it fits one focused specialist batch. Emit a bounded follow-up obligation. Each bounded follow-up obligation is scoped: obligation id; lead id(s); the pre-registered discriminator predicate; expected favored/rival values; acceptable evidence shape; in-hand keys; prior OBS ids with reuse/freshness mode (reuse settled/static context only when the cited prior OBS exactly cover the assigned predicate, source, key set/failing population, and incident time/scope — otherwise a fresh narrow read is required even if the source is "static"; a fresh read is always required when the discriminator depends on a volatile live value or a cross-source pivot); stop condition; dependency ids or an independence reason; and the allowed outcome statuses (answered / invalid-premise / unanswerable, each carrying the diagnostic payload: failed premise/source/key, cited probe result or terminal error, whether a corrected narrow discriminator is derivable, and whether the lead stays reachable/open or qualifies for a `blocked-unreachable` receipt). Independent obligations may execute as one awaited parallel-sync follow-up wave — one specialist per independent missing discriminator/obligation — and that wave is ONE follow-up round against the budget below; if the independent set exceeds the host concurrency cap, the additional awaited batches still count as that SAME one follow-up round (the concurrency cap is not the budget unit). Obligations are independent only when none depends on another's result for keys, failing-unit scope, source selection, predicate validity, materiality, or live-source safety; serialize only on a genuine dependency or source-safety constraint, and combine leads that share the same missing discriminator/source into one obligation rather than dispatching duplicates. A follow-up specialist checks only its assigned discriminator plus the minimal premise/freshness validation needed to decide answerability; an `invalid-premise`/`unanswerable` return never closes a material lead — the Grader reconciles it in the lead ledger (closed-supported/closed-refuted/open-answerable/blocked-unreachable with the required receipt and engineer next step). A reachable evidence path includes the access invariant's cross-source pivot: when an unresolved upstream/mechanism lead has any unprobed reachable path to a next-causal-layer source — via the failing units' in-hand correlation/identity keys, OR, when the primary signal is keyless, via the **generic pivot ladder (time window ∧ affected scope/entity ∧ operation/step)** joined into the next-causal-layer source the **service KB / capability map** names (telemetry, logs, traces, config, code, or control-plane state — do not narrow to telemetry/logs only). That pivot is a required first call on the follow-up budget — keep the lead `open-answerable` and spend one focused pass following that path before a final `Proximate-only` settles, recording the result in the discovery receipt; only a spent fit-for-purpose probe, or no capability-map-named source that consumes these dimensions, lets the verdict settle on that lead.

IMPORTANT: do not settle Inconclusive-blocked or a final Proximate-only — and do not emit `dispatch: no` — while the discovery receipt's cross-source-pivot, its `generic_pivot_ladder`, and `in_hand_branches_dispositioned` (and, when the trigger is signal-shaped, `signal_validity`) remain unspent within budget — each complete only as a probed source + observed result, or a terminal value cited to its source/absence (not a soft "unavailable"; for `signal_validity`, the routed capability's disposition is the completing result); see [artifact-contracts.md](artifact-contracts.md) §`5_grader/`. These passes draw from the SAME remaining follow-up budget — spend at most one focused pass each; when no reachable in-hand-keyed path and no capability-map-named source consuming the generic ladder's dimensions remain, or that budget is exhausted, that terminal IS the receipt — settle and name the engineer next step; do not loop.

<bad-example>
Lead: failures with in-hand correlation ids to a provider or next-causal-layer telemetry source whose schema was already discovered this run.
Wrong: settle "Inconclusive-blocked — provider telemetry unavailable."
Why wrong: the source was reachable (its schema was just read); "unavailable" was asserted, not probed — an assumed-block.
Correct: run the failing-unit join against the source, cite the observed result, then settle at the verdict the evidence supports.
</bad-example>

Mark a lead `blocked-unreachable` when the discriminator requires evidence this agent cannot reach. A causal lead may close `blocked-unreachable` only after either (1) a fit-for-purpose probe (per the "Empty is not absent" invariant in [investigation-invariants.md](investigation-invariants.md)) was attempted and the next discriminating evidence is genuinely unavailable, or (2) bounded discovery (the 2-3 round budget below) could not identify an authoritative signal/key/path — where "could not identify" is established by a live schema/catalog probe of the candidate service evidence source, not a service-knowledge or documentation lookup alone — recording the attempted discovery and remaining gap. Then stop and name the engineer's suggested next step; do not spin on weaker proxies, and do not re-probe beyond the budget. When the block is a present-but-redacted field (a cited returned redaction sentinel, not genuine emptiness — see "Redacted is not absent" in [investigation-invariants.md](investigation-invariants.md)), label the receipt `present-but-redacted` with the field name and the richer-access resolver (UI or elevation), so the report renders the content-exists framing and the kit's resolution path instead of "no evidence".

A documentation gap never satisfies `blocked-unreachable` (see the access invariant in [investigation-invariants.md](investigation-invariants.md)): an unattempted or never-targeted evidence source — including the service's own telemetry source when only the incident-record source was probed — is `open-answerable`, not blocked. The block is honest only once either a fit-for-purpose probe of the source that answers the lead came back genuinely empty or denied per "Empty is not absent", or the bounded-discovery budget was spent without finding a probeable source; the spent budget, not an endless search for another source, is the terminal. When the service exposes a curated guidance catalog, the bounded discovery preceding a `blocked-unreachable` settlement MUST include consulting the best-fit guidance asset — recording the asset and the probeable source it named (then probed), or that no covering asset exists. The docs-only catalog never satisfies the block itself (it is documentation, per the access invariant above) and never closes a lead or promotes a verdict; it only surfaces the probeable source the block still requires probing. This forbids skipping a named, cheap, where-to-look lead before punting, without ever letting a docs-only asset close the lead.

An unread incident discussion thread is likewise not a block: a causal lead cannot close `blocked-unreachable` until the incident's own discussion thread — the discussion-thread summary captured at intake and carried by scout (human comments, transfers, prior RCA/mitigation, linked change/rollout notes) — was read or recorded genuinely unavailable; an unread thread is not "unavailable" (apply the access invariant in [investigation-invariants.md](investigation-invariants.md)), and skipping it is a probe defect, not a true block. A human-stated cause or mitigation read from the thread is still a claim, not authority: it requires the mechanism-discriminator gate's authoritative corroboration before it can close, confirm, or downgrade a lead.

## Over-rejection calibration

Be adversarial but fair: stop at a grounded-enough verdict when the cause precedes the symptom, the mechanism source was checked, the decisive discriminator supports the theory, evidence is cited from reachable non-narrative observations, major alternatives are refuted or bounded as non-material, and remaining gaps would not reasonably change the user-facing RCA.

Budget: 2 follow-up rounds by default; a 3rd only if the prior round produced new material evidence. Stop if two consecutive rounds add nothing material.

## Report binding

Report wording matches the verdict class. Reserve root-cause, likely, or probable language for **Likely-rooted** or **Confirmed**. A **Proximate-only** or **Inconclusive-blocked** result is reported as such with the unresolved "why" and the engineer suggested next step. Never give a clean all-clear while material leads are open. When the unresolved "why" or a verdict cap rests on a decisive discriminator that needs a human-only or out-of-band capability, the report carries the Manual Investigation Kit (canonical shape in `artifact-contracts.md` §`6_report/`); the kit's decisive check is the one that would lift the cap.
