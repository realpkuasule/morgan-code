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
}
