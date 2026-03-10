use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use crate::error::{MorganError, Result};
use crate::tools::tool::{Tool, ToolResult};

pub struct EditTool;

impl EditTool {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Deserialize)]
struct EditParams {
    file_path: String,
    old_string: String,
    new_string: String,
}

#[async_trait]
impl Tool for EditTool {
    fn name(&self) -> &str {
        "edit"
    }

    fn description(&self) -> &str {
        "Replace a specific string in a file with a new string"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "The absolute path to the file to edit"
                },
                "old_string": {
                    "type": "string",
                    "description": "The string to replace"
                },
                "new_string": {
                    "type": "string",
                    "description": "The replacement string"
                }
            },
            "required": ["file_path", "old_string", "new_string"]
        })
    }

    async fn execute(&self, params: serde_json::Value) -> Result<ToolResult> {
        let params: EditParams = serde_json::from_value(params)
            .map_err(|e| MorganError::InvalidParameter(format!("Invalid parameters: {}", e)))?;

        let path = PathBuf::from(&params.file_path);

        if !path.exists() {
            return Ok(ToolResult::error(format!("File not found: {}", params.file_path)));
        }

        let content = tokio::fs::read_to_string(&path).await
            .map_err(|e| MorganError::ToolExecution(format!("Failed to read file: {}", e)))?;

        if !content.contains(&params.old_string) {
            return Ok(ToolResult::error("Old string not found in file".to_string()));
        }

        let new_content = content.replace(&params.old_string, &params.new_string);

        match tokio::fs::write(&path, new_content).await {
            Ok(_) => Ok(ToolResult::success(format!("Successfully edited {}", params.file_path))),
            Err(e) => Ok(ToolResult::error(format!("Failed to write file: {}", e))),
        }
    }
}
