use std::pin::Pin;

use anywho::Error;
use serde::Deserialize;
use serde_json::from_str;

use crate::domain::{
    entities::audio_source_layer::AudioSourceLayer, ports::audio_source::AudioSource,
};

#[derive(Debug, Clone)]
pub struct LocalAdapter;

impl LocalAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl AudioSource for LocalAdapter {
    async fn handle(&self, layer: &mut AudioSourceLayer<'_>) -> Result<(), Error> {
        if let Ok(body) = from_str::<Message>(&layer.audio_buffer.streamed_content) {
            if body.event == "media" {
                let pcm = body.content.clone();
                layer.process(&pcm).await;
            }
        }

        Ok(())
    }

    fn send_audio(
        &self,
        bytes: &Vec<i16>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let bytes = bytes.clone();
        Box::pin(async move {
            println!("Local send_audio with {} samples", bytes.len());
            Ok(())
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Message {
    pub event: String,
    pub content: Vec<i16>,
}
