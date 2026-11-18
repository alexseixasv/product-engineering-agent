use async_trait::async_trait;
use serde_json::{json, Value};

use crate::provider::{LlmError, LlmProvider};

/// OpenAI-compatible chat completions provider (HTTP).
pub struct HttpOpenAiCompatibleProvider {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    client: reqwest::Client,
}

impl HttpOpenAiCompatibleProvider {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: api_key.into(),
            model: model.into(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for HttpOpenAiCompatibleProvider {
    async fn complete_structured(
        &self,
        system: &str,
        user: &str,
    ) -> Result<Value, LlmError> {
        let url = format!(
            "{}/chat/completions",
            self.base_url.trim_end_matches('/')
        );
        let body = json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": user}
            ],
            "response_format": {"type": "json_object"}
        });
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Http(e.to_string()))?;
        if !response.status().is_success() {
            return Err(LlmError::Http(format!(
                "status {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }
        let payload: Value = response
            .json()
            .await
            .map_err(|e| LlmError::Http(e.to_string()))?;
        let content = payload["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| LlmError::InvalidOutput("missing content".into()))?;
        serde_json::from_str(content).map_err(|e| LlmError::InvalidOutput(e.to_string()))
    }
}
