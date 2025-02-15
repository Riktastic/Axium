use axum::{
    extract::{State, Extension, Path},
    Json,
    http::StatusCode,
};
use sqlx::postgres::PgPool;
use uuid::Uuid;
use serde_json::json;
use tracing::instrument; // For logging
use crate::models::todo::*;
use crate::models::user::*;
use crate::database::todos::{fetch_all_todos_from_db, fetch_todo_by_id_from_db};

// --- Route Handlers ---

// Get all todos
#[utoipa::path(
    get,
    path = "/todos/all",
    tag = "todo",
    security(
        ("jwt_token" = [])
    ),
    responses(
        (status = 200, description = "Successfully fetched all todos", body = [Todo]),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument(skip(pool))]
pub async fn get_all_todos(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,  // Extract current user from the request extensions
) -> Result<Json<Vec<Todo>>, (StatusCode, Json<serde_json::Value>)> {
    match fetch_all_todos_from_db(&pool, user.id).await {
        Ok(todos) => Ok(Json(todos)),
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
) -> Result<Json<Todo>, (StatusCode, Json<serde_json::Value>)> {
    let uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid UUID format." })),
            ));
        }
    };

    match fetch_todo_by_id_from_db(&pool, uuid, user.id).await {
        Ok(Some(todo)) => Ok(Json(todo)),
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
