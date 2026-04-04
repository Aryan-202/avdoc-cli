use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use anyhow::Result;
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderConfig {
    pub api_keys: HashMap<String, String>,  
    pub default_provider: Option<String>,
    pub default_model: Option<String>,
    pub provider_models: HashMap<String, String>, 
}

impl ProviderConfig {
    pub fn get_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".avdoc");
        path.push("provider.json");
        path
    }

    pub fn load() -> Result<Self> {
        let path = Self::get_path();
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&content).unwrap_or_default())
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    pub fn set_api_key(&mut self, provider: &str, key: String) {
        self.api_keys.insert(provider.to_string(), key);
    }
    
    pub fn get_api_key(&self, provider: &str) -> Option<&String> {
        self.api_keys.get(provider)
    }
    
    pub fn set_default(&mut self, provider: &str, model: &str) {
        self.default_provider = Some(provider.to_string());
        self.default_model = Some(model.to_string());
    }
    
    pub fn set_provider_model(&mut self, provider: &str, model: &str) {
        self.provider_models.insert(provider.to_string(), model.to_string());
    }
    
    pub fn get_current_provider_model(&self) -> Option<(String, String)> {
        let provider = self.default_provider.as_ref()?;
        let model = self.default_model.as_ref()
            .or_else(|| self.provider_models.get(provider))
            .cloned()?;
        Some((provider.clone(), model))
    }
}