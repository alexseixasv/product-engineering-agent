# AI Usage Log

Disclosure log for AI-assisted work on this selection-process exercise.  
Every substantive AI contribution must be recorded here.

## Entry template

| Field | Description |
|---|---|
| Date | ISO date |
| Phase | Phase number |
| Tool / model | e.g. Cursor agent / Composer |
| Purpose | Why AI was used |
| Artifacts | Paths created or edited |
| Human approval | Who approved continuation |
| Residual risk | What still needs human judgment |

---

## Entries

### 2026-11-18 — Phase 0 bootstrap

| Field | Value |
|---|---|
| Date | 2026-11-18 |
| Phase | 0 |
| Tool / model | Cursor agent (Composer) |
| Purpose | Scaffold Compound Engineering documentation system, role definitions, phase gates, project skills, initial PDR/ADR |
| Artifacts | `AGENTS.md`; `docs/**`; `.cursor/skills/**`; `docs/pdr/PDR-001-*.md`; `docs/adr/ADR-001-*.md` |
| Human approval | User approved tactical plan and requested full implementation |
| Residual risk | Architecture ADRs and schemas deferred to Phase 1; must not invent runtime behavior before contracts exist |

### 2026-11-18 — Phase 1 architecture freeze

| Field | Value |
|---|---|
| Date | 2026-11-18 |
| Phase | 1 |
| Tool / model | Cursor agent (Composer) |
| Purpose | Formalize architecture ADRs and machine-readable contracts |
| Artifacts | `docs/adr/ADR-002`…`ADR-007`; `docs/specs/*` |
| Human approval | Implied by full-plan execution request |
| Residual risk | Persistence choice and LLM vendor details may need revisit after first integration test |

### 2026-11-18 — Phases 2–8 implementation

| Field | Value |
|---|---|
| Date | 2026-11-18 |
| Phase | 2–8 |
| Tool / model | Cursor agent (Composer) |
| Purpose | Implement fixture app, Rust workspace, repo intel, LLM pipeline, API/UI, design snapshot, DoD hardening |
| Artifacts | `fixtures/sample-app/**`; `crates/**`; `frontend/**`; `docs/adr/ADR-008-*`; `docs/solutions/**`; `docs/plans/dod-evidence.md`; `README.md` (quick start) |
| Human approval | Implied by full-plan execution request; commits remain manual via GitHub Desktop |
| Residual risk | Humans must review HITL semantics, evidence quality, and UX before treating the system as production-ready; Windows builds require MinGW on PATH; run store is in-memory (ADR-008) |

### Verification performed by AI (same session)

| Check | Result |
|---|---|
| `cargo test --workspace` | PASS (13 tests) |
| `fixtures/sample-app` build | PASS |
| `frontend` build | PASS |
| Playwright analytics prototype | PASS → `artifacts/playwright/analytics-prototype.png` |

### 2026-11-18 — Docker + Postgres persistence

| Field | Value |
|---|---|
| Date | 2026-11-18 |
| Phase | infra follow-up |
| Tool / model | Cursor agent (Composer) |
| Purpose | Add Docker images, Compose stack, Postgres run store, migrations, ADR-009 |
| Artifacts | `Dockerfile`; `frontend/Dockerfile`; `docker-compose.yml`; `migrations/`; `crates/db/**`; API/store refactor |
| Human approval | User requested Docker/DB adjustments + commit |
| Residual risk | Full `docker compose build` should be verified on a machine with Docker; image build time is substantial |
