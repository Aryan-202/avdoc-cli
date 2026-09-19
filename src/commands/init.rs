use crate::providers::deepseek::connect_deepseek;
use dialoguer::{theme::ColorfulTheme, Select, Confirm};
use std::fs;

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
        .items(&main_options)
        .interact()?;

    match selection {
        0 => handle_create_agent(&theme)?,
        1 => println!("Selecting existing agent..."),
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
        .items(&handle_create_agent_options)
        .interact()?;

    match selection {
        0 => connect_deepseek()?,
        1 => println!("gemini selected"), // TODO: Implement actual connection
        2 => println!("groq selected"),   // TODO: Implement actual connection
        3 => println!("openai selected"), // TODO: Implement actual connection
        4 => println!("openrouter selected"), // TODO: Implement actual connection
        5 => println!("anthropic selected"), // TODO: Implement actual connection
        _ => unreachable!(),
    }

    Ok(())
}

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