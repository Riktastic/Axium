use axum::{
    extract::{State, Extension, Path},
    Json,
    http::StatusCode,
};
use serde_json::json;
use tracing::instrument;
use std::sync::Arc;

use crate::database::users::update_user_in_db;
use crate::models::user::{User, UserUpdateBody, UserUpdateResponse};
use crate::routes::AppState;

// --- Route Handler ---

/// Updates a user's profile fields.
///
/// This endpoint allows a user to update their own profile, or an admin to update any user's profile.
/// Fields not included in the request body will remain unchanged. Fields set to `null` (if supported by the struct)
/// will be set to `NULL` in the database.
///
/// - Regular users can only update their own profile (`/users/current` or their own UUID).
/// - Admins (role_level == 2) can update any user's profile.
///
/// # Request body
/// Partial user profile information to update. Omitted fields are not changed.
///
/// # Responses
/// - 200: Profile updated successfully
/// - 400: Validation error
/// - 401: Unauthorized
/// - 403: Not allowed
/// - 500: Internal server error
///
#[utoipa::path(
    patch,
    path = "/users/{id}",
    tag = "user",
    security(
        ("jwt_token" = []),
    ),
    request_body = UserUpdateBody,
    params(
        ("id" = String, Path, description = "User UUID or 'current' for the current user"),
    ),
    responses(
        (status = 200, description = "Profile updated successfully", body = UserUpdateResponse),
        (status = 400, description = "Validation error", body = String),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 403, description = "Not allowed", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = String)
    ),
)]
#[instrument(skip(state, current_user, update))]
pub async fn patch_user_profile(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<User>,
    Json(update): Json<UserUpdateBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Permission check: allow self or admin
    let is_admin = current_user.role_level == 2;
    let target_user_id = if id == "current" {
        current_user.id
    } else {
        match uuid::Uuid::parse_str(&id) {
            Ok(uuid) => {
                if uuid != current_user.id && !is_admin {
                    return Err((StatusCode::FORBIDDEN, Json(json!({ "error": "Not allowed" }))));  // 403
                }
                uuid
            }
            Err(_) => return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid UUID" })))), // 400
        }
    };

    // Validate fields in `UserUpdateBody`
    let mut validation_errors = Vec::new();

    // Check for any unknown fields in the update struct
    let known_fields = vec![
        "first_name", "last_name", "country_code", "language_code", 
        "birthday", "description", "role_level", "tier_level"
    ];

    // Convert `update` to a map to easily check for unknown fields
    let update_value = serde_json::to_value(&update).unwrap_or_default();
    let update_map = update_value.as_object().cloned().unwrap_or_default();
    
    for field in update_map.keys() {
        if !known_fields.contains(&field.as_str()) {
            validation_errors.push(format!("Unknown field: {}", field));
        }
    }

    // Role level validation
    if let Some(role_level) = update.role_level {
        if is_admin && (role_level != 1 && role_level != 2) {
            validation_errors.push("Invalid role level".to_string());
        } else if !is_admin && role_level != current_user.role_level {
            validation_errors.push("Cannot change your own role level".to_string());
        }
    }

    // Tier level validation
    if let Some(tier_level) = update.tier_level {
        // Add your tier_level validation logic here
        if is_admin {
            // Example: Validate tier levels 1-4 for admin
            if tier_level < 1 || tier_level > 4 {
                validation_errors.push("Tier level must be between 1-4".to_string());
            }
        } else if tier_level != current_user.tier_level {
            validation_errors.push("Cannot change your own tier level".to_string());
        }
    }

    // Birthday validation
    if let Some(birthday) = update.birthday {
        // Get current date as NaiveDate
        let current_date = chrono::Utc::now().naive_utc().date();
        
        // Compare NaiveDate with NaiveDate
        if birthday > Some(current_date) {
            validation_errors.push("Birthday cannot be in the future".to_string());
        }
    }

    // Return validation errors if any
    if !validation_errors.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(json!({ "errors": validation_errors }))));
    }

    // Proceed to update the database (if no validation errors)
    match update_user_in_db(&state.database, target_user_id, update).await {
        Ok(_) => Ok(Json(json!({ "success": true }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("{e}") })))), // 500
    }
}