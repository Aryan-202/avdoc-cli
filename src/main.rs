use anyhow::Result;
use avdoc::cli;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "avdoc")]
#[command(about = "Vibe Coding CLI - AI powered project assistant", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Update or create project README with AI
    Update {
        #[command(subcommand)]
        target: UpdateCommands,
    },

    /// Add documentation or LLM configuration
    Add {
        #[command(subcommand)]
        target: AddCommands,
    },

    /// Generate folder structure (structure.md)
    Make {
        #[command(subcommand)]
        target: MakeCommands,
    },

    /// AI-assisted coding helper (Vibe Code)
    Vibe {
        #[command(subcommand)]
        target: VibeCommands,
    },
}

#[derive(Parser, Debug)]
pub enum UpdateCommands {
    /// Update README.md using AI
    Readme {
        /// Path to the repository (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
}

#[derive(Parser, Debug)]
pub enum AddCommands {
    /// Add AI-generated documentation about the project
    Docs {
        /// Path to the repository (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
    /// Configure and chat with an LLM
    Llm,
}

#[derive(Parser, Debug)]
pub enum MakeCommands {
    /// Create structure.md of the project
    Structure {
        /// Path to the repository (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
}

#[derive(Parser, Debug)]
pub enum VibeCommands {
    /// Start interactive AI-assisted coding helper
    Code,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Update { target } => match target {
            UpdateCommands::Readme { path } => {
                cli::update_readme::run(path).await?;
            }
        },
        Commands::Add { target } => match target {
            AddCommands::Docs { path } => {
                cli::add_docs::run(path).await?;
            }
            AddCommands::Llm => {
                cli::add_llm::run().await?;
            }
        },
        Commands::Make { target } => match target {
            MakeCommands::Structure { path } => {
                cli::make_structure::run(path).await?;
            }
        },
        Commands::Vibe { target } => match target {
            VibeCommands::Code => {
                cli::vibe_code::run().await?;
            }
        },
    }

    Ok(())
}
