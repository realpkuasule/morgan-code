use ratatui::{
    layout::Rect,
    style::Color,
    style::Modifier,
    style::Style,
    text::{Line, Span},
    widgets::Borders,
    Frame,
};

pub struct StatusWidget {
    show_help: bool,
}

impl StatusWidget {
    pub fn new() -> Self {
        Self {
            show_help: false,
        }
    }

    pub fn with_help(mut self, show_help: bool) -> Self {
        self.show_help = show_help;
        self
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        status_message: Option<&str>,
        is_processing: bool,
    ) {
        let status_color = if is_processing {
            Color::Yellow
        } else {
            Color::Green
        };

        let status_text = if let Some(msg) = status_message {
            msg.to_string()
        } else if is_processing {
            "Processing...".to_string()
        } else {
            "Ready".to_string()
        };

        let status_line = Line::from(vec![
            Span::styled(
                "─".repeat(area.width as usize),
                Style::default().fg(Color::DarkGray),
            ),
        ]);

        frame.render_widget(status_line, area);

        // Render status indicator
        let indicator = if is_processing { "⟳" } else { "✓" };
        let status_spans = vec![
            Span::raw(" "),
            Span::styled(
                indicator,
                Style::default().fg(status_color),
            ),
            Span::raw(" "),
            Span::styled(
                status_text,
                Style::default().fg(status_color).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | "),
            Span::styled(
                "Press ? for help",
                Style::default().fg(Color::Cyan),
            ),
        ];

        let text_line = Line::from(status_spans);
        frame.render_widget(text_line, Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        });
    }

    pub fn render_help(
        &self,
        frame: &mut Frame,
        area: Rect,
    ) {
        let block = ratatui::widgets::Block::default()
            .borders(Borders::ALL)
            .title(" Help ")
            .border_style(ratatui::style::Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let help_text = vec![
            Line::from(vec![
                Span::styled("Keyboard Shortcuts:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::default(),
            Line::from(vec![
                Span::styled("  Ctrl+C / Ctrl+D", Style::default().fg(Color::White)),
                Span::raw(" - "),
                Span::styled("Quit", Style::default().fg(Color::Gray)),
            ]),
            Line::from(vec![
                Span::styled("  Enter", Style::default().fg(Color::White)),
                Span::raw(" - "),
                Span::styled("Submit input", Style::default().fg(Color::Gray)),
            ]),
            Line::from(vec![
                Span::styled("  Ctrl+L", Style::default().fg(Color::White)),
                Span::raw(" - "),
                Span::styled("Clear context", Style::default().fg(Color::Gray)),
            ]),
            Line::default(),
            Line::from(vec![
                Span::styled("Navigation:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::default(),
            Line::from(vec![
                Span::styled("  ↑ / ↓", Style::default().fg(Color::White)),
                Span::raw(" - "),
                Span::styled("Scroll chat", Style::default().fg(Color::Gray)),
            ]),
            Line::from(vec![
                Span::styled("  PageUp / PageDown", Style::default().fg(Color::White)),
                Span::raw(" - "),
                Span::styled("Scroll page", Style::default().fg(Color::Gray)),
            ]),
            Line::from(vec![
                Span::styled("  Home / End", Style::default().fg(Color::White)),
                Span::raw(" - "),
                Span::styled("Jump to top/bottom", Style::default().fg(Color::Gray)),
            ]),
            Line::from(vec![
                Span::styled("  Tab", Style::default().fg(Color::White)),
                Span::raw(" - "),
                Span::styled("Toggle tool panel", Style::default().fg(Color::Gray)),
            ]),
            Line::from(vec![
                Span::styled("  d", Style::default().fg(Color::White)),
                Span::raw(" - "),
                Span::styled("Toggle tool details", Style::default().fg(Color::Gray)),
            ]),
            Line::default(),
            Line::from(vec![
                Span::styled("Input:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::default(),
            Line::from(vec![
                Span::styled("  ← / →", Style::default().fg(Color::White)),
                Span::raw(" - "),
                Span::styled("Move cursor", Style::default().fg(Color::Gray)),
            ]),
            Line::from(vec![
                Span::styled("  Backspace", Style::default().fg(Color::White)),
                Span::raw(" - "),
                Span::styled("Delete character", Style::default().fg(Color::Gray)),
            ]),
            Line::from(vec![
                Span::styled("  Delete", Style::default().fg(Color::White)),
                Span::raw(" - "),
                Span::styled("Delete character (right)", Style::default().fg(Color::Gray)),
            ]),
        ];

        let help_text_widget = ratatui::text::Text::from(help_text);
        frame.render_widget(help_text_widget, Rect {
            x: inner.x,
            y: inner.y,
            width: inner.width,
            height: inner.height,
        });
    }
}

impl Default for StatusWidget {
    fn default() -> Self {
        Self::new()
    }
}
