use async_trait::async_trait;
use anyhow::{Result, bail};
use crate::providers::traits::{Provider, ModelInfo};
use reqwest::Client;
use serde_json::json;

pub struct GeminiProvider {
    api_key: Option<String>,
}

impl GeminiProvider {
    pub fn new() -> Self {
        Self { api_key: None }
    }
}

#[async_trait]
impl Provider for GeminiProvider {
    fn name(&self) -> &str {
        "gemini"
    }
    
    fn get_api_key(&self) -> Option<String> {
        self.api_key.clone()
    }
    
    fn set_api_key(&mut self, key: String) {
        self.api_key = Some(key);
    }
    
    async fn list_models(&self) -> Vec<ModelInfo> { vec![] }
    
    async fn chat(&self, model: &str, system: &str, user: &str) -> Result<String> {
        let key = self.api_key.as_ref().ok_or_else(|| anyhow::anyhow!("Gemini API key not set"))?;
        
        let client = Client::new();
        
        // Gemini API uses a different schema
        let request_body = json!({
            "system_instruction": {
                "parts": { "text": system }
            },
            "contents": [
                {
                    "role": "user",
                    "parts": [{ "text": user }]
                }
            ]
        });

        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", model, key);

        let response = client.post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            bail!("Gemini API error: {}", error_text);
        }

        let resp_json: serde_json::Value = response.json().await?;
        if let Some(content) = resp_json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
            Ok(content.to_string())
        } else {
            bail!("Invalid response format from Gemini")
        }
    }
}
