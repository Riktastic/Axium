use axum::{
    extract::{State, Extension, Path}, 
    Json, 
    response::IntoResponse, 
    http::StatusCode
};
use sqlx::postgres::PgPool;
use uuid::Uuid;
use serde_json::json;
use tracing::instrument; // For logging
use crate::models::todo::*;
use crate::models::user::*;

// Get all todos
#[utoipa::path(
    get,
    path = "/todos/all",
    tag = "todo",
    responses(
        (status = 200, description = "Successfully fetched all todos", body = [Todo]),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument(skip(pool))]
pub async fn get_all_todos(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,  // Extract current user from the request extensions
) -> impl IntoResponse {
    let todos = sqlx::query_as!(Todo, 
        "SELECT id, user_id, task, description, creation_date, completion_date, completed FROM todos WHERE user_id = $1", 
        user.id
    )
    .fetch_all(&pool) // Borrow the connection pool
    .await;

    match todos {
        Ok(todos) => Ok(Json(todos)), // Return all todos as JSON
        Err(_err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not fetch the details of the todo." })),
        )),
    }
}

// Get a single todo by id
#[utoipa::path(
    get,
    path = "/todos/{id}",
    tag = "todo",
    params(
        ("id" = String, Path, description = "Todo ID")
    ),
    responses(
        (status = 200, description = "Successfully fetched todo by ID", body = Todo),
        (status = 400, description = "Invalid UUID format"),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument(skip(pool))]
pub async fn get_todos_by_id(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,  // Extract current user from the request extensions
    Path(id): Path<String>, // Use Path extractor here
) -> impl IntoResponse {
    let uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid UUID format." })))),
    };

    let todo = sqlx::query_as!(Todo, 
        "SELECT id, user_id, task, description, creation_date, completion_date, completed FROM todos WHERE id = $1 AND user_id = $2", 
        uuid, 
        user.id
    )
    .fetch_optional(&pool) // Borrow the connection pool
    .await;

    match todo {
        Ok(Some(todo)) => Ok(Json(todo)), // Return the todo as JSON if found
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": format!("Todo with ID '{}' not found.", id) })),
        )),
        Err(_err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not fetch the details of the todo." })),
        )),
    }
}
