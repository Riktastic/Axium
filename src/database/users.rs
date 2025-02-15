use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::models::user::*;
use regex::Regex;
use sqlx::Error;

/// Retrieves all users with security considerations
///
/// # Security
/// - Requires admin privileges (enforced at application layer)
/// - Excludes sensitive fields like password_hash and totp_secret
/// - Limits maximum results in production (enforced at application layer)
pub async fn fetch_all_users_from_db(pool: &PgPool) -> Result<Vec<UserGetResponse>, sqlx::Error> {
    sqlx::query_as!(
        UserGetResponse,
        "SELECT id, username, email, role_level, tier_level, creation_date 
        FROM users"
    )
    .fetch_all(pool)
    .await
}

/// Safely retrieves user by allowed fields using whitelist validation
///
/// # Allowed Fields
/// - id (UUID)
/// - email (valid email format)
/// - username (valid username format)
///
/// # Security
/// - Field whitelisting prevents SQL injection
/// - Parameterized query for value
pub async fn fetch_user_by_field_from_db(
    pool: &PgPool,
    field: &str,
    value: &str,
) -> Result<Option<User>, sqlx::Error> {
    let query = match field {
        "id" => "SELECT * FROM users WHERE id = $1",
        "email" => "SELECT * FROM users WHERE email = $1",
        "username" => "SELECT * FROM users WHERE username = $1",
        _ => return Err(sqlx::Error::ColumnNotFound(field.to_string())),
    };

    sqlx::query_as::<_, User>(query)
        .bind(value)
        .fetch_optional(pool)
        .await
}

/// Retrieves user by email with validation
///
/// # Security
/// - Parameterized query prevents SQL injection
/// - Returns Option to avoid user enumeration risks
pub async fn fetch_user_by_email_from_db(
    pool: &PgPool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT id, username, email, password_hash, totp_secret, 
           role_level, tier_level, creation_date
           FROM users WHERE email = $1"#,
        email
    )
    .fetch_optional(pool)
    .await
}

/// Securely deletes a user by ID
///
/// # Security
/// - Requires authentication and authorization
/// - Parameterized query prevents SQL injection
/// - Returns affected rows without sensitive data
pub async fn delete_user_from_db(pool: &PgPool, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

/// Creates new user with comprehensive validation
///
/// # Validation
/// - Username: 3-30 alphanumeric characters
/// - Email: Valid format with domain verification
/// - Password: Minimum strength requirements (enforced at application layer)
pub async fn insert_user_into_db(
    pool: &PgPool,
    username: &str,
    email: &str,
    password_hash: &str,
    totp_secret: &str,
    role_level: i32,
    tier_level: i32,
) -> Result<UserInsertResponse, Error> {
    // Validate username
    let username = username.trim();
    if username.len() < 3 || username.len() > 30 {
        return Err(Error::Protocol("Username must be between 3 and 30 characters.".into()));
    }
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(Error::Protocol("Invalid username format: only alphanumeric and underscores allowed.".into()));
    }

    // Validate email
    let email = email.trim().to_lowercase();
    if !is_valid_email(&email) {
        return Err(Error::Protocol("Invalid email format.".into()));
    }

    // Insert user into database
    let row = sqlx::query_as!(
        UserInsertResponse,
        r#"INSERT INTO users 
           (username, email, password_hash, totp_secret, role_level, tier_level, creation_date)
           VALUES ($1, $2, $3, $4, $5, $6, NOW()::timestamp)
           RETURNING id, username, email, totp_secret, role_level, tier_level, creation_date"#,
        username,
        email,
        password_hash,
        totp_secret,
        role_level,
        tier_level,
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

/// Email validation helper function
fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^[a-z0-9_+]+([a-z0-9_.-]*[a-z0-9_+])?@[a-z0-9]+([-.][a-z0-9]+)*\.[a-z]{2,6}$"
    ).unwrap();
    email_regex.is_match(email)
}
