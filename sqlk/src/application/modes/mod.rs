use anyhow::Result;
use cli_clipboard::ClipboardProvider;
use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    application::{app::App, state::AppMode},
    ui::ToastType,
};

impl App {
    pub async fn handle_searching_keys(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Enter => {
                if let Some(viewer) = &mut self.table_viewer {
                    viewer.search(&self.search_input);
                }
                self.current_mode = AppMode::TableViewer;
            }
            KeyCode::Char(c) => self.search_input.push(c),
            KeyCode::Backspace => {
                self.search_input.pop();
            }
            KeyCode::Esc => {
                self.search_input.clear();
                self.current_mode = AppMode::TableViewer;
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn handle_foreign_key_viewer_keys(&mut self, key: KeyEvent) -> Result<()> {
        if !self.yank_sequence.is_empty() {
            match key.code {
                KeyCode::Char(c) => self.yank_sequence.push(c),
                _ => {
                    self.yank_sequence.clear();
                    return Ok(());
                }
            }

            let sequence = self.yank_sequence.clone();
            let should_reset = !matches!(sequence.as_str(), "y" | "yi");

            if should_reset {
                self.yank_sequence.clear();

                match sequence.as_str() {
                    "yy" => {
                        if let Some(viewer) = &self.foreign_key_viewer {
                            if let Some((headers, row_values)) =
                                viewer.get_current_row_with_headers()
                            {
                                let formatted_row = headers
                                    .iter()
                                    .zip(row_values.iter())
                                    .map(|(header, value)| format!("{}: {}", header, value))
                                    .collect::<Vec<String>>()
                                    .join(", ");

                                match self.clipboard.set_contents(formatted_row.clone()) {
                                    Ok(_) => {
                                        self.ui.add_toast(
                                            format!("Yanked: {}", formatted_row),
                                            ToastType::Success,
                                        );
                                    }
                                    Err(e) => {
                                        self.ui.add_toast(
                                            format!("Failed to copy to clipboard: {}", e),
                                            ToastType::Error,
                                        );
                                    }
                                }
                            } else {
                                self.ui
                                    .add_toast("No row to yank".to_string(), ToastType::Info);
                            }
                        }
                    }
                    "yiw" => {
                        if let Some(viewer) = &self.foreign_key_viewer {
                            if let Some(cell_value) = viewer.get_current_cell_value() {
                                match self.clipboard.set_contents(cell_value.to_string()) {
                                    Ok(_) => {
                                        self.ui.add_toast(
                                            format!("Yanked: {}", cell_value),
                                            ToastType::Success,
                                        );
                                    }
                                    Err(e) => {
                                        self.ui.add_toast(
                                            format!("Failed to copy to clipboard: {}", e),
                                            ToastType::Error,
                                        );
                                    }
                                }
                            } else {
                                self.ui
                                    .add_toast("No cell to yank".to_string(), ToastType::Info);
                            }
                        }
                    }
                    _ => {}
                }
            }
            return Ok(());
        }

        match key.code {
            KeyCode::Char('y') => {
                self.yank_sequence.push('y');
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                self.foreign_key_viewer = None;
                self.current_mode = AppMode::TableViewer;
            }
            KeyCode::Char('?') => {
                self.previous_mode = Some(self.current_mode.clone());
                self.current_mode = AppMode::Help;
            }
            _ => {
                if let Some(viewer) = &mut self.foreign_key_viewer {
                    viewer.handle_key(key).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn handle_file_view_keys(&mut self, key: KeyEvent) -> Result<()> {
        if let KeyCode::Char(c) = key.code &&
            c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap() as usize;
                let current_multiplier = self.movement_multiplier.unwrap_or(0);
                self.movement_multiplier = Some(current_multiplier * 10 + digit);
                return Ok(());
        }

        let count = self.movement_multiplier.take().unwrap_or(1);

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.cursor_line = self.cursor_line.saturating_sub(count);
                self.adjust_scroll();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max_line = self.file_content.lines().count().saturating_sub(1);
                self.cursor_line = (self.cursor_line + count).min(max_line);
                self.adjust_scroll();
            }
            KeyCode::PageUp => {
                let page_size = 10;
                self.cursor_line = self.cursor_line.saturating_sub(count * page_size);
                self.adjust_scroll();
            }
            KeyCode::PageDown => {
                let page_size = 10;
                let max_line = self.file_content.lines().count().saturating_sub(1);
                self.cursor_line = (self.cursor_line + (count * page_size)).min(max_line);
                self.adjust_scroll();
            }
            KeyCode::Home => {
                self.cursor_line = 0;
                self.adjust_scroll();
            }
            KeyCode::End => {
                self.cursor_line = self.file_content.lines().count().saturating_sub(1);
                self.adjust_scroll();
            }
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('e') => self.execute_current_query().await?,
            KeyCode::Char('?') => {
                self.previous_mode = Some(self.current_mode.clone());
                self.current_mode = AppMode::Help;
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn handle_table_viewer_keys(&mut self, key: KeyEvent) -> Result<()> {
        if !self.yank_sequence.is_empty() {
            match key.code {
                KeyCode::Char(c) => self.yank_sequence.push(c),
                _ => {
                    self.yank_sequence.clear();
                    return Ok(());
                }
            }

            let sequence = self.yank_sequence.clone();
            let should_reset = !matches!(sequence.as_str(), "y" | "yi");

            if should_reset {
                self.yank_sequence.clear();

                match sequence.as_str() {
                    "yy" => {
                        if let Some(viewer) = &self.table_viewer {
                            if let Some((headers, row_values)) =
                                viewer.get_current_row_with_headers()
                            {
                                let formatted_row = headers
                                    .iter()
                                    .zip(row_values.iter())
                                    .map(|(header, value)| format!("{}: {}", header, value))
                                    .collect::<Vec<String>>()
                                    .join(", ");

                                match self.clipboard.set_contents(formatted_row.clone()) {
                                    Ok(_) => {
                                        self.ui.add_toast(
                                            format!("Yanked: {}", formatted_row),
                                            ToastType::Success,
                                        );
                                    }
                                    Err(e) => {
                                        self.ui.add_toast(
                                            format!("Failed to copy to clipboard: {}", e),
                                            ToastType::Error,
                                        );
                                    }
                                }
                            }
                        } else {
                            self.ui
                                .add_toast("No row to yank".to_string(), ToastType::Info);
                        }
                    }
                    "yiw" => {
                        if let Some(viewer) = &self.table_viewer {
                            if let Some(cell_value) = viewer.get_current_cell_value() {
                                match self.clipboard.set_contents(cell_value.to_string()) {
                                    Ok(_) => {
                                        self.ui.add_toast(
                                            format!("Yanked: {}", cell_value),
                                            ToastType::Success,
                                        );
                                    }
                                    Err(e) => {
                                        self.ui.add_toast(
                                            format!("Failed to copy to clipboard: {}", e),
                                            ToastType::Error,
                                        );
                                    }
                                }
                            }
                        } else {
                            self.ui
                                .add_toast("No cell to yank".to_string(), ToastType::Info);
                        }
                    }
                    _ => {}
                }
            }
            return Ok(());
        }

        match key.code {
            KeyCode::Char('y') => {
                self.yank_sequence.push('y');
            }
            KeyCode::Char('K') => {
                if let Some(viewer) = &self.table_viewer {
                    match viewer.get_cell_info(self).await {
                        Ok(cell_info) => {
                            self.cell_info = Some(cell_info);
                            self.current_mode = AppMode::CellInfoView;
                        }
                        Err(e) => {
                            self.ui.add_toast(
                                format!("Failed to get cell info: {}", e),
                                ToastType::Error,
                            );
                        }
                    }
                }
            }
            KeyCode::Char('F') => {
                if let Some(viewer) = &self.table_viewer {
                    match viewer.lookup_foreign_key(self).await {
                        Ok(Some(new_viewer)) => {
                            self.foreign_key_viewer = Some(new_viewer);
                            self.current_mode = AppMode::ForeignKeyView;
                        }
                        Ok(None) => {
                            self.ui.add_toast(
                                "No foreign key data found for this cell".to_string(),
                                ToastType::Info,
                            );
                        }
                        Err(e) => {
                            self.ui
                                .add_toast(format!("FK lookup failed: {}", e), ToastType::Error);
                        }
                    }
                }
            }
            KeyCode::Char('/') => {
                self.search_input.clear();
                self.current_mode = AppMode::Searching;
            }
            _ => {
                if let Some(viewer) = &mut self.table_viewer {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            if viewer.show_chart {
                                viewer.toggle_chart(0);
                            } else {
                                self.current_mode = AppMode::FileView;
                                self.table_viewer = None;
                            }
                        }
                        KeyCode::Char('?') => {
                            self.previous_mode = Some(self.current_mode.clone());
                            self.current_mode = AppMode::Help;
                        }
                        _ => {
                            viewer.handle_key(key).await?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn handle_help_keys(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('?') => {
                self.current_mode = self.previous_mode.take().unwrap_or(AppMode::FileView);
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn handle_cell_info_keys(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.cell_info = None;
                self.current_mode = AppMode::TableViewer;
            }
            KeyCode::Char('F') => {
                if let Some(cell_info) = &self.cell_info &&
                    cell_info.foreign_key_info.is_some() &&
                        let Some(viewer) = &self.table_viewer {
                            match viewer.lookup_foreign_key(self).await {
                                Ok(Some(new_viewer)) => {
                                    self.foreign_key_viewer = Some(new_viewer);
                                    self.current_mode = AppMode::ForeignKeyView;
                                }
                                Ok(None) => {}
                                Err(e) => {
                                    self.ui.add_toast(
                                        format!("FK lookup failed: {}", e),
                                        ToastType::Error,
                                    );
                                }
                            }
                }
            }
            KeyCode::Char('?') => {
                self.previous_mode = Some(self.current_mode.clone());
                self.current_mode = AppMode::Help;
            }
            _ => {}
        }
        Ok(())
    }
}
