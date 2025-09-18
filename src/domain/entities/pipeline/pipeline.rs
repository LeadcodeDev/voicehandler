use anywho::Error;
use chrono::Utc;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{
    application::stt::SttList,
    domain::{
        entities::{
            audio_source_layer::SendAudioCallback,
            history::{history_event::HistoryEventPayload, history_member::HistoryMember},
        },
        ports::stt::{Stt, SttPayload},
        utils::Reactive,
    },
};

#[derive(Clone)]
pub struct Pipeline {
    pub id: Uuid,
    pub generation: u64,
    pub stt: SttList,
    pub cancellation_token: CancellationToken,
    pub send_audio: SendAudioCallback,
    pub status: Reactive<PipelineStatus>,
    pub transcripted: Arc<Mutex<Vec<HistoryEventPayload>>>,
}

impl Pipeline {
    pub fn new(
        id: Uuid,
        generation: u64,
        stt: SttList,
        cancellation_token: CancellationToken,
        send_audio: SendAudioCallback,
    ) -> Self {
        Pipeline {
            id,
            generation,
            stt,
            cancellation_token,
            send_audio,
            status: Reactive::new(PipelineStatus::Pending),
            transcripted: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn execute_stt(&mut self, bytes: &Vec<i16>) -> Result<SttPayload, Error> {
        let result = self.stt.execute(bytes).await?;

        let mut transcripted = self.transcripted.lock().await;
        transcripted.push(HistoryEventPayload {
            member: HistoryMember::User,
            content: result.text.clone(),
            created_at: Utc::now(),
        });

        Ok(result)
    }

    pub async fn execute_llm(&self) -> Result<(), Error> {
        call_future().await.map(|_| ())
    }

    pub async fn execute_tts(&self) -> Result<(), Error> {
        call_future().await.map(|_| ())
    }

    pub async fn execute_send_audio(&mut self, bytes: &Vec<i16>) -> Result<(), Error> {
        let result = timeout(Duration::from_secs(5), async {
            loop {
                if self.status.get() == PipelineStatus::CanSendAudio {
                    return self.send_audio.call(&bytes).await;
                }

                self.status.changed().await?
            }
        })
        .await;

        match result {
            Ok(res) => res,
            Err(_) => Err(Error::msg("timeout waiting for CanSendAudio")),
        }
    }
}

use std::{sync::Arc, time::Duration};
use tokio::{
    sync::Mutex,
    time::{sleep, timeout},
};

async fn call_future() -> Result<String, Error> {
    sleep(Duration::from_millis(150)).await;
    Ok("CALL_SERVICE_X".to_string())
}

#[derive(Clone, PartialEq)]
pub enum PipelineStatus {
    Pending,
    Running,
    CanSendAudio,
}
