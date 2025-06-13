use clap::{Parser, Subcommand};
use crate::models::HookType;

#[derive(Parser)]
#[command(
    name = "hookman",
    about = "A Git hooks manager",
    version,
    author
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new .hookman directory in the current Git repository
    Init,
    
    /// Add a command to a specific hook type
    Add {
        /// The type of Git hook
        hook_type: HookType,
        
        /// The command to execute
        command: String,
        
        /// Unique identifier for this command
        #[arg(short, long)]
        id: String,
        
        /// Human-readable description of what this command does
        #[arg(short, long)]
        description: Option<String>,
    },
    
    /// Remove a command from a hook
    Remove {
        /// The type of Git hook
        hook_type: HookType,
        
        /// The ID of the command to remove
        command_id: String,
    },
    
    /// List all hooks or commands for a specific hook
    List {
        /// Optional: specific hook type to list
        hook_type: Option<HookType>,
    },
    
    /// Apply the hook configuration to the Git repository
    Apply {
        /// Show what would be done without making changes
        #[arg(short, long)]
        dry_run: bool,
    },
    
    /// Show the current hook configuration status
    Status,
}