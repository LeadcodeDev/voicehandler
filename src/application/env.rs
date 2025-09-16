use clap::Parser;

use crate::application::env::{elevenlabs::ElevenLabsEnv, logger::LoggerEnv};

pub mod elevenlabs;
pub mod logger;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[command(flatten)]
    pub elevenlabs: ElevenLabsEnv,

    #[command(flatten)]
    pub logger: LoggerEnv,
}
