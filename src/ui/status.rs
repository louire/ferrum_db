use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::AppState;

pub struct StatusPanel;

/// Draws the status bar panel
pub fn draw(frame: &mut Frame, state: &AppState, area: Rect) {
    let db_name = state.current_database.as_deref().unwrap_or("Not Connected");
    let schema = state.current_schema.as_deref().unwrap_or("public");
    
    let status = Line::from(vec![
        Span::styled("DB: ", Style::default().fg(Color::Gray)),
        Span::styled(db_name, Style::default().fg(Color::Green)),
        Span::raw(" | "),
        Span::styled("Schema: ", Style::default().fg(Color::Gray)),
        Span::styled(schema, Style::default().fg(Color::Green)),
    ]);

    let status_widget = Paragraph::new(status)
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL));
    
    frame.render_widget(status_widget, area);
}