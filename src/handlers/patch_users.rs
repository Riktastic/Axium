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
use crate::models::error::ErrorResponse;
use crate::routes::AppState;

use validator::Validate;

// --- Route Handler ---

/// Updates a user's profile fields with comprehensive validation
///
/// This endpoint allows a user to update their own profile, or an admin to update any user's profile.
/// Fields not included in the request body will remain unchanged. Fields set to `null` (if supported by the struct)
/// will be set to `NULL` in the database.
///
/// # Validation Layers
/// 1. **Structural Validation**: Handled by `UserUpdateBody`'s `deny_unknown_fields` attribute
/// 2. **Business Logic Validation**: Manual checks for role_level, tier_level, and birthday
///
/// # Request Flow
/// 1. Permission check (self or admin)
/// 2. UUID validation
/// 3. Business logic validation
/// 4. Database update
///
/// # Error Responses
/// - **400 Bad Request**: Automatic for unknown fields + manual validation errors
/// - **403 Forbidden**: Authorization failures
/// - **500 Internal Server Error**: Database errors
/// 
///  ToDo: Haven't been able to clean up the error messages. Deserialization fails in most cases.
#[utoipa::path(
    patch,
    path = "/users/{id}",
    tag = "user",
    security(("jwt_token" = [])),
    request_body = UserUpdateBody,
    params(("id" = String, Path, description = "User UUID or 'current'")),
    responses(
        (status = 200, description = "Profile updated successfully", body = UserUpdateResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 403, description = "Not allowed", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
)]
#[instrument(skip(state, current_user, update))]
pub async fn patch_user_profile(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<User>,
    Json(update): Json<UserUpdateBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // --- Permission Validation ---
    let is_admin = current_user.role_level == 2;
    let target_user_id = if id == "current" {
        current_user.id
    } else {
        match uuid::Uuid::parse_str(&id) {
            Ok(uuid) => {
                if uuid != current_user.id && !is_admin {
                    return Err((StatusCode::FORBIDDEN, Json(json!({ "error": "Not allowed" }))));
                }
                uuid
            }
            Err(_) => return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid UUID" })))),
        }
    };

    // --- Business Logic Validation ---
    let mut validation_errors = Vec::new();

    // Role Level Validation
    if let Some(role_level) = update.role_level {
        validate_role_level(role_level, is_admin, current_user.role_level, &mut validation_errors);
    }

    // Tier Level Validation
    if let Some(tier_level) = update.tier_level {
        validate_tier_level(tier_level, is_admin, current_user.tier_level, &mut validation_errors);
    }

    // Birthday Validation
    if let Some(birthday) = update.birthday {
        validate_birthday(birthday, &mut validation_errors);
    }

    // --- Error Handling ---
    if let Err(validation_errors) = update.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Validation failed",
                "details": validation_errors
            }))
        ));
    }

    // --- Database Operation ---
    match update_user_in_db(&state.database, target_user_id, update).await {
        Ok(_) => Ok(Json(json!({ "success": true }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) }))
        )),
    }
}

// --- Validation Helpers ---

/// Validates role level changes
/// - Admins can only set 1 (regular) or 2 (admin)
/// - Regular users can't change their role
fn validate_role_level(
    new_level: i32,
    is_admin: bool,
    current_level: i32,
    errors: &mut Vec<String>
) {
    if is_admin {
        if ![1, 2].contains(&new_level) {
            errors.push("Role level must be 1 (regular) or 2 (admin)".into());
        }
    } else if new_level != current_level {
        errors.push("Cannot modify your own role level".into());
    }
}

/// Validates tier level changes
/// - Admins can set 1-4
/// - Regular users can't change their tier
fn validate_tier_level(
    new_level: i32,
    is_admin: bool,
    current_level: i32,
    errors: &mut Vec<String>
) {
    if is_admin {
        if !(1..=4).contains(&new_level) {
            errors.push("Tier level must be between 1-4".into());
        }
    } else if new_level != current_level {
        errors.push("Cannot modify your own tier level".into());
    }
}

/// Validates birthday is not in the future
fn validate_birthday(
    birthday: Option<chrono::NaiveDate>,
    errors: &mut Vec<String>
) {
    if let Some(bdate) = birthday {
        let today = chrono::Utc::now().naive_utc().date();
        if bdate > today {
            errors.push("Birthday cannot be in the future".into());
        }
    }
}