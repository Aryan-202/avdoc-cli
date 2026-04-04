use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "avdoc", version, about = "AI-powered documentation and code assistant")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List directory contents (existing)
    Ls {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    /// Initialize avdoc in current directory
    Init {
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}