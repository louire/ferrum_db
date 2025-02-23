use anyhow::Result;
use config::{Config, File};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Represents the application configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    /// Database connection settings
    pub database: DatabaseConfig,
}

/// Database connection configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database host
    pub host: String,
    /// Database port
    pub port: u16,
    /// Database username
    pub username: String,
    /// Database password
    pub password: String,
    /// Database name
    pub database: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                username: "postgres".to_string(),
                password: "postgres".to_string(),
                database: "postgres".to_string(),
            },
        }
    }
}

impl AppConfig {
    /// Loads configuration from config.toml file
    pub fn load() -> Result<Self> {
        let config_path = Path::new("config.toml");
        
        // If config file doesn't exist, create one with default values
        if !config_path.exists() {
            let default_config = Self::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let config = Config::builder()
            .add_source(File::with_name("config"))
            .build()?;

        Ok(config.try_deserialize()?)
    }

    /// Saves configuration to config.toml file
    pub fn save(&self) -> Result<()> {
        let toml = toml::to_string_pretty(&self)?;
        std::fs::write("config.toml", toml)?;
        Ok(())
    }
}

/// Converts config to database connection config
impl From<DatabaseConfig> for crate::database::DatabaseConfig {
    fn from(config: DatabaseConfig) -> Self {
        Self {
            host: config.host,
            port: config.port,
            username: config.username,
            password: config.password,
            database: config.database,
        }
    }
}