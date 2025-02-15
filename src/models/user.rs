use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDate;
use utoipa::ToSchema;

/// Represents a user in the system.
#[derive(Deserialize, Debug, Serialize, FromRow, Clone, ToSchema)]
#[sqlx(rename_all = "snake_case")]  // Ensures that field names are mapped to snake_case in SQL
pub struct User {
    /// The unique identifier for the user.
    pub id: Uuid,
    
    /// The username of the user.
    pub username: String,
    
    /// The email of the user.
    pub email: String,
    
    /// The hashed password for the user.
    pub password_hash: String,
    
    /// The TOTP secret for the user.
    pub totp_secret: Option<String>,
    
    /// Current role of the user.
    pub role_level: i32,
    
    /// Current tier level of the user.
    pub tier_level: i32,
    
    /// Date when the user was created.
    pub creation_date: Option<NaiveDate>,  // Nullable, default value in SQL is CURRENT_DATE
}



/// Represents a user in the system.
#[derive(Deserialize, Debug, Serialize, FromRow, Clone, ToSchema)]
#[sqlx(rename_all = "snake_case")]  // Ensures that field names are mapped to snake_case in SQL
pub struct UserResponse {
    /// The unique identifier for the user.
    pub id: Uuid,
    
    /// The username of the user.
    pub username: String,
    
    /// The email of the user.
    pub email: String,
    
    /// Current role of the user.
    pub role_level: i32,
    
    /// Current tier level of the user.
    pub tier_level: i32,
    
    /// Date when the user was created.
    pub creation_date: Option<NaiveDate>,  // Nullable, default value in SQL is CURRENT_DATE
}
