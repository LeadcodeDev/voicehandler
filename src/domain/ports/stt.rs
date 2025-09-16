use anywho::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttPayload {
    pub text: Option<String>,
    pub language_code: Option<String>,
    pub language_probability: Option<f32>,
}

pub trait Stt: Clone + Send + Sync {
    fn execute(&self, audio: &Vec<i16>) -> impl Future<Output = Result<SttPayload, Error>>;
    fn write_audio_file(
        &self,
        filename: String,
        bytes: &Vec<i16>,
    ) -> impl Future<Output = Result<(), Error>>;
}
