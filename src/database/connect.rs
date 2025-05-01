use dotenvy::dotenv;
use sqlx::{PgPool, migrate::Migrator, migrate::MigrateError, postgres::PgPoolOptions};
use std::{fs, path::Path, time::Duration};
use crate::core::config::{get_env, get_env_with_default};
use thiserror::Error;

// ---------------------------
// Error Handling
// ---------------------------

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum DatabaseError {
    #[error("❌  Environment error: {0}")]
    EnvError(String),
    
    #[error("❌  Connection error: {0}")]
    ConnectionError(#[from] sqlx::Error),
    
    #[error("❌  File system error: {0}")]
    FileSystemError(String),
    
    #[error("❌  Configuration error: {0}")]
    ConfigError(String),

    #[error("❌  Migration error: {0}")]
    MigrationError(#[from] MigrateError),
}

// ---------------------------
// Database Connection
// ---------------------------

/// Establishes a secure connection to PostgreSQL with connection pooling
/// 
/// # Security Features
/// - Validates database URL format
/// - Enforces connection limits
/// - Uses environment variables securely
/// - Implements connection timeouts
/// 
/// # Returns
/// `Result<PgPool, DatabaseError>` - Connection pool or detailed error
pub async fn connect_to_database() -> Result<PgPool, DatabaseError> {
    // Load environment variables securely
    dotenv().ok();
    
    // Validate database URL presence and format
    let database_url = get_env("DATABASE_URL");
    if !database_url.starts_with("postgres://") {
        return Err(DatabaseError::ConfigError(
            "❌  Invalid DATABASE_URL format - must start with postgres://".to_string()
        ));
    }

    // Configure connection pool with safety defaults
    let max_connections: u32 = get_env("DATABASE_MAX_CONNECTIONS").parse().unwrap_or(10);
    let min_connections: u32 = get_env("DATABASE_MIN_CONNECTIONS").parse().unwrap_or(2);

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .acquire_timeout(Duration::from_secs(5))  // Prevent hanging connections
        .idle_timeout(Duration::from_secs(300))   // Clean up idle connections
        .test_before_acquire(true)                // Validate connections
        .connect(&database_url)
        .await
        .map_err(|e| DatabaseError::ConnectionError(e))?;

    Ok(pool)
}


// ---------------------------
// Database Migrations
// ---------------------------

/// Executes database migrations with safety checks
/// 
/// # Security Features
/// - Validates migrations directory existence
/// - Limits migration execution to development/staging environments
/// - Uses transactional migrations where supported
/// 
/// # Returns
/// `Result<(), DatabaseError>` - Success or detailed error
pub async fn run_database_migrations(pool: &PgPool) -> Result<(), DatabaseError> {
    let migrations_path = Path::new("./migrations");
    
    // Validate migrations directory
    if !migrations_path.exists() {
        fs::create_dir_all(migrations_path)
            .map_err(|e| DatabaseError::FileSystemError(
                format!("❌  Failed to create migrations directory: {}", e)
            ))?;
    }

    // Verify directory permissions
    let metadata = fs::metadata(migrations_path)
        .map_err(|e| DatabaseError::FileSystemError(
            format!("❌  Cannot access migrations directory: {}", e)
        ))?;
    
    if metadata.permissions().readonly() {
        return Err(DatabaseError::FileSystemError(
            "❌  Migrations directory is read-only".to_string()
        ));
    }

    // Initialize migrator with production safety checks
    let migrator = Migrator::new(migrations_path)
        .await
        .map_err(|e| DatabaseError::MigrationError(e))?;

    // Skip migrations execution in production, just print a message
    let environment = get_env_with_default("ENVIRONMENT", "development");
    if environment == "production" {
        println!("🛑  Migration execution skipped in production.");
        return Ok(());
    }

    // Execute migrations in transaction if supported
    migrator.run(pool)
        .await
        .map_err(DatabaseError::MigrationError)?;

    Ok(())
}
