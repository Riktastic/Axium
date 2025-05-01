// Axum for web server and routing
use axum::Router;
use axum::http::{HeaderValue, HeaderName, Method};

// Middleware layers from tower_http
use tower_http::compression::{CompressionLayer, CompressionLevel};  // For HTTP response compression
use tower_http::trace::TraceLayer;  // For HTTP request/response tracing
use tower_http::cors::{CorsLayer, AllowCredentials};

// Local crate imports for database connection and configuration
use crate::database::connect::connect_to_database;  // Function to connect to the database
use crate::database::connect::run_database_migrations;  // Function to run database migrations
use crate::storage::connect::connect_to_storage;  // Function to connect to storage
use crate::cache::connect::connect_to_cache;  // Function to connect to cache
use crate::mail::connect::connect_to_mail;  // Function to connect to mail service  
use crate::config;  // Environment configuration helper
use crate::routes::create_routes;  // Function to create application routes

use std::time::Duration;

use crate::routes::AppState;  // Application state structure
use std::sync::Arc;  // For thread-safe reference counting

/// Function to create and configure the Axum server.
pub async fn create_server() -> Router<()> {
    // === Database Setup ===
    let database = connect_to_database().await
        .expect("❌  Failed to connect to database.");
    println!("✔️   Connected to the database.");

    run_database_migrations(&database).await
        .expect("❌  Failed to run database migrations.");

    // === Storage Setup ===
    let storage = connect_to_storage().await
        .expect("❌  Failed to connect to storage.");
    println!("✔️   Connected to storage.");

    // === Cache Setup ===
    let cache = connect_to_cache().await
        .expect("❌  Failed to connect to cache.");
    println!("✔️   Connected to cache.");

    // === Mail Setup ===
    let mail = connect_to_mail().await
        .expect("❌  Failed to connect to mail.");
    println!("✔️   Connected to mail.");

    let shared_state = Arc::new(AppState { database: database, storage: storage, cache: cache, mail: mail });

    // === Application Routes ===
    let mut app = create_routes(shared_state);

    // === Tracing Middleware ===
    if config::get_env_bool("SERVER_TRACE_ENABLED", true) {
        app = app.layer(TraceLayer::new_for_http());
        println!("✔️   Trace has been enabled.");
    }

    // === Compression Middleware ===
    if config::get_env_bool("SERVER_COMPRESSION_ENABLED", true) {
        let level = config::get_env("SERVER_COMPRESSION_LEVEL").parse().unwrap_or(6);
        app = app.layer(
            CompressionLayer::new()
                .br(true)
                .quality(CompressionLevel::Precise(level))
        );
        println!("✔️   Brotli compression enabled with compression quality level {}.", level);
    }

    // === CORS Middleware Configuration ===

    // Allowed HTTP methods for CORS
    let methods: Vec<Method> = config::get_env("CORS_ALLOW_METHODS")
        .parse()
        .unwrap_or("GET,POST,PUT,DELETE,OPTIONS".to_string())
        .split(',')
        .filter_map(|m| m.trim().parse().ok())
        .collect();

    // Allowed origins for CORS (comma-separated in .env)
    let allowed_origins: Vec<HeaderValue> = config::get_env("CORS_ALLOW_ORIGIN")
        .split(',')
        .map(|s| HeaderValue::from_str(s.trim()).expect("Invalid CORS_ALLOW_ORIGIN value."))
        .collect();

    // Allowed headers for CORS
    let allowed_headers = config::get_env("CORS_ALLOW_HEADERS")
        .parse()
        .unwrap_or("Authorization,Content-Type,Origin".to_string())
        .split(',')
        .map(|h| h.trim())
        .filter(|h| !h.is_empty())
        .map(|h| HeaderName::from_bytes(h.as_bytes()).expect("Invalid header in CORS_ALLOW_HEADERS."))
        .collect::<Vec<_>>();

    // CORS max age (preflight cache)
    let max_age_secs = config::get_env("CORS_MAX_AGE").parse().unwrap_or(3600);

    // Allow credentials in CORS
    let allow_credentials = config::get_env_bool("CORS_ALLOW_CREDENTIALS", false);

    // Print allowed origins for debugging
    println!(
        "✔️   CORS will be allowed for origin(s): {}",
        allowed_origins
            .iter()
            .map(|hv| hv.to_str().unwrap_or("<invalid UTF-8>"))
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Build the CORS layer
    let mut cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods(methods)
        .allow_headers(allowed_headers)
        .max_age(Duration::from_secs(max_age_secs));
    if allow_credentials {
        cors = cors.allow_credentials(AllowCredentials::yes());
    }

    // === Attach CORS Middleware Globally ===
    app = app.layer(cors);

    // === Return the fully configured application ===
    app
}
