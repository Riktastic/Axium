use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::models::todo::*;

/// Inserts a new Todo into the database with robust input validation and ownership enforcement
///
/// # Validation
/// - Task must be 1-100 characters after trimming
/// - Description (if provided) must be ≤500 characters after trimming
/// - Automatically associates todo with the requesting user
///
/// # Security
/// - Uses parameterized queries to prevent SQL injection
/// - Trims input to prevent whitespace abuse
pub async fn insert_todo_into_db(
    pool: &PgPool,
    task: String,
    description: Option<String>,
    user_id: Uuid,
) -> Result<Todo, sqlx::Error> {
    // Sanitize and validate task
    let task = task.trim();
    if task.is_empty() {
        return Err(sqlx::Error::Protocol("Task cannot be empty".into()));
    }
    if task.len() > 100 {
        return Err(sqlx::Error::Protocol("Task exceeds maximum length of 100 characters".into()));
    }

    // Sanitize and validate optional description
    let description = description.map(|d| d.trim().to_string())
        .filter(|d| !d.is_empty());
    if let Some(desc) = &description {
        if desc.len() > 500 {
            return Err(sqlx::Error::Protocol("Description exceeds maximum length of 500 characters".into()));
        }
    }

    // Insert with ownership enforcement
    let row = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (task, description, user_id) 
        VALUES ($1, $2, $3) 
        RETURNING id, user_id, task, description, creation_date, completion_date, completed",
        task,
        description,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

/// Retrieves all Todos for a specific user with strict ownership filtering
///
/// # Security
/// - Uses WHERE clause with user_id to ensure data isolation
/// - Parameterized query prevents SQL injection
pub async fn fetch_all_todos_from_db(pool: &PgPool, user_id: Uuid) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as!(
        Todo,
        "SELECT id, user_id, task, description, creation_date, completion_date, completed 
        FROM todos WHERE user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

/// Safely retrieves a single Todo by ID with ownership verification
///
/// # Security
/// - Combines ID and user_id in WHERE clause to prevent unauthorized access
/// - Returns Option<Todo> to avoid exposing existence of other users' todos
pub async fn fetch_todo_by_id_from_db(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Option<Todo>, sqlx::Error> {
    let todo = sqlx::query_as!(
        Todo,
        "SELECT id, user_id, task, description, creation_date, completion_date, completed 
        FROM todos WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(todo)
}

/// Securely deletes a Todo by ID with ownership confirmation
///
/// # Security
/// - Requires both ID and user_id for deletion
/// - Returns affected row count without exposing existence of other users' todos
pub async fn delete_todo_from_db(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "DELETE FROM todos WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}