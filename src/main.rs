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
            
            if !manager.list_providers().contains(&provider) {
                anyhow::bail!("Provider '{}' not found. Use 'avdoc provider list' to see available providers.", provider);
            }
            
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
    let global_config = commands::config::load_global_config()?;
    
    let use_provider = if let Some(p) = provider {
        p
    } else if let Some((p, _)) = global_config.providers.iter().next() {
        p.clone()
    } else {
        anyhow::bail!("No provider/model specified. Set with: avdoc config set <provider> <model> <api_key>");
    };
    
    let provider_data = global_config.providers.get(&use_provider)
        .ok_or_else(|| anyhow::anyhow!("Provider '{}' not configured. Use: avdoc config set {} <model> <api_key>", use_provider, use_provider))?;
    
    let api_key = &provider_data.api_key;
    let use_model = if let Some(m) = model { m } else { provider_data.model.clone() };
    
    if dry_run {
        println!("DRY RUN: Sending to {}/{}", use_provider, use_model);
        println!("Prompt: {}", prompt);
        println!("Dry run completed successfully (no API call made).");
        return Ok(());
    }
    
    let mut manager = ProviderManager::new();
    let provider_instance = manager.get_provider_mut(&use_provider)
        .ok_or_else(|| anyhow::anyhow!("Provider '{}' not found", use_provider))?;
    
    provider_instance.set_api_key(api_key.clone());
    
    println!("Provider initialized: {}/{}", use_provider, use_model);
    println!("Processing request...");
    
    let system_prompt = "You are a helpful coding assistant. \
If the user asks you to write or modify code, you must place the code inside a special block using the exact following format:

<avdoc_file name=\"path/to/file.ext\">
CODE_HERE
</avdoc_file>

You can create multiple files. Provide clear, working code solutions.";

    match provider_instance.chat(&use_model, system_prompt, &prompt).await {
        Ok(response) => {
            println!("\nResponse:\n{}", response);
            
            // Parse the response to extract and create generated files.
            let mut start_idx = 0;
            while let Some(start_tag) = response[start_idx..].find("<avdoc_file name=\"") {
                let actual_start = start_idx + start_tag;
                let name_start = actual_start + 18; // Offset for "<avdoc_file name=\""
                if let Some(name_end_offset) = response[name_start..].find("\">") {
                    let actual_name_end = name_start + name_end_offset;
                    let file_name = &response[name_start..actual_name_end];
                    
                    let content_start = actual_name_end + 2; // Offset for "\">"
                    if let Some(content_end_offset) = response[content_start..].find("</avdoc_file>") {
                        let content_end = content_start + content_end_offset;
                        let content = &response[content_start..content_end];
                        
                        let content = content.trim_start_matches('\n').trim_end_matches('\n');
                        
                        println!("\nCreating file: {}", file_name);
                        let path = std::path::Path::new(file_name);
                        if let Some(parent) = path.parent() {
                            if !parent.as_os_str().is_empty() {
                                if let Err(e) = std::fs::create_dir_all(parent) {
                                    eprintln!("Failed to create directories for {}: {}", file_name, e);
                                    start_idx = content_end + 13;
                                    continue;
                                }
                            }
                        }
                        
                        if let Err(e) = std::fs::write(file_name, content) {
                            eprintln!("Failed to write to file {}: {}", file_name, e);
                        } else {
                            println!("File {} written successfully.", file_name);
                        }
                        
                        start_idx = content_end + 13; // Advance index past closing tag.
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
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