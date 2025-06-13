use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}