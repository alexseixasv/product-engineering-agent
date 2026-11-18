# ADR-006: Persistence with SQLite for local challenge runs

| Field | Value |
|---|---|
| Status | Accepted |
| Date | 2026-11-18 |
| Decider(s) | Solution Architect (persona); Senior Software Engineer (persona) |
| Area | Backend |

## Context

Suggested stack includes PostgreSQL + SQLx. For a selection-process repo, local friction (Docker/Postgres) competes with demonstrability.

## Decision

- Use **SQLite** via SQLx for challenge default persistence (runs, proposals, approvals).
- Keep SQL and repository traits compatible with a future Postgres URL.
- Store proposal JSON documents validated against schemas before write.
- File artifacts (`design.md`, snapshots) live on disk under `artifacts/{run_id}/` with DB metadata pointers.

## Consequences

- `cargo test` and local demo work without Docker.
- Production-like Postgres remains a documented evolution, not a blocker.

## Alternatives rejected

| Alternative | Why rejected |
|---|---|
| Postgres-only | Higher setup cost for reviewers |
| In-memory only | Loses approval audit across restarts |

## Files touched

| Path | Change |
|---|---|
| `docs/adr/ADR-006-persistence.md` | added |
| `crates/api/**` | planned |

## Related

- ADRs: ADR-004
