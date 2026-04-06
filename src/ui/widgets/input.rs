use ratatui::{
    layout::Rect,
    style::Color,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders},
    Frame,
};

use std::collections::VecDeque;

pub struct InputWidget {
    history: VecDeque<String>,
    history_index: Option<usize>,
    temp_input: Option<String>,
}

impl InputWidget {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            history_index: None,
            temp_input: None,
        }
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        input: &str,
        cursor: usize,
        placeholder: &str,
    ) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Input ")
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let display_input = if input.is_empty() {
            placeholder
        } else {
            input
        };

        // Render input text
        let text_line = Line::from(vec![
            Span::raw(display_input),
        ]);

        frame.render_widget(text_line, Rect {
            x: inner.x,
            y: inner.y,
            width: inner.width,
            height: 1,
        });

        // Render cursor
        if cursor <= display_input.len() {
            let cursor_x = inner.x + cursor as u16;
            if cursor_x < inner.x + inner.width {
                let cursor_area = Rect {
                    x: cursor_x,
                    y: inner.y,
                    width: 1,
                    height: 1,
                };

                let cursor_char = if cursor < display_input.len() {
                    display_input.chars().nth(cursor).unwrap()
                } else {
                    ' '
                };

                let cursor_line = Line::from(vec![
                    Span::styled(
                        cursor_char.to_string(),
                        Style::default()
                            .bg(Color::White)
                            .fg(Color::Black),
                    ),
                ]);

                frame.render_widget(cursor_line, cursor_area);
            }
        }
    }

    pub fn add_to_history(&mut self, input: String) {
        if !input.trim().is_empty() {
            self.history.push_back(input);
            // Limit history size
            if self.history.len() > 1000 {
                self.history.pop_front();
            }
            self.history_index = None;
            self.temp_input = None;
        }
    }

    pub fn history_up(&mut self, current_input: String) -> Option<String> {
        if self.history.is_empty() {
            return None;
        }

        if self.history_index.is_none() {
            // Start browsing history - save current input
            self.temp_input = Some(current_input);
            self.history_index = Some(self.history.len() - 1);
        } else {
            // Move to previous entry
            let current_idx = self.history_index.unwrap();
            if current_idx > 0 {
                self.history_index = Some(current_idx - 1);
            }
        }

        self.history_index.and_then(|idx| self.history.get(idx).cloned())
    }

    pub fn history_down(&mut self, _current_input: String) -> Option<String> {
        if self.history_index.is_none() {
            return None;
        }

        let current_idx = self.history_index.unwrap();
        if current_idx < self.history.len() - 1 {
            self.history_index = Some(current_idx + 1);
            self.history.get(self.history_index.unwrap()).cloned()
        } else {
            // Back to current input
            self.history_index = None;
            self.temp_input.clone()
        }
    }

    pub fn reset_history(&mut self) {
        self.history_index = None;
        self.temp_input = None;
    }
}

impl Default for InputWidget {
    fn default() -> Self {
        Self::new()
    }
}
