#[derive(Debug, Clone)]
pub struct AudioBuffer {
    pub agent: Vec<i16>,
    pub user: Vec<i16>,
    pub streamed_content: String,
}

impl AudioBuffer {
    pub fn new() -> Self {
        AudioBuffer {
            agent: Vec::new(),
            user: Vec::new(),
            streamed_content: "".to_string(),
        }
    }

    pub fn override_streamed_buffer(&mut self, content: String) {
        self.streamed_content = content;
    }
}
