use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

use crate::application::{app::App, state::AppMode};

impl App {
    pub async fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        match self.current_mode {
            AppMode::FileView => self.handle_file_view_keys(key).await?,
            AppMode::TableViewer => self.handle_table_viewer_keys(key).await?,
            AppMode::ForeignKeyView => self.handle_foreign_key_viewer_keys(key).await?,
            AppMode::Searching => self.handle_searching_keys(key).await?,
            AppMode::CellInfoView => self.handle_cell_info_keys(key).await?,
            AppMode::MatrixLoading => {
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                    self.should_quit = true;
                }
                if !self.startup_complete && matches!(key.code, KeyCode::Enter | KeyCode::Char(' '))
                {
                    self.matrix_animation = None;
                    self.perform_startup_tasks_sync().await?;
                }
            }
            AppMode::Help => self.handle_help_keys(key).await?,
        }
        Ok(())
    }
}
