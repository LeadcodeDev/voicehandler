use anywho::Error;
use elevenlabs_stt::{ElevenLabsSTTClient, STTResponse, models::elevanlabs_models::SCRIBE_V1};
use hound::{SampleFormat, WavSpec, WavWriter};

use crate::domain::{
    ports::stt::{Stt, SttPayload},
    utils::Convert,
};

#[derive(Clone)]
pub struct ScribeAdapter {
    elevenlab_client: ElevenLabsSTTClient,
    spec: WavSpec,
}

impl ScribeAdapter {
    pub fn new(api_key: String) -> Self {
        ScribeAdapter {
            elevenlab_client: ElevenLabsSTTClient::new(api_key),
            spec: WavSpec {
                channels: 1,
                sample_rate: 16000,
                bits_per_sample: 16,
                sample_format: SampleFormat::Int,
            },
        }
    }
}

impl Stt for ScribeAdapter {
    async fn execute(&self, bytes: &Vec<i16>) -> Result<SttPayload, Error> {
        let bytes = Convert::i16_to_i8(bytes, self.spec)?;

        let response = self
            .elevenlab_client
            .speech_to_text(bytes)
            .model(SCRIBE_V1)
            .language_code("fra")
            .diarize(true)
            .execute()
            .await
            .map_err(|err| Error::msg(err.to_string()))
            .map(SttPayload::from);

        println!("Response: {:?}", response);

        response
    }

    async fn write_audio_file(&self, filename: String, bytes: &Vec<i16>) -> Result<(), Error> {
        let mut writer = WavWriter::create(filename, self.spec)?;
        for sample in bytes {
            writer.write_sample(*sample)?;
        }

        let _ = writer.finalize();
        Ok(())
    }
}

impl From<STTResponse> for SttPayload {
    fn from(response: STTResponse) -> Self {
        SttPayload {
            text: response.text,
            language_code: response.language_code,
            language_probability: response.language_probability,
        }
    }
}
