use anyhow::Result;
use colored::Colorize;
use crate::storage::{Storage, TomlStorage, get_git_dir};

pub fn execute() -> Result<()> {
    // Check if we're in a git repository
    get_git_dir()?;
    
    let storage = TomlStorage::new();
    
    if storage.is_initialized() {
        println!("{}", "Hookman is already initialized in this repository".yellow());
        return Ok(());
    }
    
    storage.init()?;
    
    println!("{}", "✓ Initialized hookman in .hookman/".green());
    println!("  Use 'hookman add' to start adding hooks");
    
    Ok(())
}