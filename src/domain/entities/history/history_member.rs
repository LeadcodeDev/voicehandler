use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum HistoryMember {
    User,
    Agent,
    System,
    ToolCall,
}

impl From<String> for HistoryMember {
    fn from(value: String) -> Self {
        match value.as_str() {
            "user" => HistoryMember::User,
            "agent" => HistoryMember::Agent,
            "system" => HistoryMember::System,
            "tool_call" => HistoryMember::ToolCall,
            _ => panic!("Invalid history member"),
        }
    }
}

impl Display for HistoryMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HistoryMember::User => write!(f, "user"),
            HistoryMember::Agent => write!(f, "agent"),
            HistoryMember::System => write!(f, "system"),
            HistoryMember::ToolCall => write!(f, "tool_call"),
        }
    }
}
