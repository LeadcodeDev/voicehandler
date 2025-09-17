use anywho::Error;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{
    application::stt::SttList,
    domain::{
        entities::audio_source_layer::SendAudioCallback,
        ports::stt::{Stt, SttPayload},
    },
};

#[derive(Clone)]
pub struct Pipeline {
    pub id: Uuid,
    pub generation: u64,
    pub stt: SttList,
    pub cancellation_token: CancellationToken,
    pub send_audio: SendAudioCallback,
    status_tx: Sender<PipelineStatus>,
    status_rx: Receiver<PipelineStatus>,
}

impl Pipeline {
    pub fn new(
        id: Uuid,
        generation: u64,
        stt: SttList,
        cancellation_token: CancellationToken,
        send_audio: SendAudioCallback,
    ) -> Self {
        let (status_tx, status_rx) = channel(PipelineStatus::Pending);

        Pipeline {
            id,
            generation,
            stt,
            cancellation_token,
            send_audio,
            status_tx,
            status_rx,
        }
    }

    pub fn set_status(&self, status: PipelineStatus) {
        let _ = self.status_tx.send(status);
    }

    pub async fn execute_stt(&self, bytes: &Vec<i16>) -> Result<SttPayload, Error> {
        self.stt.execute(bytes).await
    }

    pub async fn execute_llm(&self) -> Result<(), Error> {
        call_future().await.map(|_| ())
    }

    pub async fn execute_tts(&self) -> Result<(), Error> {
        call_future().await.map(|_| ())
    }

    pub async fn execute_send_audio(&self, bytes: &Vec<i16>) -> Result<(), Error> {
        let mut rx = self.status_rx.clone();

        let result = timeout(Duration::from_secs(5), async {
            loop {
                if *rx.borrow() == PipelineStatus::CanSendAudio {
                    return self.send_audio.call(&bytes).await;
                }

                rx.changed()
                    .await
                    .map_err(|_| Error::msg("status channel closed"))?;
            }
        })
        .await;

        match result {
            Ok(res) => res,
            Err(_) => Err(Error::msg("timeout waiting for CanSendAudio")),
        }
    }
}

use std::time::Duration;
use tokio::{
    sync::watch::{Receiver, Sender, channel},
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
