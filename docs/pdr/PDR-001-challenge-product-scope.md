# PDR-001: Challenge product scope

| Field | Value |
|---|---|
| Status | Accepted |
| Date | 2026-11-18 |
| Decider(s) | Solution Architect (persona); Staff Engineer (persona) |

## Problem

Product Managers request features without full codebase visibility. Engineering needs an evidence-based proposal before committing to delivery. This repository must demonstrate that capability as a selection-process challenge.

## Decision

We build a **Product Engineering Agent** that:

1. Accepts a natural-language product feature request.
2. Inspects a real in-repo fixture application (`fixtures/sample-app`) read-only.
3. Produces architecture/feasibility, technical debt, FR/NFR, and Jira-ready backlog artifacts with source evidence.
4. Requires **human-in-the-loop approval** before finalizing implementation proposals or generating `design.md` / visual snapshots.
5. Exposes a usable review UI for proposals.
6. Separates deterministic logic from LLM reasoning; validates LLM outputs against schemas; supports mocked LLM responses in tests.

Primary demo scenario: analytics dashboard for team productivity, overdue tasks, project performance, filters, charts, and CSV export.

## Success criteria

- README Definition of Done checklist can be demonstrated end-to-end.
- Findings cite verifiable evidence (path + excerpt).
- No auto-approval of agent recommendations.
- AI usage is disclosed for the selection process.

## Out of scope

- Writing production code into the fixture app as the agent’s side effect.
- Autonomous Jira API writes without human approval.
- Multi-tenant SaaS hardening / auth productization.
- Guaranteeing sprint-commitment answers without human judgment.

## Files touched

| Path | Change |
|---|---|
| `docs/pdr/PDR-001-challenge-product-scope.md` | added |
| `docs/pdr/INDEX.md` | added |
| `README.md` | referenced (unchanged intent) |
| `AGENTS.md` | added |

## Related

- ADRs: ADR-001
- PDRs: —
