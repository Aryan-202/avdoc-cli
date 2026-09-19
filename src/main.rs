mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use dialoguer::{theme::ColorfulTheme, Select};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            let options = [
                "Create a new agent",
                "Use an existing agent",
                "Add an agent for this project only",
                "Delete old agent configurations",
            ];

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose an initialization option")
                .default(0)
                .items(&options)
                .interact()?;

            println!("Executing: {}", options[selection]);

            match selection {
                0 => println!("Creating new agent..."),
                1 => println!("Selecting existing agent..."),
                2 => println!("Configuring project-specific agent..."),
                3 => println!("Removing old agent configurations..."),
                _ => unreachable!(),
            }
        }
    }

    Ok(())
}
