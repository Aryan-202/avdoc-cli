//! Anthropic provider integration module.
//!
//! Provides interactive CLI workflows to configure and authenticate
//! with the Anthropic Claude API.

use colored::Colorize;
use dialoguer::{Input, Password};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::StatusCode;

use crate::config::provider_config::ProviderConfig;

/// Interactively authenticates and configures the Anthropic API provider.
///
/// Prompts the user via the terminal for an Anthropic API key and target model name,
/// validates the credentials by pinging the Anthropic `/v1/models/{model}` endpoint,
/// and persists the configuration locally upon successful verification.
///
/// # Returns
///
/// - `Ok(())` if authentication succeeds and config is saved, or if handled error
///   messages were printed to `stderr` during validation.
/// - `Err(Box<dyn std::error::Error>)` if an unrecoverable I/O, prompt interaction,
///   or configuration serialization failure occurs.
///
/// # Errors
///
/// This function returns an error in the following scenarios:
/// - Terminal user input collection fails via `dialoguer`.
/// - The API key contains characters invalid for an HTTP header value.
/// - Loading or saving [`ProviderConfig`] fails on the local filesystem.
pub fn connect_anthropic() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = Password::new()
        .with_prompt("Enter your Anthropic API key")
        .interact()?;

    let model: String = Input::new()
        .with_prompt("Enter the model name")
        .default("claude-3-7-sonnet-20250219".into())
        .interact_text()?;

    let mut headers = HeaderMap::new();
    headers.insert(
        "x-api-key",
        HeaderValue::from_str(&api_key)?,
    );
    headers.insert(
        "anthropic-version",
        HeaderValue::from_static("2023-06-01"),
    );

    let client = Client::new();
    let response = client
        .get(format!("https://api.anthropic.com/v1/models/{model}"))
        .headers(headers)
        .send();

    match response {
        Ok(res) => {
            if let Err(err) = res.error_for_status_ref() {
                match err.status() {
                    Some(StatusCode::UNAUTHORIZED) | Some(StatusCode::FORBIDDEN) => {
                        eprintln!("{}", "Error: Invalid Anthropic API key.".red());
                    }
                    Some(StatusCode::NOT_FOUND) => {
                        eprintln!("{}", format!("Error: Model '{model}' not found on Anthropic.").red());
                    }
                    Some(status) => {
                        eprintln!("{}", format!("Error: Anthropic request failed with status {status}.").red());
                    }
                    None => {
                        eprintln!("{}", "Error: Unexpected response from Anthropic API.".red());
                    }
                }
                return Ok(());
            }
        }
        Err(_) => {
            eprintln!("{}", "Error: Failed to reach Anthropic servers. Please check your internet connection.".red());
            return Ok(());
        }
    }

    let mut config = ProviderConfig::load()?;
    config.set_api_key("anthropic", api_key);
    config.set_default("anthropic", &model);
    config.set_provider_model("anthropic", &model);
    config.save()?;

    println!("{}", "Anthropic connected successfully!".green().bold());
    Ok(())
}
