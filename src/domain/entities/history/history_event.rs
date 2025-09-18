use chrono::{DateTime, Utc};

use crate::domain::entities::history::history_member::HistoryMember;

#[derive(Debug, Clone)]
pub struct HistoryEvent {
    pub member: HistoryMember,
    pub content: Option<String>,
    pub created_at: DateTime<Utc>,
    pub is_saved: bool,
}

#[derive(Debug, Clone)]
pub struct HistoryEventPayload {
    pub member: HistoryMember,
    pub content: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl HistoryEvent {
    pub fn new(payload: HistoryEventPayload) -> Self {
        let is_saved = match payload.member {
            HistoryMember::User | HistoryMember::Agent | HistoryMember::ToolCall => false,
            HistoryMember::System => true,
        };

        Self {
            member: payload.member,
            content: payload.content,
            created_at: payload.created_at,
            is_saved,
        }
    }
}
