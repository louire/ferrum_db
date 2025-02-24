use anyhow::Result;
use tracing::info;

mod app;
mod config;
mod database;
mod ui;

/// Main entry point for FerrumDB
///
/// # Errors
/// Will return an error if the application fails to initialize or run
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("Starting FerrumDB...");

    // Load configuration
    let config = config::AppConfig::load()?;
    info!("Configuration loaded");

    // Initialize application state
    let mut app = app::App::new()?;
    
    // Initialize database connection
    if let Err(e) = app.init_database(config.database.into()).await {
        info!("Failed to connect to database: {}", e);
    } else {
        info!("Connected to database");
    }

    // Run the application
    app.run().await?;

    info!("Shutting down FerrumDB ...");
    Ok(())
}