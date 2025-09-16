use std::mem::take;

use crate::domain::{
    ports::vad::{Vad, VadEvent, VadState},
    utils::Utils,
};

#[derive(Debug, Clone)]
pub struct LocalVadAdapter {
    state: VadState,
    buffer: Vec<i16>,
    silence_frames: u32,
    speaking_frames: u32,
    sample_rate: u32,
    frame_size: usize,
    calibrated: bool,
    calibration_frames: u32,
    noise_acc: f64,
    threshold: f32,
}

impl LocalVadAdapter {
    pub fn new(sample_rate: u32, frame_size: usize) -> Self {
        Self {
            state: VadState::Silence,
            buffer: Vec::new(),
            silence_frames: 0,
            speaking_frames: 0,
            sample_rate,
            frame_size,
            calibrated: false,
            calibration_frames: 0,
            noise_acc: 0.0,
            threshold: 800.0,
        }
    }
}

impl Vad for LocalVadAdapter {
    fn add_bytes(&mut self, bytes: &Vec<i16>) {
        self.buffer.extend_from_slice(bytes);
    }

    fn take_bytes(&mut self) -> Vec<i16> {
        take(&mut self.buffer)
    }

    fn process_frame(&mut self, samples: &Vec<i16>) -> Option<VadEvent> {
        let energy = Utils::rms_energy(samples);
        let frame_ms = (self.frame_size as f32 / self.sample_rate as f32) * 1000.0;

        // Calibration ~500ms
        if !self.calibrated {
            self.noise_acc += energy as f64;
            self.calibration_frames += 1;

            let elapsed_ms = self.calibration_frames as f32 * frame_ms;
            if elapsed_ms >= 500.0 {
                let avg_noise = self.noise_acc / self.calibration_frames as f64;
                self.threshold = (avg_noise as f32) * 3.0;
                self.calibrated = true;

                return Some(VadEvent::CalibrationDone(self.threshold));
            }

            return None;
        }

        let end_limit_ms = 1000; // 1 secondes de silence
        let min_speaking_ms = 100.0; // au moins 50ms pour considérer que ça repart

        match self.state {
            VadState::Silence => {
                if energy > self.threshold {
                    self.speaking_frames += 1;
                    let speaking_ms = self.speaking_frames as f32 * frame_ms;
                    if speaking_ms >= min_speaking_ms {
                        self.state = VadState::Speaking;
                        self.speaking_frames = 0;
                        return Some(VadEvent::Speaking);
                    }
                } else {
                    self.speaking_frames = 0;
                    return Some(VadEvent::Silence);
                }
            }
            VadState::Speaking => {
                if energy > self.threshold {
                    self.speaking_frames += 1;
                    self.silence_frames = 0;
                    return Some(VadEvent::Speaking);
                } else {
                    self.silence_frames += 1;
                    let silence_ms = self.silence_frames as f32 * frame_ms;

                    if silence_ms >= end_limit_ms as f32 {
                        self.state = VadState::Silence;
                        self.silence_frames = 0;
                        return Some(VadEvent::EndOfTurn);
                    } else {
                        return Some(VadEvent::Silence);
                    }
                }
            }
        }

        None
    }
}
