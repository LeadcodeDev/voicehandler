#[derive(Debug, Clone)]
pub enum VadState {
    Silence,
    Speaking,
}

#[derive(Debug)]
pub enum VadEvent {
    Speaking,
    Silence,
    EndOfTurn,
    CalibrationDone(f32),
}

pub trait Vad: Clone + Send + Sync {
    fn add_bytes(&mut self, bytes: &Vec<i16>);
    fn take_bytes(&mut self) -> Vec<i16>;
    fn process_frame(&mut self, audio: &Vec<i16>) -> Option<VadEvent>;
}
