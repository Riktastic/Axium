use axum::{
    extract::{State, Extension, Path},
    Json,
    http::StatusCode,
};
use axum::response::IntoResponse;
use serde_json::json;
use tracing::instrument;
use uuid::Uuid;
use std::sync::Arc;

use crate::models::user::{User, UserGetResponse};
use crate::database::users::{fetch_all_active_users_from_db, fetch_active_user_by_field_from_db};
use crate::routes::AppState;

use crate::storage::presign_url::generate_presigned_url;

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
#[instrument(skip(state))]
pub async fn get_all_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match fetch_all_active_users_from_db(&state.database).await {
        Ok(users) => {
            // For each user, add the presigned URL if profile_picture_url is present
            let mut enriched_users = Vec::with_capacity(users.len());
            for user in users {
                let mut user_json = serde_json::to_value(&user)
                    .expect("User should serialize to JSON");

                if let Some(ref stored_url) = user.profile_picture_url {
                    let endpoint = &state.storage.endpoint_url;
                    let url = stored_url.strip_prefix(endpoint).unwrap_or(stored_url);
                    let url = url.trim_start_matches('/');
                    let mut parts = url.splitn(2, '/');
                    let bucket = parts.next().unwrap_or("");
                    let object_key = parts.next().unwrap_or("");

                    if !bucket.is_empty() && !object_key.is_empty() {
                        if let Ok(presigned_url) =
                            generate_presigned_url(&state.storage, bucket, object_key, 900).await
                        {
                            user_json["profile_picture_presigned_url"] = json!(presigned_url);
                        }
                    }
                }

                enriched_users.push(user_json);
            }

            Ok(Json(enriched_users))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not fetch the users details." })),
        )),
    }
}

// Get a single user by ID or current user
#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "user",
    params(
        ("id" = String, Path, description = "User ID or 'current'")
    ),
    responses(
        (status = 200, description = "Successfully fetched user by ID or current user", body = UserGetResponse),
        (status = 400, description = "Invalid UUID format"),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument(skip(state))]
pub async fn get_users_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Extension(current_user): Extension<User>,
) -> impl IntoResponse {

    let allowed_role_levels = vec![2, 3]; // Add any other role levels that should have access

    // Check if the current user has the required role level to fetch by custom ID
    if id != "current" && !allowed_role_levels.contains(&current_user.role_level) {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "You do not have permission to access this resource." })),
        ));
    }

    let user_id = if id == "current" {
        current_user.id
    } else {
        match Uuid::parse_str(&id) {
            Ok(uuid) => uuid,
            Err(_) => return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid UUID format." })))),
        }
    };

    match fetch_active_user_by_field_from_db(&state.database, "id", &user_id.to_string()).await {
        Ok(Some(user)) => {
            let mut user_json = serde_json::to_value(&user)
                .expect("User should serialize to JSON");
        
            if let Some(ref stored_url) = user.profile_picture_url {
                let endpoint = &state.storage.endpoint_url;
                let url = stored_url.strip_prefix(endpoint).unwrap_or(stored_url);
                let url = url.trim_start_matches('/');
                let mut parts = url.splitn(2, '/');
                let bucket = parts.next().unwrap_or("");
                let object_key = parts.next().unwrap_or("");
        
                if !bucket.is_empty() && !object_key.is_empty() {
                    if let Ok(presigned_url) = generate_presigned_url(&state.storage, bucket, object_key, 900).await {
                        // Insert the presigned URL as a new field
                        user_json["profile_picture_presigned_url"] = json!(presigned_url);
                    }
                }
            }
        
            Ok(Json(user_json))
        }
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": format!("User with ID '{}' not found", user_id) })),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not fetch the users details." })),
        )),
    }
}