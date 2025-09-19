use clap::Parser;

use crate::application::env::{
    aistudio::AiStudioEnv, elevenlabs::ElevenLabsEnv, logger::LoggerEnv,
};

pub mod aistudio;
pub mod elevenlabs;
pub mod logger;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[command(flatten)]
    pub elevenlabs: ElevenLabsEnv,

    #[command(flatten)]
    pub logger: LoggerEnv,

    #[command(flatten)]
    pub llm: AiStudioEnv,
}
