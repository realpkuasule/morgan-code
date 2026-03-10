use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;
use glob::glob;
use crate::error::{MorganError, Result};
use crate::tools::tool::{Tool, ToolResult};

pub struct GlobTool;

impl GlobTool {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Deserialize)]
struct GlobParams {
    pattern: String,
}

#[async_trait]
impl Tool for GlobTool {
    fn name(&self) -> &str {
        "glob"
    }

    fn description(&self) -> &str {
        "Find files matching a glob pattern (e.g., '**/*.rs', 'src/**/*.toml')"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "The glob pattern to match files against"
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, params: serde_json::Value) -> Result<ToolResult> {
        let params: GlobParams = serde_json::from_value(params)
            .map_err(|e| MorganError::InvalidParameter(format!("Invalid parameters: {}", e)))?;

        match glob(&params.pattern) {
            Ok(paths) => {
                let mut results = Vec::new();
                for entry in paths {
                    match entry {
                        Ok(path) => results.push(path.display().to_string()),
                        Err(e) => eprintln!("Error reading path: {}", e),
                    }
                }

                if results.is_empty() {
                    Ok(ToolResult::success("No files found matching pattern".to_string()))
                } else {
                    Ok(ToolResult::success(results.join("\n")))
                }
            }
            Err(e) => Ok(ToolResult::error(format!("Invalid glob pattern: {}", e))),
        }
    }
}
