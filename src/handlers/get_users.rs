use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use sqlx::postgres::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::models::user::*;
use crate::database::users::{fetch_all_users_from_db, fetch_user_by_field_from_db};

// Get all users
#[utoipa::path(
    get,
    path = "/users/all",
    tag = "user",
    security(
        ("jwt_token" = [])
    ),
    responses(
        (status = 200, description = "Successfully fetched all users", body = [UserGetResponse]),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument(skip(pool))]
pub async fn get_all_users(State(pool): State<PgPool>) -> impl IntoResponse {
    match fetch_all_users_from_db(&pool).await {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not fetch the users details." })),
        )),
    }
}

// Get a single user by ID
#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "user",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Successfully fetched user by ID", body = UserGetResponse),
        (status = 400, description = "Invalid UUID format"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument(skip(pool))]
pub async fn get_users_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid UUID format." })))),
    };

    match fetch_user_by_field_from_db(&pool, "id", &uuid.to_string()).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": format!("User with ID '{}' not found", id) })),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not fetch the users details." })),
        )),
    }
}
