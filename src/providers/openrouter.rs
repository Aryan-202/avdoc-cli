use colored::Colorize;
use dialoguer::{Input, Password};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::StatusCode;

use crate::config::provider_config::ProviderConfig;

pub fn connect_openrouter() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = Password::new()
        .with_prompt("Enter your OpenRouter API key")
        .interact()?;

    let model: String = Input::new()
        .with_prompt("Enter the model slug (e.g. meta-llama/llama-3.3-70b-instruct)")
        .default("meta-llama/llama-3.3-70b-instruct".into())
        .interact_text()?;

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {api_key}"))?,
    );
    headers.insert(
        "HTTP-Referer",
        HeaderValue::from_static("https://localhost"),
    );
    headers.insert(
        "X-Title",
        HeaderValue::from_static("CLI-Client"),
    );

    // https://openrouter.ai/api/v1/auth/key directly validates key validity and limits
    let client = Client::new();
    let response = client
        .get("https://openrouter.ai/api/v1/auth/key")
        .headers(headers)
        .send();

    match response {
        Ok(res) => {
            if let Err(err) = res.error_for_status_ref() {
                match err.status() {
                    Some(StatusCode::UNAUTHORIZED) => {
                        eprintln!("{}", "Error: Invalid OpenRouter API key.".red());
                    }
                    Some(status) => {
                        eprintln!("{}", format!("Error: OpenRouter request failed with status {status}.").red());
                    }
                    None => {
                        eprintln!("{}", "Error: Unexpected response from OpenRouter API.".red());
                    }
                }
                return Ok(());
            }
        }
        Err(_) => {
            eprintln!("{}", "Error: Failed to reach OpenRouter servers. Please check your internet connection.".red());
            return Ok(());
        }
    }

    let mut config = ProviderConfig::load()?;
    config.set_api_key("openrouter", api_key);
    config.set_default("openrouter", &model);
    config.set_provider_model("openrouter", &model);
    config.save()?;

    println!("{}", "OpenRouter connected successfully!".green().bold());

    Ok(())
}