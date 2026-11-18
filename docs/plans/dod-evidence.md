# Definition of Done — evidence map

| DoD item | Evidence |
|---|---|
| PM can submit a feature request | `POST /api/runs` + frontend form in `frontend/src/App.tsx` |
| Agent inspects a real sample codebase | `fixtures/sample-app` + `repo_intel::scan_repository` test |
| Architecture/components identified | Findings categories in proposal; design-system paths scanned |
| Findings include source evidence | `schemas::validate_finding` rejects facts without evidence; scanner attaches excerpts |
| Feasibility + technical debt reports | `EngineeringProposal.feasibility` + `technical_debt` |
| FR/NFR produced | `requirements.functional` / `non_functional` with acceptance criteria |
| Jira-ready tickets | `backlog` with `requirement_ids` traceability |
| Human approval enforced | HITL state machine tests; design blocked pre-approval |
| `design.md` produced | Post-approve `generate_design_and_snapshot` |
| Frontend snapshot reviewable | Playwright `frontend/e2e/analytics-prototype.spec.ts` |
| Core workflows automated tests | `cargo test --workspace` (13 tests) |

## QA Verdict — Phase 8

Status: **PASS**  
Date: 2026-11-18
