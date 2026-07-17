# Reconciliation

Turn mined candidates into a clean, current failure-knowledge target state by reconciling each against
what already exists. Reconciliation is outputs-only and evidence-gated: it adds, updates, rewrites, or
removes owned entries so the layer neither loses hard-won knowledge nor accumulates stale/conflicting
content.

<ownership_boundary>
Ownership is by **path**, not by any per-file tag. Write ONLY:
- `failure-knowledge/**` — ADD / UPDATE / REWRITE / DELETE permitted under the gates.
- `00-index/incident-clusters.toon` and `00-index/investigation-ledger.toon` — your two owned index files.
- `KB_CHANGELOG.md` — the dated audit trail.

Everything else is SOURCE-OWNED and a post-run gate rejects the WHOLE pass if you write it — most importantly
`00-index/evidence-ledger.toon` and the rest of `00-index/` (core-map, telemetry-routing-card,
ai-asset-catalog, verification-queue, ownership, task-router). READ them freely — especially
`evidence-ledger.toon`, the canonical evidence-grade record you compare against — but NEVER write them.
Code-derived contracts/invariants live outside your paths entirely. A candidate that belongs to the source
plane is a SUGGESTION row in `investigation-ledger.toon` (for the source-side pass or a human to fold in),
not a write.
</ownership_boundary>

<evidence_grade_and_mutation>
The evidence-grade record (`00-index/evidence-ledger.toon`) is canonical and source-owned; you read it, you
never write it.
- **Higher grade wins.** Never overwrite a higher-grade fact with weaker evidence. A single `observed`
  incident does not overwrite a `verified` fact — re-grade, mark contested, or suggest instead.
- **Every touched fact gets a mutation status** recorded in `00-index/investigation-ledger.toon`:
  `preserved | superseded(+record) | re-graded | new | stale | removed`. A `removed` fact also gets a dated
  `KB_CHANGELOG.md` line.
- **Protected homes are DELETE/RENAME-protected.** A `failure-knowledge/` file the ledger grades
  verified/observed or tags with a protected claim-class (authz/auth/authn/contract/routing-failure/
  ownership/escalation/telemetry-source/canonical-telemetry/pii/secret) may be ENRICHED in place but must NOT
  be deleted or renamed — a post-run gate blocks it and discards the pass.
- **Refine by suggestion.** If you believe a high-grade or protected fact (a delete-protected file, or any
  verified/contract/authz/ownership/telemetry fact) is wrong or stale, do NOT overwrite it. Record a
  SUGGESTION row in `investigation-ledger.toon` for the source-side pass (or a human) to fold in.
</evidence_grade_and_mutation>

<decision>
For every (existing entry × mined candidate) pair, and every unmatched item on either side, choose one:

| Action | When | Evidence gate |
|---|---|---|
| **KEEP** | existing entry still accurate; no new signal | none — do not churn |
| **ADD** | mined mechanism has no existing entry | candidate is trust-graded and cited |
| **UPDATE** | existing entry correct but the cohort adds evidence (new incident, recurrence count, a sharper discriminator, a confirmed owner) | additive; never lowers the existing trust grade |
| **REWRITE** | existing entry is now partly wrong, imprecise, or its discriminator misleads, and the cohort shows the correct mechanism | new evidence at least as strong as what it replaces; preserve any still-valid cited history |
| **DELETE** | existing entry is superseded by a merged/broader entry, is a proven duplicate, or was refuted by later evidence/human RCA | removal is itself evidence-gated (see below); always leaves a cited changelog entry |
</decision>

<delete_and_rewrite_gates>
Deletion and rewrite are the destructive actions; gate them hard:
- **Evidence parity.** A DELETE/REWRITE needs evidence at least as strong as the entry it removes/replaces.
  A single `observed` incident does not delete a `verified` multi-incident entry — downgrade or mark
  contested instead.
