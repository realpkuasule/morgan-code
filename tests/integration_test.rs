use morgan_code::{
    config::{Config, LLMConfig, OpenAIConfig, ToolsConfig, AgentConfig, UIConfig},
    tools::{ToolRegistry, Tool},
};
use std::sync::Arc;

#[tokio::test]
async fn test_tool_registry() {
    let registry = ToolRegistry::new();
    let tools = registry.list_tools();

    assert!(tools.contains(&"read".to_string()));
    assert!(tools.contains(&"write".to_string()));
    assert!(tools.contains(&"edit".to_string()));
    assert!(tools.contains(&"glob".to_string()));
    assert!(tools.contains(&"grep".to_string()));
    assert!(tools.contains(&"shell".to_string()));
}

#[tokio::test]
async fn test_read_tool() {
    let registry = ToolRegistry::new();
    let read_tool = registry.get("read").expect("Read tool should exist");

    assert_eq!(read_tool.name(), "read");
    assert!(!read_tool.description().is_empty());
}

#[test]
fn test_config_defaults() {
    let tools_config = ToolsConfig::default();
    assert_eq!(tools_config.shell_timeout_seconds, 120);
    assert!(tools_config.enabled.contains(&"read".to_string()));

    let agent_config = AgentConfig::default();
    assert_eq!(agent_config.max_iterations, 50);
    assert!(agent_config.enable_background_tasks);

    let ui_config = UIConfig::default();
    assert!(ui_config.show_spinner);
    assert!(ui_config.color_output);
}
