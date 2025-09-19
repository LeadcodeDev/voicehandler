use anywho::Error;

use crate::{
    domain::{
        entities::history::history_event::HistoryEvent,
        ports::llm::{Llm, LlmProcessResponse},
    },
    infrastructure::llm::gemini_adapter::GeminiAdapter,
};

#[derive(Clone)]
pub enum LlmList {
    Gemini(GeminiAdapter),
}

impl Llm for LlmList {
    async fn process(
        &mut self,
        model: String,
        history_events: Vec<HistoryEvent>,
    ) -> Result<LlmProcessResponse, Error> {
        match self {
            LlmList::Gemini(adapter) => adapter.process(model, history_events).await,
        }
    }
}
