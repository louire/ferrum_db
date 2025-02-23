use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::AppState;

mod input;
mod results;
mod sidebar;
mod status;
mod table;

pub use input::InputPanel;
pub use results::ResultsPanel;
pub use sidebar::SidebarPanel;
pub use status::StatusPanel;

/// Draws the entire user interface
pub fn draw(frame: &mut Frame, state: &AppState) {
    // Create the layout
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),  // Sidebar
            Constraint::Percentage(80),  // Main content
        ])
        .split(frame.size());

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Status bar
            Constraint::Min(1),      // Results area
            Constraint::Length(5),   // Input area
        ])
        .split(chunks[1]);

    // Draw each panel
    sidebar::draw(frame, state, chunks[0]);
    status::draw(frame, state, main_chunks[0]);
    results::draw(frame, state, main_chunks[1]);
    input::draw(frame, state, main_chunks[2]);
}