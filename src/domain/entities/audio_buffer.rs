use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AudioBuffer {
    pub agent: Vec<i16>,
    pub user: Vec<i16>,
    pub streamed_content: String,

    pub cursor: u64,
    pub start: Option<u64>,
    pub end: Option<u64>,
    pub events: HashMap<u64, String>,
}

impl AudioBuffer {
    pub fn new() -> Self {
        AudioBuffer {
            agent: Vec::new(),
            user: Vec::new(),
            streamed_content: "".to_string(),
            cursor: 0,
            start: None,
            end: None,
            events: HashMap::new(),
        }
    }

    pub fn override_streamed_buffer(&mut self, content: String) {
        self.streamed_content = content;
    }
}
