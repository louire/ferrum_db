use anyhow::Result;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::Terminal;
use tracing::debug;

mod state;
pub mod input;
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

    /// Runs the main application loop
    pub async fn run(&mut self) -> Result<()> {
        self.init_terminal()?;

        while !self.should_quit {
            self.draw()?;

            if let Ok(true) = event::poll(Duration::from_millis(100)) {
                if let Event::Key(key) = event::read()? {
                    self.handle_input(key.code, key.modifiers)?;
                }
            }
        }

        self.cleanup_terminal()?;
        Ok(())
    }

    /// Handles keyboard input based on current mode
    fn handle_input(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Result<()> {
        match self.state.mode() {
            Mode::Normal => self.handle_normal_mode(key, modifiers),
            Mode::Insert => self.handle_insert_mode(key, modifiers),
        }
    }

    /// Handles input in normal mode
    fn handle_normal_mode(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Result<()> {
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
            _ => {}
        }
        Ok(())
    }

    /// Handles input in insert mode
    fn handle_insert_mode(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Result<()> {
        match (key, modifiers) {
            (KeyCode::Esc, _) => {
                self.state.input.toggle_mode();
            }
            (KeyCode::Enter, _) => {
                let query = self.state.input.take_buffer();
                // TODO: Execute query
                self.state.set_status(format!("Executed query: {}", query));
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