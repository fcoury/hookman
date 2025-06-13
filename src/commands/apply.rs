use std::fs;
use std::os::unix::fs::PermissionsExt;
use anyhow::Result;
use colored::Colorize;
use crate::storage::{Storage, TomlStorage, get_git_hooks_dir};
use crate::generator::ScriptGenerator;
use crate::error::HookmanError;

pub fn execute(dry_run: bool) -> Result<()> {
    let storage = TomlStorage::new();
    
    if !storage.is_initialized() {
        return Err(HookmanError::NotInitialized.into());
    }
    
    let git_hooks_dir = get_git_hooks_dir()?;
    let generator = ScriptGenerator::new();
    
    let configured_hooks = storage.list_hooks()?;
    
    if configured_hooks.is_empty() {
        println!("{}", "No hooks configured to apply".yellow());
        return Ok(());
    }
    
    if dry_run {
        println!("{}", "DRY RUN - No changes will be made".yellow().bold());
        println!();
    }
    
    for hook_type in configured_hooks {
        let hook = storage.load_hook(hook_type)?;
        
        if hook.commands.is_empty() {
            continue;
        }
        
        let script = generator.generate(&hook)?;
        let hook_path = git_hooks_dir.join(hook_type.as_str());
        
        if dry_run {
            println!("{}", format!("Would create: {}", hook_path.display()).blue());
            println!("{}", "Contents:".dimmed());
            println!("{}", "---".dimmed());
            println!("{}", script.dimmed());
            println!("{}", "---".dimmed());
            println!();
        } else {
            // Backup existing hook if it exists
            if hook_path.exists() {
                let backup_path = hook_path.with_extension("backup");
                fs::copy(&hook_path, &backup_path)?;
                println!("{}", format!("  Backed up existing {} to {}", 
                    hook_type, 
                    backup_path.display()
                ).yellow());
            }
            
            // Write the hook script
            fs::write(&hook_path, script)?;
            
            // Make it executable
            let mut perms = fs::metadata(&hook_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&hook_path, perms)?;
            
            println!("{}", format!("✓ Applied {} hook", hook_type).green());
        }
    }
    
    if !dry_run {
        println!();
        println!("{}", "All hooks applied successfully!".green().bold());
    }
    
    Ok(())
}