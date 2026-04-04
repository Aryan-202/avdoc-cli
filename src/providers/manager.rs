use std::collections::HashMap;
use anyhow::Result;
use crate::providers::traits::{Provider, ModelInfo};
use crate::providers::openai::OpenAIProvider;
use crate::providers::groq::GroqProvider;
use crate::providers::deepseek::DeepseekProvider;
use crate::providers::gemini::GeminiProvider;
use crate::providers::openrouter::OpenRouterProvider;

pub struct ProviderManager {
    providers: HashMap<String, Box<dyn Provider>>,
}

impl ProviderManager {
    pub fn new() -> Self {
        let mut manager = Self {
            providers: HashMap::new(),
        };
        
        // Register all providers
        manager.register("openai", Box::new(OpenAIProvider::new()));
        manager.register("groq", Box::new(GroqProvider::new()));
        manager.register("deepseek", Box::new(DeepseekProvider::new()));
        manager.register("gemini", Box::new(GeminiProvider::new()));
        manager.register("openrouter", Box::new(OpenRouterProvider::new()));
        
        manager
    }
    
    pub fn register(&mut self, name: &str, provider: Box<dyn Provider>) {
        self.providers.insert(name.to_string(), provider);
    }
    
    pub fn list_providers(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }
    
    pub async fn list_models(&self, provider: &str) -> Result<Vec<ModelInfo>> {
        let provider = self.providers.get(provider)
            .ok_or_else(|| anyhow::anyhow!("Provider '{}' not found", provider))?;
        
        Ok(provider.list_models().await)
    }
    
    pub async fn list_all_models(&self) -> HashMap<String, Vec<ModelInfo>> {
        let mut all = HashMap::new();
        for (name, provider) in &self.providers {
            let models = provider.list_models().await;
            all.insert(name.clone(), models);
        }
        all
    }
    
    pub fn get_provider(&self, name: &str) -> Option<&Box<dyn Provider>> {
        self.providers.get(name)
    }
    
    pub fn get_provider_mut(&mut self, name: &str) -> Option<&mut Box<dyn Provider>> {
        self.providers.get_mut(name)
    }
}