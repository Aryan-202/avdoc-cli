use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "avdoc")]
#[command(about = "avdoc agentic ide", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
}
