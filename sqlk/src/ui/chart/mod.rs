use crate::ui::{centered_rect, UI};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

impl UI {
    pub fn render_chart_popup(&self, f: &mut Frame, viewer: &crate::table_viewer::TableViewer) {
        if let Some(chart_data) = &viewer.chart_data {
            let area = centered_rect(80, 70, f.area());
            f.render_widget(Clear, area);

            let value_to_highlight = viewer.get_current_cell_value().unwrap_or_default();

            let is_highlight_value_in_top_items = chart_data
                .items
                .iter()
                .any(|item| item.label == value_to_highlight);

            let label_width = 25;
            let bar_width = 30;
            let mut chart_lines: Vec<Line> = Vec::new();

            let header = Line::from(vec![
                Span::styled(
                    format!("{:<width$}", "Value", width = label_width),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
                Span::styled(
                    format!("{:<width$}", "Distribution", width = bar_width),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
                Span::styled("Count (%)", Style::default().add_modifier(Modifier::BOLD)),
            ]);
            chart_lines.push(header);
            chart_lines.push(Line::from("─".repeat(area.width as usize - 4)));

            for item in &chart_data.items {
                let label_text = if item.label.len() > label_width {
                    format!("{}...", &item.label[..label_width - 3])
                } else {
                    item.label.clone()
                };

                let bar = "█".repeat(item.bar_length.min(bar_width));

                let bar_color = if item.label == value_to_highlight
                    || item.label == "<Others>" && !is_highlight_value_in_top_items
                {
                    Color::Red
                } else {
                    Color::Cyan
                };

                let line = Line::from(vec![
                    Span::styled(
                        format!("{:<width$}", label_text, width = label_width),
                        Style::default().fg(Color::White),
                    ),
                    Span::raw(" "),
                    Span::styled(
                        format!("{:<width$}", bar, width = bar_width),
                        Style::default().fg(bar_color),
                    ),
                    Span::raw(" "),
                    Span::styled(
                        format!("{} ({:.1}%)", item.count, item.percentage),
                        Style::default().fg(Color::Yellow),
                    ),
                ]);
                chart_lines.push(line);
            }

            let summary_text = format!(
                "Total Rows: {}, Unique Values: {}, Nulls: {}",
                chart_data.total_count, chart_data.unique_count, chart_data.null_count
            );

            let chart_paragraph = Paragraph::new(chart_lines)
                .block(
                    Block::default()
                        .title(format!(" 📊 {} ", chart_data.title))
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan))
                        .padding(ratatui::widgets::Padding::new(1, 1, 1, 1)),
                )
                .wrap(Wrap { trim: false });

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(area.inner(Margin {
                    vertical: 1,
                    horizontal: 1,
                }));

            f.render_widget(
                chart_paragraph.block(
                    Block::default()
                        .title(format!(" 📊 {} ", chart_data.title))
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan)),
                ),
                area,
            );

            let summary_paragraph = Paragraph::new(summary_text)
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);

            f.render_widget(summary_paragraph, chunks[1]);
        }
    }
}
