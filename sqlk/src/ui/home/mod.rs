use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{application::app::App, ui::UI};

impl UI {
    pub fn render_file_content(&self, f: &mut Frame, app: &App, area: Rect) {
        let visible_content = app.get_visible_file_content(area.height as usize);
        let start_line_0_based = app.scroll_offset;

        let highlighted_query = app.get_current_query_block();

        let lines: Vec<Line> = visible_content
        .iter()
        .enumerate()
        .map(|(i, line_text)| {
            let current_line_0_based = start_line_0_based + i;
            let current_line_1_based = current_line_0_based + 1;

            let mut style = Style::default();

            if let Some(query) = highlighted_query
                && current_line_1_based >= query.start_line && current_line_1_based <= query.end_line {
                    style = style.bg(Color::Rgb(20, 40, 60));
                }

            if current_line_0_based == app.cursor_line {
                style = style.bg(Color::DarkGray);
            }

            let spans = vec![
                Span::styled(
                    format!("{:4} ", current_line_1_based),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(line_text.clone(), style),
            ];

            Line::from(spans)
        })
        .collect();

        let content = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title("File Content")
                .style(Style::default().fg(Color::White)),
        );

        f.render_widget(content, area);
    }
    pub fn render_file_view(&self, f: &mut Frame, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.area());

        let header_text = if let Some(ref file_path) = app.current_file {
            format!("SQLk - {}", file_path.display())
        } else {
            "SQLk - No file loaded".to_string()
        };

        let header = Paragraph::new(header_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("SQL File Viewer")
                    .style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Center);
        f.render_widget(header, chunks[0]);

        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(chunks[1]);

        self.render_file_content(f, app, content_chunks[0]);

        self.render_query_details_view(f, app, content_chunks[1]);

        self.render_footer(f, app, chunks[2]);
    }
    fn render_query_details_view(&self, f: &mut Frame, app: &App, area: Rect) {
        let details_block = Block::default()
            .title("Query Details")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow));

        if let Some(query) = app.get_current_query_block() {
            let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
            let spinner_char = spinner_chars[self.spinner_frame % spinner_chars.len()];

            let status_line = if app.is_querying {
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(
                        format!("{} EXECUTING", spinner_char),
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
                    ),
                ])
            } else {
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("● Selected", Style::default().fg(Color::Cyan)),
                ])
            };

            let details_text = vec![
                Line::from(Span::styled(
                    "Status:",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )),
                status_line,
                Line::from(""),
                Line::from(Span::styled(
                    "Lines:",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(format!("  {} to {}", query.start_line, query.end_line)),
                Line::from(""), // Spacer
                Line::from(Span::styled(
                    "Query Text:",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(
                    query
                        .text
                        .lines()
                        .take(10)
                        .map(|l| format!("  {}", l))
                        .collect::<Vec<_>>()
                        .join("\n"),
                ),
            ];

            let paragraph = Paragraph::new(details_text)
                .block(details_block)
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, area);
        } else {
            let help_text = if app.is_querying {
                let spinner_char =
                    ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'][self.spinner_frame % 10];
                format!("\n\n{} Executing query...\n\nPlease wait...", spinner_char)
            } else {
                "\n\nMove cursor over a SQL query\nto see details here.".to_string()
            };

            let paragraph = Paragraph::new(help_text)
                .block(details_block)
                .style(if app.is_querying {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                })
                .alignment(Alignment::Center);
            f.render_widget(paragraph, area);
        }
    }
}
