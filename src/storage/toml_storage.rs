use std::fs;
use anyhow::Result;
use crate::models::{Hook, HookType, Config};
use crate::storage::{Storage, get_hookman_dir, get_hooks_dir, CONFIG_FILE};
use crate::error::HookmanError;

pub struct TomlStorage;

impl TomlStorage {
    pub fn new() -> Self {
        TomlStorage
    }
}

impl Storage for TomlStorage {
    fn init(&self) -> Result<()> {
        let _hookman_dir = get_hookman_dir()?;
        let hooks_dir = get_hooks_dir()?;
        
        fs::create_dir_all(&hooks_dir)?;
        
        let config = Config::default();
        self.save_config(&config)?;
        
        Ok(())
    }
    
    fn is_initialized(&self) -> bool {
        get_hookman_dir()
            .map(|dir| dir.exists())
            .unwrap_or(false)
    }
    
    fn load_hook(&self, hook_type: HookType) -> Result<Hook> {
        let hooks_dir = get_hooks_dir()?;
        let hook_file = hooks_dir.join(format!("{}.toml", hook_type.as_str()));
        
        if !hook_file.exists() {
            return Ok(Hook::new(hook_type));
        }
        
        let contents = fs::read_to_string(&hook_file)
            .map_err(|e| HookmanError::ConfigReadError(e.to_string()))?;
        
        let mut hook: Hook = toml::from_str(&contents)?;
        hook.hook_type = hook_type;
        
        Ok(hook)
    }
    
    fn save_hook(&self, hook: &Hook) -> Result<()> {
        let hooks_dir = get_hooks_dir()?;
        let hook_file = hooks_dir.join(format!("{}.toml", hook.hook_type.as_str()));
        
        let contents = toml::to_string_pretty(&hook)?;
        fs::write(&hook_file, contents)
            .map_err(|e| HookmanError::ConfigWriteError(e.to_string()))?;
        
        Ok(())
    }
    
    fn list_hooks(&self) -> Result<Vec<HookType>> {
        let hooks_dir = get_hooks_dir()?;
        let mut hooks = Vec::new();
        
        if !hooks_dir.exists() {
            return Ok(hooks);
        }
        
        for entry in fs::read_dir(&hooks_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
                if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Ok(hook_type) = file_stem.parse::<HookType>() {
                        hooks.push(hook_type);
                    }
                }
            }
        }
        
        Ok(hooks)
    }
    
    fn load_config(&self) -> Result<Config> {
        let config_file = get_hookman_dir()?.join(CONFIG_FILE);
        
        if !config_file.exists() {
            return Ok(Config::default());
        }
        
        let contents = fs::read_to_string(&config_file)
            .map_err(|e| HookmanError::ConfigReadError(e.to_string()))?;
        
        Ok(toml::from_str(&contents)?)
    }
    
    fn save_config(&self, config: &Config) -> Result<()> {
        let config_file = get_hookman_dir()?.join(CONFIG_FILE);
        
        let contents = toml::to_string_pretty(&config)?;
        fs::write(&config_file, contents)
            .map_err(|e| HookmanError::ConfigWriteError(e.to_string()))?;
        
        Ok(())
    }
}