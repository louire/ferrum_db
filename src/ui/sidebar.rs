use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::AppState;

pub struct SidebarPanel;

/// Draws the navigation sidebar
pub fn draw(frame: &mut Frame, state: &AppState, area: Rect) {
    let sidebar = Paragraph::new("Databases will be listed here")
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Navigation")
        );
    
    frame.render_widget(sidebar, area);
}