use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunState {
    AcceptedRequest,
    Discovering,
    InspectingDesignSystem,
    AnalyzingFeasibility,
    AssessingDebt,
    DraftingRequirements,
    DraftingBacklog,
    AwaitingHumanApproval,
    Approved,
    Rejected,
    GeneratingDesign,
    SnapshotReady,
    Completed,
    Failed,
}

impl RunState {
    pub fn can_transition_to(self, next: RunState) -> bool {
        use RunState::*;
        matches!(
            (self, next),
            (AcceptedRequest, Discovering)
                | (Discovering, InspectingDesignSystem)
                | (InspectingDesignSystem, AnalyzingFeasibility)
                | (AnalyzingFeasibility, AssessingDebt)
                | (AssessingDebt, DraftingRequirements)
                | (DraftingRequirements, DraftingBacklog)
                | (DraftingBacklog, AwaitingHumanApproval)
                | (AwaitingHumanApproval, Approved)
                | (AwaitingHumanApproval, Rejected)
                | (Approved, GeneratingDesign)
                | (GeneratingDesign, SnapshotReady)
                | (SnapshotReady, Completed)
                | (Rejected, Completed)
                | (
                    Discovering
                        | InspectingDesignSystem
                        | AnalyzingFeasibility
                        | AssessingDebt
                        | DraftingRequirements
                        | DraftingBacklog
                        | GeneratingDesign
                        | SnapshotReady,
                    Failed
                )
        )
    }

    pub fn allows_human_decision(self) -> bool {
        matches!(self, RunState::AwaitingHumanApproval)
    }

    pub fn allows_design_artifacts(self) -> bool {
        matches!(
            self,
            RunState::Approved
                | RunState::GeneratingDesign
                | RunState::SnapshotReady
                | RunState::Completed
        )
    }
}
