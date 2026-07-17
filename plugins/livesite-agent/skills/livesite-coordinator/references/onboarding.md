# Onboarding (First-Run Telemetry Discovery)

Loaded on demand when the COLD condition is detected in `references/work-store.md`.
NOT loaded on WARM runs.

## When to dispatch

Load this reference and dispatch onboarding when COLD — any required artifact is missing,
invalid, or stale-by-version:
- `service.toml` is absent or unparseable.
- `cache/schema.toon` is absent, invalid, or version-mismatched.
- `cache/probes.toon` is absent, invalid, or stale.
- `cache/baselines.toon` is absent, invalid, or past TTL.
- `cache/suppressions.toon` is absent or invalid.
- `cache/component-map.toon` is absent or invalid.
- `cache/coverage.toon` is absent, fails version validation, or version-mismatches
  `service.toml`. A cache that predates `coverage.toon` (no file present) is
  unconditionally COLD.

Do not execute Stage 1 until all required cache files are written and validated.

**ONBOARD-FIRST — strict sequencing on COLD:** the coordinator dispatches onboarding and
WAITS for it to complete. Onboarding writes `service.toml` and the five discovery cache
files (`schema.toon`, `probes.toon`, `baselines.toon`, `suppressions.toon`,
`component-map.toon`); it returns per-namespace watch-set/coverage findings as output
receipts. The coordinator then writes `cache/coverage.toon` from those returned findings.
ONLY after all six cache files are present and validated does the coordinator advance to
Stage 1. The coordinator does NOT run inline telemetry discovery in parallel with this
dispatch; no duplicate telemetry pulls on COLD.

## What onboarding does

1. **Enumerate telemetry sources** — discover the available log and metric planes for
   this service using the bounded read-only telemetry-discovery capability provided by
   the host environment.

2. **Discover schema** — sample recent signal shape; identify emitter patterns,
   error-prefix taxonomy, and available metric namespaces for this service. Treat
   discovered schema as evidence, not authority; record gaps.

3. **Tune burst probes** — calibrate the burst-detection window and threshold to this
   service's actual signal volume and variance. Never hard-code generic thresholds;
   always tune to observed shape.

4. **Seed baselines** — pull a baseline window, EXCLUDING any active-incident or
   anomaly window; record the window math and exclusion evidence in `cache/baselines.toon`.
   A baseline that includes an anomaly window is invalid and must be re-pulled with
   a clean anchor.

5. **Build component map** — enumerate the binaries, roles, or partitions that emit
   telemetry for this service; write `cache/component-map.toon`.

6. **Seed initial suppressions** — identify known-noisy signals from schema and recent
   history; write `cache/suppressions.toon` with conditional rules
   (component + signature + shape + rate-envelope). Rules are never blanket; each rule
   names its condition envelope.

7. **Compute per-namespace watch-set** — for each fully swept namespace, identify the
   `watch_set`: all metric signals that bursted during this sweep OR are explicitly
   pinned high-value for this namespace. Record `last_full_sweep_utc`; derive the
   risk-weighted `resweep_due` freshness bound; classify `coverage_status`
   (`covered | overdue | never`). Return these findings as output receipts for the
   coordinator to merge into `cache/coverage.toon`. (The coordinator is the sole
   writer of `cache/coverage.toon`; onboarding does not write it directly.)

8. **Write `service.toml`** — record service identity, `cache_version`, probe schema
   version, and last-onboard timestamp.

9. **Write freshness receipts** — each cache file carries `valid_until`, `window_math`,
   and `cache_version` fields so the coordinator can evaluate freshness deterministically.

## Mini-onboard (rotating namespace resweep)

The coordinator dispatches a bounded rotating resweep as a mini-onboard during each WARM
run to advance namespace coverage. This is a DISPATCHED operation — never inline.

**Scope selection (drawn from `cache/coverage.toon`):**
- Always include all namespaces in `pinned_critical` (swept every run, never skipped for budget).
- Include all namespaces whose `resweep_due` has passed (`coverage_status: overdue`).
- Include the next uncovered namespaces per the rotation `cursor`, up to the per-run full-
  sweep `budget`. The budget is derived from the target fast-path latency share and the
  observed per-namespace sweep cost — self-tuning; never a hardcoded count.

**What a full namespace sweep produces:**
- Pull all metric signals in the namespace; burst-rank against their baselines.
- Recompute `watch_set`: signals that bursted during this sweep OR are explicitly pinned
  high-value (prior members that still qualify are retained; newly bursted signals are added).
- Refresh baselines for `watch_set` signals.
- Record `last_full_sweep_utc`; derive `resweep_due` (risk-weighted: shorter for namespaces
  with recent burst activity, high churn, sparse coverage, or high-value/critical components;
  longer for stable/quiet namespaces — never a hardcoded TTL).
