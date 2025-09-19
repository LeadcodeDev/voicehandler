use tokio::sync::Mutex;

use crate::{
    application::{audio_source::AudioSourceList, llm::LlmList, stt::SttList},
    domain::entities::pipeline::pool_manager::PoolManager,
};

pub struct AppState {
    pub pool_manager: PoolManager,
    pub stt: Mutex<SttList>,
    pub audio_sources: Mutex<Vec<AudioSourceList>>,
    pub llms: Mutex<Vec<LlmList>>,
}

impl AppState {
    pub fn new(
        pool_manager: PoolManager,
        stt: SttList,
        audio_sources: Vec<AudioSourceList>,
        llms: Vec<LlmList>,
    ) -> Self {
        Self {
            pool_manager,
            stt: Mutex::new(stt),
            audio_sources: Mutex::new(audio_sources),
            llms: Mutex::new(llms),
        }
    }
}
