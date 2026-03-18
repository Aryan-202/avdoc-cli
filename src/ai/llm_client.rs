use anyhow::{Result, anyhow};
use crate::config::provider_config::Config;
use crate::providers;

pub struct LLMClient {
    pub provider_name: String,
    pub model: String,
    pub api_key: String,
}

impl LLMClient {
    pub fn from_env() -> Result<Self> {
        let config = Config::load();
        
        // Use default provider if available
        if let Some(default_provider) = &config.default_provider {
            if let Some(provider_config) = config.providers.get(default_provider) {
                return Ok(Self {
                    provider_name: provider_config.provider.clone(),
                    model: provider_config.model.clone(),
                    api_key: provider_config.api_key.clone(),
                });
            }
        }
        
        Err(anyhow!("No LLM provider configured. Run `avdoc add llm` first."))
    }
    
    pub async fn chat_generic(&self, prompt: &str) -> Result<String> {
        let provider_name = &self.provider_name;
        let provider = providers::get_provider(provider_name)
            .ok_or_else(|| anyhow!("Provider '{}' not supported or error loading.", provider_name))?;
            
        provider.chat(&self.model, &self.api_key, prompt).await
    }
    
    pub async fn generate_documentation(&self, code: &str, language: &str) -> Result<String> {
        let prompt = format!(
            "Add detailed documentation for this {} code. Include function comments, parameters, and logical explanations. Provide only the updated code:\n\n{}",
            language, code
        );
        
        self.chat_generic(&prompt).await
    }
}