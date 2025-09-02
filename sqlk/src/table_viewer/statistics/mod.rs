use std::collections::HashMap;

use crate::{
    application::app::App,
    database::QueryResult,
    table_viewer::{CellInfo, TableViewer},
};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ColumnStats {
    pub data_type: Option<String>,
    pub value_counts: HashMap<String, usize>,
    pub unique_count: usize,
    pub null_count: usize,
}

impl TableViewer {
    pub fn compute_column_stats(data: &QueryResult, col_idx: usize) -> ColumnStats {
        let mut value_counts: HashMap<String, usize> = HashMap::new();
        let mut null_count = 0;

        for row in &data.rows {
            if let Some(cell_value) = row.get(col_idx) {
                if cell_value.is_empty() || cell_value == "NULL" {
                    null_count += 1;
                } else {
                    *value_counts.entry(cell_value.clone()).or_insert(0) += 1;
                }
            }
        }

        let unique_count = value_counts.len();

        let data_type = data.column_types.get(col_idx).cloned();

        ColumnStats {
            data_type,
            value_counts,
            unique_count,
            null_count,
        }
    }

    pub async fn get_cell_info(&self, app: &App) -> Result<CellInfo> {
        let current_value = self.get_current_cell_value().unwrap_or_default();
        let column_name = self
            .data
            .headers
            .get(self.current_col)
            .cloned()
            .unwrap_or_default();

        let stats = self.column_stats.get(&self.current_col);

        let duplicate_count = stats
            .and_then(|s| s.value_counts.get(&current_value))
            .copied()
            .unwrap_or(0);

        let unique_values_in_column = stats.map(|s| s.unique_count).unwrap_or(0);

        let total_non_null_rows = self.data.rows.len() - stats.map(|s| s.null_count).unwrap_or(0);

        let percentage_of_total = if total_non_null_rows > 0 {
            (duplicate_count as f64 / total_non_null_rows as f64) * 100.0
        } else {
            0.0
        };

        let foreign_key_info = if self.foreign_keys.contains_key(&self.current_col) {
            self.lookup_foreign_key_info(app).await?
        } else {
            None
        };

        let data_type = stats.and_then(|s| s.data_type.clone());

        Ok(CellInfo {
            value: current_value.clone(),
            column_name: column_name.clone(),
            column_index: self.current_col,
            row_index: self.current_row,
            data_type,
            is_null: current_value.is_empty() || current_value == "NULL",
            value_length: current_value.len(),
            duplicate_count,
            unique_values_in_column,
            percentage_of_total,
            foreign_key_info,
        })
    }
}
