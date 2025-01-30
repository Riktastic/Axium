use sqlx::postgres::PgPool;
use crate::models::user::*;  // Import the User struct

// Get all users
pub async fn get_user_by_email(pool: &PgPool, email: String) -> Result<User, String> {
    // Use a string literal directly in the macro
    let user = sqlx::query_as!(
        User, // Struct type to map the query result
        r#"
        SELECT id, username, email, password_hash, totp_secret, role_id
        FROM users
        WHERE email = $1
        "#,
        email // Bind the `email` parameter
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?; // Handle database errors

    // Handle optional result
    match user {
        Some(user) => Ok(user),
        None => Err(format!("User with email '{}' not found.", email)),
    }
}