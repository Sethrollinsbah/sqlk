use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph},
    Frame,
};

use crate::{application::app::App, ui::UI};

impl UI {
    pub fn render_matrix_loading(&self, f: &mut Frame, app: &App) {
        if let Some(ref matrix) = app.matrix_animation {
            let area = f.area();
            f.render_widget(Clear, area);

            let matrix_lines = matrix.get_frame();

            let matrix_text: Vec<Line> = matrix_lines
                .iter()
                .enumerate()
                .map(|(row_idx, line)| {
                    Line::from(
                        line.chars()
                            .enumerate()
                            .map(|(col_idx, c)| {
                                if let Some(sql_char) = matrix.get_overlay_char_at(row_idx, col_idx)
                                {
                                    if sql_char != ' ' {
                                        Span::styled(
                                            sql_char.to_string(),
                                            Style::default()
                                                .fg(Color::White)
                                                .add_modifier(Modifier::BOLD)
                                                .bg(Color::Black),
                                        )
                                    } else {
                                        let color = matrix.get_char_color(row_idx, col_idx);
                                        Span::styled(c.to_string(), Style::default().fg(color))
                                    }
                                } else {
                                    let color = matrix.get_char_color(row_idx, col_idx);
                                    Span::styled(c.to_string(), Style::default().fg(color))
                                }
                            })
                            .collect::<Vec<_>>(),
                    )
                })
                .collect();

            let matrix_widget = Paragraph::new(matrix_text).alignment(Alignment::Left);
            f.render_widget(matrix_widget, area);

            if area.height > 3 && area.width > 20 {
                let progress_area = Rect {
                    x: area.width - 22,
                    y: area.height - 3,
                    width: 20,
                    height: 1,
                };
                let progress = matrix.get_progress();
                let progress_text = format!("▓ {:.0}% ▓", progress * 100.0);
                let progress_widget =
                    Paragraph::new(progress_text).style(Style::default().fg(Color::DarkGray));
                f.render_widget(progress_widget, progress_area);
            }
        }
    }
}
