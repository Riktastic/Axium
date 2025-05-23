use axum::{extract::{Extension, State}, Json};
use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::json;
use tracing::instrument;
use utoipa::ToSchema;
use validator::Validate;
use std::sync::Arc;

use crate::models::todo::Todo;
use crate::models::user::User;
use crate::database::todos::insert_todo_into_db;
use crate::routes::AppState;

// Define the request body structure
#[derive(Deserialize, Validate, ToSchema)]
pub struct TodoBody {
    #[validate(length(min = 3, max = 50))]
    pub task: String,
    #[validate(length(min = 3, max = 100))]
    pub description: Option<String>,
}

// --- Route Handler ---

// Define the API endpoint
#[utoipa::path(
    post,
    path = "/todos",
    tag = "todo",
    security(
        ("jwt_token" = [])
    ),
    request_body = TodoBody,
    responses(
        (status = 200, description = "Todo created successfully", body = Todo),
        (status = 400, description = "Validation error", body = String),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[instrument(skip(state, user, todo))]
pub async fn post_todo(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(todo): Json<TodoBody>
) -> Result<Json<Todo>, (StatusCode, Json<serde_json::Value>)> {
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

    match insert_todo_into_db(&state.database, todo.task, todo.description, user.id).await {
        Ok(new_todo) => Ok(Json(new_todo)),
        Err(_err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not create a new todo." }))
        )),
    }
}
