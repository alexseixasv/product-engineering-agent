use serde::{Deserialize, Serialize};

use crate::{BacklogTicket, Evidence};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    Fact,
    Assumption,
    OpenQuestion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingCategory {
    Component,
    Api,
    Model,
    DesignSystem,
    Architecture,
    Gap,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub category: FindingCategory,
    pub summary: String,
    pub confidence: Confidence,
    #[serde(default)]
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebtImpact {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TechnicalDebtItem {
    pub title: String,
    pub impact: DebtImpact,
    pub recommendation: String,
    pub confidence: Confidence,
    #[serde(default)]
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Requirement {
    pub id: String,
    pub statement: String,
    pub acceptance_criteria: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequirementsBundle {
    pub functional: Vec<Requirement>,
    pub non_functional: Vec<Requirement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Feasibility {
    pub summary: String,
    pub approach_options: Vec<String>,
    pub risks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EngineeringProposal {
    pub run_id: String,
    pub request: String,
    pub findings: Vec<Finding>,
    pub feasibility: Feasibility,
    pub technical_debt: Vec<TechnicalDebtItem>,
    pub requirements: RequirementsBundle,
    pub backlog: Vec<BacklogTicket>,
    pub open_questions: Vec<String>,
    #[serde(default)]
    pub design_md_path: Option<String>,
    #[serde(default)]
    pub snapshot_path: Option<String>,
}
