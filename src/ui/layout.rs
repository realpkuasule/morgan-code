use ratatui::layout::{Constraint, Direction, Layout as RatatuiLayout, Rect};

#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub chat_area: Rect,
    pub tool_area: Rect,
    pub input_area: Rect,
    pub status_area: Rect,
    pub mode: LayoutMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    Normal { tool_width: u16 },
    ToolExpanded { tool_width: u16 },
    Minimal,
}

impl LayoutMode {
    pub fn toggle(self) -> Self {
        match self {
            LayoutMode::Normal { tool_width } => LayoutMode::ToolExpanded { tool_width },
            LayoutMode::ToolExpanded { tool_width } => LayoutMode::Normal { tool_width },
            LayoutMode::Minimal => LayoutMode::Normal { tool_width: 30 },
        }
    }

    pub fn tool_width(self) -> Option<u16> {
        match self {
            LayoutMode::Normal { tool_width } => Some(tool_width),
            LayoutMode::ToolExpanded { tool_width } => Some(tool_width),
            LayoutMode::Minimal => None,
        }
    }

    pub fn has_tool_panel(self) -> bool {
        matches!(self, LayoutMode::Normal { .. } | LayoutMode::ToolExpanded { .. })
    }
}

impl Layout {
    pub fn new(area: Rect, mode: LayoutMode) -> Self {
        // Split into vertical sections
        let chunks = RatatuiLayout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                Constraint::Min(0),    // Main content (chat + tool)
                Constraint::Length(3), // Input area
                Constraint::Length(1), // Status bar
            ])
            .split(area);

        let main_area = chunks[0];
        let input_area = chunks[1];
        let status_area = chunks[2];

        // Split main area into chat and tool panel
        let (chat_area, tool_area) = if mode.has_tool_panel() {
            let width = mode.tool_width().unwrap_or(30);
            let horizontal = RatatuiLayout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(area.width.saturating_sub(width)),
                    Constraint::Length(width),
                ])
                .split(main_area);

            (horizontal[0], horizontal[1])
        } else {
            (main_area, Rect::default())
        };

        Layout {
            chat_area,
            tool_area,
            input_area,
            status_area,
            mode,
        }
    }

    pub fn with_mode(mut self, mode: LayoutMode) -> Self {
        self.mode = mode;
        // Recalculate areas with new mode
        let area = self.chat_area.union(self.tool_area);
        let new_layout = Self::new(area, mode);
        new_layout
    }
}
