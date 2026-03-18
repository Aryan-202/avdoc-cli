use super::{LLMProvider, ProviderModels};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct GeminiProvider;

impl GeminiProvider {
    pub fn new() -> Self {
        Self
    }
}

impl ProviderModels for GeminiProvider {
    fn models(&self) -> Vec<String> {
        vec![
            "gemini-2.0-flash-exp".to_string(),
            "gemini-2.0-flash".to_string(),
            "gemini-1.5-pro".to_string(),
            "gemini-1.5-flash".to_string(),
            "gemini-1.5-flash-8b".to_string(),
            "gemini-1.0-pro".to_string(),
        ]
    }
}

#[async_trait]
impl LLMProvider for GeminiProvider {
    fn name(&self) -> &'static str {
        "Gemini"
    }
    
    fn supported_models(&self) -> Vec<String> {
        self.models()
    }
    
    async fn chat(&self, model: &str, api_key: &str, prompt: &str) -> Result<String> {
        let client = Client::new();
        
        // Gemini API endpoint format
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, api_key
        );
        
        let response = client
            .post(&url)
            .json(&json!({
                "contents": [{
                    "parts": [{
                        "text": prompt
                    }]
                }],
                "generationConfig": {
                    "temperature": 0.7,
                    "maxOutputTokens": 2048,
                    "topP": 0.95,
                    "topK": 40
                },
                "safetySettings": [
                    {
                        "category": "HARM_CATEGORY_HARASSMENT",
                        "threshold": "BLOCK_MEDIUM_AND_ABOVE"
                    },
                    {
                        "category": "HARM_CATEGORY_HATE_SPEECH",
                        "threshold": "BLOCK_MEDIUM_AND_ABOVE"
                    },
                    {
                        "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT",
                        "threshold": "BLOCK_MEDIUM_AND_ABOVE"
                    },
                    {
                        "category": "HARM_CATEGORY_DANGEROUS_CONTENT",
                        "threshold": "BLOCK_MEDIUM_AND_ABOVE"
                    }
                ]
            }))
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error = response.text().await?;
            return Err(anyhow!("Gemini API error: {}", error));
        }
        
        let json: serde_json::Value = response.json().await?;
        
        // Extract text from response
        json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Unexpected response format from Gemini: {:?}", json))
    }
}