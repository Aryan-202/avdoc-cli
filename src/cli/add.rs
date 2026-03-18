use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select, Password};
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;

use crate::config::provider_config::{Config, ProviderConfig};
use crate::providers::{self, LLMProvider};

pub async fn run_llm() -> Result<()> {
    // Get all available providers
    let providers_list = providers::get_all_providers();
    let provider_names: Vec<String> = providers_list
        .iter()
        .map(|p| p.name().to_string())
        .collect();
    
    // Select provider
    let provider_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an LLM provider")
        .items(&provider_names)
        .default(0)
        .interact()?;
    
    let selected_provider = &providers_list[provider_selection];
    let provider_name = selected_provider.name();
    
    // Get models for selected provider
    let models = selected_provider.supported_models();
    
    // Select model
    let model_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Select a model for {}", provider_name))
        .items(&models)
        .default(0)
        .interact()?;
    
    let selected_model = models[model_selection].clone();
    
    // Get API key
    let api_key: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Enter API key for {}", provider_name))
        .interact()?;
    
    // Save configuration
    let mut config = Config::load();
    config.add_provider(
        provider_name.to_lowercase(),
        ProviderConfig {
            provider: provider_name.to_string(),
            model: selected_model.clone(),
            api_key: api_key.clone(),
            base_url: None,
        },
    );
    config.save()?;
    
    println!("\n{} {} using model {}.", 
        "Configuration saved! Connected to".green(), 
        provider_name.bold().cyan(), 
        selected_model.bold().yellow()
    );
    println!("{}", "Type 'exit' to quit, or just start chatting!\n".dimmed());
    
    // Start chat session
    start_chat_session(selected_provider.as_ref(), &selected_model, &api_key).await?;
    
    Ok(())
}

async fn start_chat_session(
    provider: &dyn LLMProvider,
    model: &str,
    api_key: &str,
) -> Result<()> {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();
    let chat_history = Arc::new(Mutex::new(Vec::<String>::new()));
    
    let mut stdout = io::stdout();

    loop {
        // User input
        stdout.write_all(format!("{} ", "You:".bright_green().bold()).as_bytes()).await?;
        stdout.flush().await?;
        
        let input = match reader.next_line().await? {
            Some(line) => line,
            None => break,
        };
        
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        // Add to history
        {
            let mut history = chat_history.lock().await;
            history.push(format!("User: {}", input));
        }
        
        // Show typing indicator
        stdout.write_all(format!("{} ", "AI:".bright_magenta().bold()).as_bytes()).await?;
        stdout.flush().await?;
        
        // Call the provider
        match provider.chat(model, api_key, input).await {
            Ok(response) => {
                println!("{}", response);
                let mut history = chat_history.lock().await;
                history.push(format!("AI: {}", response));
            }
            Err(e) => {
                println!("{} {}", "Error:".red().bold(), e);
            }
        }
        
        println!(); // Empty line for readability
    }
    
    println!("\n{}", "Chat session ended".dimmed());
    Ok(())
}