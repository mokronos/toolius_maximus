use core::fmt;
use colored::*;
use serde_json::json;

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Agent,
    System,
    User,
    Tool,
}

impl ToString for MessageType {
    fn to_string(&self) -> String {
        match self {
            MessageType::Agent => "assistant".to_string(),
            MessageType::System => "system".to_string(),
            MessageType::User => "user".to_string(),
            MessageType::Tool => "tool".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    message_type: MessageType,
    content: String,
}

impl Message {
    pub fn new(message_type: MessageType, content: &str) -> Self {
        Self {
            message_type,
            content: content.to_string(),
        }
    }
    pub fn json(self) -> serde_json::Value {
        json!({
            "role": self.message_type.to_string(),
            "content": self.content
        })
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.message_type {
            MessageType::System => write!(
                f,
                "{} {}",
                "[System]".bold().blue(),
                self.content.green()
            ),
            MessageType::Agent => write!(
                f,
                "{} {}",
                "[Agent]".bold().yellow(),
                self.content.green()
            ),
            MessageType::User => write!(
                f,
                "{} {}",
                "[User]".bold().magenta(),
                self.content.yellow()
            ),
            MessageType::Tool => write!(
                f,
                "{} {}",
                "[Tool]".bold().cyan(),
                self.content.white()
            ),
        }
    }
}

