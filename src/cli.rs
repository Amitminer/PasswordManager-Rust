use clap::{Parser, Subcommand};

/// Command line interface for the password manager.
#[derive(Parser)]
pub struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands.
#[derive(Subcommand)]
pub enum Commands {
    /// Add a new password entry.
    Add {
        /// Website for the password.
        website: String,
        /// Username for the website.
        username: String,
    },
    /// Get a password entry.
    Get {
        /// Website to retrieve the password for.
        website: String,
    },
    /// List all password entries.
    List,
    /// Delete a password entry.
    Delete {
        /// Website to delete the password for.
        website: String,
    },
}
