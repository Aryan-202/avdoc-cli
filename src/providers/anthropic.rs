use super::{LLMProvider, ProviderModels};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct AnthropicProvider;

impl AnthropicProvider {
    pub fn new() -> Self {
        Self
    }
}

impl ProviderModels for AnthropicProvider {
    fn models(&self) -> Vec<String> {
        vec![
            "claude-3-5-sonnet-20241022".to_string(),
            "claude-3-5-haiku-20241022".to_string(),
            "claude-3-opus-20240229".to_string(),
            "claude-3-sonnet-20240229".to_string(),
            "claude-3-haiku-20240307".to_string(),
            "claude-2.1".to_string(),
            "claude-2.0".to_string(),
            "claude-instant-1.2".to_string(),
        ]
    }
}

#[async_trait]
impl LLMProvider for AnthropicProvider {
    fn name(&self) -> &'static str {
        "Anthropic"
    }
    
    fn supported_models(&self) -> Vec<String> {
        self.models()
    }
    
    async fn chat(&self, model: &str, api_key: &str, prompt: &str) -> Result<String> {
        let client = Client::new();
        
        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&json!({
                "model": model,
                "max_tokens": 2048,
                "temperature": 0.7,
                "system": "You are a helpful assistant.",
                "messages": [
                    {
                        "role": "user",
                        "content": prompt
                    }
                ]
            }))
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error = response.text().await?;
            return Err(anyhow!("Anthropic API error: {}", error));
        }
        
        let json: serde_json::Value = response.json().await?;
        
        // Anthropic returns content as an array of content blocks
        if let Some(content) = json["content"].as_array() {
            if let Some(first_block) = content.first() {
                if let Some(text) = first_block["text"].as_str() {
                    return Ok(text.to_string());
                }
            }
        }
        
        Err(anyhow!("Unexpected response format from Anthropic: {:?}", json))
    }
}