use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Mutex;

use crate::provider::{LlmError, LlmProvider};

pub struct MockLlmProvider {
    responses: Mutex<Vec<Value>>,
    fail_times: Mutex<u32>,
}

impl MockLlmProvider {
    pub fn new(responses: Vec<Value>) -> Self {
        Self {
            responses: Mutex::new(responses),
            fail_times: Mutex::new(0),
        }
    }

    pub fn with_failures(responses: Vec<Value>, fail_times: u32) -> Self {
        Self {
            responses: Mutex::new(responses),
            fail_times: Mutex::new(fail_times),
        }
    }

    pub fn default_stage_response() -> Value {
        json!({
            "notes": ["mock reasoning"],
            "items": []
        })
    }
}

#[async_trait]
impl LlmProvider for MockLlmProvider {
    async fn complete_structured(
        &self,
        _system: &str,
        _user: &str,
    ) -> Result<Value, LlmError> {
        {
            let mut fails = self.fail_times.lock().expect("lock");
            if *fails > 0 {
                *fails -= 1;
                return Err(LlmError::Provider("simulated failure".into()));
            }
        }
        let mut guard = self.responses.lock().expect("lock");
        if guard.is_empty() {
            return Ok(Self::default_stage_response());
        }
        Ok(guard.remove(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_returns_queued_json() {
        let mock = MockLlmProvider::new(vec![json!({"ok": true})]);
        let v = mock.complete_structured("s", "u").await.unwrap();
        assert_eq!(v["ok"], true);
    }

    #[tokio::test]
    async fn mock_can_fail_then_succeed() {
        let mock = MockLlmProvider::with_failures(vec![json!({"ok": true})], 1);
        assert!(mock.complete_structured("s", "u").await.is_err());
        assert!(mock.complete_structured("s", "u").await.is_ok());
    }
}
