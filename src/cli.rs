//! Command-line argument definitions and parser configurations for `avdoc`.
//!
//! Provides data structures derived via `clap` to parse, validate, and structure
//! top-level commands and subcommands supplied by the user.

use clap::{Parser, Subcommand};

/// Top-level command-line argument parser model.
///
/// Encapsulates the root command metadata and the set of dispatchable subcommands.
#[derive(Parser)]
#[command(name = "avdoc")]
#[command(about = "avdoc agentic ide", long_about = None)]
pub struct Cli {
    /// Subcommand selected for execution.
    #[command(subcommand)]
    pub command: Commands,
}

/// Enumeration of supported CLI subcommands.
#[derive(Subcommand)]
pub enum Commands {
    /// Initializes agent configuration, provider credentials, and project setup.
    Init,
}
