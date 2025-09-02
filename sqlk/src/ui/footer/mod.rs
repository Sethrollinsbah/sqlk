use crate::{
    application::{app::App, state::AppMode},
    ui::UI,
};

use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

impl UI {
    pub fn render_footer(&self, f: &mut Frame, app: &App, area: Rect) {
        let footer_text = match app.current_mode {
            AppMode::FileView => "e: Exec | ?: Help | q: Quit",
            AppMode::TableViewer => {
                "hjkl: Nav | c: Chart | K: Cell Info | F: FK Lookup | /: Search | ?: Help | q: Back"
            }
            AppMode::ForeignKeyView => "hjkl: Nav | Esc/q: Close",
            AppMode::CellInfoView => "F: View FK Data | Esc/q: Close | ?: Help",
            AppMode::MatrixLoading => "q: Quit",
            AppMode::Help => "?/Esc: Close",
            AppMode::Searching => "Searching...",
        };

        let footer = Paragraph::new(footer_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .alignment(Alignment::Center);

        f.render_widget(footer, area);
    }
}
