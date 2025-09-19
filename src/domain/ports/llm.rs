use anywho::Error;

use crate::domain::entities::history::history_event::HistoryEvent;

#[derive(Clone)]
pub struct LlmProcessResponse {}

pub trait Llm: Send + Sync + 'static {
    fn process(
        &mut self,
        model: String,
        history_events: Vec<HistoryEvent>,
    ) -> impl Future<Output = Result<LlmProcessResponse, Error>>;
}
