use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignKeyConfig {
    pub enabled: bool,
    pub manual_mapping: HashMap<String, String>,
}

impl Default for ForeignKeyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            manual_mapping: HashMap::new(),
        }
    }
}

impl ForeignKeyConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }

    pub fn with_mapping(mut self, from: String, to: String) -> Self {
        self.manual_mapping.insert(from, to);
        self
    }

    pub fn add_mapping(&mut self, from: String, to: String) {
        self.manual_mapping.insert(from, to);
    }

    pub fn get_mapping(&self, column: &str) -> Option<&String> {
        self.manual_mapping.get(column)
    }
}
