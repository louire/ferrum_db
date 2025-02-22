use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::{AppState, input::Mode};

pub struct InputPanel;

/// Draws the query input panel
pub fn draw(frame: &mut Frame, state: &AppState, area: Rect) {
    let mode_style = match state.mode() {
        Mode::Normal => Style::default().fg(Color::Gray),
        Mode::Insert => Style::default().fg(Color::Green),
    };

    let mode_indicator = match state.mode() {
        Mode::Normal => "NORMAL",
        Mode::Insert => "INSERT",
    };

    let input_text = state.input.buffer();
    let cursor_position = state.input.cursor_position();

    // Create text with cursor
    let mut text = String::with_capacity(input_text.len() + 1);
    text.push_str(&input_text[..cursor_position]);
    text.push('█'); // Cursor character
    if cursor_position < input_text.len() {
        text.push_str(&input_text[cursor_position..]);
    }

    let input = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(format!("-- {} -- ", mode_indicator), mode_style),
            Span::raw("Press ESC to switch to Normal mode, i to switch to Insert mode")
        ]),
        Line::from(text),
    ])
    .style(Style::default().fg(Color::White))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("SQL Query (Press Enter to execute)")
    );
    
    frame.render_widget(input, area);
}