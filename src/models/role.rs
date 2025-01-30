use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a user role in the system.
#[derive(Deserialize, Debug, Serialize, FromRow, Clone)]
#[sqlx(rename_all = "snake_case")]  // Ensures that field names are mapped to snake_case in SQL
pub struct Role {
    /// ID of the role.
    pub id: i32,
    /// Level of the role.
    pub level: i32,
    /// System name of the role.
    pub role: String,
    /// The name of the role.
    pub name: String,
    /// Description of the role
    pub Description: String,
}