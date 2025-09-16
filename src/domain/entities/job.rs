use uuid::Uuid;

use crate::domain::utils::Utils;

#[derive(Debug, Clone)]
pub enum JobState {
    Pending,
    ReadyToSend,
    Completed,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub id: Uuid,
    pub state: JobState,
    pub data: Vec<i16>,
}

impl Job {
    pub fn default() -> Self {
        Job {
            id: Utils::generate_uuid(),
            state: JobState::Pending,
            data: Vec::new(),
        }
    }
}
