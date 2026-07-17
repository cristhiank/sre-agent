# SRE Skills for Copilot CLI

Host-agnostic SRE incident root-cause analysis skills for Copilot CLI. The agent coordinates intake, evidence discovery, specialist analysis, confidence grading, and evidence-cited reporting while staying capability-driven rather than tied to a specific incident host.

## Install

From a local checkout of this repository:

```text
/plugin marketplace add sre-skills <path-to-this-repo>
/plugin install sre-agent@sre-skills
```

Install the onboarding plugin when you want service discovery support:

```text
/plugin install sre-onboarding@sre-skills
```

## Plugin contents

| Plugin | Skill | Purpose |
|---|---|---|
| sre-agent | sre-agent | Coordinates incident intake, capability mapping, scout, specialist passes, grading, and reports. |
| sre-agent | sre-agent-dependency | Analyzes dependency, downstream, provider-health, retry, and latency-propagation hypotheses. |
| sre-agent | sre-agent-timeline | Analyzes onset timing, changepoints, rollout correlation, and cause-before-effect ordering. |
| sre-agent | sre-agent-signal-validity | Checks whether signals are scoped, reliable, comparable, and strong enough to support claims. |
| sre-agent | sre-agent-service-mechanics | Verifies telemetry-plausible mechanisms against implementation, configuration, and lifecycle behavior. |
| sre-agent | service-catalog | Orients investigations to service docs, dependencies, ownership, runbooks, known failure modes, and telemetry pointers. |
| sre-agent | ado-context | Collects read-only Azure DevOps change, PR, build, release, deployment, and work-item context. |
| sre-onboarding | service-onboarding | Builds or refreshes service knowledge needed for later incident investigation and on-call work. |

## Telemetry via Kusto MCP

Install the official public Kusto MCP server separately when you want the agent to query telemetry. When a bounded read-only Kusto MCP capability is present, the agent can leverage it automatically through capability discovery. When it is absent, investigations continue with an explicit `no telemetry capability` gap instead of requiring a bundled CLI.

## Safety model

The skills are designed to be read-only by default. Any live incident-system update requires explicit authorization, preview/dry-run, capability-owned idempotency/audit checks, and a final scrub of the external payload.