use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a to-do item.
#[derive(Deserialize, Debug, Serialize, FromRow)]
#[sqlx(rename_all = "snake_case")]  // Ensures that field names are mapped to snake_case in SQL
pub struct Todo {
    /// The unique identifier for the to-do item.
    pub id: i32,
    /// The task description.
    pub task: String,
    /// An optional detailed description of the task.
    pub description: Option<String>,
    /// The unique identifier of the user who created the to-do item.
    pub user_id: i32,
}