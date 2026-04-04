use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub context_length: Option<usize>,
    pub supports_json: bool,
    pub supports_streaming: bool,
    pub is_free: bool,
}

#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    fn get_api_key(&self) -> Option<String>;
    fn set_api_key(&mut self, key: String);
    
    async fn list_models(&self) -> Vec<ModelInfo>;
    async fn chat(&self, model: &str, system: &str, user: &str) -> Result<String, anyhow::Error>;
}

#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub provider: String,
    pub model: String,
    pub api_key: Option<String>,
}