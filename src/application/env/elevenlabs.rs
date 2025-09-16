#[derive(clap::Args, Debug, Clone)]
pub struct ElevenLabsEnv {
    #[arg(
        env = "ELEVENLABS_API_KEY",
        name = "ELEVENLABS_API_KEY",
        help = "The ElevenLabs API key"
    )]
    pub elevenlabs_api_key: String,
}
