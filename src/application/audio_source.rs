use std::pin::Pin;

use anywho::Error;

use crate::{
    domain::{entities::audio_source_layer::AudioSourceLayer, ports::audio_source::AudioSource},
    infrastructure::audio_source::{
        local_source_adapter::LocalAdapter, twilio_source_adapter::TwilioAdapter,
    },
};

#[derive(Debug, Clone)]
pub enum AudioSourceList {
    Twilio(TwilioAdapter),
    Local(LocalAdapter),
}

impl AudioSource for AudioSourceList {
    async fn handle(&self, buffers: &mut AudioSourceLayer<'_>) -> Result<(), Error> {
        match self {
            AudioSourceList::Twilio(adapter) => adapter.handle(buffers).await,
            AudioSourceList::Local(adapter) => adapter.handle(buffers).await,
        }
    }

    fn send_audio(
        &self,
        bytes: &Vec<i16>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        match self {
            AudioSourceList::Twilio(adapter) => adapter.send_audio(bytes),
            AudioSourceList::Local(adapter) => adapter.send_audio(bytes),
        }
    }
}
