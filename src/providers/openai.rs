use super::{LLMProvider, ProviderModels};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct OpenAIProvider;

impl OpenAIProvider {
    pub fn new() -> Self {
        Self
    }
}

impl ProviderModels for OpenAIProvider {
    fn models(&self) -> Vec<String> {
        vec![
            "gpt-4o".to_string(),
            "gpt-4o-mini".to_string(),
            "gpt-4-turbo".to_string(),
            "gpt-4".to_string(),
            "gpt-3.5-turbo".to_string(),
            "o1-preview".to_string(),
            "o1-mini".to_string(),
        ]
    }
}

#[async_trait]
impl LLMProvider for OpenAIProvider {
    fn name(&self) -> &'static str {
        "OpenAI"
    }
    
    fn supported_models(&self) -> Vec<String> {
        self.models()
    }
    
    async fn chat(&self, model: &str, api_key: &str, prompt: &str) -> Result<String> {
        let client = Client::new();
        
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": model,
                "messages": [
                    {
                        "role": "system",
                        "content": "You are a helpful assistant."
                    },
                    {
                        "role": "user",
                        "content": prompt
                    }
                ],
                "temperature": 0.7,
                "max_tokens": 2048,
                "top_p": 0.95,
                "frequency_penalty": 0,
                "presence_penalty": 0
            }))
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error = response.text().await?;
            return Err(anyhow!("OpenAI API error: {}", error));
        }
        
        let json: serde_json::Value = response.json().await?;
        
        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Unexpected response format from OpenAI: {:?}", json))
    }
}