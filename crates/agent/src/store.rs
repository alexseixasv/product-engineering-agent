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

#[derive(Clone, Default)]
pub struct InMemoryStore {
    inner: Arc<Mutex<HashMap<String, RunRecord>>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&self, record: RunRecord) {
        self.inner
            .lock()
            .expect("lock")
            .insert(record.id.clone(), record);
    }

    pub fn get(&self, id: &str) -> Option<RunRecord> {
        self.inner.lock().expect("lock").get(id).cloned()
    }

    pub fn update<F>(&self, id: &str, f: F) -> Option<RunRecord>
    where
        F: FnOnce(&mut RunRecord),
    {
        let mut guard = self.inner.lock().expect("lock");
        let rec = guard.get_mut(id)?;
        f(rec);
        Some(rec.clone())
    }

    pub fn list(&self) -> Vec<RunRecord> {
        self.inner.lock().expect("lock").values().cloned().collect()
    }
}
