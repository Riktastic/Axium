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
use crate::models::apikey::*;
use crate::models::user::*;
use crate::models::documentation::ErrorResponse;
use crate::models::apikey::ApiKeyResponse;

// Get all API keys
#[utoipa::path(
    get,
    path = "/apikeys",
    tag = "apikey",
    responses(
        (status = 200, description = "Get all API keys", body = [ApiKeyResponse]),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    )
)]
#[instrument(skip(pool))]
pub async fn get_all_apikeys(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,  // Extract current user from the request extensions
) -> impl IntoResponse {
    let apikeys = sqlx::query_as!(ApiKeyResponse, 
        "SELECT id, user_id, description, expiration_date, creation_date FROM apikeys WHERE user_id = $1", 
        user.id
    )
    .fetch_all(&pool) // Borrow the connection pool
    .await;

    match apikeys {
        Ok(apikeys) => Ok(Json(apikeys)), // Return all API keys as JSON
        Err(_err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not get the API key."})),
        )),
    }
}

// Get a single API key by id
#[utoipa::path(
    get,
    path = "/apikeys/{id}",
    tag = "apikey",
    responses(
        (status = 200, description = "Get API key by ID", body = ApiKeyByIDResponse),
        (status = 400, description = "Invalid UUID format", body = ErrorResponse),
        (status = 404, description = "API key not found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    params(
        ("id" = Uuid, Path, description = "API key ID"),
        ("user_id" = Uuid, Path, description = "User ID")
    )
)]
#[instrument(skip(pool))]
pub async fn get_apikeys_by_id(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,  // Extract current user from the request extensions
    Path(id): Path<String>, // Use Path extractor here
) -> impl IntoResponse {
    let uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid UUID format." })))),
    };

    let apikeys = sqlx::query_as!(ApiKeyByIDResponse, 
        "SELECT id, description, expiration_date, creation_date FROM apikeys WHERE id = $1 AND user_id = $2", 
        uuid, 
        user.id
    )
    .fetch_optional(&pool) // Borrow the connection pool
    .await;

    match apikeys {
        Ok(Some(apikeys)) => Ok(Json(apikeys)), // Return the API key as JSON if found
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": format!("API key with ID '{}' not found.", id) })),
        )),
        Err(_err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not get the API key."})),
        )),
    }
}
