use std::io::{self, Write};
use crate::llm::{StreamChunk, ToolExecutionEvent};

pub struct StreamingOutput {
    reasoning_buffer: String,
    content_buffer: String,
    in_reasoning: bool,
}

impl StreamingOutput {
    pub fn new() -> Self {
        Self {
            reasoning_buffer: String::new(),
            content_buffer: String::new(),
            in_reasoning: false,
        }
    }

    pub fn handle_chunk(&mut self, chunk: &StreamChunk) {
        // Handle tool execution events
        if let Some(event) = &chunk.tool_execution_event {
            match event {
                ToolExecutionEvent::ToolCallStart { name, parameters } => {
                    // End reasoning mode if active
                    if self.in_reasoning {
                        print!("\x1b[0m\n");
                        self.in_reasoning = false;
                    }

                    // Display tool call start
                    print!("\n\x1b[36m[Tool: {}]\x1b[0m", name);

                    // Pretty print parameters if they're small enough
                    if let Ok(params_str) = serde_json::to_string_pretty(parameters) {
                        if params_str.len() < 200 {
                            print!("\n\x1b[90m{}\x1b[0m", params_str);
                        }
                    }
                    println!();
                    io::stdout().flush().unwrap();
                }
                ToolExecutionEvent::ToolCallEnd { name, result, success } => {
                    // Display result with color coding
                    if *success {
                        print!("\x1b[32m[✓ {}]\x1b[0m", name);
                    } else {
                        print!("\x1b[31m[✗ {}]\x1b[0m", name);
                    }

                    // Display truncated result
                    let display_result = if result.len() > 500 {
                        format!("{}... ({} bytes)", &result[..500], result.len())
                    } else {
                        result.clone()
                    };

                    if !display_result.is_empty() {
                        println!("\n\x1b[90m{}\x1b[0m", display_result);
                    } else {
                        println!();
                    }
                    io::stdout().flush().unwrap();
                }
            }
        }

        // Handle reasoning content (display in dark gray)
        if let Some(reasoning) = &chunk.reasoning_content {
            if !self.in_reasoning && !reasoning.is_empty() {
                print!("\x1b[90m");  // Dark gray
                self.in_reasoning = true;
            }
            print!("{}", reasoning);
            io::stdout().flush().unwrap();
            self.reasoning_buffer.push_str(reasoning);
        }

        // Handle normal content
        if !chunk.content.is_empty() {
            if self.in_reasoning {
                print!("\x1b[0m\n\n");  // Reset color, add spacing
                self.in_reasoning = false;
            }
            print!("{}", chunk.content);
            io::stdout().flush().unwrap();
            self.content_buffer.push_str(&chunk.content);
        }

        // Handle finish
        if chunk.finish_reason.is_some() {
            if self.in_reasoning {
                print!("\x1b[0m");  // Reset color
            }
            println!();
        }
    }
}
