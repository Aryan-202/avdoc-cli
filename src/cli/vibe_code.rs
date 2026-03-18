use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input};
use tokio::io::{self, AsyncWriteExt};

use crate::config::provider_config::Config;
use crate::providers;

pub async fn run() -> Result<()> {
    println!("{}", "✨ Vibe Code: AI Assisted Coding ✨".bright_magenta().bold());
    println!("{}", "Integrating with your project's codebase...".dimmed());

    // Load configuration
    let config = Config::load();
    let provider_name = config.default_provider.clone().ok_or_else(|| {
        anyhow::anyhow!("No default LLM provider configured. Run `avdoc add llm` first.")
    })?;

    let provider_config = config.providers.get(&provider_name).ok_or_else(|| {
        anyhow::anyhow!("Configuration for default provider '{}' not found.", provider_name)
    })?;

    let provider = providers::get_provider(&provider_config.provider).ok_or_else(|| {
        anyhow::anyhow!("Provider '{}' not supported or error loading.", provider_config.provider)
    })?;

    println!("\nConnected to {} ({})", provider_config.provider.cyan().bold(), provider_config.model.yellow().bold());
    println!("{}", "I'm ready to help with your code. Ask me anything about this project!".green());
    println!("{}", "Type 'exit' to end session.\n".dimmed());

    let mut stdout = io::stdout();

    loop {
        stdout.write_all(format!("{} ", "Vibe Check:".bright_magenta().bold()).as_bytes()).await?;
        stdout.flush().await?;

        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("")
            .interact_text()?;

        if input.trim().to_lowercase() == "exit" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        println!("{}", "✨ AI is vibing with your code...".italic().dimmed());

        // Call the provider
        match provider.chat(&provider_config.model, &provider_config.api_key, &input).await {
            Ok(response) => {
                println!("\n{} {}\n", "AI:".bold().magenta(), response);
            }
            Err(e) => {
                println!("{} {}\n", "Error:".red().bold(), e);
            }
        }
    }

    Ok(())
}
