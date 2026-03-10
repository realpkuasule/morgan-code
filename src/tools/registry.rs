use std::collections::HashMap;
use std::sync::Arc;
use crate::llm::ToolDefinition;
use super::tool::Tool;

pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };
        registry.register_default_tools();
        registry
    }

    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }

    pub fn to_definitions(&self) -> Vec<ToolDefinition> {
        self.tools
            .values()
            .map(|t| ToolDefinition {
                name: t.name().to_string(),
                description: t.description().to_string(),
                parameters: t.parameters_schema(),
            })
            .collect()
    }

    pub fn list_tools(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
