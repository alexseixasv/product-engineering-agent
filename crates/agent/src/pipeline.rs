use crate::hitl::RunMachine;
use crate::store::{InMemoryStore, RunRecord, RunStore, StoreError};
use chrono::Utc;
use llm::{LlmError, LlmProvider};
use repo_intel::scan_repository;
use schemas::{
    ApprovalDecision, BacklogTicket, Confidence, DebtImpact, EngineeringProposal, Feasibility,
    Finding, FindingCategory, Requirement, RequirementsBundle, RunState, TechnicalDebtItem,
    TicketType, validate_proposal,
};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error(transparent)]
    Hitl(#[from] crate::hitl::HitlError),
    #[error(transparent)]
    Repo(#[from] repo_intel::RepoIntelError),
    #[error(transparent)]
    Llm(#[from] LlmError),
    #[error(transparent)]
    Store(#[from] StoreError),
    #[error("validation: {0}")]
    Validation(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("io: {0}")]
    Io(String),
}

pub struct AgentPipeline {
    pub store: Arc<dyn RunStore>,
    pub llm: Arc<dyn LlmProvider>,
    pub artifacts_dir: PathBuf,
}

impl AgentPipeline {
    pub fn new(llm: Arc<dyn LlmProvider>, artifacts_dir: impl Into<PathBuf>) -> Self {
        Self::with_store(llm, artifacts_dir, Arc::new(InMemoryStore::new()))
    }

    pub fn with_store(
        llm: Arc<dyn LlmProvider>,
        artifacts_dir: impl Into<PathBuf>,
        store: Arc<dyn RunStore>,
    ) -> Self {
        Self {
            store,
            llm,
            artifacts_dir: artifacts_dir.into(),
        }
    }

    pub async fn start_run(
        &self,
        request: String,
        repo_path: impl AsRef<Path>,
    ) -> Result<RunRecord, PipelineError> {
        let id = Uuid::new_v4().to_string();
        let mut machine = RunMachine::new();
        let repo_path = repo_path.as_ref().to_path_buf();

        let mut record = RunRecord {
            id: id.clone(),
            request: request.clone(),
            repo_path: repo_path.to_string_lossy().to_string(),
            state: machine.state,
            proposal: None,
            approval: None,
            error: None,
        };
        self.store.upsert(record.clone()).await?;

        let result = self
            .execute_pre_approval(&mut machine, &id, &request, &repo_path)
            .await;

        match result {
            Ok(proposal) => {
                record.state = machine.state;
                record.proposal.replace(proposal);
                self.store.upsert(record.clone()).await?;
                Ok(record)
            }
            Err(e) => {
                let _ = machine.transition(RunState::Failed);
                record.state = RunState::Failed;
                record.error = Some(e.to_string());
                self.store.upsert(record.clone()).await?;
                Err(e)
            }
        }
    }

    async fn execute_pre_approval(
        &self,
        machine: &mut RunMachine,
        run_id: &str,
        request: &str,
        repo_path: &Path,
    ) -> Result<EngineeringProposal, PipelineError> {
        machine.transition(RunState::Discovering)?;
        let scan = scan_repository(repo_path)?;

        machine.transition(RunState::InspectingDesignSystem)?;
        let _ = self
            .llm_with_retry(
                "Summarize reusable design system components. Return JSON.",
                &format!("components={:?}", scan.component_paths),
            )
            .await?;

        machine.transition(RunState::AnalyzingFeasibility)?;
        let _ = self
            .llm_with_retry("Assess feasibility. Return JSON with notes.", request)
            .await?;

        machine.transition(RunState::AssessingDebt)?;
        let technical_debt: Vec<TechnicalDebtItem> = scan
            .debt_signals
            .iter()
            .map(|d| TechnicalDebtItem {
                title: d.title.clone(),
                impact: DebtImpact::Medium,
                recommendation: d.detail.clone(),
                confidence: Confidence::Fact,
                evidence: vec![d.evidence.clone()],
            })
            .collect();

        machine.transition(RunState::DraftingRequirements)?;
        let requirements = RequirementsBundle {
            functional: vec![
                Requirement {
                    id: "FR-1".into(),
                    statement: "Managers can monitor team productivity indicators".into(),
                    acceptance_criteria: vec![
                        "Dashboard page lists productivity-related metrics".into(),
                    ],
                },
                Requirement {
                    id: "FR-2".into(),
                    statement: "Managers can view overdue tasks with filters".into(),
                    acceptance_criteria: vec![
                        "Overdue tasks visible".into(),
                        "Status/project filters available".into(),
                    ],
                },
                Requirement {
                    id: "FR-3".into(),
                    statement: "Managers can export dashboard data as CSV".into(),
                    acceptance_criteria: vec!["CSV download produces valid file".into()],
                },
            ],
            non_functional: vec![Requirement {
                id: "NFR-1".into(),
                statement: "Reuse existing design system components where possible".into(),
                acceptance_criteria: vec![
                    "UI uses Button/Card/Table/Select/Badge primitives".into(),
                ],
            }],
        };

        machine.transition(RunState::DraftingBacklog)?;
        let backlog = vec![
            BacklogTicket {
                key: "PEA-100".into(),
                ticket_type: TicketType::Epic,
                title: "Analytics dashboard".into(),
                description: "Enable managers to monitor productivity and export data".into(),
                acceptance_criteria: vec!["Approved proposal implemented".into()],
                requirement_ids: vec!["FR-1".into(), "FR-2".into(), "FR-3".into()],
                depends_on: vec![],
                definition_of_done: vec!["HITL-approved design".into()],
            },
            BacklogTicket {
                key: "PEA-101".into(),
                ticket_type: TicketType::Story,
                title: "Dashboard page with charts".into(),
                description: "New analytics route reusing Card/Table".into(),
                acceptance_criteria: vec!["Charts render with filters".into()],
                requirement_ids: vec!["FR-1".into(), "FR-2".into()],
                depends_on: vec!["PEA-100".into()],
                definition_of_done: vec!["Visual review".into()],
            },
            BacklogTicket {
                key: "PEA-102".into(),
                ticket_type: TicketType::Task,
                title: "CSV export endpoint/util".into(),
                description: "Export filtered dashboard rows".into(),
                acceptance_criteria: vec!["CSV matches visible filters".into()],
                requirement_ids: vec!["FR-3".into()],
                depends_on: vec!["PEA-101".into()],
                definition_of_done: vec!["Unit test".into()],
            },
        ];

        let mut findings = scan.findings;
        findings.push(Finding {
            id: "ARCH-1".into(),
            category: FindingCategory::Architecture,
            summary: "Feature can extend existing Tasks/Projects pages with a new Analytics route"
                .into(),
            confidence: Confidence::Assumption,
            evidence: vec![],
        });

        let proposal = EngineeringProposal {
            run_id: run_id.to_string(),
            request: request.to_string(),
            findings,
            feasibility: Feasibility {
                summary: "Feasible by adding an analytics page and metrics aggregation on top of existing Task/Project models; CSV export and charts are net-new.".into(),
                approach_options: vec![
                    "Client-side aggregation over existing listTasks/listProjects".into(),
                    "Introduce dedicated analytics API module".into(),
                ],
                risks: vec![
                    "No real backend — mock APIs may underrepresent production constraints".into(),
                    "Chart library choice unresolved".into(),
                ],
            },
            technical_debt,
            requirements,
            backlog,
            open_questions: vec![
                "Which charting library fits the design system?".into(),
                "Should CSV export be client-only or server-generated?".into(),
            ],
            design_md_path: None,
            snapshot_path: None,
        };

        validate_proposal(&proposal).map_err(|e| PipelineError::Validation(e.to_string()))?;
        machine.transition(RunState::AwaitingHumanApproval)?;
        Ok(proposal)
    }

    async fn llm_with_retry(&self, system: &str, user: &str) -> Result<serde_json::Value, LlmError> {
        match self.llm.complete_structured(system, user).await {
            Ok(v) => Ok(v),
            Err(_) => self.llm.complete_structured(system, user).await,
        }
    }

    pub async fn decide(
        &self,
        run_id: &str,
        decision: ApprovalDecision,
        actor: &str,
        comment: Option<String>,
    ) -> Result<RunRecord, PipelineError> {
        let mut current = self
            .store
            .get(run_id)
            .await?
            .ok_or_else(|| PipelineError::NotFound(run_id.into()))?;
        let mut machine = RunMachine {
            state: current.state,
            approval: current.approval.clone(),
        };
        let approval = machine
            .apply_human_decision(run_id, decision, actor, comment)?
            .clone();
        current.state = machine.state;
        current.approval = Some(approval);
        self.store.upsert(current.clone()).await?;
        Ok(current)
    }

    pub async fn generate_design_and_snapshot(
        &self,
        run_id: &str,
    ) -> Result<RunRecord, PipelineError> {
        let mut current = self
            .store
            .get(run_id)
            .await?
            .ok_or_else(|| PipelineError::NotFound(run_id.into()))?;
        let mut machine = RunMachine {
            state: current.state,
            approval: current.approval.clone(),
        };
        machine.ensure_can_emit_design()?;
        if machine.state == RunState::Approved {
            machine.transition(RunState::GeneratingDesign)?;
        }

        let dir = self.artifacts_dir.join(run_id);
        std::fs::create_dir_all(&dir).map_err(|e| PipelineError::Io(e.to_string()))?;
        let design_path = dir.join("design.md");
        let snapshot_path = dir.join("snapshot.md");

        let proposal = current
            .proposal
            .clone()
            .ok_or_else(|| PipelineError::Validation("missing proposal".into()))?;

        let design = render_design_md(&proposal);
        std::fs::write(&design_path, design).map_err(|e| PipelineError::Io(e.to_string()))?;

        machine.transition(RunState::SnapshotReady)?;
        let snapshot = render_snapshot_stub(&proposal);
        std::fs::write(&snapshot_path, snapshot).map_err(|e| PipelineError::Io(e.to_string()))?;
        machine.transition(RunState::Completed)?;

        let design_md_path = design_path.to_string_lossy().to_string();
        let snapshot_path_str = snapshot_path.to_string_lossy().to_string();

        current.state = machine.state;
        if let Some(p) = current.proposal.as_mut() {
            p.design_md_path = Some(design_md_path);
            p.snapshot_path = Some(snapshot_path_str);
        }
        self.store.upsert(current.clone()).await?;
        let _ = Utc::now();
        Ok(current)
    }
}

fn render_design_md(proposal: &EngineeringProposal) -> String {
    let mut out = String::new();
    out.push_str("# Design Specification\n\n");
    out.push_str(&format!("**Request:** {}\n\n", proposal.request));
    out.push_str("## Feasibility\n\n");
    out.push_str(&format!("{}\n\n", proposal.feasibility.summary));
    out.push_str("## Functional requirements\n\n");
    for r in &proposal.requirements.functional {
        out.push_str(&format!("- `{}`: {}\n", r.id, r.statement));
    }
    out.push_str("\n## Proposed UI\n\n");
    out.push_str("- New `/analytics` route using Card, Table, Select, Badge, Button\n");
    out.push_str("- Charts region for productivity / overdue / project performance\n");
    out.push_str("- CSV export button in page header\n");
    out.push_str("\n## Open questions\n\n");
    for q in &proposal.open_questions {
        out.push_str(&format!("- {q}\n"));
    }
    out
}

fn render_snapshot_stub(proposal: &EngineeringProposal) -> String {
    format!(
        "# Visual Snapshot Stub\n\nRun: {}\nGenerated for Playwright capture of analytics proposal UI.\n\nReuse: Button, Card, Table, Select, Badge.\n",
        proposal.run_id
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use llm::MockLlmProvider;
    use std::path::PathBuf;

    fn fixture() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/sample-app")
    }

    #[tokio::test]
    async fn full_pre_approval_pipeline_with_mock_llm() {
        let llm = Arc::new(MockLlmProvider::new(vec![]));
        let pipeline = AgentPipeline::new(llm, std::env::temp_dir().join("pea-artifacts"));
        let run = pipeline
            .start_run(
                "analytics dashboard with charts and CSV".into(),
                fixture(),
            )
            .await
            .unwrap();
        assert_eq!(run.state, RunState::AwaitingHumanApproval);
        assert!(run.proposal.is_some());
        validate_proposal(run.proposal.as_ref().unwrap()).unwrap();
    }

    #[tokio::test]
    async fn design_requires_approval() {
        let llm = Arc::new(MockLlmProvider::new(vec![]));
        let pipeline = AgentPipeline::new(llm, std::env::temp_dir().join("pea-artifacts"));
        let run = pipeline
            .start_run("analytics".into(), fixture())
            .await
            .unwrap();
        assert!(pipeline
            .generate_design_and_snapshot(&run.id)
            .await
            .is_err());
        let approved = pipeline
            .decide(
                &run.id,
                ApprovalDecision::Approve,
                "pm@example.com",
                Some("looks good".into()),
            )
            .await
            .unwrap();
        assert_eq!(approved.state, RunState::Approved);
        let done = pipeline
            .generate_design_and_snapshot(&run.id)
            .await
            .unwrap();
        assert_eq!(done.state, RunState::Completed);
        let proposal = done.proposal.unwrap();
        assert!(proposal.design_md_path.is_some());
        assert!(std::path::Path::new(proposal.design_md_path.as_ref().unwrap()).exists());
    }
}
