#[derive(Debug, Clone)]
pub struct TUIState {
    pub messages: Vec<ChatMessage>,
    pub input_buffer: String,
    pub input_cursor: usize,
    pub chat_scroll_offset: usize,
    pub tool_scroll_offset: usize,
    pub active_tools: Vec<ToolExecution>,
    pub status_message: Option<String>,
    pub show_tool_panel: bool,
    pub show_help: bool,
    pub is_processing: bool,
    pub error_message: Option<String>,
}

impl Default for TUIState {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            input_buffer: String::new(),
            input_cursor: 0,
            chat_scroll_offset: 0,
            tool_scroll_offset: 0,
            active_tools: Vec::new(),
            status_message: None,
            show_tool_panel: true,
            show_help: false,
            is_processing: false,
            error_message: None,
        }
    }
}

impl TUIState {
    pub fn add_user_message(&mut self, content: String) {
        self.messages.push(ChatMessage {
            role: MessageRole::User,
            content,
            reasoning: None,
        });
        self.chat_scroll_offset = 0; // Scroll to new message
    }

    pub fn add_assistant_message(&mut self, content: String) -> usize {
        let index = self.messages.len();
        self.messages.push(ChatMessage {
            role: MessageRole::Assistant,
            content,
            reasoning: None,
        });
        self.chat_scroll_offset = 0;
        index
    }

    pub fn append_to_message(&mut self, index: usize, content: &str) {
        if let Some(msg) = self.messages.get_mut(index) {
            msg.content.push_str(content);
        }
    }

    pub fn append_reasoning_to_message(&mut self, index: usize, reasoning: &str) {
        if let Some(msg) = self.messages.get_mut(index) {
            if msg.reasoning.is_none() {
                msg.reasoning = Some(String::new());
            }
            msg.reasoning.as_mut().unwrap().push_str(reasoning);
        }
    }

    // Deprecated but kept for compatibility
    pub fn append_to_last_message(&mut self, content: &str) {
        if let Some(msg) = self.messages.last_mut() {
            msg.content.push_str(content);
        }
    }

    // Deprecated but kept for compatibility
    pub fn append_reasoning_to_last_message(&mut self, reasoning: &str) {
        if let Some(msg) = self.messages.last_mut() {
            if msg.reasoning.is_none() {
                msg.reasoning = Some(String::new());
            }
            msg.reasoning.as_mut().unwrap().push_str(reasoning);
        }
    }

    pub fn set_processing(&mut self, processing: bool) {
        self.is_processing = processing;
    }

    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    pub fn clear_input(&mut self) {
        self.input_buffer.clear();
        self.input_cursor = 0;
    }

    pub fn input_char(&mut self, c: char) {
        self.input_buffer.insert(self.input_cursor, c);
        self.input_cursor += c.len_utf8();
    }

    pub fn input_backspace(&mut self) {
        if self.input_cursor > 0 {
            self.input_cursor -= 1;
            self.input_buffer.remove(self.input_cursor);
        }
    }

    pub fn input_delete(&mut self) {
        if self.input_cursor < self.input_buffer.len() {
            self.input_buffer.remove(self.input_cursor);
        }
    }

    pub fn input_left(&mut self) {
        if self.input_cursor > 0 {
            self.input_cursor -= 1;
        }
    }

    pub fn input_right(&mut self) {
        if self.input_cursor < self.input_buffer.len() {
            self.input_cursor += 1;
        }
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
        self.chat_scroll_offset = 0;
    }
}

#[derive(Debug, Clone)]
pub struct ToolExecution {
    pub name: String,
    pub status: ToolStatus,
    pub start_time: std::time::Instant,
    pub end_time: Option<std::time::Instant>,
    pub parameters: Option<String>,
    pub result: Option<String>,
}

impl ToolExecution {
    pub fn new(name: String) -> Self {
        Self {
            name,
            status: ToolStatus::Running,
            start_time: std::time::Instant::now(),
            end_time: None,
            parameters: None,
            result: None,
        }
    }

    pub fn update_result(&mut self, result: String, success: bool) {
        self.result = Some(result);
        self.end_time = Some(std::time::Instant::now());  // Record end time
        self.status = if success {
            ToolStatus::Success
        } else {
            ToolStatus::Error
        };
    }

    pub fn duration(&self) -> std::time::Duration {
        if let Some(end) = self.end_time {
            // Return actual duration if tool completed
            end.duration_since(self.start_time)
        } else {
            // Return elapsed time if still running
            self.start_time.elapsed()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolStatus {
    Running,
    Success,
    Error,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    pub reasoning: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}
