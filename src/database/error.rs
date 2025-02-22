use thiserror::Error;

/// Custom error types for database operations
#[derive(Error, Debug)]
pub enum DatabaseError {
    /// Error occurred while connecting to the database
    #[error("Failed to connect to database: {0}")]
    ConnectionError(String),

    /// Error occurred while executing a query
    #[error("Query execution failed: {0}")]
    QueryError(String),

    /// Error occurred while parsing query results
    #[error("Failed to parse query results: {0}")]
    ResultError(String),

    /// Error occurred while managing database configuration
    #[error("Database configuration error: {0}")]
    ConfigError(String),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Database(db_err) => DatabaseError::QueryError(db_err.message().to_string()),
            sqlx::Error::Configuration(config_err) => DatabaseError::ConfigError(config_err.to_string()),
            sqlx::Error::Io(io_err) => DatabaseError::ConnectionError(io_err.to_string()),
            _ => DatabaseError::QueryError(err.to_string()),
        }
    }
}