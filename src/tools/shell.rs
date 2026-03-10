use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;
use tokio::process::Command;
use tokio::time::{timeout, Duration};
use crate::error::{MorganError, Result};
use super::tool::{Tool, ToolResult};

pub struct ShellTool {
    timeout_seconds: u64,
}

impl ShellTool {
    pub fn new(timeout_seconds: u64) -> Self {
        Self { timeout_seconds }
    }
}

#[derive(Debug, Deserialize)]
struct ShellParams {
    command: String,
}

#[async_trait]
impl Tool for ShellTool {
    fn name(&self) -> &str {
        "shell"
    }

    fn description(&self) -> &str {
        "Execute a shell command and return its output"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The shell command to execute"
                }
            },
            "required": ["command"]
        })
    }

    async fn execute(&self, params: serde_json::Value) -> Result<ToolResult> {
        let params: ShellParams = serde_json::from_value(params)
            .map_err(|e| MorganError::InvalidParameter(format!("Invalid parameters: {}", e)))?;

        let command_future = Command::new("sh")
            .arg("-c")
            .arg(&params.command)
            .output();

        let result = timeout(Duration::from_secs(self.timeout_seconds), command_future).await;

        match result {
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                if output.status.success() {
                    let mut result_text = stdout.to_string();
                    if !stderr.is_empty() {
                        result_text.push_str("\nStderr:\n");
                        result_text.push_str(&stderr);
                    }
                    Ok(ToolResult::success(result_text))
                } else {
                    Ok(ToolResult::error(format!(
                        "Command failed with exit code {:?}\nStdout: {}\nStderr: {}",
                        output.status.code(),
                        stdout,
                        stderr
                    )))
                }
            }
            Ok(Err(e)) => Ok(ToolResult::error(format!("Failed to execute command: {}", e))),
            Err(_) => Ok(ToolResult::error(format!(
                "Command timed out after {} seconds",
                self.timeout_seconds
            ))),
        }
    }
}
