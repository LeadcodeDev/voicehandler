use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use chrono::Utc;
use tokio::{
    select, spawn,
    sync::{Mutex, Semaphore},
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error};
use uuid::Uuid;

use crate::{
    application::{llm::LlmList, stt::SttList},
    domain::entities::{
        audio_source_layer::SendAudioCallback,
        history::{
            history::History,
            history_event::{HistoryEvent, HistoryEventPayload},
            history_member::HistoryMember,
        },
        pipeline::pipeline::Pipeline,
    },
};

#[derive(Clone)]
pub struct PoolManager {
    pub pipelines: Arc<Mutex<HashMap<Uuid, Pipeline>>>,
    semaphore: Arc<Semaphore>,
    gen_counter: Arc<AtomicU64>,
}

impl PoolManager {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            pipelines: Arc::new(Mutex::new(HashMap::new())),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            gen_counter: Arc::new(AtomicU64::new(0)),
        }
    }

    pub async fn start_pipeline(
        &self,
        id: Uuid,
        stt: SttList,
        llm: LlmList,
        bytes: Vec<i16>,
        send_audio: SendAudioCallback,
        history: &History,
    ) {
        let generation = self.gen_counter.fetch_add(1, Ordering::SeqCst) + 1;
        let cancellation_token = CancellationToken::new();

        {
            let mut map = self.pipelines.lock().await;
            if let Some(prev) = map.remove(&id) {
                prev.cancellation_token.cancel();
            }
        }

        let semaphore = Arc::clone(&self.semaphore);
        let pipelines_map = Arc::clone(&self.pipelines);

        let pipeline = Pipeline::new(
            id,
            generation,
            stt.clone(),
            llm.clone(),
            cancellation_token.clone(),
            send_audio.clone(),
        );

        let mut pipeline_clone = pipeline.clone();
        let mut history_events = history.events.clone();

        spawn(async move {
            let permit = semaphore.acquire_owned().await.expect("Semaphore closed");

            let _stt_response = select! {
                _ = cancellation_token.cancelled() => {
                    debug!("Pipeline {} cancelled before STT", id);
                    return ;
                }

                result = pipeline_clone.execute_stt(&bytes) => {
                    match result {
                        Ok(payload) => {
                            debug!("Pipeline {} STT OK", id);
                            let event = HistoryEventPayload {
                                member: HistoryMember::User,
                                content: payload.text.clone(),
                                created_at: Utc::now(),
                            };

                            history_events.push(HistoryEvent::new(event));
                            Ok(payload)
                        }
                        Err(e) => {
                            error!("Pipeline {} STT failed: {:?}", id, e);
                            Err(e)
                        }
                    }
                }
            };

            // let _ = pipeline_clone
            //     .stt
            //     .write_audio_file(format!("{}.wav", id), &bytes)
            //     .await;

            let _llm_result = select! {
                _ = cancellation_token.cancelled() => {
                    debug!("Pipeline {} cancelled before LLM call", id);
                    return;
                }

                llm_res = pipeline_clone.execute_llm(history_events) => {
                    match llm_res {
                        Ok(_) => {
                            debug!("LLM success for pipeline ({})", id);
                            Ok(())
                        },
                        Err(e) => {
                            error!("LLM result for pipeline ({}) failed: {:?}", id, e);
                            Err(e)
                        }
                    }
                }
            };

            let _tts_result = select! {
                _ = cancellation_token.cancelled() => {
                    debug!("Pipeline {} cancelled before TTS call", id);
                    return
                }

                llm_res = pipeline_clone.execute_tts() => {
                    match llm_res {
                        Ok(_) => {
                            debug!("LLM success for pipeline ({})", id);
                            Ok(())
                        },
                        Err(e) => {
                            error!("LLM result for pipeline ({}) failed: {:?}", id, e);
                            Err(e)
                        }
                    }
                }
            };

            let _ = pipeline_clone.execute_send_audio(&Vec::new()).await;

            debug!(
                "Pipeline {} gen={} finished; releasing permit",
                id, generation
            );

            let mut map = pipelines_map.lock().await;
            if let Some(entry) = map.get(&id) {
                if entry.generation == generation {
                    map.remove(&id);
                }
            }

            drop(permit);
        });

        let mut map = self.pipelines.lock().await;
        map.insert(id, pipeline.clone());
    }

    pub async fn stop_pipeline(&self, id: &Uuid) {
        let mut map = self.pipelines.lock().await;
        if let Some(pipeline) = map.remove(id) {
            pipeline.cancellation_token.cancel();
            debug!(
                "stop_pipeline: cancelled pipeline {} gen={}",
                id, pipeline.generation
            );
        }
    }

    pub async fn shutdown(&self) {
        let mut map = self.pipelines.lock().await;
        let entries: Vec<(Uuid, Pipeline)> = map.drain().collect();

        drop(map);

        for (_id, pipeline) in entries {
            pipeline.cancellation_token.cancel();
        }
    }
}
