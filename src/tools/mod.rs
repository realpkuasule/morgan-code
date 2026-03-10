pub mod tool;
pub mod registry;
pub mod fs;
pub mod shell;

pub use tool::{Tool, ToolResult};
pub use registry::ToolRegistry;
pub use shell::ShellTool;

use std::sync::Arc;

impl ToolRegistry {
    fn register_default_tools(&mut self) {
        self.register(Arc::new(fs::ReadTool::new()));
        self.register(Arc::new(fs::WriteTool::new()));
        self.register(Arc::new(fs::EditTool::new()));
        self.register(Arc::new(fs::GlobTool::new()));
        self.register(Arc::new(fs::GrepTool::new()));
        self.register(Arc::new(ShellTool::new(120)));
    }
}
