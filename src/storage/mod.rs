mod toml_storage;

pub use toml_storage::TomlStorage;

use std::path::PathBuf;
use anyhow::Result;
use crate::models::{Hook, HookType, Config};

pub const HOOKMAN_DIR: &str = ".hookman";
pub const HOOKS_DIR: &str = "hooks";
pub const CONFIG_FILE: &str = "config.toml";

pub trait Storage {
    fn init(&self) -> Result<()>;
    fn is_initialized(&self) -> bool;
    fn load_hook(&self, hook_type: HookType) -> Result<Hook>;
    fn save_hook(&self, hook: &Hook) -> Result<()>;
    fn list_hooks(&self) -> Result<Vec<HookType>>;
    fn load_config(&self) -> Result<Config>;
    fn save_config(&self, config: &Config) -> Result<()>;
}

pub fn get_hookman_dir() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    Ok(current_dir.join(HOOKMAN_DIR))
}

pub fn get_hooks_dir() -> Result<PathBuf> {
    Ok(get_hookman_dir()?.join(HOOKS_DIR))
}

pub fn get_git_dir() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let git_dir = current_dir.join(".git");
    if git_dir.exists() && git_dir.is_dir() {
        Ok(git_dir)
    } else {
        Err(crate::error::HookmanError::NotInGitRepo.into())
    }
}

pub fn get_git_hooks_dir() -> Result<PathBuf> {
    Ok(get_git_dir()?.join("hooks"))
}