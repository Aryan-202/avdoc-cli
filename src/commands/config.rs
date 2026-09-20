//! Global configuration management and persistent storage routines.
//!
//! Handles serialization, deserialization, and retrieval of global provider configurations
//! stored in the `.avdoc/global.json` file.

use std::path::PathBuf;
use std::fs;
use std::io::Write;
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use owo_colors::OwoColorize;

/// Configuration payload for an individual LLM provider.
///
/// Contains the chosen model identifier and corresponding API credentials.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderData {
    /// Model identifier or slug configured for the provider.
    pub model: String,
    /// Authentication secret or API key for the provider.
    pub api_key: String,
}

/// Root global configuration schema.
///
/// Encapsulates all persisted provider profiles indexed by provider name.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(Default)]
pub struct GlobalConfig {
    /// Mapping of provider identifiers to their respective [`ProviderData`].
    #[serde(default)]
    pub providers: std::collections::HashMap<String, ProviderData>,
}

/// Returns the relative filesystem path to the global configuration file.
///
/// Path resolves to `.avdoc/global.json` relative to the current working directory.
pub fn get_global_config_path() -> PathBuf {
    PathBuf::from(".avdoc").join("global.json")
}

/// Loads and deserializes the global configuration file from disk.
///
/// If the configuration file does not exist, a default empty [`GlobalConfig`] instance is returned.
///
/// # Returns
///
/// Returns `Ok(GlobalConfig)` populated from disk or default.
///
/// # Errors
///
/// Returns an error if the file exists but cannot be read or contains invalid JSON syntax.
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

/// Serializes and writes the provided global configuration to disk.
///
/// Automatically creates parent directories if they do not exist prior to writing.
///
/// # Errors
///
/// Returns an error if directory creation, JSON serialization, or file writing fails.
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

/// Sets or updates the configuration for a specific provider and saves it.
///
/// Modifies the in-memory provider map with the provided `model` and `api_key`, prints
/// status feedback to standard output, and writes changes to disk.
///
/// # Errors
///
/// Returns an error if loading the existing configuration or saving the updated configuration fails.
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

/// Retrieves a configuration attribute by key prefix notation.
///
/// Supports query keys in the format `model.<provider>` to retrieve the model name,
/// or `api_key.<provider>` to retrieve the raw API key.
///
/// # Returns
///
/// Returns `Ok(Some(String))` if the key matches a known prefix and configured provider,
/// `Ok(None)` if the key pattern does not match or the provider is missing, or an error.
///
/// # Errors
///
/// Returns an error if loading the global configuration encounters an I/O or parse failure.
pub fn get_config_value(key: &str) -> Result<Option<String>> {
    let config = load_global_config()?;
    
    if let Some(provider) = key.strip_prefix("model.") {
        Ok(config.providers.get(provider).map(|d| d.model.clone()))
    } else if let Some(provider) = key.strip_prefix("api_key.") {
        Ok(config.providers.get(provider).map(|d| d.api_key.clone()))
    } else {
        Ok(None)
    }
}

/// Prints a formatted summary table of all configured providers to standard output.
///
/// Displays each provider name, model identifier, and masked API key.
///
/// # Errors
///
/// Returns an error if loading the global configuration fails.
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

/// Retrieves the raw API key associated with a specific provider.
///
/// # Returns
///
/// Returns `Ok(Some(String))` containing the API key if configured, or `Ok(None)` if not found.
///
/// # Errors
///
/// Returns an error if loading the global configuration fails.
pub fn get_api_key(provider: &str) -> Result<Option<String>> {
    let config = load_global_config()?;
    Ok(config.providers.get(provider).map(|p| p.api_key.clone()))
}

/// Masks an API key string for secure terminal display.
///
/// If the key length is 8 characters or fewer, masks the entire string as `"****"`.
/// Otherwise, exposes the first 4 and last 4 characters separated by ellipses (`"..."`).
fn mask_api_key(key: &str) -> String {
    if key.len() <= 8 {
        "****".to_string()
    } else {
        format!("{}...{}", &key[..4], &key[key.len()-4..])
    }
}