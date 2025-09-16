use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use tokio::{
    spawn,
    sync::{Mutex, Semaphore},
};
use tokio_util::sync::CancellationToken;
use tracing::debug;
use uuid::Uuid;

use crate::{application::stt::SttList, domain::entities::pipeline::pipeline::Pipeline};

struct PipelineEntry {
    generation: u64,
    token: CancellationToken,
}

#[derive(Clone)]
pub struct PoolManager {
    pipelines: Arc<Mutex<HashMap<Uuid, PipelineEntry>>>,
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

    pub async fn start_pipeline(&self, id: Uuid, stt: SttList, bytes: Vec<i16>) {
        let generation = self.gen_counter.fetch_add(1, Ordering::SeqCst) + 1;
        let cancellation_token = CancellationToken::new();

        {
            let mut map = self.pipelines.lock().await;
            if let Some(prev) = map.remove(&id) {
                prev.token.cancel();
            }
        }

        let semaphore = Arc::clone(&self.semaphore);
        let pipelines_map = Arc::clone(&self.pipelines);

        let pipeline = Pipeline::new(id, stt.clone(), cancellation_token.clone());
        spawn(async move {
            let permit = semaphore.acquire_owned().await.expect("Semaphore closed");

            let _ = pipeline.execute_stt(&bytes).await;
            let _ = pipeline.execute_llm().await;
            let _ = pipeline.execute_tts().await;

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
        map.insert(
            id,
            PipelineEntry {
                generation,
                token: cancellation_token,
            },
        );
    }

    pub async fn stop_pipeline(&self, id: &Uuid) {
        let mut map = self.pipelines.lock().await;
        if let Some(entry) = map.remove(id) {
            entry.token.cancel();
            debug!(
                "stop_pipeline: cancelled pipeline {} gen={}",
                id, entry.generation
            );
        }
    }

    pub async fn shutdown(&self) {
        let mut map = self.pipelines.lock().await;
        let entries: Vec<(Uuid, PipelineEntry)> = map.drain().collect();

        drop(map);

        for (_id, entry) in entries {
            entry.token.cancel();
        }
    }
}
