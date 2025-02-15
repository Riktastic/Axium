use axum::{extract::{Extension, State}, Json, response::IntoResponse};
use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::json;
use sqlx::postgres::PgPool;
use tracing::instrument;
use utoipa::ToSchema;
use validator::Validate;

use crate::models::todo::Todo;
use crate::models::user::User;

// Define the request body structure
#[derive(Deserialize, Validate, ToSchema)]
pub struct TodoBody {
    #[validate(length(min = 3, max = 50))]
    pub task: String,
    #[validate(length(min = 3, max = 100))]
    pub description: Option<String>,
}

// Define the API endpoint
#[utoipa::path(
    post,
    path = "/todos",
    tag = "todo",
    request_body = TodoBody,
    responses(
        (status = 200, description = "Todo created successfully", body = Todo),
        (status = 400, description = "Validation error", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[instrument(skip(pool, user, todo))]
pub async fn post_todo(
    State(pool): State<PgPool>, 
    Extension(user): Extension<User>,
    Json(todo): Json<TodoBody>
) -> impl IntoResponse {
    // Validate input
    if let Err(errors) = todo.validate() {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(_, errors)| errors.iter().map(|e| e.message.clone().unwrap_or_default().to_string()))
            .collect();
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": error_messages.join(", ") }))
        ));
    }

    let row = sqlx::query!(
        "INSERT INTO todos (task, description, user_id) 
        VALUES ($1, $2, $3) 
        RETURNING id, task, description, user_id, creation_date, completion_date, completed",
        todo.task,
        todo.description,
        user.id
    )
    .fetch_one(&pool)
    .await;

    match row {
        Ok(row) => Ok(Json(Todo {
            id: row.id,
            task: row.task,
            description: row.description,
            user_id: row.user_id,
            creation_date: row.creation_date,
            completion_date: row.completion_date,
            completed: row.completed,
        })),
        Err(_err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not create a new todo." }))
        )),
    }
}
