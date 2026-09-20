//! Google Gemini provider integration module.
//!
//! Provides interactive terminal workflows to authenticate, validate,
//! and persist configuration credentials for Google Generative AI (Gemini).

use colored::Colorize;
use dialoguer::{Input, Password};
use reqwest::blocking::Client;
use reqwest::StatusCode;

use crate::config::provider_config::ProviderConfig;

/// Interactively authenticates and configures the Google Gemini API provider.
///
/// Prompts the user via the terminal for a Gemini API key and model name,
/// verifies the key and model accessibility against the Google Generative Language API,
/// and saves the resulting settings to the local configuration store.
///
/// If an API validation error or network failure occurs, user-friendly error
/// messages are printed to `stderr` and the function exits early without mutating
/// the local configuration.
///
/// # Returns
///
/// - `Ok(())` on successful authentication and configuration save, or when API-level
///   validation errors were reported to standard error.
/// - `Err(Box<dyn std::error::Error>)` if an unrecoverable terminal I/O or configuration
///   serialization error occurs.
///
/// # Errors
///
/// This function returns an error if:
/// - User terminal input collection fails or is interrupted via [`dialoguer`].
/// - Loading or persisting the [`ProviderConfig`] fails on the local filesystem.
pub fn connect_gemini() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = Password::new()
        .with_prompt("Enter your Gemini API key")
        .interact()?;

    let model: String = Input::new()
        .with_prompt("Enter the model name")
        .default("gemini-2.0-flash".into())
        .interact_text()?;

    let model_path = if model.starts_with("models/") {
        model.clone()
    } else {
        format!("models/{model}")
    };

    let client = Client::new();
    let response = client
        .get(format!(
            "https://generativelanguage.googleapis.com/v1beta/{model_path}"
        ))
        .header("x-goog-api-key", &api_key)
        .send();

    match response {
        Ok(res) => {
            if let Err(err) = res.error_for_status_ref() {
                match err.status() {
                    Some(StatusCode::BAD_REQUEST) | Some(StatusCode::UNAUTHORIZED) => {
                        eprintln!("{}", "Error: Invalid API key or unauthorized request.".red());
                    }
                    Some(StatusCode::NOT_FOUND) => {
                        eprintln!("{}", format!("Error: Model '{model}' not found.").red());
                    }
                    Some(status) => {
                        eprintln!("{}", format!("Error: Request failed with status {status}.").red());
                    }
                    None => {
                        eprintln!("{}", "Error: Unexpected response from Gemini API.".red());
                    }
                }
                return Ok(());
            }
        }
        Err(_) => {
            eprintln!(
                "{}",
                "Error: Failed to reach Gemini servers. Please check your internet connection."
                    .red()
            );
            return Ok(());
        }
    }

    let mut config = ProviderConfig::load()?;
    config.set_api_key("gemini", api_key);
    config.set_default("gemini", &model);
    config.set_provider_model("gemini", &model);
    config.save()?;

    println!("{}", "Gemini connected successfully!".green().bold());

    Ok(())
}
