use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_new() {
        let cmd = Command::new(
            "test".to_string(),
            "echo test".to_string(),
            Some("Test command".to_string()),
        );
        
        assert_eq!(cmd.id, "test");
        assert_eq!(cmd.command, "echo test");
        assert_eq!(cmd.description, Some("Test command".to_string()));
    }

    #[test]
    fn test_command_new_without_description() {
        let cmd = Command::new(
            "test".to_string(),
            "echo test".to_string(),
            None,
        );
        
        assert_eq!(cmd.id, "test");
        assert_eq!(cmd.command, "echo test");
        assert_eq!(cmd.description, None);
    }

    #[test]
    fn test_command_serialize_deserialize() {
        let cmd = Command::new(
            "format".to_string(),
            "cargo fmt".to_string(),
            Some("Format code".to_string()),
        );
        
        let serialized = toml::to_string(&cmd).unwrap();
        let deserialized: Command = toml::from_str(&serialized).unwrap();
        
        assert_eq!(cmd, deserialized);
    }
}