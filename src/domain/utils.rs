use std::io::Cursor;

use anywho::Error;
use base64::{Engine, engine::general_purpose};
use chrono::{Duration, Utc};
use hound::{WavSpec, WavWriter};
use tokio::sync::watch::{Receiver, Sender, channel};
use uuid::{NoContext, Timestamp, Uuid};

pub struct Convert;

impl Convert {
    pub fn base64_to_i16(input: &str) -> Result<Vec<i16>, Error> {
        let bytes = general_purpose::STANDARD.decode(input)?;

        if bytes.len() % 2 != 0 {
            return Err(Error::msg("Invalid PCM16 byte length"));
        }

        let mut samples = Vec::with_capacity(bytes.len() / 2);
        for chunk in bytes.chunks_exact(2) {
            let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
            samples.push(sample);
        }

        Ok(samples)
    }

    pub fn decode_ulaw_bytes(data: &[u8]) -> Vec<i16> {
        data.iter().map(|&b| Convert::ulaw_to_i16(b)).collect()
    }

    pub fn int16_8k_to_16k(input: &[i16]) -> Vec<i16> {
        let mut res = Vec::with_capacity(input.len() * 2);

        for (idx, &itm) in input.iter().enumerate() {
            res.push(itm);

            // on ajoute la moyenne avec l'échantillon suivant si dispo
            if idx + 1 < input.len() {
                let next = input[idx + 1];
                res.push(((itm as i32 + next as i32) / 2) as i16);
            }
        }

        res
    }

    pub fn ulaw_to_i16(value: u8) -> i16 {
        const BIAS: i16 = 0x84; // 132, standard µ-law bias

        let ulaw = !value;

        let sign = (ulaw & 0x80) != 0;
        let exponent = ((ulaw >> 4) & 0x07) as i16;
        let mantissa = (ulaw & 0x0F) as i16;

        // Formule standard µ-law
        let sample = ((mantissa << 3) + BIAS) << exponent;
        if sign { -sample } else { sample }
    }

    pub fn add_padding(bytes: &Vec<i16>, left: Duration, right: Duration) -> Vec<i16> {
        let bytes_left = vec![0; (left.num_seconds() * 8000) as usize];
        let bytes_right = vec![0; (right.num_seconds() * 8000) as usize];

        let mut result = bytes_left.clone();
        result.extend(bytes);
        result.extend(bytes_right);
        result
    }

    pub fn i16_to_i8(bytes: &Vec<i16>, spec: WavSpec) -> Result<Vec<u8>, Error> {
        let mut cursor = Cursor::new(Vec::new());
        let mut writer = match WavWriter::new(&mut cursor, spec) {
            Ok(writer) => writer,
            Err(err) => {
                return Err(Error::msg(format!("Error creating WavWriter: {}", err)));
            }
        };

        for sample in bytes.iter() {
            if let Err(e) = writer.write_sample(*sample) {
                eprintln!("Error writing sample Skipping \n{}", e);
                continue;
            }
        }

        let _ = writer.finalize();

        Ok(cursor.into_inner())
    }
}

pub struct Utils;

impl Utils {
    pub fn generate_uuid() -> Uuid {
        let now = Utc::now();
        let seconds: u64 = now.timestamp().try_into().unwrap_or(0);
        let timestamp = Timestamp::from_unix(NoContext, seconds, 0);

        Uuid::new_v7(timestamp)
    }

    pub fn rms_energy(samples: &[i16]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }
        let sum_sq: f64 = samples.iter().map(|&s| (s as f64).powi(2)).sum();
        (sum_sq / samples.len() as f64).sqrt() as f32
    }
}

#[derive(Clone)]
pub struct Reactive<T>
where
    T: Clone + Send + Sync + 'static,
{
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T> Reactive<T>
where
    T: Clone + Send + Sync,
{
    pub fn new(value: T) -> Self {
        let (tx, rx) = channel(value);
        Self { tx, rx }
    }

    pub async fn set(&self, value: T) -> Result<(), Error> {
        self.tx.send(value).map_err(|err| Error::from(err))
    }

    pub fn get(&self) -> T {
        self.rx.borrow().clone()
    }

    pub async fn changed(&mut self) -> Result<(), Error> {
        self.rx.changed().await.map_err(|err| Error::from(err))
    }
}
