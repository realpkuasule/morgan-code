use ratatui::{
    text::{Line, Span, Text},
};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct CodeWidget {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl CodeWidget {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn render_code<'a>(&self, code: &'a str, lang: &str, show_line_numbers: bool) -> Text<'a> {
        let syntax = self.syntax_set
            .find_syntax_by_token(lang)
            .or_else(|| self.syntax_set.find_syntax_by_extension(lang))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["base16-ocean.dark"];
        let mut highlighter = HighlightLines::new(syntax, theme);

        let mut lines = Vec::new();
        let max_line_num = code.lines().count();
        let line_num_width = if show_line_numbers {
            format!("{}", max_line_num).len().max(2)
        } else {
            0
        };

        for (idx, line) in LinesWithEndings::from(code).enumerate() {
            let line_num = idx + 1;
            let mut spans = Vec::new();

            if show_line_numbers {
                // Line number in dim color
                let line_num_str = format!("{:>width$} │ ", line_num, width = line_num_width);
                spans.push(Span::styled(
                    line_num_str,
                    ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray),
                ));
            }

            // Highlighted code
            let ranges = highlighter.highlight_line(line, &self.syntax_set).unwrap();
            for (style, text) in ranges {
                let fg = ratatui::style::Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                spans.push(Span::styled(text, ratatui::style::Style::default().fg(fg)));
            }

            lines.push(Line::from(spans));
        }

        Text::from(lines)
    }

    pub fn render_markdown(&self, markdown: &str) -> Text<'static> {
        use pulldown_cmark::{Parser, Event, Tag, CodeBlockKind};

        let parser = Parser::new(markdown);
        let mut lines = Vec::new();
        let mut current_line = Vec::new();
        let mut in_code_block = false;
        let mut code_block_lang = String::new();
        let mut code_block_content = String::new();
        let mut in_heading = false;
        let mut in_bold = false;
        let mut in_italic = false;

        for event in parser {
            match event {
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                    in_code_block = true;
                    code_block_lang = lang.to_string();
                    code_block_content.clear();
                }
                Event::End(Tag::CodeBlock(_)) => {
                    if in_code_block {
                        if !current_line.is_empty() {
                            lines.push(Line::from(current_line.clone()));
                            current_line.clear();
                        }

                        // Add language label if available
                        if !code_block_lang.is_empty() {
                            lines.push(Line::from(vec![
                                Span::styled(
                                    format!("[{}]", code_block_lang),
                                    ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray),
                                ),
                            ]));
                        }

                        // Render code block inline (simplified without syntax highlighting)
                        let code_lines: Vec<&str> = code_block_content.lines().collect();
                        let max_line_num = code_lines.len();
                        let line_num_width = format!("{}", max_line_num).len().max(2);

                        for (idx, line) in code_lines.iter().enumerate() {
                            let line_num = idx + 1;
                            let mut code_spans = Vec::new();

                            // Line number
                            code_spans.push(Span::styled(
                                format!("{:>width$} │ ", line_num, width = line_num_width),
                                ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray),
                            ));

                            // Code line
                            code_spans.push(Span::styled(
                                line.to_string(),
                                ratatui::style::Style::default().fg(ratatui::style::Color::White),
                            ));

                            lines.push(Line::from(code_spans));
                        }

                        in_code_block = false;
                    }
                }
                Event::Text(text) => {
                    if in_code_block {
                        code_block_content.push_str(&text);
                    } else if in_heading {
                        current_line.push(Span::styled(
                            text.to_string(),
                            ratatui::style::Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD),
                        ));
                    } else if in_bold {
                        current_line.push(Span::styled(
                            text.to_string(),
                            ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::BOLD),
                        ));
                    } else if in_italic {
                        current_line.push(Span::styled(
                            text.to_string(),
                            ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::ITALIC),
                        ));
                    } else {
                        current_line.push(Span::raw(text.to_string()));
                    }
                }
                Event::Code(code) => {
                    current_line.push(Span::styled(
                        format!("`{}`", code),
                        ratatui::style::Style::default().fg(ratatui::style::Color::Green),
                    ));
                }
                Event::Start(Tag::Strong) => {
                    in_bold = true;
                }
                Event::End(Tag::Strong) => {
                    in_bold = false;
                }
                Event::Start(Tag::Emphasis) => {
                    in_italic = true;
                }
                Event::End(Tag::Emphasis) => {
                    in_italic = false;
                }
                Event::Start(Tag::Heading(..)) => {
                    in_heading = true;
                }
                Event::End(Tag::Heading(..)) => {
                    in_heading = false;
                    if !current_line.is_empty() {
                        lines.push(Line::from(current_line.clone()));
                        current_line.clear();
                    }
                    lines.push(Line::default()); // Add blank line after heading
                }
                Event::SoftBreak | Event::HardBreak => {
                    if !current_line.is_empty() {
                        lines.push(Line::from(current_line.clone()));
                        current_line.clear();
                    }
                }
                Event::Start(Tag::Paragraph) => {}
                Event::End(Tag::Paragraph) => {
                    if !current_line.is_empty() {
                        lines.push(Line::from(current_line.clone()));
                        current_line.clear();
                    }
                    lines.push(Line::default()); // Add blank line after paragraph
                }
                _ => {}
            }
        }

        // Flush any remaining content
        if !current_line.is_empty() {
            lines.push(Line::from(current_line.clone()));
            current_line.clear();
        }

        Text::from(lines)
    }
}

impl Default for CodeWidget {
    fn default() -> Self {
        Self::new()
    }
}
