use anyhow::Result;
use tracing::info;

mod app;
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

    // Initialize application state
    let mut app = app::App::new()?;
    
    // Run the application
    app.run().await?;

    info!("Shutting down FerrumDB...");
    Ok(())

    
}