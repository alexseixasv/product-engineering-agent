# ADR-002: Multi-stage single orchestrator agent topology

| Field | Value |
|---|---|
| Status | Accepted |
| Date | 2026-11-18 |
| Decider(s) | Solution Architect (persona) |
| Area | Backend |

## Context

The challenge asks whether to use a single agent with tools or a multi-agent system. Unbounded multi-agent swarms increase nondeterminism and make HITL and testing harder.

## Decision

We use a **single orchestrator** that runs a fixed multi-stage pipeline:

1. Repository Discovery  
2. Design System Inspection  
3. Architecture & Feasibility  
4. Technical Debt Assessment  
5. FR & NFR  
6. Jira-Ready Backlog  
7. Awaiting Human Approval  
8. Design Spec (`design.md`) — post-approval only  
9. Frontend Snapshot — post-approval only  

Deterministic stages own I/O and state transitions. LLM calls occur only inside explicit reasoning steps and must return schema-validated structures.

## Consequences

- Clear state machine and test hooks per stage.
- Less emergent “agent debate”; more predictable proposals.
- Cross-stage synthesis is the orchestrator’s job, not a free-form swarm.

## Alternatives rejected

| Alternative | Why rejected |
|---|---|
| Free multi-agent swarm | Hard to test, weak HITL guarantees |
| Single monolithic LLM call | Poor evidence grounding; hard to validate partial failures |

## Files touched

| Path | Change |
|---|---|
| `docs/adr/ADR-002-multi-stage-orchestrator.md` | added |
| `docs/specs/pipeline.md` | added |
| `crates/agent/**` | planned |

## Related

- PDRs: PDR-001
- ADRs: ADR-004, ADR-005
