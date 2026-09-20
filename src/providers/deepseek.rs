//! DeepSeek provider integration module.
//!
//! Handles terminal interactions, credential authentication, and persistent
//! configuration storage for the DeepSeek API.

use dialoguer::{Input, Password};

use crate::config::provider_config::ProviderConfig;

/// Interactively authenticates and configures the DeepSeek API provider.
///
/// Prompts the user via the command line for their DeepSeek API key and model choice,
/// verifies the key against the `https://api.deepseek.com/models` endpoint inside a temporary
/// Tokio runtime, and persists the credentials to the local provider configuration.
///
/// # Returns
///
/// Returns `Ok(())` upon successful authentication and configuration persistence.
///
/// # Errors
///
/// This function returns an error if:
/// - User terminal input via [`dialoguer`] fails or is interrupted.
/// - The Tokio runtime fails to initialize.
/// - The authentication request fails (e.g., network error, invalid API key, or non-2xx HTTP status).
/// - Loading or saving the local [`ProviderConfig`] encounters an I/O or serialization error.
pub fn connect_deepseek() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = Password::new()
        .with_prompt("Enter your DeepSeek API key")
        .interact()?;

    let model: String = Input::new()
        .with_prompt("Enter the model name")
        .default("deepseek-chat".into()) // Pressing Enter keeps this default
        .interact_text()?;

    tokio::runtime::Runtime::new()?.block_on(async {
        reqwest::Client::new()
            .get("https://api.deepseek.com/models")
            .bearer_auth(&api_key)
            .send()
            .await?
            .error_for_status()?;

        println!("Successfully authenticated! Selected model: {}", model);
        Ok::<_, reqwest::Error>(())
    })?;

    let mut config = ProviderConfig::load()?;
    config.set_api_key("deepseek", api_key);
    config.set_default("deepseek", "deepseek-chat");
    config.set_provider_model("deepseek", "deepseek-chat");
    config.save()?;
    println!("Connected to DeepSeek.");

    Ok(())
}
