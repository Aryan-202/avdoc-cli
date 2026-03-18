use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use crate::config::provider_config::{Config, ProviderConfig};
use crate::providers;

pub async fn run() -> Result<()> {
    println!("{}", "🚀 Configuring LLM Provider".bright_cyan().bold());
    println!("{}", "This will help avdoc interact with AI services.".dimmed());

    let providers_list = ["openai", "gemini", "anthropic", "openrouter"];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an AI provider")
        .items(&providers_list)
        .default(0)
        .interact()?;

    let provider_name = providers_list[selection].to_string();
    
    // Get models for the selected provider
    let models = providers::get_models_for_provider(&provider_name);
    
    let model_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Select a model for {}", provider_name))
        .items(&models)
        .default(0)
        .interact()?;
        
    let model_name = models[model_selection].clone();

    let api_key: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Enter your {} API key", provider_name.to_uppercase()))
        .interact_text()?;

    let mut config = Config::load();
    
    let provider_config = ProviderConfig {
        provider: provider_name.clone(),
        model: model_name,
        api_key,
        base_url: None, // Can be extended if needed
    };

    config.add_provider(provider_name.clone(), provider_config);
    config.save()?;

    println!("\n{}", "✅ Provider configured successfully!".bright_green().bold());
    println!("Default provider set to: {}", provider_name.cyan().bold());
    
    Ok(())
}
