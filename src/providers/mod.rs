pub mod gemini;
pub mod openai;
pub mod openrouter;
pub mod anthropic;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait LLMProvider: Send + Sync {
    fn name(&self) -> &'static str;
    fn supported_models(&self) -> Vec<String>;
    async fn chat(&self, model: &str, api_key: &str, prompt: &str) -> Result<String>;
}

pub trait ProviderModels {
    fn models(&self) -> Vec<String>;
}

pub fn get_provider(provider_name: &str) -> Option<Box<dyn LLMProvider>> {
    match provider_name.to_lowercase().as_str() {
        "openrouter" => Some(Box::new(openrouter::OpenRouterProvider::new())),
        "openai" => Some(Box::new(openai::OpenAIProvider::new())),
        "gemini" => Some(Box::new(gemini::GeminiProvider::new())),
        "anthropic" => Some(Box::new(anthropic::AnthropicProvider::new())),
        _ => None,
    }
}

pub fn get_all_providers() -> Vec<Box<dyn LLMProvider>> {
    vec![
        Box::new(openrouter::OpenRouterProvider::new()),
        Box::new(openai::OpenAIProvider::new()),
        Box::new(gemini::GeminiProvider::new()),
        Box::new(anthropic::AnthropicProvider::new()),
    ]
}

// Helper function to get models for a specific provider
pub fn get_models_for_provider(provider_name: &str) -> Vec<String> {
    match provider_name.to_lowercase().as_str() {
        "openrouter" => openrouter::OpenRouterProvider::new().models(),
        "openai" => openai::OpenAIProvider::new().models(),
        "gemini" => gemini::GeminiProvider::new().models(),
        "anthropic" => anthropic::AnthropicProvider::new().models(),
        _ => vec![],
    }
}