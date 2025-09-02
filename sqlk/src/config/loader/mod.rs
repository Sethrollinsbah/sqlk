use super::parser::DatabaseUrlParser;
use super::{Config, DatabaseConfig};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub struct ConfigLoader {
    config_dir: PathBuf,
}

impl ConfigLoader {
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("sqlk");

        Self { config_dir }
    }

    pub fn with_config_dir(config_dir: PathBuf) -> Self {
        Self { config_dir }
    }

    pub fn load(&self, env_file: &Path) -> Result<Config> {
        let mut config = Config {
            env_file: env_file.to_path_buf(),
            ..Default::default()
        };

        let config_path = self.config_dir.join("config.toml");
        if config_path.exists() {
            config = self.load_config_file(&config_path)?;
            config.env_file = env_file.to_path_buf(); 
        }

        config.database = self.load_database_config(env_file)?;

        Ok(config)
    }

    pub fn save(&self, config: &Config) -> Result<()> {
        std::fs::create_dir_all(&self.config_dir)?;

        let config_path = self.config_dir.join("config.toml");
        let config_content = toml::to_string_pretty(config)?;
        std::fs::write(config_path, config_content)?;

        Ok(())
    }

    fn load_config_file(&self, config_path: &Path) -> Result<Config> {
        let config_content = std::fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }

    fn load_database_config(&self, env_file: &Path) -> Result<DatabaseConfig> {
        let mut database_config = DatabaseConfig::new();

        if env_file.exists()
            && let Some(url) = self.load_database_url_from_file(env_file)? {
                database_config = database_config.with_url(url);
            }

        if database_config.url.is_none()
            && let Ok(url) = std::env::var("DATABASE_URL") {
                database_config = database_config.with_url(url);
            }

        Ok(database_config)
    }

    fn load_database_url_from_file(&self, env_file: &Path) -> Result<Option<String>> {
        let env_content = std::fs::read_to_string(env_file)?;

        for line in env_content.lines() {
            if let Some(url) = DatabaseUrlParser::parse_from_line(line) {
                if let Err(_e) = DatabaseUrlParser::validate_url(&url) {
                    continue;
                }
                return Ok(Some(url));
            }
        }

        Ok(None)
    }

    pub fn get_config_path(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    pub fn config_exists(&self) -> bool {
        self.get_config_path().exists()
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}
