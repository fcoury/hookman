mod cli;
mod commands;
mod error;
mod generator;
mod models;
mod storage;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::execute(),
        Commands::Add {
            hook_type,
            command,
            id,
            description,
        } => commands::add::execute(hook_type, command, id, description),
        Commands::Remove {
            hook_type,
            command_id,
        } => commands::remove::execute(hook_type, command_id),
        Commands::List { hook_type } => commands::list::execute(hook_type),
        Commands::Apply { dry_run } => commands::apply::execute(dry_run),
        Commands::Status => commands::status::execute(),
    }
}
