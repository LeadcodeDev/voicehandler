use crate::domain::ports::llm::Llm;

pub struct GeminiAdapter {
}

impl GeminiAdapter {
  pub fn new(api_key: String) -> Self {
  }
}

impl Llm for GeminiAdapter {}
