use anyhow::Result;
use colored::Colorize;
use crate::models::{HookType, Command};
use crate::storage::{Storage, TomlStorage};
use crate::error::HookmanError;

pub fn execute(
    hook_type: HookType,
    command: String,
    id: String,
    description: Option<String>,
) -> Result<()> {
    let storage = TomlStorage::new();
    
    if !storage.is_initialized() {
        return Err(HookmanError::NotInitialized.into());
    }
    
    let mut hook = storage.load_hook(hook_type)?;
    
    // Check if command with this ID already exists
    if hook.commands.iter().any(|c| c.id == id) {
        return Err(HookmanError::CommandAlreadyExists(id, hook_type.to_string()).into());
    }
    
    let new_command = Command::new(id.clone(), command.clone(), description.clone());
    hook.commands.push(new_command);
    
    storage.save_hook(&hook)?;
    
    println!("{}", format!("✓ Added command '{}' to {} hook", id, hook_type).green());
    if let Some(desc) = description {
        println!("  Description: {}", desc);
    }
    println!("  Command: {}", command);
    
    Ok(())
}