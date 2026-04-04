use async_trait::async_trait;
use crate::providers::traits::{Provider, ModelInfo};

pub struct OpenAIProvider {
    api_key: Option<String>,
}

impl OpenAIProvider {
    pub fn new() -> Self {
        Self { api_key: None }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> &str {
        "openai"
    }
    
    fn get_api_key(&self) -> Option<String> {
        self.api_key.clone()
    }
    
    fn set_api_key(&mut self, key: String) {
        self.api_key = Some(key);
    }
    
    async fn list_models(&self) -> Vec<ModelInfo> {
        vec![]
    }
    
    async fn chat(&self, model: &str, system: &str, user: &str) -> anyhow::Result<String> {
        let key = self.api_key.as_ref().ok_or_else(|| anyhow::anyhow!("OpenAI API key not set"))?;
        
        let client = reqwest::Client::new();
        let request_body = serde_json::json!({
            "model": model,
            "messages": [
                {
                    "role": "system",
                    "content": system
                },
                {
                    "role": "user",
                    "content": user
                }
            ]
        });

        let response = client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("OpenAI API error: {}", error_text);
        }

        let resp_json: serde_json::Value = response.json().await?;
        if let Some(content) = resp_json["choices"][0]["message"]["content"].as_str() {
            Ok(content.to_string())
        } else {
            anyhow::bail!("Invalid response format from OpenAI")
        }
    }
}