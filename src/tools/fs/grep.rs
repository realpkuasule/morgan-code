use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;
use std::path::Path;
use walkdir::WalkDir;
use crate::error::{MorganError, Result};
use crate::tools::tool::{Tool, ToolResult};

pub struct GrepTool;

impl GrepTool {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Deserialize)]
struct GrepParams {
    pattern: String,
    path: Option<String>,
}

#[async_trait]
impl Tool for GrepTool {
    fn name(&self) -> &str {
        "grep"
    }

    fn description(&self) -> &str {
        "Search for a pattern in files (recursive text search)"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "The text pattern to search for"
                },
                "path": {
                    "type": "string",
                    "description": "The directory or file to search in (defaults to current directory)"
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, params: serde_json::Value) -> Result<ToolResult> {
        let params: GrepParams = serde_json::from_value(params)
            .map_err(|e| MorganError::InvalidParameter(format!("Invalid parameters: {}", e)))?;

        let search_path = params.path.unwrap_or_else(|| ".".to_string());
        let path = Path::new(&search_path);

        if !path.exists() {
            return Ok(ToolResult::error(format!("Path not found: {}", search_path)));
        }

        let mut results = Vec::new();

        for entry in WalkDir::new(path).max_depth(10) {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            if !entry.file_type().is_file() {
                continue;
            }

            let file_path = entry.path();

            // Skip binary files and common ignore patterns
            if let Some(ext) = file_path.extension() {
                let ext_str = ext.to_string_lossy();
                if matches!(ext_str.as_ref(), "png" | "jpg" | "jpeg" | "gif" | "pdf" | "zip" | "tar" | "gz") {
                    continue;
                }
            }

            if let Ok(content) = std::fs::read_to_string(file_path) {
                for (line_num, line) in content.lines().enumerate() {
                    if line.contains(&params.pattern) {
                        results.push(format!("{}:{}: {}", file_path.display(), line_num + 1, line.trim()));
                    }
                }
            }
        }

        if results.is_empty() {
            Ok(ToolResult::success("No matches found".to_string()))
        } else {
            Ok(ToolResult::success(results.join("\n")))
        }
    }
}
