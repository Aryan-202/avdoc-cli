use anyhow::Result;
use avdoc::cli;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "avdoc")]
#[command(about = "AI-powered documentation gatekeeper and architecture visualizer", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Lint the repository and generate a documentation score
    Lint {
        /// Path to the repository (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Minimum documentation score required (0-100)
        #[arg(short, long)]
        min_score: Option<u8>,

        /// Output format (terminal, json, markdown)
        #[arg(short, long, default_value = "terminal")]
        format: String,
    },

    /// Generate architecture diagrams and update README
    Diagram {
        /// Path to the repository (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Output diagram format (mermaid, ascii)
        #[arg(short, long, default_value = "mermaid")]
        format: String,

        /// Update README.md with the diagram
        #[arg(short, long)]
        update_readme: bool,
    },

    /// Automatically generate missing documentation
    Heal {
        /// Path to the repository (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Specific files to heal (if not specified, heals all low-scoring files)
        #[arg(short, long)]
        files: Option<Vec<String>>,

        /// Interactive mode - ask before making changes
        #[arg(short, long)]
        interactive: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Lint {
            path,
            min_score,
            format,
        } => {
            cli::lint::run(path, min_score, format).await?;
        }
        Commands::Diagram {
            path,
            format,
            update_readme,
        } => {
            cli::diagram::run(path, format, update_readme).await?;
        }
        Commands::Heal {
            path,
            files,
            interactive,
        } => {
            cli::heal::run(path, files, interactive).await?;
        }
    }

    Ok(())
}
