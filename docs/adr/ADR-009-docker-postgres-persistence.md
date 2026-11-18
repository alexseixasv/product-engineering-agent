# ADR-009: Docker Compose + PostgreSQL persistence

| Field | Value |
|---|---|
| Status | Accepted |
| Date | 2026-11-18 |
| Decider(s) | Solution Architect (persona); Senior Software Engineer (persona) |
| Area | Infra |

## Context

ADR-008 scoped the MVP to an in-memory run store for Windows toolchain friction. Reviewers and portfolio demos now need a reproducible multi-service stack (API image, database, frontend) with durable approval/proposal records across restarts.

## Decision

- Ship `Dockerfile` (API), `frontend/Dockerfile` (nginx static + API proxy), and `docker-compose.yml` with **PostgreSQL 16**, API, and frontend.
- Persist runs in Postgres via SQLx (`crates/db::PostgresStore`) when `DATABASE_URL` is set.
- Apply SQL migrations from `migrations/` at API startup.
- Keep **in-memory** store when `DATABASE_URL` is unset (local `cargo test` / quick demos).
- Artifacts (`design.md`, snapshots) remain on a Docker volume / host `artifacts/` directory.
- ADR-008 remains historically accurate for the earlier MVP; this ADR supersedes it for the default Docker deployment path.

## Consequences

- `docker compose up --build` gives a full stack with durable runs.
- API image build requires network to crates.io during CI/local build.
- Windows native demos can still run without Docker using memory store.

## Alternatives rejected

| Alternative | Why rejected |
|---|---|
| SQLite-only in container | Weaker multi-service story for portfolio; Postgres matches challenge suggestion |
| Require Postgres for all local tests | Slows unit tests; memory store remains for `cargo test` |

## Files touched

| Path | Change |
|---|---|
| `Dockerfile` | added |
| `frontend/Dockerfile` | added |
| `frontend/nginx.conf` | added |
| `docker-compose.yml` | added |
| `migrations/001_init.sql` | added |
| `crates/db/**` | added |
| `crates/agent/src/store.rs` | modified (RunStore trait) |
| `crates/api/src/main.rs` | modified |
| `.env.example` | added |
| `.dockerignore` | added |

## Related

- ADRs: ADR-006, ADR-008 (superseded for Docker default)
- PDRs: PDR-001
