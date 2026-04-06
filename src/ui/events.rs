use crossterm::event::KeyEvent;
use crate::llm::{StreamChunk, ToolExecutionEvent};

#[derive(Debug, Clone)]
pub enum TUIEvent {
    Keyboard(KeyEvent),
    Stream(StreamChunk),
    ToolExecution(ToolExecutionEvent),
    Error(String),
    Tick,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TUICommand {
    Quit,
    SubmitInput,
    ClearContext,
    ScrollUp,
    ScrollDown,
    PageUp,
    PageDown,
    ScrollToTop,
    ScrollToBottom,
    ToggleToolPanel,
    ToggleHelp,
}
