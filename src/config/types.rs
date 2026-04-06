use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use crate::error::{MorganError, Result};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub llm: LLMConfig,
    #[serde(default)]
    pub tools: ToolsConfig,
    #[serde(default)]
    pub agent: AgentConfig,
    #[serde(default)]
    pub ui: UIConfig,
    #[serde(default)]
    pub project: ProjectConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ProjectConfig {
    /// Project root directory (current working directory by default)
    #[serde(default)]
    pub project_root: Option<PathBuf>,

    /// Morgan Code home directory (for config, cache, history)
    #[serde(default)]
    pub morgan_home: Option<PathBuf>,

    /// Auto-detect project root by looking for common project files
    #[serde(default = "default_auto_detect")]
    pub auto_detect_root: bool,

    /// Show file origin labels (Project vs Morgan Code)
    #[serde(default = "default_true")]
    pub show_file_origin: bool,
}

fn default_auto_detect() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LLMConfig {
    pub provider: String,
    pub model: String,
    pub api_key_env: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub openai: Option<OpenAIConfig>,
    pub anthropic: Option<AnthropicConfig>,
    pub azure: Option<AzureConfig>,
    pub deepseek: Option<DeepSeekConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenAIConfig {
    #[serde(default = "default_openai_base_url")]
    pub base_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicConfig {
    #[serde(default = "default_anthropic_base_url")]
    pub base_url: String,
    #[serde(default = "default_anthropic_version")]
    pub version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AzureConfig {
    pub endpoint: String,
    pub deployment: String,
    pub api_version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeepSeekConfig {
    #[serde(default = "default_deepseek_base_url")]
    pub base_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolsConfig {
    #[serde(default = "default_enabled_tools")]
    pub enabled: Vec<String>,
    #[serde(default = "default_shell_timeout")]
    pub shell_timeout_seconds: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentConfig {
    #[serde(default = "default_max_iterations")]
    pub max_iterations: u32,
    #[serde(default = "default_true")]
    pub enable_background_tasks: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UIConfig {
    #[serde(default = "default_true")]
    pub show_spinner: bool,
    #[serde(default = "default_true")]
    pub color_output: bool,
    #[serde(default = "default_ui_mode")]
    pub mode: UIMode,
    #[serde(default = "default_true")]
    pub enable_syntax_highlighting: bool,
    #[serde(default = "default_true")]
    pub show_line_numbers: bool,
    #[serde(default = "default_theme")]
    pub theme: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UIMode {
    Tui,
    Repl,
}

impl Default for ToolsConfig {
    fn default() -> Self {
        Self {
            enabled: default_enabled_tools(),
            shell_timeout_seconds: default_shell_timeout(),
        }
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_iterations: default_max_iterations(),
            enable_background_tasks: true,
        }
    }
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            show_spinner: true,
            color_output: true,
            mode: default_ui_mode(),
            enable_syntax_highlighting: true,
            show_line_numbers: true,
            theme: default_theme(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .map_err(|e| MorganError::Config(format!("Failed to read config file: {}", e)))?;
            toml::from_str(&content)
                .map_err(|e| MorganError::Config(format!("Failed to parse config: {}", e)))
        } else {
            Err(MorganError::Config(format!(
                "Config file not found at {}. Please create one.",
                config_path.display()
            )))
        }
    }

    pub fn config_path() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| MorganError::Config("Cannot find home directory".to_string()))?;
        Ok(home.join(".morgan-code").join("config.toml"))
    }

    pub fn get_api_key(&self) -> Result<String> {
        std::env::var(&self.llm.api_key_env)
            .map_err(|_| MorganError::Config(format!(
                "Environment variable {} not set",
                self.llm.api_key_env
            )))
    }
}

fn default_openai_base_url() -> String {
    "https://api.openai.com/v1".to_string()
}

fn default_anthropic_base_url() -> String {
    "https://api.anthropic.com/v1".to_string()
}

fn default_anthropic_version() -> String {
    "2023-06-01".to_string()
}

fn default_deepseek_base_url() -> String {
    "https://api.deepseek.com/v1".to_string()
}

fn default_enabled_tools() -> Vec<String> {
    vec![
        "read".to_string(),
        "write".to_string(),
        "edit".to_string(),
        "glob".to_string(),
        "grep".to_string(),
        "shell".to_string(),
    ]
}

fn default_shell_timeout() -> u64 {
    120
}

fn default_max_iterations() -> u32 {
    50
}

fn default_true() -> bool {
    true
}

fn default_ui_mode() -> UIMode {
    UIMode::Tui
}

fn default_theme() -> String {
    "base16-ocean.dark".to_string()
}
