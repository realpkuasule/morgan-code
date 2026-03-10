pub mod types;
pub mod traits;
pub mod openai;
pub mod deepseek;

pub use types::*;
pub use traits::*;
pub use openai::OpenAIProvider;
pub use deepseek::DeepSeekProvider;

use crate::config::LLMConfig;
use crate::error::{MorganError, Result};

pub struct LLMFactory;

impl LLMFactory {
    pub fn create(config: &LLMConfig, api_key: String) -> Result<Box<dyn LLMProvider>> {
        match config.provider.as_str() {
            "openai" => {
                let openai_config = config.openai.clone()
                    .unwrap_or_else(|| crate::config::OpenAIConfig {
                        base_url: "https://api.openai.com/v1".to_string(),
                    });
                Ok(Box::new(OpenAIProvider::new(
                    openai_config,
                    config.model.clone(),
                    api_key,
                )?))
            }
            "deepseek" => {
                let deepseek_config = config.deepseek.clone()
                    .unwrap_or_else(|| crate::config::DeepSeekConfig {
                        base_url: "https://api.deepseek.com/v1".to_string(),
                    });
                Ok(Box::new(DeepSeekProvider::new(
                    deepseek_config,
                    config.model.clone(),
                    api_key,
                )?))
            }
            "anthropic" => {
                Err(MorganError::LLMProvider("Anthropic provider not yet implemented".to_string()))
            }
            "azure" => {
                Err(MorganError::LLMProvider("Azure provider not yet implemented".to_string()))
            }
            _ => Err(MorganError::LLMProvider(format!("Unknown provider: {}", config.provider))),
        }
    }
}
