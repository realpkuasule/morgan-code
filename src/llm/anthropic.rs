use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::config::AnthropicConfig;
use crate::error::{MorganError, Result};
use super::traits::{LLMProvider, CompletionStream};
use super::types::*;

pub struct AnthropicProvider {
    client: Client,
    config: AnthropicConfig,
    model: String,
    api_key: String,
}

impl AnthropicProvider {
    pub fn new(config: AnthropicConfig, model: String, api_key: String) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            config,
            model,
            api_key,
        })
    }

    fn convert_messages(&self, messages: &[Message]) -> Vec<AnthropicMessage> {
        messages.iter().map(|m| AnthropicMessage {
            role: match m.role {
                Role::System => "system".to_string(),
                Role::User => "user".to_string(),
                Role::Assistant => "assistant".to_string(),
                Role::Tool => "tool".to_string(),
            },
            content: m.content.clone(),
        }).collect()
    }

    fn convert_tools(&self, tools: &[ToolDefinition]) -> Vec<AnthropicTool> {
        tools.iter().map(|t| AnthropicTool {
            name: t.name.clone(),
            description: t.description.clone(),
            input_schema: t.parameters.clone(),
        }).collect()
    }
}

#[async_trait]
impl LLMProvider for AnthropicProvider {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let messages = self.convert_messages(&request.messages);
        let tools = request.tools.as_ref().map(|t| self.convert_tools(t));

        let req_body = AnthropicRequest {
            model: self.model.clone(),
            messages,
            max_tokens: request.max_tokens.unwrap_or(4096),
            temperature: request.temperature,
            tools,
            stream: false,
        };

        let response = self.client
            .post(format!("{}/messages", self.config.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", &self.config.version)
            .header("content-type", "application/json")
            .json(&req_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(MorganError::LLMProvider(format!("Anthropic API error: {}", error_text)));
        }

        let anthropic_response: AnthropicResponse = response.json().await?;

        let content = anthropic_response.content
            .iter()
            .filter_map(|c| {
                if c.r#type == "text" {
                    Some(c.text.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join("");

        let tool_calls = anthropic_response.content
            .iter()
            .filter_map(|c| {
                if c.r#type == "tool_use" {
                    Some(ToolCall {
                        id: c.id.clone().unwrap_or_default(),
                        name: c.name.clone().unwrap_or_default(),
                        parameters: c.input.clone().unwrap_or(serde_json::json!({})),
                    })
                } else {
                    None
                }
            })
            .collect();

        let finish_reason = match anthropic_response.stop_reason.as_str() {
            "end_turn" => FinishReason::Stop,
            "max_tokens" => FinishReason::Length,
            "tool_use" => FinishReason::ToolCalls,
            "stop_sequence" => FinishReason::Stop,
            _ => FinishReason::Stop,
        };

        Ok(CompletionResponse {
            content,
            tool_calls,
            finish_reason,
            usage: TokenUsage {
                prompt_tokens: anthropic_response.usage.input_tokens,
                completion_tokens: anthropic_response.usage.output_tokens,
                total_tokens: anthropic_response.usage.input_tokens + anthropic_response.usage.output_tokens,
            },
        })
    }

    async fn stream(&self, request: CompletionRequest) -> Result<CompletionStream> {
        use eventsource_stream::Eventsource;
        use futures::stream::StreamExt;

        let messages = self.convert_messages(&request.messages);
        let tools = request.tools.as_ref().map(|t| self.convert_tools(t));

        let req_body = AnthropicRequest {
            model: self.model.clone(),
            messages,
            max_tokens: request.max_tokens.unwrap_or(4096),
            temperature: request.temperature,
            tools,
            stream: true,
        };

        let response = self.client
            .post(format!("{}/messages", self.config.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", &self.config.version)
            .header("content-type", "application/json")
            .header("accept", "text/event-stream")
            .json(&req_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(MorganError::LLMProvider(format!("Anthropic API error: {}", error_text)));
        }

        let stream = response
            .bytes_stream()
            .eventsource()
            .map(|event| {
                match event {
                    Ok(event) => {
                        if event.event == "message_stop" {
                            return Ok(StreamChunk {
                                content: String::new(),
                                reasoning_content: None,
                                tool_calls: vec![],
                                tool_call_chunks: vec![],
                                tool_execution_event: None,
                                finish_reason: Some(FinishReason::Stop),
                            });
                        }

                        if event.event != "content_block_delta" && event.event != "message_start" {
                            return Ok(StreamChunk {
                                content: String::new(),
                                reasoning_content: None,
                                tool_calls: vec![],
                                tool_call_chunks: vec![],
                                tool_execution_event: None,
                                finish_reason: None,
                            });
                        }

                        let delta: AnthropicStreamDelta = serde_json::from_str(&event.data)
                            .map_err(|e| MorganError::LLMProvider(format!("Parse error: {}", e)))?;

                        let mut content = String::new();
                        let mut tool_calls = vec![];
                        let mut tool_call_chunks = vec![];

                        if let Some(text_delta) = delta.delta.and_then(|d| d.text) {
                            content = text_delta;
                        } else if let Some(tool_use) = delta.delta.and_then(|d| d.tool_use) {
                            tool_call_chunks.push(ToolCallChunk {
                                index: tool_use.index.unwrap_or(0) as usize,
                                id: tool_use.id,
                                name: tool_use.name,
                                arguments: tool_use.input.map(|i| serde_json::to_string(&i).unwrap_or_default()),
                            });
                        }

                        Ok(StreamChunk {
                            content,
                            reasoning_content: None,
                            tool_calls,
                            tool_call_chunks,
                            tool_execution_event: None,
                            finish_reason: None,
                        })
                    }
                    Err(e) => Err(MorganError::LLMProvider(format!("Stream error: {}", e))),
                }
            });

        Ok(Box::pin(stream))
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<AnthropicTool>>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct AnthropicTool {
    name: String,
    description: String,
    input_schema: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    id: String,
    r#type: String,
    role: String,
    content: Vec<AnthropicResponseContent>,
    stop_reason: String,
    usage: AnthropicUsage,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponseContent {
    r#type: String,
    text: Option<String>,
    id: Option<String>,
    name: Option<String>,
    input: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct AnthropicStreamDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delta: Option<AnthropicStreamDeltaData>,
}

#[derive(Debug, Deserialize)]
struct AnthropicStreamDeltaData {
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_use: Option<AnthropicStreamToolUse>,
}

#[derive(Debug, Deserialize)]
struct AnthropicStreamToolUse {
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<serde_json::Value>,
}