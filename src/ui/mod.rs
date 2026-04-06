pub mod spinner;
pub mod streaming;

// TUI modules
pub mod events;
pub mod layout;
pub mod renderer;
pub mod state;
pub mod tui;
pub mod widgets;

pub use spinner::Spinner;
pub use streaming::StreamingOutput;

// TUI exports
pub use events::{TUICommand, TUIEvent};
pub use layout::{Layout, LayoutMode};
pub use renderer::TUIRenderer;
pub use state::{ChatMessage, MessageRole, TUIState, ToolExecution, ToolStatus};
pub use tui::{run_tui, TUIApplication};
pub use widgets::{ChatWidget, CodeWidget, InputWidget, StatusWidget, ToolPanelWidget};
