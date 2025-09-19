#[derive(clap::Args, Debug, Clone)]
pub struct AiStudioEnv {
    #[arg(
        env = "LLM_AISTUDIO_GOOGLE_API_KEY",
        name = "LLM_AISTUDIO_GOOGLE_API_KEY",
        help = "The AI Studio API key"
    )]
    pub aistudio_api_key: String,

    #[arg(
        env = "LLM_AISTUDIO_BASE_URL",
        name = "LLM_AISTUDIO_BASE_URL",
        help = "The AI Studio base URL"
    )]
    pub aistudio_base_url: String,
}
