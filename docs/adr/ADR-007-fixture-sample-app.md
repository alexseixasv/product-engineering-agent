# ADR-007: In-repo fixture sample application

| Field | Value |
|---|---|
| Status | Accepted |
| Date | 2026-11-18 |
| Decider(s) | Solution Architect (persona); Senior Software Engineer (persona) |
| Area | Cross-cutting |

## Context

The agent must inspect a real application. External repos reduce reproducibility for reviewers.

## Decision

- Ship `fixtures/sample-app` as a React + TypeScript app (Vite) with Tailwind and shadcn-style UI primitives.
- Domain: users, projects, tasks (productivity app) with REST-ish API modules and in-memory/mock data.
- Intentionally **omit** analytics dashboard, charts aggregation API, and CSV export — so the agent must detect gaps.
- Include reusable components (Button, Card, Table, Badge, Select) for design-system inspection.
- Document “exists vs missing” in the fixture README.

## Consequences

- Fully offline reproducible discovery demos.
- Controlled technical debt signals can be planted for detection tests.

## Alternatives rejected

| Alternative | Why rejected |
|---|---|
| External real product repo | Unstable for CI and reviewers |
| Empty stub with 2 files | Too weak for architecture reasoning |

## Files touched

| Path | Change |
|---|---|
| `docs/adr/ADR-007-fixture-sample-app.md` | added |
| `fixtures/sample-app/**` | planned |

## Related

- PDRs: PDR-001
