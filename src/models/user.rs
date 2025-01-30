use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a user in the system.
#[derive(Deserialize, Debug, Serialize, FromRow, Clone)]
#[sqlx(rename_all = "snake_case")]  // Ensures that field names are mapped to snake_case in SQL
pub struct User {
    /// The unique identifier for the user.
    pub id: i32,
    /// The username of the user.
    pub username: String,
    /// The email of the user.
    pub email: String,
    /// The hashed password for the user.
    pub password_hash: String,
    /// The TOTP secret for the user.
    pub totp_secret: Option<String>,
    /// Current role of the user..
    pub role_id: i32,
}