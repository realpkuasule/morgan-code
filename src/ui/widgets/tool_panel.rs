use ratatui::{
    layout::Rect,
    style::Color,
    style::Modifier,
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use crate::ui::state::{ToolExecution, ToolStatus};

pub struct ToolPanelWidget {
    show_details: bool,
}

impl ToolPanelWidget {
    pub fn new() -> Self {
        Self {
            show_details: true,  // Default to show details
        }
    }

    pub fn with_details(mut self, show_details: bool) -> Self {
        self.show_details = show_details;
        self
    }

    pub fn toggle_details(&mut self) {
        self.show_details = !self.show_details;
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        tools: &[ToolExecution],
        scroll_offset: usize,
    ) -> usize {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Tools ")
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if tools.is_empty() {
            let _empty_text = Text::from(vec![
                Line::from(vec![
                    Span::styled(
                        "No active tools",
                        Style::default().fg(Color::DarkGray),
                    ),
                ]),
            ]);

            let _empty_area = Rect {
                x: inner.x + (inner.width.saturating_sub(16) / 2),
                y: inner.y + inner.height.saturating_sub(1) / 2,
                width: 16.min(inner.width),
                height: 1,
            };
            return 0;
        }

        // Calculate total height needed
        let mut tool_heights: Vec<usize> = Vec::new();
        for tool in tools {
            let height = self.calculate_tool_height(tool, inner.width, self.show_details);
            tool_heights.push(height);
        }

        let total_height: usize = tool_heights.iter().sum();
        let visible_height = inner.height as usize;

        // Adjust scroll offset
        let effective_scroll = if total_height <= visible_height {
            0
        } else {
            scroll_offset.min(total_height - visible_height)
        };

        let mut y: u16 = 0;
        let mut current_height = 0;

        for (idx, tool) in tools.iter().enumerate() {
            let tool_height = tool_heights[idx];

            // Skip if above scroll position
            if current_height + tool_height <= effective_scroll {
                current_height += tool_height;
                continue;
            }

            // Calculate how much of this tool to render
            let top_skip = effective_scroll.saturating_sub(current_height);
            let remaining_space = visible_height as u16;

            if y >= remaining_space {
                break;
            }

            let rendered_lines = self.render_tool(
                frame,
                tool,
                Rect {
                    x: inner.x,
                    y: inner.y + y,
                    width: inner.width,
                    height: (tool_height - top_skip).min(remaining_space as usize) as u16,
                },
                top_skip,
                self.show_details,
            );

            y += rendered_lines as u16;
            current_height += tool_height;

            if y >= remaining_space {
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

    fn calculate_tool_height(&self, tool: &ToolExecution, width: u16, show_details: bool) -> usize {
        let mut height = 2; // Header line + status line

        if show_details {
            // Parameters section
            if let Some(ref params) = tool.parameters {
                let param_lines = self.wrap_text(params, width - 4);
                height += param_lines.len() + 1; // +1 for header
            }

            // Result section
            if let Some(ref result) = tool.result {
                let result_lines = self.wrap_text(result, width - 4);
                height += result_lines.len() + 1; // +1 for header
            }
        }

        height
    }

    fn render_tool(
        &self,
        frame: &mut Frame,
        tool: &ToolExecution,
        area: Rect,
        top_skip: usize,
        show_details: bool,
    ) -> usize {
        let mut y = 0;

        // Render tool name and status
        if y >= top_skip as u16 && y < area.height {
            let (status_icon, status_color) = match tool.status {
                ToolStatus::Running => ("⏳", Color::Yellow),
                ToolStatus::Success => ("✓", Color::Green),
                ToolStatus::Error => ("✗", Color::Red),
            };

            let header = Line::from(vec![
                Span::styled(
                    status_icon,
                    Style::default().fg(status_color),
                ),
                Span::raw(" "),
                Span::styled(
                    &tool.name,
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
            ]);

            frame.render_widget(header, Rect {
                x: area.x,
                y: area.y + y - top_skip as u16,
                width: area.width,
                height: 1,
            });
            y += 1;
        } else if y < top_skip as u16 {
            y += 1;
        }

        // Render status info
        if y >= top_skip as u16 && y < area.height {
            let duration_text = if tool.status == ToolStatus::Running {
                format!("Running for {:.1}s", tool.duration().as_secs_f64())
            } else {
                // Tool is complete - show final duration (time it took)
                format!("Took {:.1}s", tool.duration().as_secs_f64())
            };

            let status_line = Line::from(vec![
                Span::styled(
                    duration_text,
                    Style::default().fg(Color::DarkGray),
                ),
            ]);

            frame.render_widget(status_line, Rect {
                x: area.x + 2,
                y: area.y + y - top_skip as u16,
                width: area.width - 2,
                height: 1,
            });
            y += 1;
        } else if y < top_skip as u16 {
            y += 1;
        }

        if show_details {
            // Render parameters
            if let Some(ref params) = tool.parameters {
                if y >= top_skip as u16 && y < area.height {
                    let param_header = Line::from(vec![
                        Span::styled(
                            "Parameters:",
                            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                        ),
                    ]);

                    frame.render_widget(param_header, Rect {
                        x: area.x,
                        y: area.y + y - top_skip as u16,
                        width: area.width,
                        height: 1,
                    });
                    y += 1;
                } else if y < top_skip as u16 {
                    y += 1;
                }

                let param_lines = self.wrap_text(params, area.width - 4);
                for (i, line) in param_lines.iter().enumerate() {
                    let current_y = y + i as u16;
                    if current_y >= top_skip as u16 && current_y < area.height {
                        let param_line = Line::from(vec![
                            Span::styled(
                                line.clone(),
                                Style::default().fg(Color::DarkGray),
                            ),
                        ]);

                        frame.render_widget(param_line, Rect {
                            x: area.x + 2,
                            y: area.y + current_y - top_skip as u16,
                            width: area.width - 2,
                            height: 1,
                        });
                    }
                }
                y += param_lines.len() as u16;
            }

            // Render result
            if let Some(ref result) = tool.result {
                if y >= top_skip as u16 && y < area.height {
                    let result_header = Line::from(vec![
                        Span::styled(
                            "Result:",
                            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                        ),
                    ]);

                    frame.render_widget(result_header, Rect {
                        x: area.x,
                        y: area.y + y - top_skip as u16,
                        width: area.width,
                        height: 1,
                    });
                    y += 1;
                } else if y < top_skip as u16 {
                    y += 1;
                }

                let result_lines = self.wrap_text(result, area.width - 4);
                for (i, line) in result_lines.iter().enumerate() {
                    let current_y = y + i as u16;
                    if current_y >= top_skip as u16 && current_y < area.height {
                        let result_line = Line::from(vec![
                            Span::styled(
                                line.clone(),
                                Style::default().fg(Color::DarkGray),
                            ),
                        ]);

                        frame.render_widget(result_line, Rect {
                            x: area.x + 2,
                            y: area.y + current_y - top_skip as u16,
                            width: area.width - 2,
                            height: 1,
                        });
                    }
                }
                y += result_lines.len() as u16;
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
}

impl Default for ToolPanelWidget {
    fn default() -> Self {
        Self::new()
    }
}
