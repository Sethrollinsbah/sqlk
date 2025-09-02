use anyhow::Result;
use std::path::PathBuf;

use crate::{application::app::App, ui::ToastType};

impl App {
    pub async fn load_file(&mut self, file_path: &PathBuf) -> Result<()> {
        match std::fs::read_to_string(file_path) {
            Ok(content) => {
                self.file_content = content;
                self.current_file = Some(file_path.clone());
                self.cursor_line = 0;
                self.scroll_offset = 0;

                self.query_blocks = self.query_parser.parse_query_blocks(&self.file_content)?;

                if self.startup_complete || !self.config.matrix.enabled {
                    self.ui.add_toast(
                        format!("Loaded file {}", file_path.display()),
                        ToastType::Info,
                    );
                }
            }
            Err(e) => {
                self.ui
                    .add_toast(format!("Failed to load file: {}", e), ToastType::Error);
            }
        }
        Ok(())
    }
}
