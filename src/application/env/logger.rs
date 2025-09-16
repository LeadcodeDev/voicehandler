use clap::ValueEnum;
use std::fmt::Display;

#[derive(clap::Args, Debug, Clone)]
pub struct LoggerEnv {
    #[arg(
        env = "LOG_LEVEL",
        name = "LOG_LEVEL",
        help = "The log level used in the application"
    )]
    pub level: LogLevel,

    #[arg(
        env = "LOG_PRETTIFY",
        name = "LOG_PRETTIFY",
        help = "Whether to prettify the log output",
        action = clap::ArgAction::Set
    )]
    pub prettify: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum, Default)]
pub enum LogLevel {
    #[default]
    Info,
    Debug,
}

impl From<String> for LogLevel {
    fn from(value: String) -> Self {
        match value.as_str() {
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            _ => LogLevel::Info, // Default to Info if unknown
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "info"),
            LogLevel::Debug => write!(f, "debug"),
        }
    }
}
