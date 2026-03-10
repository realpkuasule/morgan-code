use crate::llm::Message;

pub struct SessionContext {
    messages: Vec<Message>,
}

impl SessionContext {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn with_system_message(system_message: impl Into<String>) -> Self {
        let mut context = Self::new();
        context.add_message(Message::system(system_message));
        context
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn messages(&self) -> Vec<Message> {
        self.messages.clone()
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}

impl Default for SessionContext {
    fn default() -> Self {
        Self::new()
    }
}
