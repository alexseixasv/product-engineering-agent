use crate::{
    ApprovalRecord, BacklogTicket, Confidence, EngineeringProposal, Evidence, Finding,
};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("{0}")]
    Message(String),
}

pub fn validate_evidence(evidence: &Evidence) -> Result<(), ValidationError> {
    evidence
        .validate()
        .map_err(ValidationError::Message)
}

pub fn validate_finding(finding: &Finding) -> Result<(), ValidationError> {
    if finding.id.trim().is_empty() || finding.summary.trim().is_empty() {
        return Err(ValidationError::Message(
            "finding id/summary required".into(),
        ));
    }
    for e in &finding.evidence {
        validate_evidence(e)?;
    }
    if finding.confidence == Confidence::Fact && finding.evidence.is_empty() {
        return Err(ValidationError::Message(
            "fact findings require evidence".into(),
        ));
    }
    Ok(())
}

pub fn validate_proposal(proposal: &EngineeringProposal) -> Result<(), ValidationError> {
    if proposal.run_id.trim().is_empty() || proposal.request.trim().is_empty() {
        return Err(ValidationError::Message(
            "proposal run_id/request required".into(),
        ));
    }
    for f in &proposal.findings {
        validate_finding(f)?;
    }
    for d in &proposal.technical_debt {
        if d.confidence == Confidence::Fact && d.evidence.is_empty() {
            return Err(ValidationError::Message(
                "fact technical debt requires evidence".into(),
            ));
        }
        for e in &d.evidence {
            validate_evidence(e)?;
        }
    }
    for req in proposal
        .requirements
        .functional
        .iter()
        .chain(proposal.requirements.non_functional.iter())
    {
        if req.id.trim().is_empty() || req.statement.trim().is_empty() {
            return Err(ValidationError::Message(
                "requirement id/statement required".into(),
            ));
        }
        if req.acceptance_criteria.is_empty() {
            return Err(ValidationError::Message(
                "requirement needs acceptance criteria".into(),
            ));
        }
    }
    for ticket in &proposal.backlog {
        ticket
            .validate()
            .map_err(ValidationError::Message)?;
    }
    Ok(())
}

pub fn validate_approval(record: &ApprovalRecord) -> Result<(), ValidationError> {
    record.validate().map_err(ValidationError::Message)
}

pub fn validate_ticket(ticket: &BacklogTicket) -> Result<(), ValidationError> {
    ticket.validate().map_err(ValidationError::Message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DebtImpact, Feasibility, FindingCategory, RequirementsBundle, TechnicalDebtItem,
        TicketType,
    };

    fn sample_evidence() -> Evidence {
        Evidence {
            path: "src/api/tasks.ts".into(),
            start_line: 1,
            end_line: 3,
            excerpt: "export async function listTasks".into(),
        }
    }

    #[test]
    fn rejects_fact_without_evidence() {
        let finding = Finding {
            id: "F1".into(),
            category: FindingCategory::Api,
            summary: "has tasks API".into(),
            confidence: Confidence::Fact,
            evidence: vec![],
        };
        assert!(validate_finding(&finding).is_err());
    }

    #[test]
    fn accepts_fact_with_evidence() {
        let finding = Finding {
            id: "F1".into(),
            category: FindingCategory::Api,
            summary: "has tasks API".into(),
            confidence: Confidence::Fact,
            evidence: vec![sample_evidence()],
        };
        assert!(validate_finding(&finding).is_ok());
    }

    #[test]
    fn proposal_round_trip_json() {
        let proposal = EngineeringProposal {
            run_id: "run-1".into(),
            request: "analytics dashboard".into(),
            findings: vec![Finding {
                id: "F1".into(),
                category: FindingCategory::Gap,
                summary: "no charts".into(),
                confidence: Confidence::Assumption,
                evidence: vec![],
            }],
            feasibility: Feasibility {
                summary: "feasible with new pages".into(),
                approach_options: vec!["reuse Table + new charts page".into()],
                risks: vec!["no metrics API".into()],
            },
            technical_debt: vec![TechnicalDebtItem {
                title: "mock APIs only".into(),
                impact: DebtImpact::Medium,
                recommendation: "introduce real API contract".into(),
                confidence: Confidence::Assumption,
                evidence: vec![],
            }],
            requirements: RequirementsBundle {
                functional: vec![crate::Requirement {
                    id: "FR-1".into(),
                    statement: "Managers can view overdue tasks".into(),
                    acceptance_criteria: vec!["Overdue list visible".into()],
                }],
                non_functional: vec![],
            },
            backlog: vec![BacklogTicket {
                key: "PEA-1".into(),
                ticket_type: TicketType::Story,
                title: "Overdue widget".into(),
                description: "".into(),
                acceptance_criteria: vec!["Shows overdue tasks".into()],
                requirement_ids: vec!["FR-1".into()],
                depends_on: vec![],
                definition_of_done: vec!["Tests pass".into()],
            }],
            open_questions: vec!["Which chart library?".into()],
            design_md_path: None,
            snapshot_path: None,
        };
        let json = serde_json::to_string(&proposal).unwrap();
        let back: EngineeringProposal = serde_json::from_str(&json).unwrap();
        assert_eq!(back.run_id, "run-1");
        assert!(validate_proposal(&back).is_ok());
    }
}
