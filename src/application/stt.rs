use anywho::Error;

use crate::{
    domain::ports::stt::{Stt, SttPayload},
    infrastructure::stt::scribe_adapter::ScribeAdapter,
};

#[derive(Clone)]
pub enum SttList {
    Scribe(ScribeAdapter),
}

impl Stt for SttList {
    async fn execute(&self, bytes: &Vec<i16>) -> Result<SttPayload, Error> {
        match self {
            SttList::Scribe(adapter) => adapter.execute(bytes).await,
        }
    }

    async fn write_audio_file(&self, filename: String, bytes: &Vec<i16>) -> Result<(), Error> {
        match self {
            SttList::Scribe(adapter) => adapter.write_audio_file(filename, bytes).await,
        }
    }
}
