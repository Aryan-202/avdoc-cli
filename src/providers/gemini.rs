use colored::Colorize;
use dialoguer::{Input, Password};
use reqwest::blocking::Client;
use reqwest::StatusCode;

use crate::config::provider_config::ProviderConfig;

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
            eprintln!("{}", "Error: Failed to reach Gemini servers. Please check your internet connection.".red());
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