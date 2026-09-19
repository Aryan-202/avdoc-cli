mod cli;

use clap::Parser;
use cli::{Cli, Commands};

use avdoc::commands::init::run_init_menu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            run_init_menu()?;
        }
    }

    Ok(())
}
