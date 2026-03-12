use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::config::OpenAIConfig;
use crate::error::{MorganError, Result};
use super::traits::{LLMProvider, CompletionStream};
use super::types::*;

pub struct OpenAIProvider {
    client: Client,
    config: OpenAIConfig,
    model: String,
    api_key: String,
}

impl OpenAIProvider {
    pub fn new(config: OpenAIConfig, model: String, api_key: String) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            config,
            model,
            api_key,
        })
    }

    fn convert_messages(&self, messages: &[Message]) -> Vec<OpenAIMessage> {
        messages.iter().map(|m| OpenAIMessage {
            role: match m.role {
                Role::System => "system".to_string(),
                Role::User => "user".to_string(),
                Role::Assistant => "assistant".to_string(),
                Role::Tool => "tool".to_string(),
            },
            content: m.content.clone(),
        }).collect()
    }

    fn convert_tools(&self, tools: &[ToolDefinition]) -> Vec<OpenAITool> {
        tools.iter().map(|t| OpenAITool {
            r#type: "function".to_string(),
            function: OpenAIFunction {
                name: t.name.clone(),
                description: t.description.clone(),
                parameters: t.parameters.clone(),
            },
        }).collect()
    }
}

#[async_trait]
impl LLMProvider for OpenAIProvider {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let req_body = OpenAIRequest {
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
            return Err(MorganError::LLMProvider(format!("OpenAI API error: {}", error_text)));
        }

        let openai_response: OpenAIResponse = response.json().await?;

        let choice = openai_response.choices.first()
            .ok_or_else(|| MorganError::LLMProvider("No choices in response".to_string()))?;

        let tool_calls = choice.message.tool_calls.as_ref()
            .map(|calls| calls.iter().map(|tc| ToolCall {
                id: tc.id.clone(),
                name: tc.function.name.clone(),
                parameters: serde_json::from_str(&tc.function.arguments).unwrap_or(serde_json::json!({})),
            }).collect())
            .unwrap_or_default();

        Ok(CompletionResponse {
            content: choice.message.content.clone().unwrap_or_default(),
            tool_calls,
            finish_reason: match choice.finish_reason.as_str() {
                "stop" => FinishReason::Stop,
                "length" => FinishReason::Length,
                "tool_calls" => FinishReason::ToolCalls,
                "content_filter" => FinishReason::ContentFilter,
                _ => FinishReason::Stop,
            },
            usage: TokenUsage {
                prompt_tokens: openai_response.usage.prompt_tokens,
                completion_tokens: openai_response.usage.completion_tokens,
                total_tokens: openai_response.usage.total_tokens,
            },
        })
    }

    async fn stream(&self, request: CompletionRequest) -> Result<CompletionStream> {
        use eventsource_stream::Eventsource;
        use futures::stream::StreamExt;

        let req_body = OpenAIRequest {
            model: self.model.clone(),
            messages: self.convert_messages(&request.messages),
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            tools: request.tools.as_ref().map(|t| self.convert_tools(t)),
            stream: true,
        };

        let response = self.client
            .post(format!("{}/chat/completions", self.config.base_url))
            .bearer_auth(&self.api_key)
            .json(&req_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(MorganError::LLMProvider(format!("OpenAI API error: {}", error_text)));
        }

        let stream = response
            .bytes_stream()
            .eventsource()
            .map(|event| {
                match event {
                    Ok(event) => {
                        if event.data == "[DONE]" {
                            return Ok(StreamChunk {
                                content: String::new(),
                                reasoning_content: None,
                                tool_calls: vec![],
                                tool_call_chunks: vec![],
                                tool_execution_event: None,
                                finish_reason: Some(FinishReason::Stop),
                            });
                        }

                        let chunk: OpenAIStreamChunk = serde_json::from_str(&event.data)
                            .map_err(|e| MorganError::LLMProvider(format!("Parse error: {}", e)))?;

                        if chunk.choices.is_empty() {
                            return Ok(StreamChunk {
                                content: String::new(),
                                reasoning_content: None,
                                tool_calls: vec![],
                                tool_call_chunks: vec![],
                                tool_execution_event: None,
                                finish_reason: None,
                            });
                        }

                        let delta = &chunk.choices[0].delta;

                        let tool_call_chunks = delta.tool_calls.as_ref()
                            .map(|calls| calls.iter().map(|tc| {
                                let id = tc.id.clone().unwrap_or_default();
                                let name = tc.function.as_ref()
                                    .and_then(|f| f.name.clone())
                                    .unwrap_or_default();
                                let arguments = tc.function.as_ref()
                                    .and_then(|f| f.arguments.clone())
                                    .unwrap_or_default();

                                ToolCallChunk {
                                    index: tc.index,
                                    id: if id.is_empty() { None } else { Some(id) },
                                    name: if name.is_empty() { None } else { Some(name) },
                                    arguments: if arguments.is_empty() { None } else { Some(arguments) },
                                }
                            }).collect())
                            .unwrap_or_default();

                        let finish_reason = chunk.choices[0].finish_reason.as_ref()
                            .map(|r| match r.as_str() {
                                "stop" => FinishReason::Stop,
                                "length" => FinishReason::Length,
                                "tool_calls" => FinishReason::ToolCalls,
                                "content_filter" => FinishReason::ContentFilter,
                                _ => FinishReason::Stop,
                            });

                        Ok(StreamChunk {
                            content: delta.content.clone().unwrap_or_default(),
                            reasoning_content: None,  // OpenAI doesn't have reasoning_content
                            tool_calls: vec![],
                            tool_call_chunks,
                            tool_execution_event: None,
                            finish_reason,
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
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAITool>>,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OpenAITool {
    r#type: String,
    function: OpenAIFunction,
}

#[derive(Debug, Serialize)]
struct OpenAIFunction {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
    usage: OpenAIUsage,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIResponseMessage,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponseMessage {
    content: Option<String>,
    tool_calls: Option<Vec<OpenAIToolCall>>,
}

#[derive(Debug, Deserialize)]
struct OpenAIToolCall {
    id: String,
    function: OpenAIFunctionCall,
}

#[derive(Debug, Deserialize)]
struct OpenAIFunctionCall {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamChunk {
    choices: Vec<OpenAIStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamChoice {
    delta: OpenAIStreamDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamDelta {
    content: Option<String>,
    tool_calls: Option<Vec<OpenAIStreamToolCall>>,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamToolCall {
    #[serde(default)]
    index: usize,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    function: Option<OpenAIStreamFunctionCall>,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamFunctionCall {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    arguments: Option<String>,
}
