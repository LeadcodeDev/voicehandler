use std::{pin::Pin, sync::Arc};

use anywho::Error;
use uuid::Uuid;

use crate::{
    application::{http::app_state::AppState, vad::VadList},
    domain::entities::{audio_buffer::AudioBuffer, audio_source_layer::AudioSourceLayer},
};

pub struct AudioSourcePayload<'a> {
    pub id: Uuid,
    pub state: Arc<AppState>,
    pub vad: &'a mut VadList,
    pub audio_buffer: AudioBuffer,
}

pub trait AudioSource: Clone + Send + Sync {
    fn handle(&self, layer: &mut AudioSourceLayer) -> impl Future<Output = Result<(), Error>>;
    fn send_audio(
        &self,
        bytes: &Vec<i16>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
}
