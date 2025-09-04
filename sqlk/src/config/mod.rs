pub mod db;
pub mod fk;
pub mod loader;
pub mod matrix;
pub mod parser;

pub use db::{DatabaseConfig, DatabaseType};
pub use fk::ForeignKeyConfig;
pub use loader::ConfigLoader;
pub use matrix::MatrixConfig;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub env_file: PathBuf,
    pub matrix: MatrixConfig,
    pub foreign_keys: ForeignKeyConfig,
    pub database: DatabaseConfig,
    pub toast_level: String
}


impl Default for Config {
    fn default() -> Self {
        Self {
            env_file: PathBuf::from(".env"),
            matrix: MatrixConfig::default(),
            foreign_keys: ForeignKeyConfig::default(),
            database: DatabaseConfig::default(),
            toast_level: String::from("ERROR")
        }
    }
}

impl Config {
    pub fn load(env_file: &std::path::Path, toast_level: String) -> Result<Self> {
        ConfigLoader::new().load(env_file, toast_level)
    }

    pub fn save(&self) -> Result<()> {
        ConfigLoader::new().save(self)
    }

    pub fn get_database_url(&self) -> Option<&str> {
        self.database.url.as_deref()
    }

    pub fn detect_database_type(&self) -> Option<DatabaseType> {
        self.database.detect_type()
    }
}
