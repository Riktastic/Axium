use axum::extract::{State, Path};
use axum::Json;
use axum::response::IntoResponse;
use sqlx::postgres::PgPool;
use crate::models::todo::*;

// Get all todos
pub async fn get_all_todos(State(pool): State<PgPool>,) -> impl IntoResponse {
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todos") // Your table name
        .fetch_all(&pool) // Borrow the connection pool
        .await;

    match todos {
        Ok(todos) => Ok(Json(todos)), // Return all todos as JSON
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error fetching todos: {}", err),
        )),
    }
}


// Get a single todo by id
pub async fn get_todos_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>, // Use Path extractor here
) -> impl IntoResponse {
    let todo = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = $1", id)
        .fetch_optional(&pool) // Borrow the connection pool
        .await;

    match todo {
        Ok(Some(todo)) => Ok(Json(todo)), // Return the todo as JSON if found
        Ok(None) => Err((
            axum::http::StatusCode::NOT_FOUND,
            format!("Todo with id {} not found", id),
        )),
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error fetching todo: {}", err),
        )),
    }
}