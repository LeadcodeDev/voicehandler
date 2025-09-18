use std::{pin::Pin, sync::Arc};

use anywho::Error;
use chrono::Utc;
use tracing::warn;
use uuid::Uuid;

use crate::{
    application::{stt::SttList, vad::VadList},
    domain::{
        entities::{
            audio_buffer::AudioBuffer,
            history::history::History,
            pipeline::{pipeline::PipelineStatus, pool_manager::PoolManager},
        },
        ports::{
            stt::Stt,
            vad::{Vad, VadEvent},
        },
    },
};

pub struct AudioSourceLayer<'a> {
    pub id: Uuid,
    pub vad: &'a mut VadList,
    pub stt: SttList,
    pub pool_manager: PoolManager,
    pub history: &'a mut History,
    pub audio_buffer: &'a mut AudioBuffer,
    pub send_audio: SendAudioCallback,
}

impl AudioSourceLayer<'_> {
    pub async fn process(&mut self, pcm: &Vec<i16>) {
        self.audio_buffer.user.extend_from_slice(&pcm);

        match self.vad.process_audio(self.audio_buffer) {
            VadEvent::SpeechStarted => {
                // TODO handle user interuption
                println!("Event {:?}", VadEvent::SpeechStarted);
            }
            VadEvent::SpeechPaused(start, end) => {
                println!("Event {:?}", VadEvent::SpeechPaused(start, end));
                // let _ = self
                //     .stt
                //     .write_audio_file(
                //         format!("{}-{}.wav", self.id, Utc::now().to_string()),
                //         &self.audio_buffer.user[start as usize..end as usize].to_vec(),
                //     )
                //     .await;

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
                println!("Event {:?}", VadEvent::SpeechResumed);
                self.pool_manager.stop_pipeline(&self.id).await;
            }
            VadEvent::SpeechFullStop => {
                warn!("Event {:?}", VadEvent::SpeechFullStop);
                let map = self.pool_manager.pipelines.lock().await;
                let pipeline = map.get(&self.id);

                if let Some(pipeline) = pipeline {
                    let transcripted = pipeline.transcripted.lock().await;
                    for entry in transcripted.iter() {
                        self.history.add(entry.clone());
                    }

                    let _ = pipeline.status.set(PipelineStatus::CanSendAudio);
                }
            }
            VadEvent::WaitingMoreChunks => {
                //println!("Event {:?}", VadEvent::WaitingMoreChunks);
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
