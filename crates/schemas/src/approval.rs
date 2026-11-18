use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecision {
    Approve,
    Reject,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub run_id: String,
    pub decision: ApprovalDecision,
    pub actor: String,
    pub decided_at: DateTime<Utc>,
    #[serde(default)]
    pub comment: Option<String>,
}

impl ApprovalRecord {
    pub fn validate(&self) -> Result<(), String> {
        if self.run_id.trim().is_empty() {
            return Err("approval.run_id must be non-empty".into());
        }
        if self.actor.trim().is_empty() {
            return Err("approval.actor must be non-empty".into());
        }
        Ok(())
    }
}
