use std::collections::HashMap;

use tokio::{
    spawn,
    sync::mpsc::{Sender, channel},
};
use uuid::Uuid;

use crate::{
    application::stt::SttList,
    domain::{
        entities::job::{Job, JobState},
        ports::stt::Stt,
    },
};

#[derive(Clone)]
pub struct Pool {
    pub jobs: HashMap<Uuid, Job>,
}

impl Pool {
    pub fn new() -> Self {
        Self {
            jobs: HashMap::new(),
        }
    }

    pub fn compute(&mut self, id: Uuid, bytes: Vec<i16>) {
        self.jobs.insert(
            id,
            Job {
                id,
                data: bytes,
                state: JobState::Pending,
            },
        );
    }

    pub async fn send(&self, pool: Sender<Job>, id: Uuid) {
        if let Some(job) = self.jobs.get(&id).cloned() {
            let _ = pool.send(job).await;
        }
    }

    pub fn create_pool(&mut self, stt: SttList) -> Sender<Job> {
        let (tx, mut rx) = channel::<Job>(100);
        println!("Pool created");
        let stt = stt.clone();

        spawn(async move {
            while let Some(job) = rx.recv().await {
                let _ = stt.execute(&job.data).await;
                let _ = stt
                    .write_audio_file(format!("job_{}.wav", job.id), &job.data)
                    .await;
            }
        });

        tx
    }
}
