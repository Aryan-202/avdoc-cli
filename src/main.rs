mod cli;
use avdoc::helpers;
use avdoc::commands;

use clap::Parser;
use cli::{Cli, Commands, ConfigAction, ProviderAction};
use owo_colors::OwoColorize;
use helpers::files;
use avdoc::providers::manager::ProviderManager;
use avdoc::config::provider_config::ProviderConfig;

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ls { path } => {
            match helpers::files::get_files(&path) {
                Ok(files) => {
                    if files.is_empty() {
                        println!("{}", "Directory is empty.".yellow());
                        return Ok(());
                    }
                    let max_name_len = files.iter().map(|f| f.name.len()).max().unwrap_or(20).max(10);
                    let name_col_width = std::cmp::min(max_name_len, 50) + 2;

                    let type_col_width = 8;
                    let size_col_width = 12;

                    let header_name = format!("{:<width$}", "Name", width = name_col_width);
                    let header_type = format!("{:<width$}", "Type", width = type_col_width);
                    let header_size = format!("{:<width$}", "Size", width = size_col_width);
                    
                    println!(
                        "{} | {} | {}",
                        header_name.bold().underline(),
                        header_type.bold().underline(),
                        header_size.bold().underline()
                    );
                    
                    let separator_len = name_col_width + type_col_width + size_col_width + 6;
                    println!("{}", "-".repeat(separator_len));

                    for file in files {
                        let display_name = if file.name.len() > name_col_width - 2 {
                            let mut truncated = file.name[..name_col_width - 5].to_string();
                            truncated.push_str("...");
                            truncated
                        } else {
                            file.name.clone()
                        };

                        let padded_name = format!("{:<width$}", display_name, width = name_col_width);
                        let padded_type = format!("{:<width$}", if file.is_dir { "Dir" } else { "File" }, width = type_col_width);
                        let padded_size = format!("{:<width$}", if file.is_dir { "-".to_string() } else { files::format_size(file.size) }, width = size_col_width);

                        let final_name = if file.is_dir {
                            padded_name.blue().bold().to_string()
                        } else {
                            padded_name
                        };

                        let final_type = if file.is_dir {
                            padded_type.magenta().to_string()
                        } else {
                            padded_type.green().to_string()
                        };

                        let final_size = padded_size.cyan().to_string();

                        println!("{} | {} | {}", final_name, final_type, final_size);
                    }
                }
                Err(e) => {
                    eprintln!("{} {}", "Failed to list directory:".red().bold(), e);
                }
            }
        },
        Commands::Init { path } => {
            if let Err(e) = avdoc::commands::init::init(path) {
                eprintln!("{} {}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        }
        Commands::Config { action } => {
            match action {
                ConfigAction::Set { provider, model, api_key } => {
                    if let Err(e) = commands::config::set_config(&provider, &model, &api_key) {
                        eprintln!("{} {}", "Error:".red().bold(), e);
                        std::process::exit(1);
                    }
                }
                ConfigAction::Get { key } => {
                    match commands::config::get_config_value(&key) {
                        Ok(Some(value)) => println!("{}", value),
                        Ok(None) => {
                            eprintln!("{} Configuration key '{}' not found", "Error:".red().bold(), key);
                            std::process::exit(1);
                        }
                        Err(e) => {
                            eprintln!("{} {}", "Error:".red().bold(), e);
                            std::process::exit(1);
                        }
                    }
                }
                ConfigAction::List => {
                    if let Err(e) = commands::config::list_config() {
                        eprintln!("{} {}", "Error:".red().bold(), e);
                        std::process::exit(1);
                    }
                }
            }
        }
        Commands::Provider { action } => match action {
            ProviderAction::List => {
                let manager = ProviderManager::new();
            println!("Available Providers:");
            for provider in manager.list_providers() {
                println!("  - {}", provider);
            }
        }
        
        ProviderAction::SetKey { provider, key } => {
            let mut config = ProviderConfig::load()?;
            config.set_api_key(&provider, key);
            config.save()?;
            println!("API key saved for {}", provider);
        }
        
        ProviderAction::Use { provider, model } => {
            let manager = ProviderManager::new();
            
            // Verify provider exists
            if !manager.list_providers().contains(&provider) {
                anyhow::bail!("Provider '{}' not found. Use 'avdoc provider list' to see available providers.", provider);
            }
            
            // Use dynamically provided model
            println!("Using dynamic model: {}", model);
            
            let mut config = ProviderConfig::load()?;
            config.set_default(&provider, &model);
            config.save()?;
            
            println!("Default set to {}/{}", provider, model);
        }
        
        ProviderAction::Show => {
            let config = ProviderConfig::load()?;
            if let Some((provider, model)) = config.get_current_provider_model() {
                println!("Current configuration:");
                println!("  Provider: {}", provider);
                println!("  Model: {}", model);
            } else {
                println!("No default provider/model set.");
                println!("Run: avdoc provider use <provider> <model>");
            }
            
            println!("\nAPI Keys configured:");
            for (provider, _) in config.api_keys {
                println!("  - {}", provider);
            }
        }
        
        ProviderAction::ListModels { provider } => {
            let manager = ProviderManager::new();
            let models = manager.list_models(&provider).await?;
            
            println!("Models for {}:", provider);
            for model in models {
                let free_tag = if model.is_free { " [FREE]" } else { "" };
                println!("  - {} ({}){}", model.name, model.id, free_tag);
                if let Some(ctx) = model.context_length {
                    println!("      Context: {} tokens", ctx);
                }
            }
        }
    },
    
    Commands::Models { provider } => {
        let manager = ProviderManager::new();
        
        if let Some(p) = provider {
            let models = manager.list_models(&p).await?;
            println!("Models for {}:", p);
            for model in models {
                println!("  - {} ({})", model.name, model.id);
            }
        } else {
            let all_models = manager.list_all_models().await;
            for (provider_name, models) in all_models {
                println!("\n{}:", provider_name);
                for model in models {
                    println!("  - {} ({})", model.name, model.id);
                }
            }
        }
    },
    
    Commands::Run { prompt, provider, model, dry_run } => {
        let config = ProviderConfig::load()?;
        
        // Determine which provider/model to use
        let (use_provider, use_model) = if let (Some(p), Some(m)) = (provider, model) {
            (p, m)
        } else if let Some((p, m)) = config.get_current_provider_model() {
            (p, m)
        } else {
            anyhow::bail!("No provider/model specified. Set default with: avdoc provider use <provider> <model>");
        };
        
        // Get API key
        let api_key = config.get_api_key(&use_provider)
            .ok_or_else(|| anyhow::anyhow!("No API key for {}. Set with: avdoc provider set-key {} <key>", use_provider, use_provider))?;
        
        // Get provider instance
        let mut manager = ProviderManager::new();
        let provider_instance = manager.get_provider_mut(&use_provider)
            .ok_or_else(|| anyhow::anyhow!("Provider '{}' not found", use_provider))?;
        
        provider_instance.set_api_key(api_key.clone());
        
        println!("Using: {}/{}", use_provider, use_model);
        println!("Planning...");
    }
    };

    Ok(())
}

#[tokio::main]
pub async fn main() {
    if let Err(e) = run().await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}