//! OpenAI provider integration module.
//!
//! Provides interactive terminal workflows to authenticate, validate,
//! and persist configuration credentials for the OpenAI API.

use colored::Colorize;
use dialoguer::{Input, Password};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::StatusCode;

use crate::config::provider_config::ProviderConfig;

/// Interactively authenticates and configures the OpenAI API provider.
///
/// Prompts the user via the terminal for an OpenAI API key and target model name,
/// validates access via OpenAI's `GET /v1/models/{model}` endpoint, and persists
/// the validated credentials to the local provider configuration.
///
/// If an API validation error or network failure occurs, descriptive error
/// messages are written to `stderr` and the function exits without modifying
/// the persistent configuration.
///
/// # Returns
///
/// - `Ok(())` if authentication succeeds and config is saved, or if handled API
///   validation errors were reported to `stderr`.
/// - `Err(Box<dyn std::error::Error>)` if an unrecoverable terminal I/O, header
///   formatting, or configuration serialization failure occurs.
///
/// # Errors
///
/// This function returns an error if:
/// - User terminal input collection fails or is interrupted via [`dialoguer`].
/// - The generated `Authorization` header value contains invalid ASCII or control characters.
/// - Reading or writing the local [`ProviderConfig`] fails on the filesystem.
pub fn connect_openai() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = Password::new()
        .with_prompt("Enter your OpenAI API key")
        .interact()?;

    let model: String = Input::new()
        .with_prompt("Enter the model name")
        .default("gpt-4o-mini".into())
        .interact_text()?;

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {api_key}"))?,
    );

    let client = Client::new();
    let response = client
        .get(format!("https://api.openai.com/v1/models/{model}"))
        .headers(headers)
        .send();

    match response {
        Ok(res) => {
            if let Err(err) = res.error_for_status_ref() {
                match err.status() {
                    Some(StatusCode::UNAUTHORIZED) => {
                        eprintln!("{}", "Error: Invalid OpenAI API key.".red());
                    }
                    Some(StatusCode::NOT_FOUND) => {
                        eprintln!("{}", format!("Error: Model '{model}' not found on OpenAI.").red());
                    }
                    Some(status) => {
                        eprintln!("{}", format!("Error: OpenAI request failed with status {status}.").red());
                    }
                    None => {
                        eprintln!("{}", "Error: Unexpected response from OpenAI API.".red());
                    }
                }
                return Ok(());
            }
        }
        Err(_) => {
            eprintln!(
                "{}",
                "Error: Failed to reach OpenAI servers. Please check your internet connection."
                    .red()
            );
            return Ok(());
        }
    }

    let mut config = ProviderConfig::load()?;
    config.set_api_key("openai", api_key);
    config.set_default("openai", &model);
    config.set_provider_model("openai", &model);
    config.save()?;

    println!("{}", "OpenAI connected successfully!".green().bold());

    Ok(())
}
