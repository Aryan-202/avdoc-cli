//! Active provider discovery and validation helpers.
//!
//! Provides routines for querying configured provider credentials against the set
//! of supported LLM backends.

use crate::config::provider_config::ProviderConfig;

/// Filters and returns the list of supported providers that have non-empty API keys configured.
///
/// Iterates over the predefined list of known providers (`anthropic`, `deepseek`, `gemini`,
/// `groq`, `openai`, `openrouter`), checking the supplied [`ProviderConfig`] for presence
/// of a non-empty, non-whitespace API key.
///
/// # Returns
///
/// A vector of provider name strings that have valid configured credentials.
pub fn get_available_providers(config: &ProviderConfig) -> Vec<String> {
    const KNOWN_PROVIDERS: [&str; 6] = [
        "anthropic",
        "deepseek",
        "gemini",
        "groq",
        "openai",
        "openrouter",
    ];

    KNOWN_PROVIDERS
        .iter()
        .filter(|&&provider| {
            config
                .get_api_key(provider)
                .map(|key| !key.trim().is_empty())
                .unwrap_or(false)
        })
        .map(|&s| s.to_string())
        .collect()
}
