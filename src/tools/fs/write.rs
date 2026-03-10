use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use crate::error::{MorganError, Result};
use crate::tools::tool::{Tool, ToolResult};

pub struct WriteTool;

impl WriteTool {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Deserialize)]
struct WriteParams {
    file_path: String,
    content: String,
}

#[async_trait]
impl Tool for WriteTool {
    fn name(&self) -> &str {
        "write"
    }

    fn description(&self) -> &str {
        "Write content to a file, creating it if it doesn't exist"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "The absolute path to the file to write"
                },
                "content": {
                    "type": "string",
                    "description": "The content to write to the file"
                }
            },
            "required": ["file_path", "content"]
        })
    }

    async fn execute(&self, params: serde_json::Value) -> Result<ToolResult> {
        let params: WriteParams = serde_json::from_value(params)
            .map_err(|e| MorganError::InvalidParameter(format!("Invalid parameters: {}", e)))?;

        let path = PathBuf::from(&params.file_path);

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                tokio::fs::create_dir_all(parent).await
                    .map_err(|e| MorganError::ToolExecution(format!("Failed to create directory: {}", e)))?;
            }
        }

        match tokio::fs::write(&path, params.content).await {
            Ok(_) => Ok(ToolResult::success(format!("Successfully wrote to {}", params.file_path))),
            Err(e) => Ok(ToolResult::error(format!("Failed to write file: {}", e))),
        }
    }
}
