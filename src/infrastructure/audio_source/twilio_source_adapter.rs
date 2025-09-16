use anywho::Error;
use base64::{Engine, engine::general_purpose};
use serde::Deserialize;
use serde_json::from_str;

use crate::domain::{
    entities::audio_source_layer::AudioSourceLayer, ports::audio_source::AudioSource,
    utils::Convert,
};

#[derive(Debug, Clone)]
pub struct TwilioAdapter;

impl TwilioAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl AudioSource for TwilioAdapter {
    async fn handle(&self, layer: &mut AudioSourceLayer<'_>) -> Result<(), Error> {
        if let Ok(envelope) = from_str::<MediaEnvelope>(&layer.audio_buffer.streamed_content) {
            if envelope.event == "media" {
                if let Ok(raw_bytes) = general_purpose::STANDARD.decode(&envelope.media.payload) {
                    let int16_8k = Convert::decode_ulaw_bytes(&raw_bytes);
                    let pcm = Convert::int16_8k_to_16k(&int16_8k);

                    layer.process(&pcm).await;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct MediaEnvelope {
    pub event: String,

    #[serde(rename = "sequenceNumber")]
    pub sequence_number: String,
    pub media: Media,

    #[serde(rename = "streamSid")]
    pub stream_sid: String,
}

#[derive(Debug, Deserialize)]
pub struct Media {
    pub track: String,
    pub chunk: String,
    pub timestamp: String,
    pub payload: String,
}
