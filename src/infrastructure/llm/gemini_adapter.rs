use std::sync::Arc;

use anywho::Error;
use openai_api_rs::v1::{
    api::OpenAIClient,
    chat_completion::{
        ChatCompletionMessage, ChatCompletionRequest, Content, MessageRole, ToolChoiceType,
    },
};
use tokio::sync::Mutex;

use crate::domain::{
    entities::history::{history_event::HistoryEvent, history_member::HistoryMember},
    ports::llm::{Llm, LlmProcessResponse},
};

#[derive(Clone)]
pub struct GeminiAdapter {
    client: Arc<Mutex<OpenAIClient>>,
}

impl GeminiAdapter {
    pub fn new(api_key: String, endpoint: String) -> Result<Self, Error> {
        let client = OpenAIClient::builder()
            .with_api_key(api_key)
            .with_endpoint(endpoint)
            .build()
            .map_err(|err| {
                let message = format!("Error during OpenAI client instanciation: {}", err);
                Error::msg(message)
            })?;

        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })
    }
}

impl Llm for GeminiAdapter {
    async fn process(
        &mut self,
        model: String,
        history_events: Vec<HistoryEvent>,
    ) -> Result<LlmProcessResponse, Error> {
        let messages: Vec<ChatCompletionMessage> = history_events
            .iter()
            .map(|event| ChatCompletionMessage {
                name: None,
                tool_call_id: None,
                tool_calls: None,
                role: match event.member {
                    HistoryMember::User => MessageRole::user,
                    HistoryMember::Agent => MessageRole::assistant,
                    HistoryMember::ToolCall => MessageRole::tool,
                    HistoryMember::System => MessageRole::system,
                },
                content: match &event.content {
                    Some(content) => Content::Text(content.clone()),
                    None => Content::Text("".to_string()),
                },
            })
            .collect();

        let mut client = self.client.lock().await;
        let response = client
            .chat_completion(ChatCompletionRequest {
                model,
                messages,
                temperature: Some(0.2),
                top_p: Some(0.8),
                n: None,
                response_format: None,
                stream: Some(true),
                stop: None,
                max_tokens: Some(512),
                presence_penalty: None,
                frequency_penalty: None,
                logit_bias: None,
                user: None,
                seed: None,
                tools: Some(Vec::new()),
                parallel_tool_calls: None,
                tool_choice: Some(ToolChoiceType::Auto),
                reasoning: None,
                transforms: None,
            })
            .await;

        println!("LLM RESPONSE: {:?}", response);

        Ok(LlmProcessResponse {})
    }
}
