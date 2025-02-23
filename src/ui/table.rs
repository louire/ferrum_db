use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    text::Text,
};

/// Creates a styled table widget from query results
pub fn create_table(headers: Vec<String>, data: Vec<Vec<String>>) -> Table<'static> {
    // Get column count before consuming headers
    let column_count = headers.len();

    // Convert headers into owned Cells
    let header_cells: Vec<Cell<'static>> = headers
        .into_iter()
        .map(|h| {
            Cell::from(h.to_string()).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            )
        })
        .collect();

    // Create header row
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::DarkGray));

    // Convert data into owned Rows
    let rows: Vec<Row<'static>> = data
        .into_iter()
        .map(|row| {
            let cells = row
                .into_iter()
                .map(|c| Cell::from(c.to_string()));
            Row::new(cells).style(Style::default().fg(Color::White))
        })
        .collect();

    // Calculate column widths
    let width = 100 / column_count as u16;
    let constraints = vec![Constraint::Percentage(width); column_count];

    // Create and return the table
    Table::new(rows, constraints)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Results"))
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
}