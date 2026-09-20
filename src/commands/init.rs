//! Interactive initialization command handlers.
//!
//! Provides CLI menu workflows for configuring new LLM agents, selecting existing
//! provider credentials, and managing stored configuration directories.

use crate::config::provider_config::ProviderConfig;
use crate::providers::anthropic::connect_anthropic;
use crate::providers::deepseek::connect_deepseek;
use crate::providers::gemini::connect_gemini;
use crate::providers::groq::connect_groq;
use crate::providers::openai::connect_openai;
use crate::providers::openrouter::connect_openrouter;

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::fs;

use crate::helpers::get_existing_agent_helper::get_available_providers;

/// Launches the interactive initialization menu loop.
///
/// Prompts the user to choose from available initialization actions, including creating
/// a new agent, activating an existing configured agent, setting project-specific agents,
/// deleting legacy configurations, or terminating the setup process.
///
/// # Returns
///
/// Returns `Ok(())` upon successful completion of the selected workflow or exit.
///
/// # Errors
///
/// Returns `Err(Box<dyn std::error::Error>)` if user interaction fails, terminal I/O is
/// interrupted, or downstream provider connection/deletion routines fail.
pub fn run_init_menu() -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme::default();

    let main_options = [
        "Create a new agent",
        "Use an existing agent",
        "Add an agent for this project only",
        "Delete old agent configurations",
        "Exit",
    ];

    let selection = Select::with_theme(&theme)
        .with_prompt("Choose an initialization option")
        .default(0)
        .items(main_options)
        .interact()?;

    match selection {
        0 => handle_create_agent(&theme)?,
        1 => {
            get_existing_agent(&theme)?;
        }
        2 => println!("Configuring project-specific agent..."),
        3 => handle_delete_configs()?,
        4 => {
            println!("Exiting setup...");
            return Ok(());
        }
        _ => unreachable!(),
    }

    if selection != 0 && selection != 3 {
        return Ok(());
    }

    Ok(())
}

/// Prompts the user to select an LLM provider and delegates to its setup routine.
///
/// Presents a selection list of supported LLM backend providers and invokes the corresponding
/// connection handler upon selection.
///
/// # Errors
///
/// Returns `Err(Box<dyn std::error::Error>)` if terminal interaction fails or if the invoked
/// provider connection handler encounters an unrecoverable error.
fn handle_create_agent(theme: &ColorfulTheme) -> Result<(), Box<dyn std::error::Error>> {
    let handle_create_agent_options = [
        "deepseek",
        "gemini",
        "groq",
        "openai",
        "openrouter",
        "anthropic",
    ];

    let selection = Select::with_theme(theme)
        .with_prompt("Select your agent provider...")
        .default(0)
        .items(handle_create_agent_options)
        .interact()?;

    match selection {
        0 => connect_deepseek()?,
        1 => connect_gemini()?,
        2 => connect_groq()?,
        3 => connect_openai()?,
        4 => connect_openrouter()?,
        5 => connect_anthropic()?,
        _ => unreachable!(),
    }

    Ok(())
}

/// Deletes persisted agent configuration files after explicit user confirmation.
///
/// Locates the user's home directory configuration folder (`.avdoc`) and prompts for interactive
/// confirmation before recursively removing the directory and its contents.
///
/// # Errors
///
/// Returns `Err(Box<dyn std::error::Error>)` if:
/// - The user's home directory cannot be resolved.
/// - Terminal confirmation prompt encounters an I/O error.
/// - Filesystem directory removal fails due to permission or I/O constraints.
fn handle_delete_configs() -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = dirs::home_dir()
        .ok_or("Could not determine the home directory")?
        .join(".avdoc");

    if config_dir.exists() {
        let proceed = Confirm::new()
            .with_prompt("Are you sure you want to delete all saved agent configs?")
            .default(false)
            .interact()?;

        if proceed {
            fs::remove_dir_all(&config_dir)?;
            println!("Deleted old agent configurations.");
        } else {
            println!("Operation cancelled.");
        }
    } else {
        println!("No old agent configurations found.");
    }

    Ok(())
}

/// Resolves and prompts selection among existing configured providers.
///
/// Loads the global configuration store and checks for available providers with valid API keys.
/// If exactly one provider is configured, it is auto-selected. If multiple exist, an interactive
/// selection prompt is shown. If no providers are configured, prompts to create one.
///
/// # Returns
///
/// Returns `Ok(Some(String))` with the selected provider name, or `Ok(None)` if no provider
/// was selected or a new provider setup was initiated.
///
/// # Errors
///
/// Returns `Err(Box<dyn std::error::Error>)` if terminal prompt interaction fails.
pub fn get_existing_agent(theme: &ColorfulTheme) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let config = match ProviderConfig::load() {
        Ok(cfg) => cfg,
        Err(_) => {
            println!("{}", "No saved configurations found.".yellow());
            return prompt_create_agent(theme);
        }
    };

    let available = get_available_providers(&config);

    if available.is_empty() {
        println!("{}", "No configured agents found.".yellow());
        return prompt_create_agent(theme);
    }

    if available.len() == 1 {
        println!("✔ Auto-selected only available agent: {}", available[0]);
        return Ok(Some(available[0].clone()));
    }

    let selection = Select::with_theme(theme)
        .with_prompt("Select an existing agent to activate")
        .items(&available)
        .interact()?;

    Ok(Some(available[selection].clone()))
}

/// Prompts the user to initiate creation of a new agent configuration.
///
/// Displays a confirmation prompt asking whether the user wants to configure a new agent now.
/// If confirmed, delegates to [`handle_create_agent`].
///
/// # Returns
///
/// Returns `Ok(None)` after the prompt or subsequent agent configuration completes.
///
/// # Errors
///
/// Returns `Err(Box<dyn std::error::Error>)` if interactive confirmation or agent creation fails.
fn prompt_create_agent(theme: &ColorfulTheme) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let create = Confirm::with_theme(theme)
        .with_prompt("Would you like to configure a new agent now?")
        .default(true)
        .interact()?;

    if create {
        handle_create_agent(theme)?;
    }
    Ok(None)
}