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
    Ls {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    Init {
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    Provider {
        #[command(subcommand)]
        action: ProviderAction,
    },
    
    Models {
        #[arg(short, long)]
        provider: Option<String>,
    },

    Run {
        prompt: String,
        #[arg(long)]
        provider: Option<String>,
        #[arg(long)]
        model: Option<String>,
        #[arg(long)]
        dry_run: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum ConfigAction {
    Set {
        provider: String,
        model: String,
        api_key: String,
    },
    
    Get {
        key: String,
    },
    
    List,
}

#[derive(Debug, Subcommand)]
pub enum ProviderAction {
    List,
    
    SetKey {
        provider: String,
        key: String,
    },
    
    Use {
        provider: String,
        model: String,
    },
    
    Show,
    
    ListModels {
        provider: String,
    },
}