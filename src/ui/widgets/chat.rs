use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use super::code::CodeWidget;
use crate::ui::state::{ChatMessage, MessageRole};

pub struct ChatWidget {
    code_widget: CodeWidget,
    _show_line_numbers: bool,
}

impl ChatWidget {
    pub fn new(show_line_numbers: bool) -> Self {
        Self {
            code_widget: CodeWidget::new(),
            _show_line_numbers: show_line_numbers,
        }
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        messages: &[ChatMessage],
        scroll_offset: usize,
    ) -> usize {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Chat ")
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let mut y = inner.y;
        let mut rendered_height = 0;

        // Calculate total height needed for all messages
        let mut message_heights: Vec<usize> = Vec::new();
        for msg in messages {
            let height = self.calculate_message_height(msg, inner.width);
            message_heights.push(height);
        }

        let total_height: usize = message_heights.iter().sum();
        let visible_height = inner.height as usize;

        // Adjust scroll offset if needed
        let effective_scroll = if total_height <= visible_height {
            0
        } else {
            scroll_offset.min(total_height - visible_height)
        };

        let mut current_height = 0;

        for (idx, msg) in messages.iter().enumerate() {
            let msg_height = message_heights[idx];

            // Skip if above scroll position
            if current_height + msg_height <= effective_scroll {
                current_height += msg_height;
                continue;
            }

            // Calculate how much of this message to render
            let top_skip = effective_scroll.saturating_sub(current_height);
            let remaining_space = visible_height.saturating_sub(rendered_height);

            if remaining_space == 0 {
                break;
            }

            // Render the message
            let rendered_lines = self.render_message(
                frame,
                msg,
                Rect {
                    x: inner.x,
                    y: y,
                    width: inner.width,
                    height: (msg_height - top_skip).min(remaining_space) as u16,
                },
                top_skip,
            );

            rendered_height += rendered_lines;
            y += rendered_lines as u16;
            current_height += msg_height;

            // Add separator between messages
            if y < inner.bottom() {
                if rendered_height < visible_height {
                    let separator = Line::from(vec![
                        Span::styled(
                            "─".repeat(inner.width as usize),
                            Style::default().fg(Color::DarkGray),
                        )
                    ]);
                    frame.render_widget(separator, Rect {
                        x: inner.x,
                        y: y,
                        width: inner.width,
                        height: 1,
                    });
                    y += 1;
                    rendered_height += 1;
                }
            }

            if y >= inner.bottom() {
                break;
            }
        }

        // Render scrollbar if needed
        if total_height > visible_height {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"))
                .track_symbol(Some("│"))
                .thumb_symbol("█");

            let mut scrollbar_state = ScrollbarState::new(total_height)
                .position(effective_scroll);

            frame.render_stateful_widget(
                scrollbar,
                Rect {
                    x: area.x + area.width - 1,
                    y: inner.y,
                    width: 1,
                    height: inner.height,
                },
                &mut scrollbar_state,
            );
        }

        effective_scroll
    }

    fn calculate_message_height(&self, msg: &ChatMessage, width: u16) -> usize {
        let mut height = 1; // Role header

        // Reasoning section
        if let Some(ref reasoning) = msg.reasoning {
            let reasoning_lines = self.wrap_text(reasoning, width - 4);
            height += reasoning_lines.len() + 2; // +2 for header and spacing
        }

        // Content
        let content_text = self.code_widget.render_markdown(&msg.content);
        let content_lines = content_text.lines.len();
        height += content_lines + 1; // +1 for spacing

        height
    }

    fn render_message(
        &self,
        frame: &mut Frame,
        msg: &ChatMessage,
        area: Rect,
        top_skip: usize,
    ) -> usize {
        let mut y = 0;

        // Render role header
        if y >= top_skip as u16 && y < area.height {
            let role_text = match msg.role {
                MessageRole::User => "You",
                MessageRole::Assistant => "Morgan",
                MessageRole::System => "System",
            };
            let style = match msg.role {
                MessageRole::User => Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                MessageRole::Assistant => Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                MessageRole::System => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            };

            let header = Text::from(vec![
                Line::from(vec![
                    Span::styled(
                        format!("{}: ", role_text),
                        style,
                    ),
                ])
            ]);

            self.render_text_clipped(frame, &header, area, y, top_skip);
            y += 1;
        } else if y < top_skip as u16 {
            y += 1;
        }

        // Render reasoning if present
        if let Some(ref reasoning) = msg.reasoning {
            if y >= top_skip as u16 && y < area.height {
                let header_line = Line::from(vec![
                    Span::styled(
                        "Reasoning:",
                        Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
                    ),
                ]);
                self.render_text_clipped(frame, &Text::from(vec![header_line]), area, y, top_skip);
                y += 1;
            } else if y < top_skip as u16 {
                y += 1;
            }

            let reasoning_lines = self.wrap_text(reasoning, area.width - 4);
            for (i, line) in reasoning_lines.iter().enumerate() {
                let current_y = y + i as u16;
                if current_y >= top_skip as u16 && current_y < area.height {
                    let text_line = Line::from(vec![
                        Span::styled(
                            line.clone(),
                            Style::default().fg(Color::DarkGray),
                        ),
                    ]);
                    self.render_text_clipped(frame, &Text::from(vec![text_line]), area, current_y, top_skip);
                }
            }
            y += reasoning_lines.len() as u16 + 1;
        }

        // Render content with markdown
        let content_text = self.code_widget.render_markdown(&msg.content);
        for (i, line) in content_text.lines.iter().enumerate() {
            let current_y = y + i as u16;
            if current_y >= top_skip as u16 && current_y < area.height {
                self.render_line_clipped(frame, line, area, current_y, top_skip);
            }
        }

        (y as usize).saturating_sub(top_skip)
    }

    fn wrap_text(&self, text: &str, width: u16) -> Vec<String> {
        textwrap::wrap(text, width as usize)
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn render_text_clipped(&self, frame: &mut Frame, text: &Text<'_>, area: Rect, line: u16, top_skip: usize) {
        let adjusted_line = line - top_skip as u16;
        if adjusted_line < area.height {
            let text_area = Rect {
                x: area.x,
                y: area.y + adjusted_line,
                width: area.width,
                height: 1,
            };
            frame.render_widget(text.clone(), text_area);
        }
    }

    fn render_line_clipped(&self, frame: &mut Frame, line: &ratatui::text::Line<'_>, area: Rect, line_num: u16, top_skip: usize) {
        let adjusted_line = line_num - top_skip as u16;
        if adjusted_line < area.height {
            let line_area = Rect {
                x: area.x,
                y: area.y + adjusted_line,
                width: area.width,
                height: 1,
            };
            frame.render_widget(line.clone(), line_area);
        }
    }
}