- Return findings as output receipts; coordinator merges into `cache/coverage.toon`.

**Watch-set age-out (eviction):**
- Demote a metric from the `watch_set` after a self-tuning count of successive full sweeps
  with no burst and no material variance (count derived from observed sweep frequency and
  namespace volatility — never hardcoded).
- Enforce a maximum watch-set size per namespace: evict the longest-quiet members first
  when the maximum is exceeded. Explicitly pinned high-value members are never evicted.

**Cursor-advance rule:**
The rotation cursor advances ONLY past namespaces whose full sweep COMPLETED successfully
this run. An attempted-but-failed sweep does NOT advance the cursor.

**Failure semantics:**
- Partial or failed full sweep of a namespace: KEEP the prior `watch_set` for that namespace
  unchanged; mark `coverage_status: overdue`; do NOT advance the cursor past it; write a
  `partial-sweep` receipt row.
- Stale, invalid, or unresolvable `coverage.toon` or watch-set: treat as drift → dispatch
  a full mini-onboard for all namespaces; never pull a stale watch-set as if fresh.

## IO contract

**Inputs (injected by the coordinator at dispatch time):**
- `service_id` — the service being onboarded.
- `service_work_path` — the already-resolved service directory (from work-dir resolution
  priority 1 or 2 in `references/work-store.md`); used VERBATIM as the write root for
  all output files (`service.toml` at the root; cache files under `cache/`).
- Capability handles — the bounded read-only telemetry-discovery capability provided by
  the host environment. (Generic capability handle; no concrete tool or cluster name.)

**Outputs:**
- **Service root** (`<service_work_path>/`):
  - `service.toml` — identity, `cache_version`, probe schema version, last-onboard timestamp.
- **`cache/`** (five schema/probe/baseline files; `coverage.toon` is coordinator-owned
  and written by the coordinator on post-sweep merge — not written directly by onboarding):
  - `schema.toon` — discovered telemetry schema with `cache_version` + `valid_until`.
  - `probes.toon` — tuned probe definitions with `cache_version` + `valid_until`.
  - `baselines.toon` — per-signal baselines with TTL + `window_math` + `exclusion_record`.
  - `component-map.toon` — binary-to-component map.
  - `suppressions.toon` — conditional suppression rules.
- **Per-namespace coverage findings** (returned as output receipts; NOT written to a file
  by onboarding): for each swept namespace — `watch_set`, `last_full_sweep_utc`,
  `resweep_due`, `coverage_status`. The coordinator merges these into `cache/coverage.toon`.

This IO contract is stable. If this onboarding logic later graduates into its own
independent first-run telemetry-discovery capability, the coordinator dispatch contract
(inputs above → outputs above) remains unchanged and requires no coordinator edits.

## Onboarding discipline

- Read-only over all telemetry surfaces; no writes to production.
- Baseline windows MUST exclude any active-incident or anomaly period; record the
  exclusion evidence explicitly.
- Suppressions are conditional, not blanket; each rule names its full condition envelope.
- Treat discovered schema as evidence; record gaps and ambiguities as receipts.
- On partial onboard (some cache files already fresh): re-tune only the missing or stale
  files; leave fresh files untouched.
- Do not reference this capability by a skill name anywhere. The coordinator dispatches
  to "a bounded read-only telemetry-discovery capability"; the host routes it.

## Model-tier floor for onboarding dispatch

Onboarding requires `minimum_allowed_class: mid`. The economical class is BELOW this
floor and MUST NOT be used for the onboarding dispatch — first-run discovery, probe
tuning, and baseline seeding need mid-tier reliability to drive the read-only telemetry
capability correctly. Escalation to reasoning-heavy is allowed when the service's
telemetry shape is complex or ambiguous. A missing or ambiguous class resolution fails
safe upward (never down to economical). The coordinator MUST select the model class per
the `minimum_allowed_class: mid` floor and set it EXPLICITLY WHEN THE DISPATCH MECHANISM
EXPOSES A MODEL/CLASS SELECTOR. WHEN the host dispatch mechanism does NOT expose an
explicit class/model selector, the coordinator MUST (a) use the strongest/default dispatch
tier the host provides (which must be at least the mid floor — never an
economical/cheapest tier if one is selectable), and (b) RECORD in `run-state.toon` the
resolution basis (e.g. `"host exposes no explicit class selector; used host default
dispatch tier, treated as >= mid floor"`). A missing or ambiguous class resolution when
a selector IS available fails safe upward (never down to economical). Record the chosen
class and resolution basis in `run-state.toon`.
