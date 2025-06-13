use crate::models::HookType;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "hookman",
    about = "A Git hooks manager that helps you define, manage, and share Git hooks across your team",
    long_about = "Hookman is a command-line tool for managing Git hooks across projects with a simple, declarative configuration.

It allows you to:
• Define Git hooks in a version-controlled .hookman directory
• Manage multiple commands per hook type
• Apply hook configurations to any Git repository
• Share hook configurations with your team

Example workflow:
  $ hookman init                    # Initialize .hookman directory
  $ hookman add pre-commit \"cargo fmt -- --check\" --id format --description \"Check formatting\"
  $ hookman add pre-commit \"cargo test\" --id test --description \"Run tests\"
  $ hookman apply                   # Install hooks to .git/hooks/
  $ hookman status                  # Check current configuration",
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
    #[command(
        long_about = "Initialize a new .hookman directory in the current Git repository.

This command creates the necessary directory structure for storing hook configurations:
  .hookman/
  ├── config.toml    # Hookman configuration
  └── hooks/         # Hook definitions

The .hookman directory should be committed to version control to share
hook configurations with your team.

Example:
  $ cd my-project
  $ hookman init
  ✓ Initialized hookman in .hookman/"
    )]
    Init,

    /// Add a command to a specific hook type
    #[command(long_about = "Add a command to a specific Git hook type.

Each command must have a unique ID within the hook. Commands are executed
in the order they were added. If a command fails (non-zero exit code),
subsequent commands will not be executed.

Examples:
  # Add a formatting check to pre-commit
  $ hookman add pre-commit \"cargo fmt -- --check\" --id format --description \"Check formatting\"
  
  # Add a test runner to pre-push
  $ hookman add pre-push \"cargo test --all\" --id test --description \"Run all tests\"
  
  # Add a commit message validator
  $ hookman add commit-msg \"grep -qE '^[A-Z]{2,}-[0-9]+' $1\" --id jira --description \"Check for JIRA ticket\"")]
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
    #[command(long_about = "Remove a command from a specific Git hook type.

The command is identified by its unique ID. After removal, the remaining
commands will continue to execute in their original order.

Example:
  $ hookman remove pre-commit format
  ✓ Removed command 'format' from pre-commit hook")]
    Remove {
        /// The type of Git hook
        hook_type: HookType,

        /// The ID of the command to remove
        command_id: String,
    },

    /// List all hooks or commands for a specific hook
    #[command(
        long_about = "List all configured hooks or commands for a specific hook type.

When called without arguments, shows all hooks with their commands.
When called with a hook type, shows only commands for that specific hook.

Examples:
  # List all configured hooks
  $ hookman list
  
  # List only pre-commit hooks
  $ hookman list pre-commit"
    )]
    List {
        /// Optional: specific hook type to list
        hook_type: Option<HookType>,
    },

    /// Apply the hook configuration to the Git repository
    #[command(long_about = "Apply the hook configuration to the Git repository.

This command generates shell scripts from your hook configurations and
installs them to .git/hooks/. Existing hooks are backed up with a .backup
extension before being replaced.

The generated scripts include error handling (set -e) and will stop
execution if any command fails.

Examples:
  # Apply all configured hooks
  $ hookman apply
  
  # Preview changes without applying them
  $ hookman apply --dry-run")]
    Apply {
        /// Show what would be done without making changes
        #[arg(short, long)]
        dry_run: bool,
    },

    /// Show the current hook configuration status
    #[command(long_about = "Show the current status of hook configurations.

Displays which hooks are configured, how many commands each has, and
whether they have been applied to the Git repository. Also shows if
there are external (non-hookman) hooks present.

Status indicators:
  • Green: Hook is configured and applied
  • Red: Hook is configured but not applied
  • Yellow: External hook exists or no commands configured

Example:
  $ hookman status
  Hookman Status
  =============
  
  Configuration directory: /path/to/project/.hookman
  Git hooks directory: /path/to/project/.git/hooks
  
  Configured hooks:
    pre-commit - 2 commands, applied
    pre-push - 1 commands, not applied")]
    Status,
}
