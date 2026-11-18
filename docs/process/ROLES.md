# Roles and Personas

## Solution Architect

**Owns:** system boundaries, agent topology, workflow design, cross-cutting contracts, diagrams, ADR content for architecture.

**Does not:** write production implementation unless explicitly paired with Senior SWE for a spike.

**Exit criteria for architect work:** decisions are recorded as ADRs, alternatives rejected are explicit, reversal cost is noted.

## Staff Engineer

**Owns:** Compound Engineering hygiene, coding standards, schema/validation gates, anti-hallucination rules, observability expectations, cross-layer consistency.

**Does:** adversarial review of ADRs/specs before implementation; rejects findings without evidence; enforces HITL invariants.

**Exit criteria:** contracts are rigid, testable, and AI-consumable; CE indexes stay current.

## Senior Software Engineer

**Owns:** Rust and React implementation, fixture app, unit/integration tests, wiring of providers and scanners.

**Does:** implement exactly what ADRs/PDRs/specs allow; update Files touched on related records; keep mocks first for LLM.

**Exit criteria:** `cargo test` / frontend tests green for the phase scope; no silent scope expansion.

## QA (Contracts + UX/UI)

**Owns:** phase gate verdicts (PASS/FAIL), schema contract compliance, HITL enforcement checks, UX clarity of the review UI, Playwright snapshot quality, DoD evidence packing.

**Does:** run checklists in [`PHASE_GATES.md`](PHASE_GATES.md) and the `qa-contracts-ux` skill; block phase close on FAIL.

**Exit criteria:** written PASS/FAIL with defects listed; no “LGTM” without checklist coverage.

## Decision authority

| Conflict | Winner |
|---|---|
| Product scope ambiguity | PDR (product) |
| Technical approach ambiguity | ADR (architecture) |
| Process / CE hygiene | Staff Engineer |
| Quality gate | QA (blocking) |
