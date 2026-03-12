use std::sync::Arc;
use crate::error::{MorganError, Result};
use crate::llm::{LLMProvider, CompletionRequest, Message};
use crate::tools::ToolRegistry;
use crate::session::SessionContext;

pub struct Agent {
    llm: Box<dyn LLMProvider>,
    tools: Arc<ToolRegistry>,
    context: SessionContext,
    max_iterations: u32,
}

impl Agent {
    pub fn new(
        llm: Box<dyn LLMProvider>,
        tools: Arc<ToolRegistry>,
        max_iterations: u32,
    ) -> Self {
        Self {
            llm,
            tools,
            context: SessionContext::with_system_message(
                "You are Morgan, an AI coding assistant. You help users with programming tasks by using available tools."
            ),
            max_iterations,
        }
    }

    pub async fn run(&mut self, user_input: String) -> Result<String> {
        self.context.add_message(Message::user(user_input));

        let mut iteration = 0;

        loop {
            if iteration >= self.max_iterations {
                return Err(MorganError::Agent(
                    "Maximum iterations reached".to_string()
                ));
            }
            iteration += 1;

            let response = self.llm.complete(CompletionRequest {
                messages: self.context.messages(),
                tools: Some(self.tools.to_definitions()),
                temperature: Some(0.7),
                max_tokens: Some(4096),
                stream: false,
            }).await?;

            // If no tool calls, we're done
            if response.tool_calls.is_empty() {
                self.context.add_message(Message::assistant(response.content.clone()));
                return Ok(response.content);
            }

            // Add assistant message with tool calls
            self.context.add_message(Message::assistant_with_tool_calls(
                response.content.clone(),
                response.tool_calls.clone()
            ));

            // Execute tool calls
            for tool_call in response.tool_calls {
                let tool = self.tools.get(&tool_call.name)
                    .ok_or_else(|| MorganError::NotFound(format!("Tool not found: {}", tool_call.name)))?;

                let result = tool.execute(tool_call.parameters).await?;

                let result_message = if result.success {
                    format!("Tool '{}' result:\n{}", tool_call.name, result.output)
                } else {
                    format!("Tool '{}' error:\n{}", tool_call.name, result.error.unwrap_or_default())
                };

                self.context.add_message(Message::tool(result_message, tool_call.id));
            }
        }
    }

    pub fn clear_context(&mut self) {
        self.context.clear();
        self.context.add_message(Message::system(
            "You are Morgan, an AI coding assistant. You help users with programming tasks by using available tools."
        ));
    }

