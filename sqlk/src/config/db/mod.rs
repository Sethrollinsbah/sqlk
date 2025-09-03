use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseConfig {
    pub url: Option<String>,
    pub db_type: Option<DatabaseType>,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url.clone());
        self.db_type = Self::detect_type_from_url(&url);
        self
    }

    pub fn detect_type(&self) -> Option<DatabaseType> {
        if let Some(url) = &self.url {
            Self::detect_type_from_url(url)
        } else {
            self.db_type.clone()
        }
    }

    fn detect_type_from_url(url: &str) -> Option<DatabaseType> {
        if url.starts_with("postgres://") || url.starts_with("postgresql://") {
            Some(DatabaseType::PostgreSQL)
        } else if url.starts_with("mysql://") {
            Some(DatabaseType::MySQL)
        } else if url.starts_with("sqlite://") {
            Some(DatabaseType::SQLite)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DatabaseType {
    PostgreSQL,
    MySQL,
    SQLite,
}

impl DatabaseType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DatabaseType::PostgreSQL => "postgresql",
            DatabaseType::MySQL => "mysql",
            DatabaseType::SQLite => "sqlite",
        }
    }

    pub fn get_executable(&self) -> &'static str {
        match self {
            DatabaseType::PostgreSQL => "psql",
            DatabaseType::MySQL => "mysql",
            DatabaseType::SQLite => "sqlite3",
        }
    }

    pub fn get_default_port(&self) -> u16 {
        match self {
            DatabaseType::PostgreSQL => 5432,
            DatabaseType::MySQL => 3306,
            DatabaseType::SQLite => 0,
        }
    }
}
