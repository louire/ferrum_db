use crate::database::DatabaseError;

/// Represents the result of a query execution
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// Column headers
    pub headers: Vec<String>,
    /// Row data
    pub rows: Vec<Vec<String>>,
    /// Number of affected rows (for UPDATE, DELETE, etc.)
    pub affected_rows: Option<u64>,
    /// Execution time in milliseconds
    pub execution_time: u128,
}

impl QueryResult {
    /// Creates a new query result
    pub fn new(
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
        affected_rows: Option<u64>,
        execution_time: u128,
    ) -> Self {
        Self {
            headers,
            rows,
            affected_rows,
            execution_time,
        }
    }

    /// Gets the total number of rows
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Gets the number of columns
    pub fn column_count(&self) -> usize {
        self.headers.len()
    }
}

/// Formats a query execution error
pub fn format_error(error: &DatabaseError) -> String {
    match error {
        DatabaseError::ConnectionError(msg) => format!("Connection error: {}", msg),
        DatabaseError::QueryError(msg) => format!("Query error: {}", msg),
        DatabaseError::ConfigError(msg) => format!("Configuration error: {}", msg),
        DatabaseError::ResultError(msg) => format!("Result error: {}", msg),
        DatabaseError::Unknown(msg) => format!("Unknown error: {}", msg),
    }
}