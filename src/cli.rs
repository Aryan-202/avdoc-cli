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
    /// List directory contents
    Ls {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    /// Initialize avdoc in current directory
    Init {
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Manage AI providers
    Provider {
        #[command(subcommand)]
        action: ProviderAction,
    },
    
    /// List available models
    Models {
        #[arg(short, long)]
        provider: Option<String>,  // filter by provider
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
    /// Set configuration (provider, model, api-key)
    Set {
        /// Provider (openai, anthropic, etc.)
        provider: String,
        
        /// Model name (gpt-4, gpt-3.5-turbo, claude-3, etc.)
        model: String,
        
        /// API key for the provider
        api_key: String,
    },
    
    /// Get configuration value
    Get {
        /// Configuration key to retrieve (provider, model, api_key.PROVIDER)
        key: String,
    },
    
    /// List all configuration
    List,
}

#[derive(Debug, Subcommand)]
pub enum ProviderAction {
    /// List all available providers
    List,
    
    /// Set API key for a provider
    SetKey {
        provider: String,
        key: String,
    },
    
    /// Set default provider and model
    Use {
        provider: String,
        model: String,
    },
    
    /// Show current provider/model configuration
    Show,
    
    /// List models for a specific provider
    ListModels {
        provider: String,
    },
}