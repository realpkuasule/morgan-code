use std::{io, sync::Arc, time::Duration};

use crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::{mpsc, Mutex};

use crate::{
    agent::Agent,
    error::Result,
};
use crate::ui::{
    events::TUIEvent,
    layout::{Layout, LayoutMode},
    renderer::TUIRenderer,
    state::TUIState,
};

use crate::config::Config;

pub struct TUIApplication {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    state: TUIState,
    agent: Arc<Mutex<Agent>>,
    _config: Config,
    renderer: TUIRenderer,
    input_history: Vec<String>,
    event_tx: mpsc::UnboundedSender<TUIEvent>,
    event_rx: mpsc::UnboundedReceiver<TUIEvent>,
    current_message_index: Option<usize>,
}

impl TUIApplication {
    pub fn new(
        agent: Agent,
        config: &Config,
    ) -> Result<Self> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        let state = TUIState::default();
        let renderer = TUIRenderer::new(config);

        // Create event channel for streaming
        let (event_tx, event_rx) = mpsc::unbounded_channel::<TUIEvent>();

        Ok(Self {
            terminal,
            state,
            agent: Arc::new(Mutex::new(agent)),
            _config: config.clone(),
            renderer,
            input_history: Vec::new(),
            event_tx,
            event_rx,
            current_message_index: None,
        })
    }

    pub async fn run(mut self) -> Result<()> {
        let tick_rate = Duration::from_millis(33); // ~30 FPS
        let mut tick_interval = tokio::time::interval(tick_rate);
        tick_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        // Clone the sender for spawning tasks
        let event_tx = self.event_tx.clone();

        loop {
            tokio::select! {
                // Keyboard events
                result = Self::poll_keyboard_event() => {
                    if let Ok(Some(key_event)) = result {
                        let _ = event_tx.send(TUIEvent::Keyboard(key_event));
                    }
                }

                // Streaming and other events
                Some(event) = self.event_rx.recv() => {
                    if let Err(e) = self.handle_tui_event(event).await {
                        self.state.set_error(e.to_string());
                    }
                }

                // Periodic rendering tick
                _ = tick_interval.tick() => {
                    // Force a render
                }
            }

            // Render on every loop iteration
            let area = self.terminal.size()?;
            let layout = Layout::new(area, self.get_layout_mode());
            self.terminal.draw(|f| {
                self.renderer.render(f, &mut self.state, layout);
            })?;

            // Check for quit condition
            if self.state.error_message.as_deref() == Some("Quit") {
                break;
            }
        }

        // Cleanup
        self.cleanup()?;
        Ok(())
    }

    async fn poll_keyboard_event() -> io::Result<Option<crossterm::event::KeyEvent>> {
        use std::time::Duration;
        if crossterm::event::poll(Duration::from_millis(0))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == KeyEventKind::Press {
                    return Ok(Some(key));
                }
            }
        }
        Ok(None)
    }

    fn get_layout_mode(&self) -> LayoutMode {
        if self.state.show_tool_panel {
            LayoutMode::Normal { tool_width: 30 }
        } else {
            LayoutMode::Minimal
        }
    }

    async fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        // If error overlay is shown, any key dismisses it
        if self.state.error_message.is_some() {
            self.state.clear_error();
            return Ok(());
        }

        // If help is shown, ? or Esc closes it
        if self.state.show_help {
            if key_event.code == KeyCode::Char('?') || key_event.code == KeyCode::Esc {
                self.state.show_help = false;
            }
            return Ok(());
        }

        match (key_event.code, key_event.modifiers) {
            // Quit commands
            (KeyCode::Char('c'), KeyModifiers::CONTROL) |
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                self.state.set_error("Quit".to_string());
            }

            // Toggle help
            (KeyCode::Char('?'), KeyModifiers::NONE) => {
                self.state.show_help = !self.state.show_help;
            }

            // Toggle tool panel
            (KeyCode::Tab, KeyModifiers::NONE) => {
                self.state.show_tool_panel = !self.state.show_tool_panel;
            }

            // Toggle tool details (d for details)
            (KeyCode::Char('d'), KeyModifiers::NONE) => {
                self.renderer.tool_panel_widget().toggle_details();
            }

            // Submit input
            (KeyCode::Enter, KeyModifiers::NONE) => {
                if !self.state.input_buffer.trim().is_empty() {
                    let input = self.state.input_buffer.clone();
                    self.input_history.push(input.clone());
                    self.renderer.input_widget().add_to_history(input.clone());
                    self.state.clear_input();

                    // Add user message to chat
                    self.state.add_user_message(input.clone());

                    // Process the input
                    self.process_input(input).await?;
                }
            }

            // Clear context
            (KeyCode::Char('l'), KeyModifiers::CONTROL) => {
                self.state.clear_messages();
                self.state.status_message = Some("Context cleared".to_string());
            }

            // Navigation
            (KeyCode::Up, KeyModifiers::NONE) => {
                if self.state.chat_scroll_offset > 0 {
                    self.state.chat_scroll_offset = self.state.chat_scroll_offset.saturating_sub(1);
                }
            }
            (KeyCode::Down, KeyModifiers::NONE) => {
                self.state.chat_scroll_offset += 1;
            }
            (KeyCode::PageUp, KeyModifiers::NONE) => {
                self.state.chat_scroll_offset = self.state.chat_scroll_offset.saturating_sub(5);
            }
            (KeyCode::PageDown, KeyModifiers::NONE) => {
                self.state.chat_scroll_offset += 5;
            }
            (KeyCode::Home, KeyModifiers::NONE) => {
                self.state.chat_scroll_offset = 0;
            }
            (KeyCode::End, KeyModifiers::NONE) => {
                self.state.chat_scroll_offset = usize::MAX;
            }

            // Input editing
            (KeyCode::Char(c), KeyModifiers::NONE) => {
                self.state.input_char(c);
            }
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                self.state.input_backspace();
            }
            (KeyCode::Delete, KeyModifiers::NONE) => {
                self.state.input_delete();
            }
            (KeyCode::Left, KeyModifiers::NONE) => {
                self.state.input_left();
            }
            (KeyCode::Right, KeyModifiers::NONE) => {
                self.state.input_right();
            }

            _ => {}
        }

        Ok(())
    }

    async fn handle_tui_event(&mut self, event: TUIEvent) -> Result<()> {
        match event {
            TUIEvent::Keyboard(key) => {
                self.handle_key_event(key).await?;
            }
            TUIEvent::Stream(chunk) => {
                // Handle reasoning
                if let Some(ref reasoning) = chunk.reasoning_content {
                    if let Some(index) = self.current_message_index {
                        self.state.append_reasoning_to_message(index, reasoning);
                    }
                }

                // Handle content
                if !chunk.content.is_empty() {
                    if let Some(index) = self.current_message_index {
                        self.state.append_to_message(index, &chunk.content);
                    }
                }

                // Handle tool execution events
                if let Some(ref tool_event) = chunk.tool_execution_event {
                    match tool_event {
                        crate::llm::ToolExecutionEvent::ToolCallStart { name, .. } => {
                            let tool = crate::ui::state::ToolExecution::new(name.clone());
                            self.state.active_tools.push(tool);
                            self.state.tool_scroll_offset = 0;
                        }
                        crate::llm::ToolExecutionEvent::ToolCallEnd { name, result, success, .. } => {
                            if let Some(tool) = self.state.active_tools.iter_mut().find(|t| t.name == *name) {
                                tool.update_result(result.clone(), *success);
                            }
                        }
                    }
                }
            }
            TUIEvent::Error(error) => {
                // Check if this is a completion signal
                if error == "Processing complete" {
                    self.state.set_processing(false);
                    self.state.status_message = Some("Ready".to_string());
                    self.current_message_index = None; // Clear index
                } else {
                    self.state.set_error(error);
                }
            }
            TUIEvent::Quit => {
                self.state.set_error("Quit".to_string());
            }
            _ => {}
        }
        Ok(())
    }

    async fn process_input(&mut self, input: String) -> Result<()> {
        self.state.set_processing(true);
        self.state.status_message = Some("Processing...".to_string());

        // Create an empty assistant message for streaming and get its index
        let message_index = self.state.add_assistant_message(String::new());
        self.current_message_index = Some(message_index);

        // Spawn streaming task
        let agent_ref = self.agent.clone();
        let event_tx = self.event_tx.clone();

        tokio::spawn(async move {
            let mut agent = agent_ref.lock().await;
            if let Err(e) = agent.run_streaming(input.clone(), |chunk| {
                let _ = event_tx.send(TUIEvent::Stream(chunk.clone()));
            }).await {
                let _ = event_tx.send(TUIEvent::Error(format!("Input processing error: {}", e)));
            } else {
                // Send completion signal
                let _ = event_tx.send(TUIEvent::Error("Processing complete".to_string()));
            }
        });

        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

// Main entry point for TUI mode
pub async fn run_tui(agent: Agent, config: &Config) -> Result<()> {
    let app = TUIApplication::new(agent, config)?;
    app.run().await?;

    Ok(())
}

