// Axum for web server and routing
use axum::Router;

// Middleware layers from tower_http
use tower_http::compression::{CompressionLayer, CompressionLevel};  // For HTTP response compression.
use tower_http::trace::TraceLayer;  // For HTTP request/response tracing.

// Local crate imports for database connection and configuration
use crate::database::connect::connect_to_database;  // Function to connect to the database.
use crate::database::connect::run_database_migrations;  // Function to run database migrations.
use crate::config;  // Environment configuration helper

/// Function to create and configure the Axum server.
pub async fn create_server() -> Router {
    // Establish a connection to the database
    let db = connect_to_database().await.expect("❌  Failed to connect to database.");

    run_database_migrations(&db).await.expect("❌  Failed to run database migrations.");
    
    // Initialize the routes for the server
    let mut app = crate::routes::create_routes(db);

    // Enable tracing middleware if configured
    if config::get_env_bool("SERVER_TRACE_ENABLED", true) {
        app = app.layer(TraceLayer::new_for_http());
        println!("✔️   Trace hads been enabled.");
    }

    // Enable compression middleware if configured
    if config::get_env_bool("SERVER_COMPRESSION_ENABLED", true) {
        // Parse compression level from environment or default to level 6
        let level = config::get_env("SERVER_COMPRESSION_LEVEL").parse().unwrap_or(6);
        // Apply compression layer with Brotli (br) enabled and the specified compression level
        app = app.layer(CompressionLayer::new().br(true).quality(CompressionLevel::Precise(level)));
        println!("✔️   Brotli compression enabled with compression quality level {}.", level);

    }

    // Return the fully configured application
    app
}
