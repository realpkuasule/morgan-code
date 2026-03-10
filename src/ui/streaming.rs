use std::io::{self, Write};
use crate::llm::StreamChunk;

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
