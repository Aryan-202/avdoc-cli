use std::path::PathBuf;
use std::fs;
use std::io::Write;
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use owo_colors::OwoColorize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderData {
    pub model: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalConfig {
    #[serde(default)]
    pub providers: std::collections::HashMap<String, ProviderData>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self { 
            providers: std::collections::HashMap::new(),
        }
    }
}

pub fn get_global_config_path() -> PathBuf {
    PathBuf::from(".avdoc").join("global.json")
}

pub fn load_global_config() -> Result<GlobalConfig> {
    let config_path = get_global_config_path();
    
    if !config_path.exists() {
        return Ok(GlobalConfig::default());
    }
    
    let content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
    
    let config: GlobalConfig = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON config file: {}", config_path.display()))?;
    
    Ok(config)
}

pub fn save_global_config(config: &GlobalConfig) -> Result<()> {
    let config_path = get_global_config_path();
    
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let content = serde_json::to_string_pretty(config)
        .context("Failed to serialize config to JSON")?;
    
    let mut file = fs::File::create(&config_path)
        .with_context(|| format!("Failed to create config file: {}", config_path.display()))?;
    
    file.write_all(content.as_bytes())?;
    
    Ok(())
}

pub fn set_config(provider: &str, model: &str, api_key: &str) -> Result<()> {
    let mut config = load_global_config()?;
    
    config.providers.insert(
        provider.to_string(),
        ProviderData {
            model: model.to_string(),
            api_key: api_key.to_string(),
        }
    );
    
    println!("{}", format!("Added configuration for {}", provider).green());
    println!("{}", format!("Set model for {}: {}", provider, model).green());
    println!("{}", format!("Kept API key for {}: {}", provider, mask_api_key(api_key)).green());
    
    save_global_config(&config)?;
    
    Ok(())
}

pub fn get_config_value(key: &str) -> Result<Option<String>> {
    let config = load_global_config()?;
    
    if key.starts_with("model.") {
        let provider = &key[6..];
        Ok(config.providers.get(provider).map(|d| d.model.clone()))
    } else if key.starts_with("api_key.") {
        let provider = &key[8..];
        Ok(config.providers.get(provider).map(|d| d.api_key.clone()))
    } else {
        Ok(None)
    }
}

pub fn list_config() -> Result<()> {
    let config = load_global_config()?;
    
    println!("{}", "Current Configuration:".bold().underline());
    println!();
    
    println!("{}", "Providers:".bold());
    if config.providers.is_empty() {
        println!("  No providers configured");
    } else {
        for (provider, p_config) in &config.providers {
            println!(" {}", provider.yellow());
            println!("     Model: {}", p_config.model);
            println!("     API Key: {}", mask_api_key(&p_config.api_key));
        }
    }
    
    Ok(())
}

pub fn get_api_key(provider: &str) -> Result<Option<String>> {
    let config = load_global_config()?;
    Ok(config.providers.get(provider).map(|p| p.api_key.clone()))
}

fn mask_api_key(key: &str) -> String {
    if key.len() <= 8 {
        "****".to_string()
    } else {
        format!("{}...{}", &key[..4], &key[key.len()-4..])
    }
}