use core::fmt;
use colored::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Message {
    System(SystemMessage),
    User(UserMessage),
    Agent(AgentMessage),
    Tool(ToolMessage),
}

impl Message {
    pub fn system(content: String) -> Self {
        Self::System(SystemMessage::new(content))
    }

    pub fn user(content: String) -> Self {
        Self::User(UserMessage::new(content))
    }

    pub fn agent(content: String) -> Self {
        Self::Agent(AgentMessage::new(content))
    }

    pub fn tool(content: String, tool_call_id: String) -> Self {
        Self::Tool(ToolMessage::new(content, tool_call_id))
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Message::System(m) => write!(f, "{}", m),
            Message::User(m) => write!(f, "{}", m),
            Message::Agent(m) => write!(f, "{}", m),
            Message::Tool(m) => write!(f, "{}", m),
        }
    }
}

// === AGENT MESSAGE ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    role: String,
    content: String,
}

impl AgentMessage {
    pub fn new(content: String) -> Self {
        Self {
            role: "assistant".to_string(),
            content
        }
    }
}

impl fmt::Display for AgentMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", "[Agent]:".bold().yellow(), self.content.green())
    }
}

// === SYSTEM MESSAGE ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMessage {
    role: String,
    content: String,
}

impl SystemMessage {
    pub fn new(content: String) -> Self {
        Self {
            role: "system".to_string(),
            content
        }
    }
}

impl fmt::Display for SystemMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", "[System]:".bold().blue(), self.content.green())
    }
}

// === USER MESSAGE ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    role: String,
    content: String,
}

impl UserMessage {
    pub fn new(content: String) -> Self {
        Self {
            role: "user".to_string(),
            content
        }
    }
}


impl fmt::Display for UserMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", "[User]:".bold().magenta(), self.content.yellow())
    }
}

// === TOOL MESSAGE ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMessage {
    role: String,
    content: String,
    tool_call_id: String
}

impl ToolMessage {
    pub fn new(content: String, tool_call_id: String) -> Self {
        Self {
            role: "tool".to_string(),
            content,
            tool_call_id
        }
    }
}

impl fmt::Display for ToolMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", "[Tool]:".bold().cyan(), self.content.white())
    }
}
