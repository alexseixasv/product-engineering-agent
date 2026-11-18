use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TicketType {
    Epic,
    Story,
    Task,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BacklogTicket {
    pub key: String,
    #[serde(rename = "type")]
    pub ticket_type: TicketType,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub acceptance_criteria: Vec<String>,
    pub requirement_ids: Vec<String>,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub definition_of_done: Vec<String>,
}

impl BacklogTicket {
    pub fn validate(&self) -> Result<(), String> {
        if self.key.trim().is_empty() || self.title.trim().is_empty() {
            return Err("backlog ticket key/title required".into());
        }
        if self.requirement_ids.is_empty() {
            return Err("backlog ticket must trace to requirement_ids".into());
        }
        Ok(())
    }
}
