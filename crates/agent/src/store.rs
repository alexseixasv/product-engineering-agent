use async_trait::async_trait;
use schemas::{ApprovalRecord, EngineeringProposal, RunState};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct RunRecord {
    pub id: String,
    pub request: String,
    pub repo_path: String,
    pub state: RunState,
    pub proposal: Option<EngineeringProposal>,
    pub approval: Option<ApprovalRecord>,
    pub error: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("store: {0}")]
    Message(String),
}

#[async_trait]
pub trait RunStore: Send + Sync {
    async fn upsert(&self, record: RunRecord) -> Result<(), StoreError>;
    async fn get(&self, id: &str) -> Result<Option<RunRecord>, StoreError>;
    async fn list(&self) -> Result<Vec<RunRecord>, StoreError>;
}

#[derive(Clone, Default)]
pub struct InMemoryStore {
    inner: Arc<Mutex<HashMap<String, RunRecord>>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl RunStore for InMemoryStore {
    async fn upsert(&self, record: RunRecord) -> Result<(), StoreError> {
        self.inner
            .lock()
            .map_err(|e| StoreError::Message(e.to_string()))?
            .insert(record.id.clone(), record);
        Ok(())
    }

    async fn get(&self, id: &str) -> Result<Option<RunRecord>, StoreError> {
        Ok(self
            .inner
            .lock()
            .map_err(|e| StoreError::Message(e.to_string()))?
            .get(id)
            .cloned())
    }

    async fn list(&self) -> Result<Vec<RunRecord>, StoreError> {
        Ok(self
            .inner
            .lock()
            .map_err(|e| StoreError::Message(e.to_string()))?
            .values()
            .cloned()
            .collect())
    }
}
