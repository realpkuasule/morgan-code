use console::style;
use pulldown_cmark::{Parser, Event, Tag, CodeBlockKind};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub struct MarkdownRenderer {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn print(&self, markdown: &str) {
        let parser = Parser::new(markdown);
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
                        self.print_code_block(&code_block_content, &code_block_lang);
                        in_code_block = false;
                    }
                }
                Event::Text(text) => {
                    if in_code_block {
                        code_block_content.push_str(&text);
                    } else if in_heading {
                        print!("{}", style(&*text).bold().cyan());
                    } else if in_bold {
                        print!("{}", style(&*text).bold());
                    } else if in_italic {
                        print!("{}", style(&*text).italic());
                    } else {
                        print!("{}", text);
                    }
                }
                Event::Code(code) => {
                    print!("{}", style(format!("`{}`", code)).green());
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
                    print!("\n");
                    in_heading = true;
                }
                Event::End(Tag::Heading(..)) => {
                    in_heading = false;
                    println!();
                }
                Event::SoftBreak | Event::HardBreak => {
                    println!();
                }
                Event::Start(Tag::Paragraph) => {}
                Event::End(Tag::Paragraph) => {
                    println!();
                }
                _ => {}
            }
        }
    }

    fn print_code_block(&self, code: &str, lang: &str) {
        println!(); // Blank line before code block

        // Print language label if available
        if !lang.is_empty() {
            println!("  {}", style(format!("[{}]", lang)).dim());
        }

        let lines: Vec<&str> = code.lines().collect();
        let line_count = lines.len();
        let line_num_width = format!("{}", line_count).len().max(2);

        // Simple syntax highlighting without themes (no background colors)
        let syntax = self.syntax_set
            .find_syntax_by_token(lang)
            .or_else(|| self.syntax_set.find_syntax_by_extension(lang))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["base16-ocean.dark"];
        let mut highlighter = HighlightLines::new(syntax, theme);

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;

            // Print line number with padding and separator
            print!("  {} {} ",
                style(format!("{:>width$}", line_num, width = line_num_width)).dim(),
                style("│").dim()
            );

            // Highlight and print the code line - ONLY use foreground colors
            let line_with_newline = format!("{}\n", line);
            let ranges = highlighter.highlight_line(&line_with_newline, &self.syntax_set).unwrap();

            // Print ONLY foreground colors, explicitly ignore background
            for (s, text) in ranges {
                let fg = s.foreground;
                // Only set foreground color, no background
                print!("\x1b[38;2;{};{};{}m{}", fg.r, fg.g, fg.b, text.trim_end());
            }

            // Reset all attributes at end of line
            println!("\x1b[0m");
        }

        println!(); // Extra line after code block
    }
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        Self::new()
    }
}
