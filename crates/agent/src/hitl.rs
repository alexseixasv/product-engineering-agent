use chrono::Utc;
use schemas::{ApprovalDecision, ApprovalRecord, RunState, validate_approval};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum HitlError {
    #[error("illegal transition from {from:?} to {to:?}")]
    IllegalTransition { from: RunState, to: RunState },
    #[error("human decision only allowed in AwaitingHumanApproval")]
    HumanDecisionNotAllowed,
    #[error("design artifacts require Approved (or later) state")]
    DesignNotAllowed,
    #[error("validation: {0}")]
    Validation(String),
}

#[derive(Debug, Clone)]
pub struct RunMachine {
    pub state: RunState,
    pub approval: Option<ApprovalRecord>,
}

impl RunMachine {
    pub fn new() -> Self {
        Self {
            state: RunState::AcceptedRequest,
            approval: None,
        }
    }

    pub fn transition(&mut self, next: RunState) -> Result<(), HitlError> {
        if !self.state.can_transition_to(next) {
            return Err(HitlError::IllegalTransition {
                from: self.state,
                to: next,
            });
        }
        self.state = next;
        Ok(())
    }

    pub fn apply_human_decision(
        &mut self,
        run_id: &str,
        decision: ApprovalDecision,
        actor: &str,
        comment: Option<String>,
    ) -> Result<&ApprovalRecord, HitlError> {
        if !self.state.allows_human_decision() {
            return Err(HitlError::HumanDecisionNotAllowed);
        }
        let record = ApprovalRecord {
            run_id: run_id.to_string(),
            decision: decision.clone(),
            actor: actor.to_string(),
            decided_at: Utc::now(),
            comment,
        };
        validate_approval(&record).map_err(|e| HitlError::Validation(e.to_string()))?;
        let next = match decision {
            ApprovalDecision::Approve => RunState::Approved,
            ApprovalDecision::Reject => RunState::Rejected,
        };
        self.transition(next)?;
        self.approval = Some(record);
        Ok(self.approval.as_ref().expect("just set"))
    }

    pub fn ensure_can_emit_design(&self) -> Result<(), HitlError> {
        if self.state.allows_design_artifacts()
            || matches!(self.approval.as_ref().map(|a| &a.decision), Some(ApprovalDecision::Approve))
        {
            // Still require Approved or later — reject Completed-from-Rejected
            if matches!(self.state, RunState::Rejected) {
                return Err(HitlError::DesignNotAllowed);
            }
            if matches!(
                self.state,
                RunState::Approved
                    | RunState::GeneratingDesign
                    | RunState::SnapshotReady
                    | RunState::Completed
            ) {
                if matches!(self.state, RunState::Completed) {
                    if let Some(ApprovalRecord {
                        decision: ApprovalDecision::Reject,
                        ..
                    }) = &self.approval
                    {
                        return Err(HitlError::DesignNotAllowed);
                    }
                }
                return Ok(());
            }
        }
        Err(HitlError::DesignNotAllowed)
    }
}

impl Default for RunMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_reaches_awaiting_approval() {
        let mut m = RunMachine::new();
        for next in [
            RunState::Discovering,
            RunState::InspectingDesignSystem,
            RunState::AnalyzingFeasibility,
            RunState::AssessingDebt,
            RunState::DraftingRequirements,
            RunState::DraftingBacklog,
            RunState::AwaitingHumanApproval,
        ] {
            m.transition(next).unwrap();
        }
        assert_eq!(m.state, RunState::AwaitingHumanApproval);
    }

    #[test]
    fn rejects_auto_approve_skip() {
        let mut m = RunMachine::new();
        m.transition(RunState::Discovering).unwrap();
        assert!(m.transition(RunState::Approved).is_err());
    }

    #[test]
    fn human_approve_then_design_allowed() {
        let mut m = RunMachine::new();
        for next in [
            RunState::Discovering,
            RunState::InspectingDesignSystem,
            RunState::AnalyzingFeasibility,
            RunState::AssessingDebt,
            RunState::DraftingRequirements,
            RunState::DraftingBacklog,
            RunState::AwaitingHumanApproval,
        ] {
            m.transition(next).unwrap();
        }
        m.apply_human_decision("r1", ApprovalDecision::Approve, "pm@example.com", None)
            .unwrap();
        assert_eq!(m.state, RunState::Approved);
        assert!(m.ensure_can_emit_design().is_ok());
    }

    #[test]
    fn design_blocked_before_approval() {
        let m = RunMachine::new();
        assert!(m.ensure_can_emit_design().is_err());
    }

    #[test]
    fn human_decision_blocked_outside_awaiting() {
        let mut m = RunMachine::new();
        assert!(m
            .apply_human_decision("r1", ApprovalDecision::Approve, "pm", None)
            .is_err());
    }
}
