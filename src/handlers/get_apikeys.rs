use axum::{
    extract::{State, Extension, Path}, 
    Json,
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
use crate::database::apikeys::{fetch_all_apikeys_from_db, fetch_apikey_by_id_from_db};

// --- Route Handlers ---

// Get all API keys
#[utoipa::path(
    get,
    path = "/apikeys",
    tag = "apikey",
    security(
        ("jwt_token" = [])
    ),
    responses(
        (status = 200, description = "Get all API keys", body = [ApiKeyResponse]),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
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
) -> Result<Json<Vec<ApiKeyResponse>>, (StatusCode, Json<serde_json::Value>)> {
    match fetch_all_apikeys_from_db(&pool, user.id).await {
        Ok(apikeys) => Ok(Json(apikeys)), // Return all API keys as JSON
        Err(_err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not get the API keys."})),
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
) -> Result<Json<ApiKeyByIDResponse>, (StatusCode, Json<serde_json::Value>)> {
    let uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid UUID format." })))),
    };

    match fetch_apikey_by_id_from_db(&pool, uuid, user.id).await {
        Ok(Some(apikey)) => Ok(Json(apikey)), // Return the API key as JSON if found
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