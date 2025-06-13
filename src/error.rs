use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum HookmanError {
    #[error("Not in a Git repository")]
    NotInGitRepo,
    
    #[error("Hookman not initialized. Run 'hookman init' first")]
    NotInitialized,
    
    #[error("Hook type '{0}' not found")]
    HookNotFound(String),
    
    #[error("Command with ID '{0}' not found in hook '{1}'")]
    CommandNotFound(String, String),
    
    #[error("Command with ID '{0}' already exists in hook '{1}'")]
    CommandAlreadyExists(String, String),
    
    #[error("Failed to read configuration: {0}")]
    ConfigReadError(String),
    
    #[error("Failed to write configuration: {0}")]
    ConfigWriteError(String),
    
    #[error("Failed to generate hook script: {0}")]
    ScriptGenerationError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),
    
    #[error("TOML serialization error: {0}")]
    TomlSerError(#[from] toml::ser::Error),
}