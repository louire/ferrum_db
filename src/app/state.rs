use std::time::Instant;
use crate::database::{DatabaseManager, DatabaseConfig, DatabaseError};
use super::input::{InputHandler, Mode};
use super::query::{QueryResult, format_error};

/// Represents the current state of the application
#[derive(Debug)]
pub struct AppState {
    /// The input handler
    pub input: InputHandler,
    /// The currently selected database
    pub current_database: Option<String>,
    /// The currently selected schema
    pub current_schema: Option<String>,
    /// The database connection manager
    pub db_manager: Option<DatabaseManager>,
    /// The last query result
    pub query_result: Option<QueryResult>,
    /// Status message to display
    pub status_message: Option<String>,
    /// Last error message
    pub last_error: Option<String>,
}

impl AppState {
    /// Creates a new application state
    pub fn new() -> Self {
        Self {
            input: InputHandler::new(),
            current_database: None,
            current_schema: None,
            db_manager: None,
            query_result: None,
            status_message: None,
            last_error: None,
        }
    }

    /// Sets the current database
    pub fn set_database(&mut self, database: String) {
        self.current_database = Some(database);
    }

    /// Sets the current schema
    pub fn set_schema(&mut self, schema: String) {
        self.current_schema = Some(schema);
    }

    /// Sets a status message
    pub fn set_status(&mut self, message: String) {
        self.status_message = Some(message);
        self.last_error = None;
    }

    /// Sets an error message
    pub fn set_error(&mut self, error: String) {
        self.last_error = Some(error);
        self.status_message = None;
    }

    /// Initializes the database connection
    pub async fn init_database(&mut self, config: DatabaseConfig) -> anyhow::Result<()> {
        self.db_manager = Some(DatabaseManager::new(config).await?);
        Ok(())
    }

    /// Gets the current mode
    pub fn mode(&self) -> Mode {
        self.input.mode()
    }

    /// Executes a query
    pub async fn execute_query(&mut self, query: String) {
        let start_time = Instant::now();
        
        match &self.db_manager {
            Some(db) => {
                match db.execute_query(&query).await {
                    Ok(result) => {
                        let execution_time = start_time.elapsed().as_millis();
                        let headers = result.columns();
                        let rows = result.rows_as_strings();
                        let row_count = rows.len();
                        
                        self.query_result = Some(QueryResult::new(
                            headers,
                            rows,
                            None,
                            execution_time,
                        ));

                        self.set_status(format!(
                            "Query executed successfully in {}ms ({} rows)",
                            execution_time,
                            row_count
                        ));
                    }
                    Err(e) => {
                        let db_error: DatabaseError = e.into();
                        self.set_error(format_error(&db_error));
                    }
                }
            }
            None => {
                self.set_error("Not connected to database".to_string());
            }
        }
    }
}