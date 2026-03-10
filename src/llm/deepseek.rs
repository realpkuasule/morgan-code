use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::config::DeepSeekConfig;
use crate::error::{MorganError, Result};
use super::traits::{LLMProvider, CompletionStream};
use super::types::*;

pub struct DeepSeekProvider {
    client: Client,
    config: DeepSeekConfig,
    model: String,
    api_key: String,
}

impl DeepSeekProvider {
    pub fn new(config: DeepSeekConfig, model: String, api_key: String) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            config,
            model,
            api_key,
        })
    }

    fn convert_messages(&self, messages: &[Message]) -> Vec<DeepSeekMessage> {
        messages.iter().map(|m| {
            let role = match m.role {
                Role::System => "system".to_string(),
                Role::User => "user".to_string(),
                Role::Assistant => "assistant".to_string(),
                Role::Tool => "tool".to_string(),
            };

            // For assistant messages with tool calls, add empty reasoning_content
            let reasoning_content = if matches!(m.role, Role::Assistant) && m.tool_calls.is_some() {
                Some(String::new())
            } else {
                None
            };

            DeepSeekMessage {
                role,
                content: Some(m.content.clone()),
                tool_call_id: m.tool_call_id.clone(),
                tool_calls: m.tool_calls.as_ref().map(|calls| {
                    calls.iter().map(|tc| DeepSeekToolCall {
                        id: tc.id.clone(),
                        r#type: "function".to_string(),
                        function: DeepSeekFunctionCall {
                            name: tc.name.clone(),
                            arguments: serde_json::to_string(&tc.parameters).unwrap_or_default(),
                        },
                    }).collect()
                }),
                reasoning_content,
            }
        }).collect()
    }

    fn convert_tools(&self, tools: &[ToolDefinition]) -> Vec<DeepSeekTool> {
        tools.iter().map(|t| DeepSeekTool {
            r#type: "function".to_string(),
            function: DeepSeekFunction {
                name: t.name.clone(),
                description: t.description.clone(),
                parameters: t.parameters.clone(),
            },
        }).collect()
    }
}

#[async_trait]
impl LLMProvider for DeepSeekProvider {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let req_body = DeepSeekRequest {
            model: self.model.clone(),
            messages: self.convert_messages(&request.messages),
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            tools: request.tools.as_ref().map(|t| self.convert_tools(t)),
            stream: false,
        };

        let response = self.client
            .post(format!("{}/chat/completions", self.config.base_url))
            .bearer_auth(&self.api_key)
            .json(&req_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(MorganError::LLMProvider(format!("DeepSeek API error: {}", error_text)));
        }

        let deepseek_response: DeepSeekResponse = response.json().await?;

        let choice = deepseek_response.choices.first()
            .ok_or_else(|| MorganError::LLMProvider("No choices in response".to_string()))?;

        // Handle reasoning content if present
        let mut content = String::new();
        if let Some(reasoning) = &choice.message.reasoning_content {
            content.push_str(&format!("[Reasoning]\n{}\n\n", reasoning));
        }
        if let Some(main_content) = &choice.message.content {
            content.push_str(main_content);
        }

        let tool_calls = choice.message.tool_calls.as_ref()
            .map(|calls| calls.iter().map(|tc| ToolCall {
                id: tc.id.clone(),
                name: tc.function.name.clone(),
                parameters: serde_json::from_str(&tc.function.arguments).unwrap_or(serde_json::json!({})),
            }).collect())
            .unwrap_or_default();

        Ok(CompletionResponse {
            content,
            tool_calls,
            finish_reason: match choice.finish_reason.as_str() {
                "stop" => FinishReason::Stop,
                "length" => FinishReason::Length,
                "tool_calls" => FinishReason::ToolCalls,
                "content_filter" => FinishReason::ContentFilter,
                _ => FinishReason::Stop,
            },
            usage: TokenUsage {
                prompt_tokens: deepseek_response.usage.prompt_tokens,
                completion_tokens: deepseek_response.usage.completion_tokens,
                total_tokens: deepseek_response.usage.total_tokens,
            },
        })
    }

    async fn stream(&self, _request: CompletionRequest) -> Result<CompletionStream> {
        Err(MorganError::LLMProvider("Streaming not yet implemented".to_string()))
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn supports_streaming(&self) -> bool {
        false
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}

#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<DeepSeekMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<DeepSeekTool>>,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeepSeekMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<DeepSeekToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_content: Option<String>,
}

#[derive(Debug, Serialize)]
struct DeepSeekTool {
    r#type: String,
    function: DeepSeekFunction,
}

#[derive(Debug, Serialize)]
struct DeepSeekFunction {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    choices: Vec<DeepSeekChoice>,
    usage: DeepSeekUsage,
}

#[derive(Debug, Deserialize)]
struct DeepSeekChoice {
    message: DeepSeekResponseMessage,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponseMessage {
    content: Option<String>,
    #[serde(rename = "reasoning_content")]
    reasoning_content: Option<String>,
    tool_calls: Option<Vec<DeepSeekToolCall>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeepSeekToolCall {
    id: String,
    r#type: String,
    function: DeepSeekFunctionCall,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeepSeekFunctionCall {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct DeepSeekUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}
