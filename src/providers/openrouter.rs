use super::{LLMProvider, ProviderModels};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct OpenRouterProvider;

impl OpenRouterProvider {
    pub fn new() -> Self {
        Self
    }
}

impl ProviderModels for OpenRouterProvider {
    fn models(&self) -> Vec<String> {
        vec![
            // Free models
            "stepfun/step-3.5-flash:free".to_string(),
            "nvidia/nemotron-3-super-120b-a12b:free".to_string(),
            "arcee-ai/trinity-large-preview:free".to_string(),
            "z-ai/glm-4.5-air:free".to_string(),
            "google/gemini-2.0-flash-exp:free".to_string(),
            "meta-llama/llama-3.2-3b-instruct:free".to_string(),
            "microsoft/phi-3-mini-128k-instruct:free".to_string(),
            
            // Paid models
            "openai/gpt-4o".to_string(),
            "openai/gpt-4-turbo".to_string(),
            "anthropic/claude-3.5-sonnet".to_string(),
            "anthropic/claude-3-opus".to_string(),
            "google/gemini-1.5-pro".to_string(),
            "google/gemini-1.5-flash".to_string(),
            "mistralai/mistral-7b-instruct".to_string(),
            "meta-llama/llama-3-70b-instruct".to_string(),
            "deepseek/deepseek-chat".to_string(),
            "cohere/command-r-plus".to_string(),
        ]
    }
}

#[async_trait]
impl LLMProvider for OpenRouterProvider {
    fn name(&self) -> &'static str {
        "OpenRouter"
    }
    
    fn supported_models(&self) -> Vec<String> {
        self.models()
    }
    
    async fn chat(&self, model: &str, api_key: &str, prompt: &str) -> Result<String> {
        let client = Client::new();
        
        let response = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("HTTP-Referer", "https://github.com/Aryan-202/avdoc")
            .header("X-Title", "AVDoc")
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": model,
                "messages": [
                    {
                        "role": "user",
                        "content": prompt
                    }
                ],
                "temperature": 0.7,
                "max_tokens": 2048,
                "top_p": 0.95
            }))
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            
            // Try to parse error as JSON for better error messages
            if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(&error_text) {
                if let Some(error_msg) = error_json["error"]["message"].as_str() {
                    return Err(anyhow!("OpenRouter API error: {}", error_msg));
                }
            }
            
            return Err(anyhow!("OpenRouter API error ({}): {}", status, error_text));
        }
        
        let json: serde_json::Value = response.json().await?;
        
        // Check for errors in response body
        if let Some(error) = json.get("error") {
            let error_msg = error["message"].as_str().unwrap_or("Unknown error");
            return Err(anyhow!("OpenRouter API error: {}", error_msg));
        }
        
        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Unexpected response format from OpenRouter: {}", json))
    }
}