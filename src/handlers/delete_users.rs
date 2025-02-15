use axum::{
    extract::{State, Path},
    Json,

    http::StatusCode,
};
use sqlx::postgres::PgPool;
use uuid::Uuid;
use serde_json::json;
use tracing::instrument; // For logging
use crate::models::documentation::{ErrorResponse, SuccessResponse};
use crate::database::users::delete_user_from_db;

// --- Route Handler ---

// Delete a user by id
#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "user",
    security(
        ("jwt_token" = [])
    ),
    responses(
        (status = 200, description = "User deleted successfully", body = SuccessResponse),
        (status = 400, description = "Invalid UUID format", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    )
)]
#[instrument(skip(pool))]
pub async fn delete_user_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<String>, // Use Path extractor here
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    let uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid UUID format." })),
            ));
        }
    };

    match delete_user_from_db(&pool, uuid).await {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": format!("User with ID '{}' not found.", id) })),
                ))
            } else {
                Ok((
                    StatusCode::OK,
                    Json(json!({ "success": format!("User with ID '{}' deleted.", id) })),
                ))
            }
        }
        Err(_err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not delete the user." })),
        )),
    }
}
