pub mod fk;
pub mod manager;
pub mod postgres;
pub mod query_result;

pub use manager::*;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub row_count: usize,
    pub execution_time: Option<std::time::Duration>,
    pub column_types: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ForeignKeyInfo {
    pub column_name: String,
    pub referenced_table: String,
    pub referenced_column: String,
}

pub type SchemaCache = HashMap<String, Vec<ForeignKeyInfo>>;
