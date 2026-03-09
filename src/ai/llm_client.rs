use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

pub struct LLMClient {
    client: Client,
    api_key: String,
    model: String,
}

impl LLMClient {
    pub fn from_env() -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY")
            .or_else(|_| env::var("ANTHROPIC_API_KEY"))
            .or_else(|_| env::var("AVDOC_API_KEY"))
            .map_err(|_| anyhow!("No API key found. Please set OPENAI_API_KEY, ANTHROPIC_API_KEY, or AVDOC_API_KEY"))?;
        
        let model = env::var("AVDOC_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string());
        
        Ok(Self {
            client: Client::new(),
            api_key,
            model,
        })
    }
    
    pub async fn generate_documentation(&self, code: &str, language: &str) -> Result<String> {
        let prompt = format!(
            "Add comprehensive documentation to this {} code. Include function descriptions, parameter explanations, and return values. Return only the documented code:\n\n{}",
            language, code
        );
        
        self.call_llm(&prompt).await
    }
    
    pub async fn generate_diagram(&self, prompt: &str) -> Result<String> {
        self.call_llm(prompt).await
    }
    
    async fn call_llm(&self, prompt: &str) -> Result<String> {
        // Try OpenAI first
        if self.model.contains("gpt") {
            self.call_openai(prompt).await
        } else {
            // Fallback to generic API
            self.call_generic(prompt).await
        }
    }
    
    async fn call_openai(&self, prompt: &str) -> Result<String> {
        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You are a helpful documentation generator that produces clean, well-structured documentation.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            temperature: 0.3,
        };
        
        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error = response.text().await?;
            return Err(anyhow!("OpenAI API error: {}", error));
        }
        
        let result: OpenAIResponse = response.json().await?;
        
        result.choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow!("No response from OpenAI"))
    }
    
    async fn call_generic(&self, prompt: &str) -> Result<String> {
        // Generic API call - would need to be customized for different providers
        let response = self.client
            .post("https://api.anthropic.com/v1/complete")
            .header("X-API-Key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "prompt": prompt,
                "model": self.model,
                "max_tokens_to_sample": 1000,
            }))
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error = response.text().await?;
            return Err(anyhow!("LLM API error: {}", error));
        }
        
        let text = response.text().await?;
        Ok(text)
    }
}