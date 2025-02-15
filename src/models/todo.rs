use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDate;
use utoipa::ToSchema;

/// Represents a to-do item.
#[derive(Deserialize, Debug, Serialize, FromRow, ToSchema)]
#[sqlx(rename_all = "snake_case")]  // Ensures that field names are mapped to snake_case in SQL
pub struct Todo {
    /// The unique identifier for the to-do item.
    pub id: Uuid,
    
    /// The task description.
    pub task: String,
    
    /// An optional detailed description of the task.
    pub description: Option<String>,
    
    /// The unique identifier of the user who created the to-do item.
    pub user_id: Uuid,
    
    /// The date the task was created.
    pub creation_date: NaiveDate,
    
    /// The date the task was completed (if any).
    pub completion_date: Option<NaiveDate>,
    
    /// Whether the task is completed.
    pub completed: Option<bool>,
}
