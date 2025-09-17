use crate::domain::entities::audio_buffer::AudioBuffer;

#[derive(Debug, Clone)]
pub enum VadState {
    Silence,
    Speaking,
}

#[derive(Debug)]
pub enum VadEvent {
    SpeechStarted,
    SpeechPaused(u64, u64),
    SpeechResumed,
    SpeechFullStop,
    WaitingMoreChunks,
}

pub trait Vad: Clone + Send + Sync {
    fn process_audio<'a>(&mut self, audio_buffer: &'a mut AudioBuffer) -> VadEvent;
    fn is_speech(&self, bytes: &[i16]) -> bool;
}
