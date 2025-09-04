mod cell;
mod chart;
mod fk;
mod footer;
mod help;
mod home;
mod matrix;
mod search;
mod table;
mod toast;

pub use toast::*;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{ListState, TableState},
    Frame,
};

use crate::application::{app::App, state::AppMode};

pub struct UI {
    pub list_state: ListState,
    pub table_state: TableState,
    pub toast_messages: Vec<ToastMessage>,
    pub toast_type: ToastType,
    pub last_mouse_position: Option<(u16, u16)>,
    pub spinner_frame: usize,
}

impl Default for UI {
    fn default() -> Self {
        Self::new("DEBUG".to_string())
    }
}

impl UI {
    pub fn new(toast_level: String) -> Self {
        let toast_type: ToastType = ToastType::from(toast_level);
        Self {
            list_state: ListState::default(),
            table_state: TableState::default(),
            toast_messages: Vec::new(),
            last_mouse_position: None,
            spinner_frame: 0,
            toast_type
        }
    }

    pub fn update_mouse_position(&mut self, x: u16, y: u16) {
        self.last_mouse_position = Some((x, y));
    }

    pub fn render(&mut self, f: &mut Frame, app: &App) {
        self.spinner_frame = self.spinner_frame.wrapping_add(1);
        self.update_toasts(f.area());

        match app.current_mode {
            AppMode::FileView => self.render_file_view(f, app),
            AppMode::TableViewer | AppMode::ForeignKeyView | AppMode::CellInfoView => {
                self.render_table_viewer(f, app)
            }
            AppMode::MatrixLoading => self.render_matrix_loading(f, app),
            AppMode::Help => self.render_help(f, app),
            AppMode::Searching => self.render_search_input(f, app),
        }

        if app.current_mode == AppMode::ForeignKeyView {
            self.render_foreign_key_viewer(f, app);
        } else if app.current_mode == AppMode::CellInfoView {
            self.render_cell_info_viewer(f, app);
        } else if app.current_mode == AppMode::Help {
            self.render_help(f, app);
        }

        if app.current_mode == AppMode::TableViewer
            && let Some(viewer) = &app.table_viewer
                && viewer.show_chart {
                    self.render_chart_popup(f, viewer);
                }

        self.render_toast_notifications(f);
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
