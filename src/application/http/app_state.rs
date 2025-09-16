use tokio::sync::Mutex;

use crate::{
    application::{audio_source::AudioSourceList, stt::SttList},
    domain::entities::pipeline::pool_manager::PoolManager,
};

pub struct AppState {
    pub pool_manager: PoolManager,
    pub stt: Mutex<SttList>,
    pub audio_sources: Mutex<Vec<AudioSourceList>>,
}

impl AppState {
    pub fn new(
        pool_manager: PoolManager,
        stt: SttList,
        audio_sources: Vec<AudioSourceList>,
    ) -> Self {
        Self {
            pool_manager,
            stt: Mutex::new(stt),
            audio_sources: Mutex::new(audio_sources),
        }
    }
}
