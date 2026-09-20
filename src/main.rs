//! Main binary entry point for the `avdoc` application.
//!
//! Parses command-line arguments using `clap` and routes execution to the appropriate
//! command handler module.

mod cli;

use clap::Parser;
use cli::{Cli, Commands};

use avdoc::commands::init::run_init_menu;
use avdoc::commands::run::run_prompt;

/// Executes the CLI application entry point.
///
/// Parses command-line inputs from process arguments, maps subcommands to their
/// respective execution routines, and propagates unhandled execution errors to the runtime.
///
/// # Errors
///
/// Returns `Err(Box<dyn std::error::Error>)` if an unrecoverable error occurs during subcommand
/// execution, such as terminal I/O failure or filesystem access restriction.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            run_init_menu()?;
        }
        Commands::Run { prompt } => { 
            run_prompt(prompt)?;
        }
    }

    Ok(())
}
