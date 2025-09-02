use ratatui::{
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
    pub fn render_help(&self, f: &mut Frame, _app: &App) {
        let area = centered_rect(70, 80, f.area());
        f.render_widget(Clear, area);

        let help_text = vec![
            Line::from(Span::styled(
                "SQLk Help",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "File View Mode:",
                Style::default().fg(Color::Yellow),
            )),
            Line::from("  e       - Execute query under cursor"),
            Line::from("  j/k     - Navigate up/down"),
            Line::from("  q/Esc   - Quit"),
            Line::from(""),
            Line::from(Span::styled(
                "Table Viewer Mode:",
                Style::default().fg(Color::Yellow),
            )),
            Line::from("  hjkl    - Navigate table"),
            Line::from("  c       - Toggle column statistics chart"),
            Line::from("  K       - Show comprehensive cell information"),
            Line::from("  F       - Direct foreign key lookup"),
            Line::from("  /       - Search table"),
            Line::from("  yy      - Yank (copy) entire row"),
            Line::from("  yiw     - Yank current cell value"),
            Line::from("  q/Esc   - Return to file view"),
            Line::from(""),
            Line::from(Span::styled(
                "Cell Information View:",
                Style::default().fg(Color::Yellow),
            )),
            Line::from("  F       - View foreign key data (if available)"),
            Line::from("  Esc/q   - Close and return to table"),
            Line::from(""),
            Line::from(Span::styled(
                "Foreign Key View:",
                Style::default().fg(Color::Yellow),
            )),
            Line::from("  hjkl    - Navigate foreign key results"),
            Line::from("  yy/yiw  - Yank row/cell from FK data"),
            Line::from("  Esc/q   - Close FK view"),
            Line::from(""),
            Line::from(Span::styled(
                "Press ? or Esc to close help",
                Style::default().fg(Color::Green),
            )),
        ];

        let help_widget = Paragraph::new(help_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help")
                .style(Style::default().fg(Color::White)),
        );

        f.render_widget(help_widget, area);
    }
}
