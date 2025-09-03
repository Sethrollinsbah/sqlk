use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::{
    application::app::App,
    ui::{centered_rect, UI},
};

impl UI {
    pub fn render_search_input(&self, f: &mut Frame, app: &App) {
        let area = centered_rect(60, 20, f.area());
        f.render_widget(Clear, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(2),
            ])
            .margin(1)
            .split(area);

        let title = Paragraph::new("Search Table")
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            );
        f.render_widget(title, chunks[0]);

        // The text to display in the input box
        let input_text = if app.search_input.is_empty() {
            "Type your search query..."
        } else {
            app.search_input.as_str()
        };

        // The style for the text
        let input_style = if app.search_input.is_empty() {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default().fg(Color::White)
        };

        let search_input = Paragraph::new(input_text).style(input_style).block(
            Block::default()
                .borders(Borders::ALL)
                .title("🔍 Search")
                .border_style(Style::default().fg(Color::Yellow)),
        );

        f.render_widget(search_input, chunks[1]);

        let instructions = vec![Line::from(vec![
            Span::styled(
                "Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(": Search | "),
            Span::styled(
                "Esc",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(": Cancel"),
        ])];

        let help_text = Paragraph::new(instructions)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray));

        f.render_widget(help_text, chunks[2]);

        // Position the cursor at the end of the input text
        f.set_cursor_position(
            Position {
            // x-coordinate: start of the block + border + text length
            x: chunks[1].x + app.search_input.len() as u16 + 1,
            // y-coordinate: start of the block + top border
            y: chunks[1].y + 1,
            }
        );
    }
}
