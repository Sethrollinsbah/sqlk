use crate::{application::app::App, ui::UI};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

impl UI {
    pub fn render_table_viewer(&self, f: &mut Frame, app: &App) {
        if let Some(ref viewer) = app.table_viewer {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(2)])
                .split(f.area());

            let view_data = viewer.get_visible_data(f.area().width, f.area().height - 2);

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
                .map(|(row_idx, row)| {
                    let cells: Vec<Cell> = row
                        .iter()
                        .enumerate()
                        .map(|(col_idx, cell)| {
                            let formatted = viewer.format_cell(cell);
                            let mut style = Style::default();

                            if view_data.show_chart {
                                style = style.add_modifier(Modifier::DIM);
                            } else if row_idx == view_data.current_row_relative
                                && col_idx == view_data.current_col_relative
                            {
                                style = style
                                    .bg(Color::Blue)
                                    .fg(Color::White)
                                    .add_modifier(Modifier::BOLD);
                            } else if row_idx == view_data.current_row_relative {
                                style = style.bg(Color::DarkGray);
                            }

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
                        .title("SQL Results")
                        .style(Style::default().fg(Color::White)),
                )
                .column_spacing(1);

            f.render_widget(table, chunks[0]);
            self.render_table_status(f, app, viewer, &view_data, chunks[1]);
        }
    }

    fn render_table_status(
        &self,
        f: &mut Frame,
        _app: &App,
        viewer: &crate::table_viewer::TableViewer,
        view_data: &crate::table_viewer::TableViewData,
        area: Rect,
    ) {
        let fk_indicator = if viewer.foreign_keys.contains_key(&viewer.current_col) {
            " [FK]"
        } else {
            ""
        };

        let time_info = if let Some(duration) = view_data.execution_time {
            format!(" | Time: {}ms", duration.as_millis())
        } else {
            String::new()
        };

        let status_text = format!(
            "Rows: {}/{} | Cell: ({},{}):{} | Cols: {}-{}/{} | FK: K | Help: ? | Chart: c | Quit: q{}",
            viewer.current_row + 1,
            view_data.total_rows,
            viewer.current_row + 1,
            viewer.current_col + 1,
            fk_indicator,
            view_data.start_col + 1,
            view_data.start_col + view_data.headers.len(),
            view_data.total_cols,
            time_info
        );

        let status = Paragraph::new(status_text)
            .style(Style::default().fg(Color::White).bg(Color::DarkGray));

        f.render_widget(status, area);
    }
}
