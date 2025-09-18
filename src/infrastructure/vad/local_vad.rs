use crate::domain::{
    entities::audio_buffer::AudioBuffer,
    ports::vad::{Vad, VadEvent},
    utils::Utils,
};
use std::cmp::max;

#[derive(Debug, Clone)]
pub struct LocalVadAdapter {
    frame_size: u64,
    threshold: f32,
    full_stop_bytes: u64,
    min_speech_bytes: u64,
}

impl LocalVadAdapter {
    pub fn new(frame_size: u64) -> Self {
        Self {
            frame_size,
            // TODO: use voice-handler data
            threshold: 800.0,
            full_stop_bytes: 64_000,
            min_speech_bytes: 16_000,
        }
    }
}

impl Vad for LocalVadAdapter {
    fn process_audio<'a>(&mut self, audio_buffer: &'a mut AudioBuffer) -> VadEvent {
        while audio_buffer.user.len() >= (audio_buffer.cursor + self.frame_size) as usize {
            let range = audio_buffer.cursor..audio_buffer.cursor + self.frame_size;
            audio_buffer.cursor += self.frame_size;
            let frame = &audio_buffer.user[range.start as usize..range.end as usize];

            let is_speech = self.is_speech(frame);

            match (is_speech, audio_buffer.start, audio_buffer.end) {
                (true, None, None) => {
                    let start = max(audio_buffer.cursor - 3 * self.frame_size, 0);
                    audio_buffer.start = Some(start);

                    return VadEvent::SpeechStarted;
                }
                (true, Some(_), None) => {}
                (true, Some(_), Some(_)) => {
                    audio_buffer.end = None;
                    return VadEvent::SpeechResumed;
                }

                (false, None, None) => {}
                (false, Some(start), None) => {
                    if audio_buffer.cursor - start >= self.min_speech_bytes {
                        let end = audio_buffer.cursor;
                        audio_buffer.end = Some(end);

                        return VadEvent::SpeechPaused(start, end);
                    }
                }
                (false, Some(_), Some(end)) => {
                    println!("cursor={} end={}", audio_buffer.cursor, end);
                    if (audio_buffer.cursor - end) > self.full_stop_bytes {
                        audio_buffer.start = None;
                        audio_buffer.end = None;

                        return VadEvent::SpeechFullStop;
                    }
                }
                _ => panic!("End cannot exists without start index"),
            }
        }

        return VadEvent::WaitingMoreChunks;
    }

    fn is_speech(&self, bytes: &[i16]) -> bool {
        let energy = Utils::rms_energy(bytes);
        return energy > self.threshold;
    }
}
