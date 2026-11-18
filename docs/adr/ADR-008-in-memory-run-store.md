# ADR-008: In-memory run store with filesystem artifacts

| Field | Value |
|---|---|
| Status | Superseded by ADR-009 (Docker/Postgres default; memory remains for tests) |
| Date | 2026-11-18 |
| Decider(s) | Solution Architect (persona); Senior Software Engineer (persona) |
| Area | Backend |

## Context

ADR-006 selected SQLite for persistence. For the challenge MVP, wiring SQLx increased build/tooling friction on Windows without improving the demonstrable HITL/evidence story. Approval audit within a process lifetime plus on-disk `design.md`/snapshots already satisfy reviewer workflows.

## Decision

- Default persistence for proposal runs is an **in-memory store** (`InMemoryStore`) inside the API process.
- Durable artifacts (`design.md`, snapshot stubs/images) are written under `artifacts/{run_id}/`.
- ADR-006 remains the north-star for a future SQLite/Postgres adapter behind the same store trait; this ADR scopes the MVP.

## Consequences

- Restarts clear run history (documented limitation).
- Faster local `cargo test` and simpler Windows toolchain story.
- UI “recent runs” is session-scoped.

## Alternatives rejected

| Alternative | Why rejected (for MVP) |
|---|---|
| Block shipping on SQLite | Tooling cost outweighed demo value |
| No persistence of artifacts | Would break design.md / snapshot DoD |

## Files touched

| Path | Change |
|---|---|
| `docs/adr/ADR-008-in-memory-run-store.md` | added |
| `crates/agent/src/store.rs` | added |
| `Cargo.toml` | modified (sqlx deferred) |

## Related

- ADRs: ADR-006
