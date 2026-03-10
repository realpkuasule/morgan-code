use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use crate::error::{MorganError, Result};
use crate::tools::tool::{Tool, ToolResult};

pub struct ReadTool;

impl ReadTool {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Deserialize)]
struct ReadParams {
    file_path: String,
}

#[async_trait]
impl Tool for ReadTool {
    fn name(&self) -> &str {
        "read"
    }

    fn description(&self) -> &str {
        "Read the contents of a file from the filesystem"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "The absolute path to the file to read"
                }
            },
            "required": ["file_path"]
        })
    }

    async fn execute(&self, params: serde_json::Value) -> Result<ToolResult> {
        let params: ReadParams = serde_json::from_value(params)
            .map_err(|e| MorganError::InvalidParameter(format!("Invalid parameters: {}", e)))?;

        let path = PathBuf::from(&params.file_path);

        if !path.exists() {
            return Ok(ToolResult::error(format!("File not found: {}", params.file_path)));
        }

        match tokio::fs::read_to_string(&path).await {
            Ok(content) => Ok(ToolResult::success(content)),
            Err(e) => Ok(ToolResult::error(format!("Failed to read file: {}", e))),
        }
    }
}
