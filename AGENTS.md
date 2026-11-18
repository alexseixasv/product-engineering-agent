# AGENTS.md — AI-Ready Repository Map

This repository implements the **Product Engineering Agent** challenge: a Rust-powered AI engineering assistant that turns product feature requests into evidence-based, human-approved engineering proposals.

## Operating rules (binding)

1. **No commits by agents.** Humans commit via GitHub Desktop after approving a phase.
2. **Phase gates.** Do not start Phase N+1 until Phase N has a QA PASS and human approval.
3. **Compound Engineering.** Every material decision lands in a PDR (product) or ADR (architecture) with **Files touched**, decider, and rationale.
4. **AI usage.** Log every substantive AI-assisted change in [`docs/process/AI_USAGE.md`](docs/process/AI_USAGE.md).
5. **Language.** All technical documentation is **English**.

## Personas

See [`docs/process/ROLES.md`](docs/process/ROLES.md).

| Persona | When to invoke |
|---|---|
| Solution Architect | Topology, contracts, tradeoffs, diagrams |
| Staff Engineer | CE hygiene, schema gates, anti-hallucination |
| Senior Software Engineer | Rust/React implementation and tests |
| QA (Contracts + UX/UI) | Phase gate PASS/FAIL, DoD, Playwright |

## Project skills

Located under [`.cursor/skills/`](.cursor/skills/):

| Skill | Purpose |
|---|---|
| `ce-phase-gate` | Close a phase (docs + ADR/PDR + AI_USAGE) |
| `ce-adr-author` | Author ADRs with Files touched |
| `ce-pdr-author` | Author PDRs with Files touched |
| `solution-architect` | Architecture decisions only (no impl) |
| `staff-engineer-review` | Contract / evidence / CE review |
| `senior-impl-rust` | Rust implementation aligned to specs |
| `qa-contracts-ux` | Schema, HITL, UX/UI, DoD validation |

## Documentation layout

```text
docs/
  adr/          Architecture Decision Records
  pdr/          Product Decision Records
  plans/        Phase execution plans
  specs/        API and JSON schemas / contracts
  solutions/    Durable CE learnings
  process/      Roles, gates, AI usage
  guides/       How-to for humans and agents
```

## Source layout (target)

```text
crates/                 Rust workspace
  schemas/              Shared Serde types + validation
  repo_intel/           Read-only repository intelligence
  llm/                  HTTP + mock LLM providers
  agent/                Pipeline orchestration + HITL
  api/                  Axum HTTP API
fixtures/sample-app/    Target application under analysis
frontend/               React review UI for proposals
```

## Definition of Done

Track against the root [`README.md`](README.md) DoD checklist. Phase 8 must prove each item with evidence.

## Phase map

| Phase | Focus |
|---|---|
| 0 | Documentation & agent OS |
| 1 | Architecture freeze + contracts |
| 2 | Fixture sample app |
| 3 | Rust workspace + HITL core |
| 4 | Repository intelligence |
| 5 | LLM + pipeline stages |
| 6 | API + React review UI |
| 7 | design.md + Playwright snapshot |
| 8 | Hardening + DoD proof + CE compound |

Gates: [`docs/process/PHASE_GATES.md`](docs/process/PHASE_GATES.md).