    pub async fn run_streaming<F>(&mut self, user_input: String, mut on_chunk: F) -> Result<String>
    where
        F: FnMut(&crate::llm::StreamChunk),
    {
        use futures::stream::StreamExt;

        self.context.add_message(Message::user(user_input));
        let mut iteration = 0;

        loop {
            if iteration >= self.max_iterations {
                return Err(MorganError::Agent("Maximum iterations reached".to_string()));
            }
            iteration += 1;

            let mut stream = self.llm.stream(CompletionRequest {
                messages: self.context.messages(),
                tools: Some(self.tools.to_definitions()),
                temperature: Some(0.7),
                max_tokens: Some(4096),
                stream: true,
            }).await?;

            let mut accumulated_content = String::new();
            let mut accumulated_reasoning = String::new();
            let mut tool_call_accumulator: std::collections::HashMap<usize, (String, String, String)> = std::collections::HashMap::new();
            let mut has_tool_calls = false;

            while let Some(chunk_result) = stream.next().await {
                let chunk = chunk_result?;

                accumulated_content.push_str(&chunk.content);
                if let Some(reasoning) = &chunk.reasoning_content {
                    accumulated_reasoning.push_str(reasoning);
                }

                // Accumulate tool call chunks
                for tc_chunk in &chunk.tool_call_chunks {
                    has_tool_calls = true;
                    let entry = tool_call_accumulator.entry(tc_chunk.index).or_insert((String::new(), String::new(), String::new()));

                    if let Some(id) = &tc_chunk.id {
                        if !id.is_empty() {
                            entry.0 = id.clone();
                        }
                    }
                    if let Some(name) = &tc_chunk.name {
                        if !name.is_empty() {
                            entry.1 = name.clone();
                        }
                    }
                    if let Some(args) = &tc_chunk.arguments {
                        entry.2.push_str(args);
                    }
                }

                on_chunk(&chunk);

                // Don't break on finish_reason if we're still accumulating tool calls
                // Wait for the stream to naturally end
                if chunk.finish_reason.is_some() && !chunk.tool_call_chunks.is_empty() {
                    // Continue to get remaining tool call chunks
                    continue;
                }

                if chunk.finish_reason.is_some() {
                    break;
                }
            }

            if has_tool_calls {
                // Parse accumulated tool calls
                let mut tool_calls = vec![];
                for (_, (id, name, args_str)) in tool_call_accumulator {
                    // Only try to parse if we have non-empty arguments
                    if args_str.is_empty() {
                        eprintln!("Warning: Tool call '{}' has empty arguments, skipping", name);
                        continue;
                    }

                    // Validate JSON is complete before parsing
                    let trimmed = args_str.trim();
                    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
                        eprintln!("Warning: Tool call '{}' has incomplete JSON arguments: {}", name, args_str);
                        continue;
                    }

                    match serde_json::from_str(&args_str) {
                        Ok(parameters) => {
                            tool_calls.push(crate::llm::ToolCall {
                                id,
                                name,
                                parameters,
                            });
                        }
                        Err(e) => {
                            eprintln!("Warning: Failed to parse tool call '{}' arguments: {}", name, e);
                            eprintln!("Arguments string: {}", args_str);
                            continue;
                        }
                    }
                }

                // Only proceed if we successfully parsed at least one tool call
                if tool_calls.is_empty() {
                    eprintln!("Warning: No valid tool calls parsed, treating as regular response");
                    let final_content = if !accumulated_reasoning.is_empty() {
                        format!("[Reasoning]\n{}\n\n{}", accumulated_reasoning, accumulated_content)
                    } else {
                        accumulated_content
                    };
                    self.context.add_message(Message::assistant(final_content.clone()));
                    return Ok(final_content);
                }
                self.context.add_message(Message::assistant_with_tool_calls(
                    accumulated_content.clone(),
                    tool_calls.clone()
                ));

                for tool_call in tool_calls {
                    // Emit start event
                    let start_chunk = crate::llm::StreamChunk {
                        content: String::new(),
                        reasoning_content: None,
                        tool_calls: vec![],
                        tool_call_chunks: vec![],
                        tool_execution_event: Some(crate::llm::ToolExecutionEvent::ToolCallStart {
                            name: tool_call.name.clone(),
                            parameters: tool_call.parameters.clone(),
                        }),
                        finish_reason: None,
                    };
                    on_chunk(&start_chunk);

                    // Execute tool
                    let tool = self.tools.get(&tool_call.name)
                        .ok_or_else(|| MorganError::NotFound(format!("Tool not found: {}", tool_call.name)))?;

                    let result = tool.execute(tool_call.parameters).await?;

                    // Emit end event
                    let end_chunk = crate::llm::StreamChunk {
                        content: String::new(),
                        reasoning_content: None,
                        tool_calls: vec![],
                        tool_call_chunks: vec![],
                        tool_execution_event: Some(crate::llm::ToolExecutionEvent::ToolCallEnd {
                            name: tool_call.name.clone(),
                            result: if result.success {
                                result.output.clone()
                            } else {
                                result.error.clone().unwrap_or_default()
                            },
                            success: result.success,
                        }),
                        finish_reason: None,
                    };
                    on_chunk(&end_chunk);

                    // Add to context
                    let result_message = if result.success {
                        format!("Tool '{}' result:\n{}", tool_call.name, result.output)
                    } else {
                        format!("Tool '{}' error:\n{}", tool_call.name, result.error.unwrap_or_default())
                    };

                    self.context.add_message(Message::tool(result_message, tool_call.id));
                }

                accumulated_content.clear();
                accumulated_reasoning.clear();
                continue;
            }

            let final_content = if !accumulated_reasoning.is_empty() {
                format!("[Reasoning]\n{}\n\n{}", accumulated_reasoning, accumulated_content)
            } else {
                accumulated_content
            };

            self.context.add_message(Message::assistant(final_content.clone()));
            return Ok(final_content);
        }
    }
}
