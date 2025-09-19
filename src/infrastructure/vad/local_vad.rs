use crate::domain::{
    entities::audio_buffer::AudioBuffer,
    ports::vad::{Vad, VadEvent},
    utils::{Utils, convert::Convert},
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
    pub fn new() -> Self {
        Self {
            threshold: 800.0,
            frame_size: Convert::ms_to_int16(32),
            full_stop_bytes: Convert::ms_to_int16(2000),
            min_speech_bytes: Convert::ms_to_int16(200),
        }
    }
}

impl Vad for LocalVadAdapter {
    fn process_audio<'a>(&mut self, audio_buffer: &'a mut AudioBuffer) -> VadEvent {
        while audio_buffer.user.len() as u64 >= (audio_buffer.cursor + self.frame_size) {
            let range = audio_buffer.cursor..audio_buffer.cursor + self.frame_size;
            audio_buffer.cursor += self.frame_size;
            let frame = &audio_buffer.user[range.start as usize..range.end as usize];

            let is_speech = self.is_speech(frame);

            match (is_speech, audio_buffer.start, audio_buffer.end) {
                (true, None, None) => {
                    // speech started for the first time this turn
                    let start = max(audio_buffer.cursor - 3 * self.frame_size, 0);
                    audio_buffer.start = Some(start);

                    return VadEvent::SpeechStarted;
                }
                (true, Some(_), None) => {} // speech is continuing no pause yet
                (true, Some(_), Some(_)) => {
                    // speech has paused but the user resume speacking
                    audio_buffer.end = None;
                    return VadEvent::SpeechResumed;
                }
                (false, None, None) => {} // the user still did not talk this turn
                (false, Some(start), None) => {
                    // the user paused a pipeline shall start
                    if audio_buffer.cursor - start >= self.min_speech_bytes {
                        let end = audio_buffer.cursor;
                        audio_buffer.end = Some(end);

                        //return VadEvent::SpeechPaused(start, end);
                    }
                }
                (false, Some(start), Some(end)) => {
                    // the user is still pausing it may be a full stop
                    if audio_buffer.cursor - end > self.full_stop_bytes {
                        audio_buffer.start = None;
                        audio_buffer.end = None;

                        return VadEvent::SpeechFullStop;
                    }

                    if (audio_buffer.cursor - end) >= 2 * self.frame_size {
                        return VadEvent::SpeechPaused(start, end + 2 * self.frame_size);
                    } // letting 3 trailling frames after a pause
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
