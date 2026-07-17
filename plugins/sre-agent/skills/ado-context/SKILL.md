---
name: ado-context
description: >-
  Read-only Azure DevOps collector that uses ADO MCP tools for livesite investigations:
  fetches PRs, commits, changed files/diffs, work items, code references, and
  build/release/deployment context. Use when the incident may involve a recent change:
  deployments, releases, rollouts, PRs/commits, changed files, work items, build/pipeline
  results, or config-as-code around the incident window. Use for trigger phrases: "ADO PR",
  "recent deployments", "what changed", "code references", "work items", and
  "build/release for this service". Boundary: read-only, does not mutate ADO.
---

# ADO Context

Read-only collector for Azure DevOps change and rollout evidence around an incident window.
ADO MCP handles auth, bounding parameters, and pagination; this skill preserves scope,
provenance, and gaps in compact observations.

## What it fetches

- PRs, commits, changed files, and line-level diffs for the bounded service/repo/window.
- Linked work items, labels, identities, and code references that explain intent or scope.
- Builds, releases, deployments, artifacts, stages, results, rollback and rollout signals.
- Config-as-code from ADO repos: branches, directories, file content, and revisions.
- Optional Advanced Security alerts when security findings may explain the incident.

## Success criteria

- Each material fact is an observation with a source pointer: ADO MCP tool plus ids.
- Records project, repository/repositoryId, branch, window, filters, caps, and `bounds_applied`.
- Preserves who/what/when/where rollout attributes: actor, artifact, stage, region/ring, result.
- States truncation, skipped pagination, auth/tool gaps, and dropped-out-of-window counts honestly.
- Uses the honesty floor: collectors are read-only; secrets stay out of output; gaps are explicit;
  observations cite source; ADO narrative stays an unverified claim until corroborated.

## How it fetches

| Evidence need | ADO MCP read tools |
|---|---|
| PR discovery/details | `ado-repo_list_pull_requests_by_repo_or_project`, `ado-repo_get_pull_request_by_id`, `ado-repo_list_pull_requests_by_commits` |
| PR files and diffs | `ado-repo_get_pull_request_changes` |
| Commits and work item links | `ado-repo_search_commits`, `ado-repo_get_pull_request_by_id` with `includeWorkItemRefs` |
| Code & work-item search/detail | `ado-search_code`, `ado-search_workitem`, `ado-wit_get_work_item`, `ado-wit_get_work_items_batch_by_ids`, `ado-wit_get_work_item_attachment` |
| Repos, branches, config-as-code | `ado-repo_list_repos_by_project`, `ado-repo_get_repo_by_name_or_id`, `ado-repo_list_branches_by_repo`, `ado-repo_list_directory`, `ado-repo_get_file_content` |
| Builds, deployments, rollout signals | `ado-pipelines_get_builds`, `ado-pipelines_get_build_status`, `ado-pipelines_get_build_changes`, `ado-pipelines_get_build_definitions`, `ado-pipelines_list_runs`, `ado-pipelines_get_run`, `ado-pipelines_list_artifacts`, `ado-pipelines_download_artifact`, `ado-testplan_show_test_results_from_build_id` |
| Wiki/runbook context | `ado-search_wiki` |
| Projects, teams, identities | `ado-core_list_projects`, `ado-core_list_project_teams`, `ado-core_get_identity_ids` |
| Optional security evidence | `ado-advsec_get_alerts`, `ado-advsec_get_alert_details` |

Use tool parameters for bounds: `top`, `fromDate`/`toDate`, `minTime`/`maxTime`, `status`,
`resultFilter`, `statusFilter`, `targetRefName`, `branchName`, `definitions`, `author`,
`created_by_user`, and `searchText`. Use `continuationToken` or `skip` when exposed by the
tool; if not followed, record the gap.

## Observation examples

```text
- obs_id: OBS210
  source: ado-repo_get_pull_request_by_id:4567
  time: 2026-05-28T09:40Z
  entity: repo/<project>/<repo>
  finding: PR 4567 "Adjust service heartbeat timeout" completed into main at 09:40Z; 6 files changed.
  attributes: { pr: 4567, target: main, files_changed: 6, areas: [src/service/heartbeat],
                author: "<name>", work_items: [123456], bounds_applied: "window+top=50" }
  confidence: high
  data_quality: complete

- obs_id: OBS211
  source: ado-pipelines_get_build_status:991
  time: 2026-05-28T10:05Z..2026-05-28T10:25Z
  entity: pipeline/<service>-release
  finding: Build 991 artifact R-88 deployed to prod ring wus2-03 at 10:05Z and succeeded.
  attributes: { build_id: 991, artifact: R-88, stage: "prod/wus2-03",
                rollout_reached: ["wus2-03@10:05Z"], result: succeeded, rollback_available: true }
  confidence: high
  data_quality: complete
```

## Read-only boundary

Call ONLY the read tools in the How it fetches table. Any ADO MCP tool not listed is out of scope — every `create_*`, `update_*`, `run_*`, `add_*`, `link_*`, `vote_*`, comment/reply/thread-write, and test-plan create/update tool mutates ADO and must not be called.

For `ado-pipelines_download_artifact` and `ado-wit_get_work_item_attachment`, do not write to disk (omit `destinationPath`); if an artifact/attachment is large or sensitive, record a gap instead of pulling it inline.

## Stop rules

- Missing project or incident window: ask for bounds before fetching.
- Missing repository/repositoryId: ask only before repo, PR, diff, or code-file calls; build/deployment and work-item queries can proceed with project scope.
- Missing pipeline/build/definition id: discover it via a bounded project query (e.g. `ado-pipelines_get_build_definitions` / `ado-pipelines_get_builds`) or ask, before pipeline-specific calls.
- ADO MCP or auth unavailable: surface the typed blocker and record an explicit gap; do not fall
  back to interactive prompts or alternate backends.
- Do not exceed the window: filter by date/time, and record dropped-out-of-window counts.
- Asked to mutate ADO: stop with the read-only boundary.

## Invariants

Read-only; observations cite source; secrets stay out of output; ADO/change narrative stays an unverified claim until corroborated; gaps are explicit; never fabricate.
