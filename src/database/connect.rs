use dotenvy::dotenv;
use sqlx::{PgPool, migrate::Migrator, postgres::PgPoolOptions};
use std::fs;
use std::env;
use std::path::Path;

/// Connects to the database using the DATABASE_URL environment variable.
pub async fn connect_to_database() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();
    let database_url = &env::var("DATABASE_URL").expect("❌ 'DATABASE_URL' environment variable not fount.");

    // Read max and min connection values from environment variables, with defaults
    let max_connections: u32 = env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "10".to_string()) // Default to 10
        .parse()
        .expect("❌ Invalid 'DATABASE_MAX_CONNECTIONS' value; must be a number.");
    
    let min_connections: u32 = env::var("DATABASE_MIN_CONNECTIONS")
        .unwrap_or_else(|_| "2".to_string()) // Default to 2
        .parse()
        .expect("❌ Invalid 'DATABASE_MIN_CONNECTIONS' value; must be a number.");

    // Create and configure the connection pool
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .connect(&database_url)
        .await?;
    
    Ok(pool)
}

/// Run database migrations
pub async fn run_database_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Define the path to the migrations folder
    let migrations_path = Path::new("./migrations");

    // Check if the migrations folder exists, and if not, create it
    if !migrations_path.exists() {
        fs::create_dir_all(migrations_path).expect("❌ Failed to create migrations directory. Make sure you have the necessary permissions.");
        println!("✔️ Created migrations directory: {:?}", migrations_path);
    }

    // Create a migrator instance that looks for migrations in the `./migrations` folder
    let migrator = Migrator::new(migrations_path).await?;

    // Run all pending migrations
    migrator.run(pool).await?;

    Ok(())
}