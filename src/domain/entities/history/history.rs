use chrono::{DateTime, Utc};

use crate::domain::entities::history::{
    history_event::{HistoryEvent, HistoryEventPayload},
    history_member::HistoryMember,
};

pub struct History {
    pub events: Vec<HistoryEvent>,
}

impl History {
    pub fn new() -> Self {
        History { events: Vec::new() }
    }

    pub fn add(&mut self, payload: HistoryEventPayload) {
        let event = HistoryEvent::new(payload);
        self.events.push(event);
    }

    pub fn create_mark(&self, datetime: DateTime<Utc>) -> Vec<HistoryEvent> {
        let date_text_french = datetime.format("%A %d %B %Y").to_string();
        let date_format_french = datetime.format("%d/%m/%Y").to_string();
        let hour_min = datetime.format("%-Hh%M").to_string();

        let content = vec![
            String::from("Voici des informations supplémentaires qui pourraient t'aider à répondre au client :"),
            format!("- Aujourd'hui, nous sommes le : {}", date_text_french),
            format!(
                "- Au format dd/mm/yyyy nous somme le {}",
                date_format_french
            ),
            format!("- Il est actuellement {}", hour_min),
        ]
        .join("\n");

        let system_event = HistoryEvent::new(HistoryEventPayload {
            member: HistoryMember::System,
            content: Some(content),
            created_at: datetime,
        });

        let mut events = Vec::<HistoryEvent>::with_capacity(self.events.len() + 1);
        events.push(system_event);
        events.extend(self.events.iter().cloned());

        events
    }
}
