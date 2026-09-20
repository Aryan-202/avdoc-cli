//! Provider configuration data model and filesystem persistence.
//!
//! Manages API keys, active default providers, and default model associations
//! saved to `~/.avdoc/provider.json`.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use anyhow::Result;
use std::path::PathBuf;
use std::fs;

/// Data model representing user-configured providers, keys, and model mappings.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderConfig {
    /// Mapping of provider identifiers to their secret API keys.
    pub api_keys: HashMap<String, String>,  
    /// Default active provider identifier, if configured.
    pub default_provider: Option<String>,
    /// Default model name for the default provider, if configured.
    pub default_model: Option<String>,
    /// Mapping of provider identifiers to their default model names.
    pub provider_models: HashMap<String, String>, 
}

impl ProviderConfig {
    /// Resolves the filesystem path to the user's `provider.json` configuration file.
    ///
    /// Locates the file at `$HOME/.avdoc/provider.json`, falling back to `./.avdoc/provider.json`
    /// if the user's home directory cannot be resolved.
    pub fn get_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".avdoc");
        path.push("provider.json");
        path
    }

    /// Loads and deserializes [`ProviderConfig`] from disk.
    ///
    /// If the configuration file does not exist, returns [`ProviderConfig::default`].
    ///
    /// # Returns
    ///
    /// Returns `Ok(ProviderConfig)` on successful read and parse.
    ///
    /// # Errors
    ///
    /// Returns an error if reading the file fails due to I/O permissions.
    pub fn load() -> Result<Self> {
        let path = Self::get_path();
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&content).unwrap_or_default())
        } else {
            Ok(Self::default())
        }
    }

    /// Serializes and writes the configuration to disk.
    ///
    /// Ensures that parent directories exist before creating or overwriting the file.
    ///
    /// # Errors
    ///
    /// Returns an error if directory creation, JSON serialization, or file write operations fail.
    pub fn save(&self) -> Result<()> {
        let path = Self::get_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    /// Registers or updates the API key for the specified provider.
    pub fn set_api_key(&mut self, provider: &str, key: String) {
        self.api_keys.insert(provider.to_string(), key);
    }
    
    /// Retrieves a reference to the stored API key for the specified provider.
    ///
    /// # Returns
    ///
    /// Returns `Some(&String)` containing the key if configured, or `None` otherwise.
    pub fn get_api_key(&self, provider: &str) -> Option<&String> {
        self.api_keys.get(provider)
    }
    
    /// Sets the active default provider and its associated model.
    pub fn set_default(&mut self, provider: &str, model: &str) {
        self.default_provider = Some(provider.to_string());
        self.default_model = Some(model.to_string());
    }
    
    /// Records the default model name for a specific provider.
    pub fn set_provider_model(&mut self, provider: &str, model: &str) {
        self.provider_models.insert(provider.to_string(), model.to_string());
    }
    
    /// Resolves the currently active provider and its designated model identifier.
    ///
    /// Checks `default_provider` and attempts to resolve its model via `default_model`
    /// or `provider_models`.
    ///
    /// # Returns
    ///
    /// Returns `Some((provider, model))` if both are resolved, or `None` if no default is set.
    pub fn get_current_provider_model(&self) -> Option<(String, String)> {
        let provider = self.default_provider.as_ref()?;
        let model = self.default_model.as_ref()
            .or_else(|| self.provider_models.get(provider))
            .cloned()?;
        Some((provider.clone(), model))
    }
}