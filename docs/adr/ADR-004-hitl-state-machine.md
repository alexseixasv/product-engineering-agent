# ADR-004: Human-in-the-loop state machine

| Field | Value |
|---|---|
| Status | Accepted |
| Date | 2026-11-18 |
| Decider(s) | Solution Architect (persona); Staff Engineer (persona) |
| Area | Backend |

## Context

Human approval must be recorded and enforced before finalizing implementation proposals or external/final artifacts (`design.md`, snapshots).

## Decision

Proposal run states:

```text
AcceptedRequest
  -> Discovering
  -> InspectingDesignSystem
  -> AnalyzingFeasibility
  -> AssessingDebt
  -> DraftingRequirements
  -> DraftingBacklog
  -> AwaitingHumanApproval
  -> Approved -> GeneratingDesign -> SnapshotReady -> Completed
  -> Rejected -> Completed
  -> Failed
```

Rules:

- Only `AwaitingHumanApproval` accepts `approve` / `reject` commands.
- `Approved` is required before `GeneratingDesign` or snapshot generation.
- Approval records store actor, timestamp, decision, optional comment.
- There is **no** auto-approve path in code or config defaults.

## Consequences

- UI and API share the same state enum.
- Tests can assert illegal transitions fail.
- Rejected runs still retain proposal artifacts for audit.

## Alternatives rejected

| Alternative | Why rejected |
|---|---|
| Soft “recommended approve” without hard gate | Violates challenge constraint |
| Approval only in UI without persisted record | Not auditable |

## Files touched

| Path | Change |
|---|---|
| `docs/adr/ADR-004-hitl-state-machine.md` | added |
| `docs/specs/approval.schema.json` | added |
| `docs/specs/hitl-states.md` | added |
| `crates/agent/**` | planned |

## Related

- PDRs: PDR-001
- ADRs: ADR-002
