use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: String,
    pub command: String,
    pub description: Option<String>,
}

impl Command {
    pub fn new(id: String, command: String, description: Option<String>) -> Self {
        Command {
            id,
            command,
            description,
        }
    }
}