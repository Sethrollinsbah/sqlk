use crate::database::QueryResult;
use anyhow::Result;
use sqlx::{postgres::PgRow, Column, Row, TypeInfo, ValueRef};
use std::time::Instant;

#[derive(Clone)]
pub struct PostgresFormatter;

impl Default for PostgresFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl PostgresFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn rows_to_query_result(
        &self,
        rows: Vec<PgRow>,
        start_time: Instant,
    ) -> Result<QueryResult> {
        let execution_time = start_time.elapsed();

        if rows.is_empty() {
            return Ok(QueryResult {
                headers: vec![],
                rows: vec![],
                row_count: 0,
                execution_time: Some(execution_time),
                column_types: vec![],
            });
        }

        let headers: Vec<String> = rows[0]
            .columns()
            .iter()
            .map(|col| col.name().to_string())
            .collect();

        let column_types: Vec<String> = rows[0]
            .columns()
            .iter()
            .enumerate()
            .map(|(idx, _col)| self.get_column_type_from_rows(&rows, idx))
            .collect();

        let mut result_rows = Vec::new();
        for row in &rows {
            let mut string_row = Vec::new();
            for col_idx in 0..row.columns().len() {
                let value = self.format_value(row, col_idx)?;
                string_row.push(value);
            }
            result_rows.push(string_row);
        }

        Ok(QueryResult {
            headers,
            rows: result_rows,
            row_count: rows.len(),
            execution_time: Some(execution_time),
            column_types,
        })
    }

    fn get_column_type_from_rows(&self, rows: &[PgRow], col_idx: usize) -> String {
        for row in rows {
            if let Ok(value_ref) = row.try_get_raw(col_idx) &&
                !value_ref.is_null() {
                    let type_info = value_ref.type_info();
                    return self.normalize_type_name(type_info.name());
            }
        }

        if let Some(first_row) = rows.first() &&
            let Ok(value_ref) = first_row.try_get_raw(col_idx) {
                let type_info = value_ref.type_info();
                return self.normalize_type_name(type_info.name());
        }

        "UNKNOWN".to_string()
    }

    fn normalize_type_name(&self, type_name: &str) -> String {
        match type_name {
            "TEXT" | "VARCHAR" | "CHAR" | "BPCHAR" => "TEXT".to_string(),
            "INT2" | "SMALLINT" => "SMALLINT".to_string(),
            "INT4" | "INTEGER" => "INTEGER".to_string(),
            "INT8" | "BIGINT" => "BIGINT".to_string(),
            "FLOAT4" | "REAL" => "REAL".to_string(),
            "FLOAT8" | "DOUBLE PRECISION" => "DOUBLE PRECISION".to_string(),
            "NUMERIC" | "DECIMAL" => "NUMERIC".to_string(),
            "BOOL" | "BOOLEAN" => "BOOLEAN".to_string(),
            "TIMESTAMP" => "TIMESTAMP".to_string(),
            "TIMESTAMPTZ" => "TIMESTAMPTZ".to_string(),
            "DATE" => "DATE".to_string(),
            "TIME" => "TIME".to_string(),
            "TIMETZ" => "TIMETZ".to_string(),
            "UUID" => "UUID".to_string(),
            "JSON" => "JSON".to_string(),
            "JSONB" => "JSONB".to_string(),
            other => other.to_string(),
        }
    }

    fn format_value(&self, row: &PgRow, col_idx: usize) -> Result<String> {
        let value_ref = row.try_get_raw(col_idx)?;
        if value_ref.is_null() {
            return Ok("NULL".to_string());
        }

        let type_info = value_ref.type_info();
        let type_name = type_info.name();

        match type_name {
            "TEXT" | "VARCHAR" | "CHAR" | "BPCHAR" => {
                Ok(row.try_get::<String, _>(col_idx).unwrap_or_default())
            }
            "INT2" | "INT4" | "INT8" | "SMALLINT" | "INTEGER" | "BIGINT" => {
                if let Ok(val) = row.try_get::<i64, _>(col_idx) {
                    Ok(val.to_string())
                } else if let Ok(val) = row.try_get::<i32, _>(col_idx) {
                    Ok(val.to_string())
                } else if let Ok(val) = row.try_get::<i16, _>(col_idx) {
                    Ok(val.to_string())
                } else {
                    Ok("0".to_string())
                }
            }
            "FLOAT4" | "FLOAT8" | "REAL" | "DOUBLE PRECISION" | "NUMERIC" | "DECIMAL" => {
                if let Ok(val) = row.try_get::<f64, _>(col_idx) {
                    Ok(val.to_string())
                } else if let Ok(val) = row.try_get::<f32, _>(col_idx) {
                    Ok(val.to_string())
                } else {
                    Ok("0.0".to_string())
                }
            }
            "BOOL" | "BOOLEAN" => {
                if let Ok(val) = row.try_get::<bool, _>(col_idx) {
                    Ok(val.to_string())
                } else {
                    Ok("false".to_string())
                }
            }
            "TIMESTAMP" => Ok(row
                .try_get::<chrono::NaiveDateTime, _>(col_idx)
                .map_or("".to_string(), |v| {
                    v.format("%Y-%m-%d %H:%M:%S").to_string()
                })),
            "TIMESTAMPTZ" => Ok(row
                .try_get::<chrono::DateTime<chrono::Utc>, _>(col_idx)
                .map_or("".to_string(), |v| {
                    v.format("%Y-%m-%d %H:%M:%S %Z").to_string()
                })),
            "DATE" => Ok(row
                .try_get::<chrono::NaiveDate, _>(col_idx)
                .map_or("".to_string(), |v| v.format("%Y-%m-%d").to_string())),
            "TIME" | "TIMETZ" => Ok(row.try_get::<String, _>(col_idx).unwrap_or_default()),
            "UUID" => {
                if let Ok(val) = row.try_get::<String, _>(col_idx) {
                    Ok(val)
                } else {
                    Ok("".to_string())
                }
            }
            "JSON" | "JSONB" => Ok(row.try_get::<String, _>(col_idx).unwrap_or_default()),
            _ => row
                .try_get::<String, _>(col_idx)
                .or_else(|_| Ok(format!("<{}>", type_name))),
        }
    }
}
