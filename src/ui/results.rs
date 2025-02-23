use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::AppState;
use super::table::create_table;

pub struct ResultsPanel;

/// Draws the query results panel
pub fn draw(frame: &mut Frame, state: &AppState, area: Rect) {
    match &state.query_result {
        Some(result) => {
            let table = create_table(
                result.headers.clone(),
                result.rows.clone()
            );
            frame.render_widget(table, area);
        }
        None => {
            let message = if let Some(error) = &state.last_error {
                format!("Error: {}", error)
            } else if let Some(msg) = &state.status_message {
                msg.clone()
            } else {
                "No query results yet".to_string()
            };

            let results = Paragraph::new(message)
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL).title("Results"));
            
            frame.render_widget(results, area);
        }
    }
}