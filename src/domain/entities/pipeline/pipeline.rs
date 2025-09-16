use anywho::Error;
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error};
use uuid::Uuid;

use crate::{application::stt::SttList, domain::ports::stt::Stt};

pub struct Pipeline {
    pub id: Uuid,
    pub stt: SttList,
    pub cancellation_token: CancellationToken,
}

impl Pipeline {
    pub fn new(id: Uuid, stt: SttList, cancellation_token: CancellationToken) -> Self {
        Pipeline {
            id,
            stt,
            cancellation_token,
        }
    }

    pub async fn execute_stt(&self, bytes: &Vec<i16>) -> Result<Option<String>, Error> {
        select! {
            _ = self.cancellation_token.cancelled() => {
                debug!("Pipeline {} cancelled before STT", self.id);
                Err(Error::msg("Pipeline cancelled before STT"))
            }

            result = self.stt.execute(bytes) => {
                match result {
                    Ok(payload) => {
                        debug!("Pipeline {} STT OK", self.id);
                        Ok(payload.text)
                    }
                    Err(e) => {
                        error!("Pipeline {} STT failed: {:?}", self.id, e);
                        Err(e)
                    }
                }
            }
        }
    }

    pub async fn execute_llm(&self) -> Result<(), Error> {
        select! {
            _ = self.cancellation_token.cancelled() => {
                debug!("Pipeline {} cancelled before LLM call", self.id);
                Err(Error::msg("Pipeline cancelled before LLM call"))
            }

            llm_res = call_future() => {
                match llm_res {
                    Ok(_) => {
                        println!("Pipeline {}", self.id);
                        Ok(())
                    },
                    Err(e) => {
                        eprintln!("Pipeline {} LLM error: {:?}", self.id, e);
                        Err(Error::msg("Pipeline LLM error"))
                    }
                }
            }
        }
    }

    pub async fn execute_tts(&self) -> Result<(), Error> {
        select! {
            _ = self.cancellation_token.cancelled() => {
                debug!("Pipeline {} cancelled before TTS", self.id);
                Err(Error::msg("Pipeline cancelled before TTS"))
            }

            llm_res = call_future() => {
                match llm_res {
                    Ok(_) => {
                        println!("Pipeline {}", self.id);
                        Ok(())
                    },
                    Err(e) => {
                        eprintln!("Pipeline {} TTS error: {:?}", self.id, e);
                        Err(Error::msg("Pipeline TTS error"))
                    }
                }
            }
        }
    }
}

use std::time::Duration;
use tokio::time::sleep;

async fn call_future() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    sleep(Duration::from_millis(150)).await;
    Ok("CALL_SERVICE_X".to_string())
}
