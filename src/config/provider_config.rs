use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderConfig {
    pub provider: String,
    pub model: String,
    pub api_key: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub providers: HashMap<String, ProviderConfig>,
    pub default_provider: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(config_path) {
                if let Ok(config) = toml::from_str(&content) {
                    return config;
                }
            }
        }
        Self::default()
    }
    
    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = Self::get_config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }
    
    pub fn add_provider(&mut self, name: String, config: ProviderConfig) {
        self.providers.insert(name.clone(), config);
        if self.default_provider.is_none() {
            self.default_provider = Some(name);
        }
    }
    
    fn get_config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("avdoc")
            .join("config.toml")
    }
}