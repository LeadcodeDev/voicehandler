use tokio::sync::Mutex;

use crate::{
    application::{audio_source::AudioSourceList, stt::SttList},
    domain::entities::pool::Pool,
};

pub struct AppState {
    pub pool_manager: Pool,
    pub stt: Mutex<SttList>,
    pub audio_sources: Mutex<Vec<AudioSourceList>>,
}

impl AppState {
    pub fn new(pool: Pool, stt: SttList, audio_sources: Vec<AudioSourceList>) -> Self {
        Self {
            pool_manager: pool,
            stt: Mutex::new(stt),
            audio_sources: Mutex::new(audio_sources),
        }
    }
}
