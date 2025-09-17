use std::{pin::Pin, sync::Arc};

use anywho::Error;
use chrono::Duration;
use tracing::debug;
use uuid::Uuid;

use crate::{
    application::{stt::SttList, vad::VadList},
    domain::{
        entities::{
            audio_buffer::AudioBuffer,
            pipeline::{pipeline::PipelineStatus, pool_manager::PoolManager},
        },
        ports::vad::{Vad, VadEvent},
        utils::Convert,
    },
};

pub struct AudioSourceLayer<'a> {
    pub id: Uuid,
    pub vad: &'a mut VadList,
    pub stt: SttList,
    pub pool_manager: PoolManager,
    pub audio_buffer: &'a mut AudioBuffer,
    pub send_audio: SendAudioCallback,
}

impl AudioSourceLayer<'_> {
    pub async fn process(&mut self, pcm: &Vec<i16>) {
        self.audio_buffer.user.extend_from_slice(&pcm);

        loop {
            match self.vad.process_audio(self.audio_buffer) {
                VadEvent::SpeechStarted => {
                    // TODO handle user interuption
                }
                VadEvent::SpeechPaused(start, end) => {
                    self.pool_manager
                        .start_pipeline(
                            self.id,
                            self.stt.clone(),
                            self.audio_buffer.user[start as usize..end as usize].to_vec(),
                            self.send_audio.clone(),
                        )
                        .await;
                }
                VadEvent::SpeechResumed => {
                    self.pool_manager.stop_pipeline(&self.id).await;
                }
                VadEvent::SpeechFullStop => {
                    let pipeline = self.pool_manager.get_pipeline(&self.id).await;
                    if let Some(pipeline) = pipeline {
                        let _ = pipeline.set_status(PipelineStatus::CanSendAudio);
                    }
                }
                VadEvent::WaitingMoreChunks => {}
            }
        }
    }
}

pub type SendAudioCallbackFnReturn = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
pub type SendAudioCallbackFn =
    dyn Fn(&Vec<i16>) -> SendAudioCallbackFnReturn + Send + Sync + 'static;

#[derive(Clone)]
pub struct SendAudioCallback {
    inner: Arc<dyn Fn(&Vec<i16>) -> SendAudioCallbackFnReturn + Send + Sync>,
}

impl SendAudioCallback {
    pub fn new<F, Fut>(f: F) -> Self
    where
        F: Fn(&Vec<i16>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), Error>> + Send + 'static,
    {
        Self {
            inner: Arc::new(move |bytes| Box::pin(f(bytes))),
        }
    }

    pub fn call(&self, bytes: &Vec<i16>) -> SendAudioCallbackFnReturn {
        (self.inner)(bytes)
    }
}
