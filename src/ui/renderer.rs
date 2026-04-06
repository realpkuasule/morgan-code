use ratatui::{
    layout::Rect,
    widgets::{Borders, Clear},
    Frame,
};

use crate::config::Config;
use crate::ui::layout::Layout;
use crate::ui::state::TUIState;
use crate::ui::widgets::{
    chat::ChatWidget,
    input::InputWidget,
    status::StatusWidget,
    tool_panel::ToolPanelWidget,
};

pub struct TUIRenderer {
    chat_widget: ChatWidget,
    tool_panel_widget: ToolPanelWidget,
    input_widget: InputWidget,
    status_widget: StatusWidget,
}

impl TUIRenderer {
    pub fn new(config: &Config) -> Self {
        Self {
            chat_widget: ChatWidget::new(config.ui.show_line_numbers),
            tool_panel_widget: ToolPanelWidget::new(),
            input_widget: InputWidget::new(),
            status_widget: StatusWidget::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame, state: &mut TUIState, layout: Layout) {
        // Clear the frame
        frame.render_widget(Clear, frame.size());

        // Determine if we should render tool panel
        if layout.mode.has_tool_panel() {
            // Render tool panel
            self.tool_panel_widget.render(
                frame,
                layout.tool_area,
                &state.active_tools,
                state.tool_scroll_offset,
            );
        }

        // Render chat area
        let actual_scroll = self.chat_widget.render(
            frame,
            layout.chat_area,
            &state.messages,
            state.chat_scroll_offset,
        );
        state.chat_scroll_offset = actual_scroll;

        // Render input area
        self.input_widget.render(
            frame,
            layout.input_area,
            &state.input_buffer,
            state.input_cursor,
            "Type your message...",
        );

        // Render status bar
        self.status_widget.render(
            frame,
            layout.status_area,
            state.status_message.as_deref(),
            state.is_processing,
        );

        // Render help overlay if enabled
        if state.show_help {
            self.render_help_overlay(frame);
        }

        // Render error overlay if present
        if let Some(ref error) = state.error_message {
            self.render_error_overlay(frame, error);
        }
    }

    pub fn render_help_overlay(&self, frame: &mut Frame) {
        let area = frame.size();

        let help_width = 50.min(area.width - 4);
        let help_height = 20.min(area.height - 4);
        let help_x = (area.width - help_width) / 2;
        let help_y = (area.height - help_height) / 2;

        let help_area = Rect {
            x: help_x,
            y: help_y,
            width: help_width,
            height: help_height,
        };

        // Draw background for help
        let help_block = ratatui::widgets::Block::default()
            .borders(Borders::ALL)
            .title(" Help ")
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan))
            .style(ratatui::style::Style::default().bg(ratatui::style::Color::Black));

        let inner = help_block.inner(help_area);
        frame.render_widget(help_block, help_area);

        let help_text = vec![
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("Keyboard Shortcuts:", ratatui::style::Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD)),
            ]),
            ratatui::text::Line::default(),
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("  Ctrl+C / Ctrl+D", ratatui::style::Style::default().fg(ratatui::style::Color::White)),
                ratatui::text::Span::raw(" - "),
                ratatui::text::Span::styled("Quit", ratatui::style::Style::default().fg(ratatui::style::Color::Gray)),
            ]),
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("  Enter", ratatui::style::Style::default().fg(ratatui::style::Color::White)),
                ratatui::text::Span::raw(" - "),
                ratatui::text::Span::styled("Submit input", ratatui::style::Style::default().fg(ratatui::style::Color::Gray)),
            ]),
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("  Ctrl+L", ratatui::style::Style::default().fg(ratatui::style::Color::White)),
                ratatui::text::Span::raw(" - "),
                ratatui::text::Span::styled("Clear context", ratatui::style::Style::default().fg(ratatui::style::Color::Gray)),
            ]),
            ratatui::text::Line::default(),
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("  ↑ / ↓", ratatui::style::Style::default().fg(ratatui::style::Color::White)),
                ratatui::text::Span::raw(" - "),
                ratatui::text::Span::styled("Scroll chat", ratatui::style::Style::default().fg(ratatui::style::Color::Gray)),
            ]),
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("  PageUp / PageDown", ratatui::style::Style::default().fg(ratatui::style::Color::White)),
                ratatui::text::Span::raw(" - "),
                ratatui::text::Span::styled("Scroll page", ratatui::style::Style::default().fg(ratatui::style::Color::Gray)),
            ]),
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("  Tab", ratatui::style::Style::default().fg(ratatui::style::Color::White)),
                ratatui::text::Span::raw(" - "),
                ratatui::text::Span::styled("Toggle tool panel", ratatui::style::Style::default().fg(ratatui::style::Color::Gray)),
            ]),
            ratatui::text::Line::default(),
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("  ?", ratatui::style::Style::default().fg(ratatui::style::Color::White)),
                ratatui::text::Span::raw(" - "),
                ratatui::text::Span::styled("Close help", ratatui::style::Style::default().fg(ratatui::style::Color::Gray)),
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

    pub fn render_error_overlay(&self, frame: &mut Frame, error: &str) {
        let area = frame.size();

        let error_width = 60.min(area.width - 4);
        let error_x = (area.width - error_width) / 2;
        let error_y = area.height.saturating_sub(6) / 2;

        let error_area = Rect {
            x: error_x,
            y: error_y,
            width: error_width,
            height: 4,
        };

        // Draw error box
        let error_block = ratatui::widgets::Block::default()
            .borders(Borders::ALL)
            .title(" Error ")
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Red))
            .style(ratatui::style::Style::default().bg(ratatui::style::Color::Black));

        let inner = error_block.inner(error_area);
        frame.render_widget(error_block, error_area);

        // Render error message (truncated if too long)
        let error_lines: Vec<_> = textwrap::wrap(error, (inner.width - 2) as usize)
            .into_iter()
            .take(2) // Show at most 2 lines
            .collect();

        for (i, line) in error_lines.iter().enumerate() {
            let y = inner.y + i as u16;
            if y < inner.bottom() {
                let error_line = ratatui::text::Line::from(vec![
                    ratatui::text::Span::styled(
                        line.as_ref(),
                        ratatui::style::Style::default().fg(ratatui::style::Color::Red),
                    ),
                ]);
                frame.render_widget(error_line, Rect {
                    x: inner.x,
                    y,
                    width: inner.width,
                    height: 1,
                });
            }
        }

        // Render "Press any key to dismiss"
        if inner.height > 2 {
            let dismiss_line = ratatui::text::Line::from(vec![
                ratatui::text::Span::styled(
                    "Press any key to dismiss",
                    ratatui::style::Style::default().fg(ratatui::style::Color::Gray).add_modifier(ratatui::style::Modifier::ITALIC),
                ),
            ]);
            frame.render_widget(dismiss_line, Rect {
                x: inner.x + (inner.width.saturating_sub(24)) / 2,
                y: inner.bottom() - 1,
                width: 24.min(inner.width),
                height: 1,
            });
        }
    }

    pub fn input_widget(&mut self) -> &mut InputWidget {
        &mut self.input_widget
    }

    pub fn tool_panel_widget(&mut self) -> &mut ToolPanelWidget {
        &mut self.tool_panel_widget
    }

    pub fn status_widget(&mut self) -> &mut StatusWidget {
        &mut self.status_widget
    }
}
