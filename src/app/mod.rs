use anyhow::Result;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::Terminal;
use tracing::debug;

mod state;
pub mod input;
pub mod query;
pub use state::AppState;
pub use input::Mode;

/// The main application struct for FerrumDB
pub struct App {
    /// The current state of the application
    state: AppState,
    /// Whether the application should exit
    should_quit: bool,
}

impl App {
    /// Creates a new instance of the application
    pub fn new() -> Result<Self> {
        Ok(Self {
            state: AppState::new(),
            should_quit: false,
        })
    }

    /// Initializes the database connection
    pub async fn init_database(&mut self, config: crate::database::DatabaseConfig) -> Result<()> {
        self.state.init_database(config).await
    }

    /// Runs the main application loop
    pub async fn run(&mut self) -> Result<()> {
        // Initialize terminal
        self.init_terminal()?;

        // Main event loop
        while !self.should_quit {
            self.draw()?;

            if let Ok(true) = event::poll(Duration::from_millis(100)) {
                if let Event::Key(key) = event::read()? {
                    self.handle_input(key.code, key.modifiers).await?;
                }
            }
        }

        // Cleanup terminal
        self.cleanup_terminal()?;
        Ok(())
    }

    /// Handles keyboard input based on current mode
    async fn handle_input(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Result<()> {
        match self.state.mode() {
            Mode::Normal => self.handle_normal_mode(key, modifiers).await,
            Mode::Insert => self.handle_insert_mode(key, modifiers).await,
        }
    }

    /// Handles input in normal mode
    async fn handle_normal_mode(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Result<()> {
        match (key, modifiers) {
            (KeyCode::Char('q'), _) => {
                self.should_quit = true;
            }
            (KeyCode::Char('i'), _) => {
                self.state.input.toggle_mode();
            }
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            (KeyCode::Char('r'), _) => {
                // Clear current results
                self.state.query_result = None;
                self.state.set_status("Results cleared".to_string());
            }
            (KeyCode::Char('d'), _) => {
                // Toggle database list (to be implemented)
                self.state.set_status("Database list toggle (not implemented)".to_string());
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles input in insert mode
    async fn handle_insert_mode(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Result<()> {
        match (key, modifiers) {
            (KeyCode::Esc, _) => {
                self.state.input.toggle_mode();
            }
            (KeyCode::Enter, KeyModifiers::CONTROL) | (KeyCode::Enter, KeyModifiers::ALT) => {
                // Execute query without leaving insert mode
                let query = self.state.input.take_buffer();
                self.state.execute_query(query).await;
            }
            (KeyCode::Enter, _) => {
                // Execute query and return to normal mode
                let query = self.state.input.take_buffer();
                self.state.execute_query(query).await;
                self.state.input.toggle_mode();
            }
            (KeyCode::Char(c), _) => {
                self.state.input.insert_char(c);
            }
            (KeyCode::Backspace, _) => {
                self.state.input.delete_char();
            }
            (KeyCode::Left, _) => {
                self.state.input.move_cursor_left();
            }
            (KeyCode::Right, _) => {
                self.state.input.move_cursor_right();
            }
            (KeyCode::Delete, _) => {
                // Delete character under cursor
                if self.state.input.cursor_position() < self.state.input.buffer().len() {
                    self.state.input.move_cursor_right();
                    self.state.input.delete_char();
                }
            }
            (KeyCode::Home, _) | (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                // Move cursor to start of line
                while self.state.input.cursor_position() > 0 {
                    self.state.input.move_cursor_left();
                }
            }
            (KeyCode::End, _) | (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                // Move cursor to end of line
                while self.state.input.cursor_position() < self.state.input.buffer().len() {
                    self.state.input.move_cursor_right();
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Initializes the terminal for the TUI
    fn init_terminal(&self) -> Result<()> {
        debug!("Initializing terminal");
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::EnterAlternateScreen,
            crossterm::event::EnableMouseCapture
        )?;
        Ok(())
    }

    /// Cleans up the terminal state
    fn cleanup_terminal(&self) -> Result<()> {
        debug!("Cleaning up terminal");
        crossterm::execute!(
            std::io::stdout(),
            crossterm::event::DisableMouseCapture,
            crossterm::terminal::LeaveAlternateScreen
        )?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Draws the user interface
    fn draw(&self) -> Result<()> {
        let mut terminal = Terminal::new(
            ratatui::backend::CrosstermBackend::new(std::io::stdout())
        )?;

        terminal.draw(|frame| {
            crate::ui::draw(frame, &self.state);
        })?;

        Ok(())
    }
}