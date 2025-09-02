use crate::{
    application::app::App,
    ui::{centered_rect, UI},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Wrap},
    Frame,
};

impl UI {
    pub fn render_foreign_key_section(
        &self,
        f: &mut Frame,
        cell_info: &crate::table_viewer::CellInfo,
        area: Rect,
    ) {
        if let Some(ref fk_data) = cell_info.foreign_key_info {
            let fk_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(6), Constraint::Min(0)])
                .split(area);

            let fk_info_lines = vec![
                Line::from(vec![Span::styled(
                    "Foreign Key Detected!",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        "References: ",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!(
                            "{}.{}",
                            fk_data.foreign_key_info.referenced_table,
                            fk_data.foreign_key_info.referenced_column
                        ),
                        Style::default().fg(Color::Cyan),
                    ),
                ]),
                Line::from(vec![
                    Span::styled(
                        "Lookup Results: ",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{} rows found", fk_data.lookup_data.rows.len()),
                        Style::default().fg(Color::Green),
                    ),
                ]),
            ];

            let fk_info = Paragraph::new(fk_info_lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Foreign Key Information")
                    .style(Style::default().fg(Color::Green)),
            );

            f.render_widget(fk_info, fk_chunks[0]);

            if !fk_data.lookup_data.rows.is_empty() {
                let preview_lines: Vec<Line> = fk_data
                    .lookup_data
                    .rows
                    .iter()
                    .take(5)
                    .enumerate()
                    .map(|(i, row)| {
                        let row_text = if row.len() > 3 {
                            format!(
                                "{} | {} | {} | ...",
                                row.first().unwrap_or(&"".to_string()),
                                row.get(1).unwrap_or(&"".to_string()),
                                row.get(2).unwrap_or(&"".to_string())
                            )
                        } else {
                            row.join(" | ")
                        };

                        Line::from(vec![
                            Span::styled(
                                format!("{}. ", i + 1),
                                Style::default().fg(Color::DarkGray),
                            ),
                            Span::styled(row_text, Style::default().fg(Color::White)),
                        ])
                    })
                    .collect();

                let preview = Paragraph::new(preview_lines)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Preview (Press F for full view)")
                            .style(Style::default().fg(Color::Blue)),
                    )
                    .wrap(Wrap { trim: true });

                f.render_widget(preview, fk_chunks[1]);
            }
        } else {
            let no_fk_info = Paragraph::new(vec![
                Line::from(vec![Span::styled(
                    "No Foreign Key",
                    Style::default()
                        .fg(Color::Gray)
                        .add_modifier(Modifier::ITALIC),
                )]),
                Line::from(""),
                Line::from(vec![Span::raw(
                    "This column does not reference another table.",
                )]),
            ])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Foreign Key Information")
                    .style(Style::default().fg(Color::Gray)),
            );

            f.render_widget(no_fk_info, area);
        }
    }
    pub fn render_foreign_key_viewer(&self, f: &mut Frame, app: &App) {
        if let Some(ref viewer) = app.foreign_key_viewer {
            let area = centered_rect(80, 50, f.area());
            f.render_widget(Clear, area);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(2)])
                .split(area);

            let view_data = viewer.get_visible_data(area.width, area.height.saturating_sub(3));

            let headers: Vec<Cell> = view_data
                .headers
                .iter()
                .enumerate()
                .map(|(i, header)| {
                    let mut style = Style::default().fg(Color::Yellow);

                    if i == view_data.current_col_relative {
                        style = style.bg(Color::DarkGray).add_modifier(Modifier::BOLD);
                    }

                    if view_data.foreign_keys.contains_key(&i) {
                        Cell::from(format!("🔗 {}", header)).style(style)
                    } else {
                        Cell::from(header.as_str()).style(style)
                    }
                })
                .collect();

            let rows: Vec<Row> = view_data
                .rows
                .iter()
                .enumerate()
                .map(|(row_idx, row_data)| {
                    let cells: Vec<Cell> = row_data
                        .iter()
                        .enumerate()
                        .map(|(col_idx, cell)| {
                            let formatted = viewer.format_cell(cell);
                            let mut style = Style::default();

                            if row_idx == view_data.current_row_relative
                                && col_idx == view_data.current_col_relative
                            {
                                style = style
                                    .bg(Color::Blue)
                                    .fg(Color::White)
                                    .add_modifier(Modifier::BOLD);
                            }
                            style = style.bg(Color::DarkGray);

                            if view_data.foreign_keys.contains_key(&col_idx) {
                                style = style.fg(Color::Yellow);
                            }

                            Cell::from(formatted).style(style)
                        })
                        .collect();

                    Row::new(cells)
                })
                .collect();

            let widths: Vec<Constraint> = (0..view_data.headers.len())
                .map(|_| Constraint::Length(viewer.col_width as u16))
                .collect();

            let table = Table::new(rows, widths)
                .header(Row::new(headers).height(1).bottom_margin(1))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("🔗 Foreign Key Lookup")
                        .style(Style::default().fg(Color::Cyan)),
                )
                .column_spacing(1);

            f.render_widget(table, chunks[0]);

            self.render_foreign_key_status(f, viewer, &view_data, chunks[1]);
        }
    }

    fn render_foreign_key_status(
        &self,
        f: &mut Frame,
        viewer: &crate::table_viewer::TableViewer,
        view_data: &crate::table_viewer::TableViewData,
        area: Rect,
    ) {
        let current_cell_value =
            if let Some(row) = view_data.rows.get(view_data.current_row_relative) {
                if let Some(cell) = row.get(view_data.current_col_relative) {
                    viewer.format_cell(cell)
                } else {
                    "N/A".to_string()
                }
            } else {
                "N/A".to_string()
            };

        let fk_indicator = if viewer.foreign_keys.contains_key(&viewer.current_col) {
            " [FK]"
        } else {
            ""
        };

        let status_text = format!(
            "FK View | Rows: {}/{} | Cell: ({},{}):{} | Value: {} | y: Yank | Esc/q: Close",
            viewer.current_row + 1,
            view_data.total_rows,
            viewer.current_row + 1,
            viewer.current_col + 1,
            fk_indicator,
            if current_cell_value.len() > 20 {
                format!("{}...", &current_cell_value[..17])
            } else {
                current_cell_value
            }
        );

        let status = Paragraph::new(status_text)
            .style(Style::default().fg(Color::White).bg(Color::DarkGray));

        f.render_widget(status, area);
    }
}
