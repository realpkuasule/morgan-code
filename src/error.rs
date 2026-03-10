use thiserror::Error;

#[derive(Error, Debug)]
pub enum MorganError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("LLM provider error: {0}")]
    LLMProvider(String),

    #[error("Tool execution error: {0}")]
    ToolExecution(String),

    #[error("Agent error: {0}")]
    Agent(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Timeout: {0}")]
    Timeout(String),
}

pub type Result<T> = std::result::Result<T, MorganError>;
