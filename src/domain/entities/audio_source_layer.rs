use chrono::Duration;
use tracing::debug;
use uuid::Uuid;

use crate::{
    application::{stt::SttList, vad::VadList},
    domain::{
        entities::{audio_buffer::AudioBuffer, pool::Pool},
        ports::vad::{Vad, VadEvent},
        utils::Convert,
    },
};

pub struct AudioSourceLayer<'a> {
    pub id: Uuid,
    pub vad: &'a mut VadList,
    pub stt: SttList,
    pub pool_manager: Pool,
    pub audio_buffer: &'a mut AudioBuffer,
}

impl AudioSourceLayer<'_> {
    pub async fn process(&mut self, pcm: &Vec<i16>) {
        if let Some(event) = self.vad.process_frame(&pcm) {
            match event {
                VadEvent::Speaking => {
                    self.audio_buffer.user.extend_from_slice(&pcm);
                    self.vad.add_bytes(&Convert::add_padding(
                        &pcm,
                        Duration::milliseconds(100),
                        Duration::milliseconds(100),
                    ));
                }
                VadEvent::Silence => {
                    self.audio_buffer
                        .user
                        .extend_from_slice(&Convert::add_padding(
                            &pcm,
                            Duration::milliseconds(100),
                            Duration::milliseconds(100),
                        ));
                }
                VadEvent::EndOfTurn => {
                    debug!("Full stop");
                    self.pool_manager.compute(self.id, self.vad.take_bytes());

                    let pool = self.pool_manager.create_pool(self.stt.clone());
                    self.pool_manager.send(pool, self.id).await;
                }
                VadEvent::CalibrationDone(th) => {
                    debug!("Calibration terminée id={} (seuil={:.2})", self.id, th);
                }
            }
        }
    }
}
