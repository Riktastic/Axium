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
use crate::models::user::User;
use crate::models::documentation::{ErrorResponse, SuccessResponse};

// Delete a todo by id
#[utoipa::path(
    delete,
    path = "/todos/{id}",
    tag = "todo",
    responses(
        (status = 200, description = "Todo deleted successfully", body = SuccessResponse),
        (status = 400, description = "Invalid UUID format", body = ErrorResponse),
        (status = 404, description = "Todo not found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    params(
        ("id" = Uuid, Path, description = "Todo ID"),
        ("user_id" = Uuid, Path, description = "User ID")
    )
)]
#[instrument(skip(pool))]
pub async fn delete_todo_by_id(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,
    Path(id): Path<String>, // Use Path extractor here
) -> impl IntoResponse {
    let uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid UUID format." })),)),
    };

    let result = sqlx::query!("DELETE FROM todos WHERE id = $1 AND user_id = $2", uuid, user.id)
        .execute(&pool) // Borrow the connection pool
        .await;

    match result {
        Ok(res) => {
            if (res.rows_affected() == 0) {
                Err((StatusCode::NOT_FOUND, Json(json!({ "error": format!("Todo with ID '{}' not found.", id) })),))
            } else {
                Ok((StatusCode::OK, Json(json!({ "success": format!("Todo with ID '{}' deleted.", id) })),))
            }
        }
        Err(_err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not delete the todo." })),
        )),
    }
}
