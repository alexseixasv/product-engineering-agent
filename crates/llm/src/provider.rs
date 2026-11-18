use async_trait::async_trait;
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("provider error: {0}")]
    Provider(String),
    #[error("invalid structured output: {0}")]
    InvalidOutput(String),
    #[error("http error: {0}")]
    Http(String),
}

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn complete_structured(
        &self,
        system: &str,
        user: &str,
    ) -> Result<Value, LlmError>;
}
