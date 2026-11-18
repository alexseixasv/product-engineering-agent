# HITL state machine

```mermaid
stateDiagram-v2
  [*] --> AcceptedRequest
  AcceptedRequest --> Discovering
  Discovering --> InspectingDesignSystem
  InspectingDesignSystem --> AnalyzingFeasibility
  AnalyzingFeasibility --> AssessingDebt
  AssessingDebt --> DraftingRequirements
  DraftingRequirements --> DraftingBacklog
  DraftingBacklog --> AwaitingHumanApproval
  AwaitingHumanApproval --> Approved: approve
  AwaitingHumanApproval --> Rejected: reject
  Approved --> GeneratingDesign
  GeneratingDesign --> SnapshotReady
  SnapshotReady --> Completed
  Rejected --> Completed
  Discovering --> Failed: error
  InspectingDesignSystem --> Failed: error
  AnalyzingFeasibility --> Failed: error
  AssessingDebt --> Failed: error
  DraftingRequirements --> Failed: error
  DraftingBacklog --> Failed: error
  GeneratingDesign --> Failed: error
  SnapshotReady --> Failed: error
```

## Invariants

1. `approve` / `reject` only from `AwaitingHumanApproval`.
2. `design.md` and snapshots only after `Approved`.
3. ApprovalRecord required on approve/reject transitions.
4. No implicit transition to `Approved`.
