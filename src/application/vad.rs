use crate::{
    domain::{
        entities::audio_buffer::AudioBuffer,
        ports::vad::{Vad, VadEvent},
    },
    infrastructure::vad::local_vad::LocalVadAdapter,
};

#[derive(Debug, Clone)]
pub enum VadList {
    Local(LocalVadAdapter),
}

impl Vad for VadList {
    fn process_audio(&mut self, audio_buffer: &mut AudioBuffer) -> VadEvent {
        match self {
            VadList::Local(adapter) => adapter.process_audio(audio_buffer),
        }
    }

    fn is_speech(&self, bytes: &[i16]) -> bool {
        match self {
            VadList::Local(adapter) => adapter.is_speech(bytes),
        }
    }
}
