use tracing::warn;

use crate::domain::{
    entities::audio_buffer::AudioBuffer,
    ports::vad::{Vad, VadEvent},
    utils::Utils,
};

#[derive(Debug, Clone)]
pub struct LocalVadAdapter {
    frame_size: u64,
    threshold: f32,
    full_stop_bytes: u64,
}

impl LocalVadAdapter {
    pub fn new(frame_size: u64) -> Self {
        Self {
            frame_size,
            threshold: 800.0,
            full_stop_bytes: 1000,
        }
    }
}

impl Vad for LocalVadAdapter {
    fn process_audio<'a>(&mut self, audio_buffer: &'a mut AudioBuffer) -> VadEvent {
        while audio_buffer.user.len() >= (audio_buffer.cursor + self.frame_size) as usize {
            println!(
                "audio buffer = {:?}, cursor = {}",
                audio_buffer.user.len(),
                audio_buffer.cursor
            );
            let range = audio_buffer.cursor..audio_buffer.cursor + self.frame_size;
            let frame = &audio_buffer.user[range.start as usize..range.end as usize];

            let is_speech = self.is_speech(frame);

            match (is_speech, audio_buffer.start, audio_buffer.end) {
                (true, None, None) => {
                    audio_buffer.start = Some(audio_buffer.cursor);

                    // TODO handle rollback padding
                    warn!("Event SpeechStarted");
                    return VadEvent::SpeechStarted;
                }
                (true, Some(_), None) => {}
                (true, Some(_), Some(_)) => {
                    audio_buffer.end = None;
                    warn!("Event SpeechResumed");
                    return VadEvent::SpeechResumed;
                }

                (false, None, None) => {}
                (false, Some(start), None) => {
                    audio_buffer.end = Some(audio_buffer.cursor);

                    // TODO Handle min_speech_duration
                    warn!("Event SpeechPaused");
                    return VadEvent::SpeechPaused(start, audio_buffer.cursor);
                }
                (false, Some(_), Some(end)) => {
                    if (audio_buffer.cursor - end) > self.full_stop_bytes {
                        audio_buffer.start = None;
                        audio_buffer.end = None;

                        warn!("Event SpeechFullStop");
                        return VadEvent::SpeechFullStop;
                    }
                }
                _ => panic!("End cannot exists without start index"),
            }

            println!("cc");
            audio_buffer.cursor += self.frame_size;
        }

        return VadEvent::WaitingMoreChunks;
    }

    fn is_speech(&self, bytes: &[i16]) -> bool {
        let energy = Utils::rms_energy(bytes);
        return energy > self.threshold;
    }
}
