use anyhow::Result;
use colored::Colorize;
use crate::models::HookType;
use crate::storage::{Storage, TomlStorage};
use crate::error::HookmanError;

pub fn execute(hook_type: HookType, command_id: String) -> Result<()> {
    let storage = TomlStorage::new();
    
    if !storage.is_initialized() {
        return Err(HookmanError::NotInitialized.into());
    }
    
    let mut hook = storage.load_hook(hook_type)?;
    
    let initial_len = hook.commands.len();
    hook.commands.retain(|c| c.id != command_id);
    
    if hook.commands.len() == initial_len {
        return Err(HookmanError::CommandNotFound(command_id, hook_type.to_string()).into());
    }
    
    storage.save_hook(&hook)?;
    
    println!("{}", format!("✓ Removed command '{}' from {} hook", command_id, hook_type).green());
    
    Ok(())
}