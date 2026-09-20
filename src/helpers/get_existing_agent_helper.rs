use crate::config::provider_config::ProviderConfig;

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
            // Check if this provider has an API key stored.
            // Adapt this line depending on your ProviderConfig API (e.g. config.get_api_key(provider))
            config
                .get_api_key(provider)
                .map(|key| !key.trim().is_empty())
                .unwrap_or(false)
        })
        .map(|&s| s.to_string())
        .collect()
}