- **Supersede, don't orphan.** Delete only when the knowledge is preserved elsewhere (merged into a broader
  entry) or genuinely refuted. If a signature still occurs but is now understood differently, REWRITE — do
  not DELETE then ADD (that loses cited history).
- **Refutation is explicit.** Delete-by-refutation requires a cited refuting incident or a human-confirmed
  correction, not mere absence of recent recurrence.
- **Contested beats destructive.** When the mined evidence and the existing entry diverge without a clear
  winner, mark the entry `contested` with the discriminator to resolve it — never silently overwrite.
- **No silent drops.** Every DELETE/REWRITE writes a changelog entry naming what changed and the citation.
</delete_and_rewrite_gates>

<clean_target_state>
The layer must read as if authored fresh against current reality. Do NOT leave migration breadcrumbs —
no "moved from", "formerly", "renamed", "previously said", or transition notes in the failure prose. The
audit trail lives ONLY in the changelog. Preserve stable entry ids where an entry survives (UPDATE/REWRITE
keep the id; DELETE retires it); reuse retired ids never.
</clean_target_state>

<changelog_and_edits>
- **Changelog per pass.** Append one KB_CHANGELOG section per reconciliation pass summarizing the new
  knowledge and every REWRITE/DELETE with its citation. Author names, verbatim human text, and timestamps
  live here, never in consumer prose. A DELETE means removing the file AND adding a one-line dated citation
  to `KB_CHANGELOG.md` explaining why — never a silent drop.
- **Allowlisted writes only.** Write only these paths: `failure-knowledge/**`,
  `00-index/incident-clusters.toon`, `00-index/investigation-ledger.toon`, and `KB_CHANGELOG.md`. Record
  every touched fact's mutation status in `investigation-ledger.toon`. Never write any other `00-index/`
  artifact (`evidence-ledger.toon`, core-map, telemetry-routing-card, ai-asset-catalog, verification-queue,
  ownership, task-router) or anything else — those are source-owned; the gate discards the pass if you do.
- **Edit-only contract.** EDIT files in the working tree only. Do NOT run git, and do NOT stage, commit, or
  push — the launching driver gates the working-tree changes and owns all git, validation, and any push. Do
  NOT write anything under `.git/`, and do NOT add or modify `.gitattributes` or `.gitignore` anywhere (the
  driver rejects the whole pass if you do). Leave your edits in the working tree for the driver to adopt; do
  not clean, reset, or otherwise manage repo state. The driver only launches you on a clean tree, so
  reconcile solely onto the state you were given.
- **Mark collected.** After applying the pass, mark the processed runs collected so future cohorts skip
  them unless they change.
</changelog_and_edits>

<convergence>
Reconciliation must CONVERGE: re-running against unchanged evidence produces NO change. A pass where the
layer already matches the evidence is a valid, SUCCESSFUL no-op — make no edits, invent no changes. Bias to
KEEP; only ADD/UPDATE/REWRITE/DELETE when the evidence demands it. Do not re-litigate a `contested` entry
each run — leave it contested until new evidence resolves it. Process the cohort delta (runs new or changed
since last collected), not the full history every time; skip runs already marked collected unless they
changed. The launching driver treats an unchanged working tree as success, so a converged pass never needs a
cosmetic change to look complete.
</convergence>

<self_check>
Before finishing, confirm: every write lands on an allowlisted path (`failure-knowledge/**`,
`00-index/incident-clusters.toon`, `00-index/investigation-ledger.toon`, `KB_CHANGELOG.md`) — no
source-owned artifact touched; every write is cited and trust-graded; no DELETE/REWRITE violated the
evidence gates; weaker evidence never overwrote a higher-grade fact; every touched fact has a mutation
status in `investigation-ledger.toon`; no protected-home file was deleted or renamed, and every high-grade/
protected conflict is a SUGGESTION row, not an overwrite; prose carries no migration breadcrumbs; the
changelog records every destructive change; stable ids preserved; the edits are the smallest coherent
target-state change, not a churn of unchanged entries; all changes are left in the working tree with no git
run.
</self_check>
