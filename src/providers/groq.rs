use colored::Colorize;
use dialoguer::{Input, Password};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::StatusCode;

use crate::config::provider_config::ProviderConfig;

pub fn connect_groq() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = Password::new()
        .with_prompt("Enter your Groq API key")
        .interact()?;

    let model: String = Input::new()
        .with_prompt("Enter the model name")
        .default("llama-3.3-70b-versatile".into())
        .interact_text()?;

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {api_key}"))?,
    );

    let client = Client::new();
    let response = client
        .get(format!("https://api.groq.com/openai/v1/models/{model}"))
        .headers(headers)
        .send();

    match response {
        Ok(res) => {
            if let Err(err) = res.error_for_status_ref() {
                match err.status() {
                    Some(StatusCode::UNAUTHORIZED) => {
                        eprintln!("{}", "Error: Invalid Groq API key.".red());
                    }
                    Some(StatusCode::NOT_FOUND) => {
                        eprintln!("{}", format!("Error: Model '{model}' not found on Groq.").red());
                    }
                    Some(status) => {
                        eprintln!("{}", format!("Error: Groq request failed with status {status}.").red());
                    }
                    None => {
                        eprintln!("{}", "Error: Unexpected response from Groq API.".red());
                    }
                }
                return Ok(());
            }
        }
        Err(_) => {
            eprintln!("{}", "Error: Failed to reach Groq servers. Please check your internet connection.".red());
            return Ok(());
        }
    }

    let mut config = ProviderConfig::load()?;
    config.set_api_key("groq", api_key);
    config.set_default("groq", &model);
    config.set_provider_model("groq", &model);
    config.save()?;

    println!("{}", "Groq connected successfully!".green().bold());

    Ok(())
}