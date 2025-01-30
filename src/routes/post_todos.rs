use axum::{extract::{State, Extension}, Json};
use axum::response::IntoResponse;
use sqlx::postgres::PgPool;
use crate::models::todo::*;
use crate::models::user::*;
use serde::Deserialize;
use axum::http::StatusCode;
use serde_json::json;

#[derive(Deserialize)]
pub struct TodoBody {
    pub task: String,
    pub description: Option<String>,
    pub user_id: i32,
}

// Add a new todo
pub async fn post_todo(
    State(pool): State<PgPool>, 
    Extension(user): Extension<User>,  // Extract current user from the request extensions
    Json(todo): Json<TodoBody>
) -> impl IntoResponse {
    // Ensure the user_id from the request matches the current user's id
    if todo.user_id != user.id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "User is not authorized to create a todo for another user" }))
        ));
    }

    // Insert the todo into the database
    let row = sqlx::query!(
        "INSERT INTO todos (task, description, user_id) VALUES ($1, $2, $3) RETURNING id, task, description, user_id",
        todo.task,
        todo.description,
        todo.user_id
    )
    .fetch_one(&pool)
    .await;

    match row {
        Ok(row) => Ok(Json(Todo {
            id: row.id,
            task: row.task,
            description: row.description,
            user_id: row.user_id,
        })),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Error: {}", err) }))
        )),
    }
}