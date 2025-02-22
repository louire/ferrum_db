use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::AppState;

pub struct ResultsPanel;

/// Draws the query results panel
pub fn draw(frame: &mut Frame, state: &AppState, area: Rect) {
    let results = Paragraph::new("No query results yet")
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Results")
        );
    
    frame.render_widget(results, area);
}