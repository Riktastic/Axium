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

// Delete a API key by id
#[utoipa::path(
    delete,
    path = "/apikeys/{id}",
    tag = "apikey",
    params(
        ("id" = String, Path, description = "API key ID")
    ),
    responses(
        (status = 200, description = "API key deleted successfully", body = String),
        (status = 400, description = "Invalid UUID format", body = String),
        (status = 404, description = "API key not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[instrument(skip(pool))]
pub async fn delete_apikey_by_id(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,
    Path(id): Path<String>, // Use Path extractor here
) -> impl IntoResponse {
    // Parse the id string to UUID
    let uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({ "error": format!("Invalid UUID format.")}))),
    };

    let result = sqlx::query!("DELETE FROM apikeys WHERE id = $1 AND user_id = $2", uuid, user.id)
        .execute(&pool) // Borrow the connection pool
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                (StatusCode::NOT_FOUND, Json(json!({ "error": format!("API key with ID '{}' not found.", id) })))
            } else {
                (StatusCode::OK, Json(json!({ "success": format!("API key with ID '{}' deleted.", id)})))
            }
        }
        Err(_err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Could not delete API key '{}'.", id)}))
        ),
    }
}