use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row, Column};
use std::time::Duration;

mod error;
pub use error::DatabaseError;

/// Configuration for database connection
#[derive(Debug, Clone, Default)]
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

impl DatabaseConfig {
    /// Creates a new database configuration with default values
    pub fn new() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: "postgres".to_string(),
            database: "postgres".to_string(),
        }
    }
}

/// Database connection manager
#[derive(Debug)]
pub struct DatabaseManager {
    pool: Pool<Postgres>,
    config: DatabaseConfig,
}

impl DatabaseManager {
    /// Creates a new database connection pool
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.username, config.password, config.host, config.port, config.database
        );

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&connection_string)
            .await?;

        Ok(Self { pool, config })
    }

    /// Executes a query and returns the results
    pub async fn execute_query(&self, query: &str) -> Result<QueryResult> {
        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResult::new(rows))
    }

    /// Gets a list of all databases
    pub async fn list_databases(&self) -> Result<Vec<String>> {
        let rows = sqlx::query("SELECT datname FROM pg_database WHERE datistemplate = false;")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .iter()
            .map(|row| row.get("datname"))
            .collect())
    }

    /// Gets a list of all schemas in the current database
    pub async fn list_schemas(&self) -> Result<Vec<String>> {
        let rows = sqlx::query("SELECT schema_name FROM information_schema.schemata;")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .iter()
            .map(|row| row.get("schema_name"))
            .collect())
    }

    /// Gets the current database name
    pub fn current_database(&self) -> &str {
        &self.config.database
    }
}

/// Represents the result of a database query
pub struct QueryResult {
    rows: Vec<sqlx::postgres::PgRow>,
}

impl QueryResult {
    /// Creates a new QueryResult
    pub fn new(rows: Vec<sqlx::postgres::PgRow>) -> Self {
        Self { rows }
    }

    /// Gets the number of rows in the result
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Gets the column names of the result
    pub fn columns(&self) -> Vec<String> {
        if let Some(row) = self.rows.first() {
            row.columns()
                .iter()
                .map(|col| col.name().to_string())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Gets the rows as vectors of strings
    pub fn rows_as_strings(&self) -> Vec<Vec<String>> {
        self.rows
            .iter()
            .map(|row| {
                (0..row.columns().len())
                    .map(|i| {
                        row.try_get::<String, _>(i)
                            .unwrap_or_else(|_| "NULL".to_string())
                    })
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_connection() {
        let config = DatabaseConfig::new();
        let result = DatabaseManager::new(config).await;
        assert!(result.is_ok());
    }
}