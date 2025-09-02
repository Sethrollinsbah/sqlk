use std::collections::HashMap;

use anyhow::Result;
use unicode_width::UnicodeWidthStr;

use crate::config::Config;
use crate::database::DatabaseManager;
use crate::database::ForeignKeyInfo;

use crate::database::QueryResult;
use crate::table_viewer::ChartData;
use crate::table_viewer::ColumnStats;
use crate::table_viewer::ForeignKeyLookupResult;
use crate::table_viewer::SearchState;

#[derive(Debug)]
pub struct CellInfo {
    pub value: String,
    pub column_name: String,
    pub column_index: usize,
    pub row_index: usize,
    pub data_type: Option<String>,
    pub is_null: bool,
    pub value_length: usize,
    pub duplicate_count: usize,
    pub unique_values_in_column: usize,
    pub percentage_of_total: f64,
    pub foreign_key_info: Option<ForeignKeyLookupResult>,
}

#[derive(Debug, Clone, Copy)]
pub struct CellPosition {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct TableViewData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub current_row_relative: usize,
    pub current_col_relative: usize,
    pub total_rows: usize,
    pub total_cols: usize,
    pub start_row: usize,
    pub start_col: usize,
    pub foreign_keys: HashMap<usize, ForeignKeyInfo>,
    pub search_matches: Vec<CellPosition>,
    pub execution_time: Option<std::time::Duration>,
    pub column_types: Vec<String>,
    pub show_chart: bool,
    pub chart_display: Option<Vec<String>>,
}
#[derive(Debug)]
pub struct TableViewer {
    pub data: QueryResult,
    pub current_row: usize,
    pub current_col: usize,
    pub scroll_offset_y: usize,
    pub scroll_offset_x: usize,
    pub search_state: SearchState,
    pub foreign_keys: HashMap<usize, ForeignKeyInfo>,
    pub col_width: usize,
    pub movement_multiplier: Option<usize>,
    pub column_stats: HashMap<usize, ColumnStats>,
    pub show_chart: bool,
    pub chart_data: Option<ChartData>,
}

impl TableViewer {
    pub fn new(data: QueryResult, _config: &Config, db_manager: &DatabaseManager) -> Result<Self> {
        let mut foreign_keys = HashMap::new();
        let mut column_stats = HashMap::new();

        for (idx, header) in data.headers.iter().enumerate() {
            if let Ok(fk_info) = db_manager.get_foreign_key_info(header) {
                foreign_keys.insert(idx, fk_info.clone());
            }

            let stats = Self::compute_column_stats(&data, idx);
            column_stats.insert(idx, stats);
        }

        Ok(Self {
            data,
            current_row: 0,
            current_col: 0,
            scroll_offset_y: 0,
            scroll_offset_x: 0,
            search_state: SearchState::default(),
            foreign_keys,
            col_width: 20,
            movement_multiplier: None,
            column_stats,
            show_chart: false,
            chart_data: None,
        })
    }

    pub fn get_column_type(&self, col_idx: usize) -> Option<&String> {
        self.data.column_types.get(col_idx)
    }

    pub fn get_column_types(&self) -> &Vec<String> {
        &self.data.column_types
    }

    pub fn get_current_cell_value(&self) -> Option<String> {
        self.data
            .rows
            .get(self.current_row)?
            .get(self.current_col)
            .cloned()
    }

    pub fn get_visible_data(&self, width: u16, height: u16) -> TableViewData {
        let available_width = width as usize;
        let available_height = height as usize;

        let col_width_with_padding = self.col_width + 3;
        let max_visible_cols = (available_width / col_width_with_padding).max(1);

        let start_col = self.scroll_offset_x;
        let end_col = (start_col + max_visible_cols).min(self.data.headers.len());

        let max_data_rows = available_height.saturating_sub(5);

        let start_row = self.scroll_offset_y;
        let end_row = (start_row + max_data_rows).min(self.data.rows.len());

        let headers = self.data.headers[start_col..end_col].to_vec();
        let rows = self.data.rows[start_row..end_row]
            .iter()
            .map(|row| {
                row.get(start_col..end_col.min(row.len()))
                    .unwrap_or_default()
                    .to_vec()
            })
            .collect();

        let column_types = self.data.column_types[start_col..end_col].to_vec();

        TableViewData {
            headers,
            rows,
            current_row_relative: self.current_row.saturating_sub(start_row),
            current_col_relative: self.current_col.saturating_sub(start_col),
            total_rows: self.data.rows.len(),
            total_cols: self.data.headers.len(),
            start_row,
            start_col,
            foreign_keys: self.get_visible_foreign_keys(start_col, end_col),
            search_matches: self.get_visible_search_matches(start_row, end_row, start_col, end_col),
            execution_time: self.data.execution_time,
            column_types,
            show_chart: self.show_chart,
            chart_display: if self.show_chart {
                Some(self.get_chart_display(available_width.saturating_sub(4)))
            } else {
                None
            },
        }
    }

    fn get_visible_foreign_keys(
        &self,
        start_col: usize,
        end_col: usize,
    ) -> HashMap<usize, ForeignKeyInfo> {
        self.foreign_keys
            .iter()
            .filter(|(col_idx, _)| **col_idx >= start_col && **col_idx < end_col)
            .map(|(col_idx, fk_info)| (col_idx - start_col, fk_info.clone()))
            .collect()
    }

    pub fn format_cell(&self, content: &str) -> String {
        let display_width = UnicodeWidthStr::width(content);

        if display_width > self.col_width {
            let mut truncated = String::new();
            let mut current_width = 0;

            for ch in content.chars() {
                let char_width = UnicodeWidthStr::width(ch.to_string().as_str());
                if current_width + char_width > self.col_width.saturating_sub(3) {
                    truncated.push_str("...");
                    break;
                }
                truncated.push(ch);
                current_width += char_width;
            }

            let padding_needed = self
                .col_width
                .saturating_sub(UnicodeWidthStr::width(truncated.as_str()));
            truncated + &" ".repeat(padding_needed)
        } else {
            let padding_needed = self.col_width.saturating_sub(display_width);
            content.to_string() + &" ".repeat(padding_needed)
        }
    }

    pub fn get_current_row_with_headers(&self) -> Option<(&Vec<String>, &Vec<String>)> {
        if let Some(row) = self.data.rows.get(self.current_row) {
            Some((&self.data.headers, row))
        } else {
            None
        }
    }
}
