use async_trait::async_trait;
use anyhow::Result;
use crate::providers::traits::{Provider, ModelInfo};

pub struct OpenAIProvider {
    api_key: Option<String>,
}

impl OpenAIProvider {
    pub fn new() -> Self {
        Self { api_key: None }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> &str {
        "openai"
    }
    
    fn get_api_key(&self) -> Option<String> {
        self.api_key.clone()
    }
    
    fn set_api_key(&mut self, key: String) {
        self.api_key = Some(key);
    }
    
    async fn list_models(&self) -> Vec<ModelInfo> {
        vec![]
    }
    
    async fn chat(&self, _model: &str, _system: &str, _user: &str) -> Result<String> {
        
        todo!()
    }
}