use async_trait::async_trait;
use anyhow::{Result, bail};
use crate::providers::traits::{Provider, ModelInfo};
use reqwest::Client;
use serde_json::json;

pub struct GroqProvider {
    api_key: Option<String>,
}

impl GroqProvider {
    pub fn new() -> Self {
        Self { api_key: None }
    }
}

#[async_trait]
impl Provider for GroqProvider {
    fn name(&self) -> &str {
        "groq"
    }
    
    fn get_api_key(&self) -> Option<String> {
        self.api_key.clone()
    }
    
    fn set_api_key(&mut self, key: String) {
        self.api_key = Some(key);
    }
    
    async fn list_models(&self) -> Vec<ModelInfo> { vec![] }
    
    async fn chat(&self, model: &str, system: &str, user: &str) -> Result<String> {
        let key = self.api_key.as_ref().ok_or_else(|| anyhow::anyhow!("Groq API key not set"))?;
        
        let client = Client::new();
        let request_body = json!({
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

        let response = client.post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            bail!("Groq API error: {}", error_text);
        }

        let resp_json: serde_json::Value = response.json().await?;
        if let Some(content) = resp_json["choices"][0]["message"]["content"].as_str() {
            Ok(content.to_string())
        } else {
            bail!("Invalid response format from Groq")
        }
    }
}
