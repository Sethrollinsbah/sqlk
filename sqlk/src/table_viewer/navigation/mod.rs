use crate::table_viewer::TableViewer;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

impl TableViewer {
    pub async fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        if let KeyCode::Char(c) = key.code
            && c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap() as usize;
                let current_multiplier = self.movement_multiplier.unwrap_or(0);
                self.movement_multiplier = Some(current_multiplier * 10 + digit);
                return Ok(());
            }

        let count = self.movement_multiplier.take().unwrap_or(1);

        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.move_cursor_down(count),
            KeyCode::Char('k') | KeyCode::Up => self.move_cursor_up(count),
            KeyCode::Char('h') | KeyCode::Left => self.move_cursor_left(count),
            KeyCode::Char('l') | KeyCode::Right => self.move_cursor_right(count),

            KeyCode::PageDown => self.move_cursor_down(10 * count),
            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.move_cursor_down(10 * count)
            }
            KeyCode::PageUp => self.move_cursor_up(10 * count),
            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.move_cursor_up(10 * count)
            }

            KeyCode::Char('g') => {
                self.current_row = 0;
                self.update_scroll();
            }
            KeyCode::Char('G') => {
                self.current_row = self.data.rows.len().saturating_sub(1);
                self.update_scroll();
            }
            KeyCode::Char('_') | KeyCode::Home => {
                self.current_col = 0;
                self.update_scroll();
            }
            KeyCode::Char('$') | KeyCode::End => {
                self.current_col = self.data.headers.len().saturating_sub(1);
                self.update_scroll();
            }

            KeyCode::Char('n') => self.next_search_match(1),
            KeyCode::Char('N') => self.prev_search_match(1),

            KeyCode::Char('c') => {
                let bar_width = 30;
                self.toggle_chart(bar_width);
            }

            _ => {}
        }
        Ok(())
    }

    fn move_cursor_down(&mut self, count: usize) {
        if !self.data.rows.is_empty() {
            self.current_row = (self.current_row + count).min(self.data.rows.len() - 1);
            self.update_scroll();
        }
    }

    fn move_cursor_up(&mut self, count: usize) {
        self.current_row = self.current_row.saturating_sub(count);
        self.update_scroll();
    }

    fn move_cursor_left(&mut self, count: usize) {
        self.current_col = self.current_col.saturating_sub(count);
        self.update_scroll();

        if self.show_chart {
            self.show_chart = false;
            self.chart_data = None;
        }
    }

    fn move_cursor_right(&mut self, count: usize) {
        if !self.data.headers.is_empty() {
            self.current_col = (self.current_col + count).min(self.data.headers.len() - 1);
            self.update_scroll();

            if self.show_chart {
                self.show_chart = false;
                self.chart_data = None;
            }
        }
    }

    pub fn update_scroll(&mut self) {
        let visible_rows = 20;
        if self.current_row < self.scroll_offset_y {
            self.scroll_offset_y = self.current_row;
        } else if self.current_row >= self.scroll_offset_y + visible_rows {
            self.scroll_offset_y = self.current_row - visible_rows + 1;
        }

        let visible_cols = 5;
        if self.current_col < self.scroll_offset_x {
            self.scroll_offset_x = self.current_col;
        } else if self.current_col >= self.scroll_offset_x + visible_cols {
            self.scroll_offset_x = self.current_col - visible_cols + 1;
        }
    }
}
