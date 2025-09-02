use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::{
    application::app::App,
    ui::{centered_rect, UI},
};

impl UI {
    pub fn render_cell_info_viewer(&self, f: &mut Frame, app: &App) {
        if let Some(ref cell_info) = app.cell_info {
            let area = centered_rect(85, 70, f.area());
            f.render_widget(Clear, area);

            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(area);

            let title = Paragraph::new(format!(
                "Cell Information: {} [{}:{}]",
                cell_info.column_name,
                cell_info.row_index + 1,
                cell_info.column_index + 1
            ))
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

            f.render_widget(title, main_chunks[0]);

            let content_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(8),
                    Constraint::Length(6),
                    Constraint::Min(0),
                ])
                .margin(1)
                .split(main_chunks[1]);

            self.render_basic_cell_info(f, cell_info, content_chunks[0]);
            self.render_cell_statistics(f, cell_info, content_chunks[1]);

            if cell_info.foreign_key_info.is_some() {
                self.render_foreign_key_section(f, cell_info, content_chunks[2]);
            }

            let instructions = vec![Line::from(vec![
                Span::styled(
                    "F",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(": View FK Data | "),
                Span::styled(
                    "Esc/q",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(": Close | "),
                Span::styled(
                    "?",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(": Help"),
            ])];

            let help_text = Paragraph::new(instructions)
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center);

            f.render_widget(help_text, main_chunks[2]);
        }
    }

    pub fn render_basic_cell_info(
        &self,
        f: &mut Frame,
        cell_info: &crate::table_viewer::CellInfo,
        area: Rect,
    ) {
        let info_lines = vec![
            Line::from(vec![
                Span::styled(
                    "Value: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    if cell_info.value.len() > 50 {
                        format!("{}...", &cell_info.value[..47])
                    } else {
                        cell_info.value.clone()
                    },
                    if cell_info.is_null {
                        Style::default()
                            .fg(Color::Red)
                            .add_modifier(Modifier::ITALIC)
                    } else {
                        Style::default().fg(Color::White)
                    },
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "Type: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    cell_info
                        .data_type
                        .as_ref()
                        .unwrap_or(&"Unknown".to_string())
                        .clone(),
                    Style::default().fg(Color::Cyan),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "Length: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    cell_info.value_length.to_string(),
                    Style::default().fg(Color::Green),
                ),
                Span::raw(" characters"),
            ]),
            Line::from(vec![
                Span::styled(
                    "Is Null: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    if cell_info.is_null { "Yes" } else { "No" },
                    if cell_info.is_null {
                        Style::default().fg(Color::Red)
                    } else {
                        Style::default().fg(Color::Green)
                    },
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "Position: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(
                        "Row {}, Column {}",
                        cell_info.row_index + 1,
                        cell_info.column_index + 1
                    ),
                    Style::default().fg(Color::Blue),
                ),
            ]),
        ];

        let basic_info = Paragraph::new(info_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Basic Information")
                    .style(Style::default().fg(Color::White)),
            )
            .wrap(Wrap { trim: true });

        f.render_widget(basic_info, area);
    }

    pub fn render_cell_statistics(
        &self,
        f: &mut Frame,
        cell_info: &crate::table_viewer::CellInfo,
        area: Rect,
    ) {
        let stats_lines = vec![
            Line::from(vec![
                Span::styled(
                    "Occurrences: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    cell_info.duplicate_count.to_string(),
                    Style::default().fg(Color::Magenta),
                ),
                Span::raw(" times in this column"),
            ]),
            Line::from(vec![
                Span::styled(
                    "Frequency: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:.2}%", cell_info.percentage_of_total),
                    Style::default().fg(Color::Magenta),
                ),
                Span::raw(" of non-null values"),
            ]),
            Line::from(vec![
                Span::styled(
                    "Unique Values in Column: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    cell_info.unique_values_in_column.to_string(),
                    Style::default().fg(Color::Cyan),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "Rarity: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    if cell_info.percentage_of_total > 50.0 {
                        "Very Common"
                    } else if cell_info.percentage_of_total > 10.0 {
                        "Common"
                    } else if cell_info.percentage_of_total > 1.0 {
                        "Uncommon"
                    } else {
                        "Rare"
                    },
                    match cell_info.percentage_of_total {
                        p if p > 50.0 => Style::default().fg(Color::Red),
                        p if p > 10.0 => Style::default().fg(Color::Yellow),
                        p if p > 1.0 => Style::default().fg(Color::Blue),
                        _ => Style::default().fg(Color::Green),
                    },
                ),
            ]),
        ];

        let stats_info = Paragraph::new(stats_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Statistics")
                .style(Style::default().fg(Color::White)),
        );

        f.render_widget(stats_info, area);
    }
}
