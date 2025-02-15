use sqlx::postgres::PgPool;
use uuid::Uuid;

/// Records API usage with validation and security protections
///
/// # Validation
/// - Endpoint must be 1-100 characters after trimming
/// - Rejects empty or whitespace-only endpoints
///
/// # Security
/// - Uses parameterized queries to prevent SQL injection
/// - Automatically trims and sanitizes endpoint input
/// - Enforces user ownership through database constraints
// pub async fn insert_usage_into_db(
//     pool: &PgPool,
//     user_id: Uuid,
//     endpoint: String,
// ) -> Result<(), sqlx::Error> {
//     // Sanitize and validate endpoint
//     let endpoint = endpoint.trim();
//     if endpoint.is_empty() {
//         return Err(sqlx::Error::Protocol("Endpoint cannot be empty".into()));
//     }
//     if endpoint.len() > 100 {
//         return Err(sqlx::Error::Protocol("Endpoint exceeds maximum length of 100 characters".into()));
//     }

//     sqlx::query!(
//         r#"INSERT INTO usage (endpoint, user_id)
//         VALUES ($1, $2)"#,
//         endpoint,
//         user_id
//     )
//     .execute(pool)
//     .await?;

//     Ok(())
// }

/// Safely retrieves usage count for a user within a specified time period
///
/// # Security
/// - Uses parameterized query with interval casting to prevent SQL injection
/// - Explicit user ownership check
/// - COALESCE ensures always returns a number (0 if no usage)
///
/// # Example Interval Formats
/// - '1 hour'
/// - '7 days'
/// - '30 minutes'
pub async fn fetch_usage_count_from_db(
    pool: &PgPool,
    user_id: Uuid,
    interval: &str,
) -> Result<i64, sqlx::Error> {
    let count: i64 = sqlx::query_scalar(
        r#"SELECT COALESCE(COUNT(*), 0) 
        FROM usage 
        WHERE user_id = $1 
        AND creation_date > NOW() - CAST($2 AS INTERVAL)"#
    )
    .bind(user_id)
    .bind(interval)
    .fetch_one(pool)
    .await?;

    Ok(count)
}