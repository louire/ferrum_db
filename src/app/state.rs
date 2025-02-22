use crate::database::{DatabaseManager, DatabaseConfig};
use super::input::{InputHandler, Mode};

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
    /// The last query result message
    pub status_message: Option<String>,
}

impl AppState {
    /// Creates a new application state
    pub fn new() -> Self {
        Self {
            input: InputHandler::new(),
            current_database: None,
            current_schema: None,
            db_manager: None,
            status_message: None,
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
}