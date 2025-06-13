use crate::error::HookmanError;
use crate::models::HookType;
use crate::storage::{Storage, TomlStorage};
use anyhow::Result;
use colored::Colorize;

pub fn execute(hook_type: Option<HookType>) -> Result<()> {
    let storage = TomlStorage::new();

    if !storage.is_initialized() {
        return Err(HookmanError::NotInitialized.into());
    }

    match hook_type {
        Some(specific_hook) => list_specific_hook(&storage, specific_hook),
        None => list_all_hooks(&storage),
    }
}

fn list_specific_hook(storage: &TomlStorage, hook_type: HookType) -> Result<()> {
    let hook = storage.load_hook(hook_type)?;

    println!("{}", format!("Hook: {}", hook_type).bold());

    if hook.commands.is_empty() {
        println!("  No commands configured");
    } else {
        for command in &hook.commands {
            println!(
                "  {} {}",
                format!("[{}]", command.id).cyan(),
                command.command
            );
            if let Some(desc) = &command.description {
                println!("      {}", desc.dimmed());
            }
        }
    }

    Ok(())
}

fn list_all_hooks(storage: &TomlStorage) -> Result<()> {
    let configured_hooks = storage.list_hooks()?;

    if configured_hooks.is_empty() {
        println!("No hooks configured yet");
        println!("Use 'hookman add' to start adding hooks");
        return Ok(());
    }

    println!("{}", "Configured hooks:".bold());

    for hook_type in configured_hooks {
        let hook = storage.load_hook(hook_type)?;
        println!(
            "\n{} ({} commands)",
            format!("{}", hook_type).green(),
            hook.commands.len()
        );

        for command in &hook.commands {
            println!(
                "  {} {}",
                format!("[{}]", command.id).cyan(),
                command.command
            );
            if let Some(desc) = &command.description {
                println!("      {}", desc.dimmed());
            }
        }
    }

    Ok(())
}
