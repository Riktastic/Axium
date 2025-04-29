use axum::{
    extract::{Multipart, State, Extension, Path},
    Json,
    http::StatusCode,
};
use serde_json::json;
use tracing::instrument;
use validator::Validate;
use uuid::Uuid;
use std::sync::Arc;

use crate::utils::auth::{hash_password, generate_totp_secret};
use crate::utils::process_image::process_image;
use crate::database::users::{insert_user_into_db, update_user_profile_picture_in_db, fetch_profile_picture_url_from_db};
use crate::storage::upload::upload_to_storage;
use crate::storage::delete::delete_from_storage;
use crate::models::user::{UserInsertResponse, UserInsertBody, UserProfilePictureUploadBody, ProfilePictureUploadResponse, User};
use crate::routes::AppState;
use crate::core::config::{get_env, get_env_with_default};

// --- Route Handler ---

// Define the API endpoint
#[utoipa::path(
    post,
    path = "/users",
    tag = "user",
    security(
        ("jwt_token" = [])
    ),
    request_body = UserInsertBody,
    responses(
        (status = 200, description = "User created successfully", body = UserInsertResponse),
        (status = 400, description = "Validation error", body = String),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[instrument(skip(state, user))]
pub async fn post_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<UserInsertBody>,
) -> Result<Json<UserInsertResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Validate input
    if let Err(errors) = user.validate() {
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

    // Hash the password before saving it
    let hashed_password = hash_password(&user.password)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to hash password." }))))?;

    // Generate TOTP secret if totp is Some("true")
    let totp_secret = if user.totp.as_deref() == Some("true") {
        generate_totp_secret()
    } else {
        String::new() // or some other default value
    };

    match insert_user_into_db(&state.database, &user.username, &user.email, &hashed_password, &totp_secret, 1, 1).await {
        Ok(new_user) => Ok(Json(new_user)),
        Err(_err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not create the user." }))
        )),
    }
}

#[utoipa::path(
    post,
    path = "/users/{id}/profile-picture",
    tag = "user",
    security(("jwt_token" = [])),
    request_body = UserProfilePictureUploadBody,
    responses(
        (status = 200, description = "Profile picture uploaded successfully", body = ProfilePictureUploadResponse),
        (status = 400, description = "Bad request, invalid UUID or no file uploaded", body = String),
        (status = 403, description = "Forbidden, insufficient permissions", body = serde_json::Value),
        (status = 500, description = "Internal server error, file upload or database issue", body = String)
    )
)]
pub async fn post_user_profilepicture(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<User>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Config
    const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB (updated from 5MB)

    // Authorization logic
    let allowed_role_levels = vec![2];
    let user_id = if id == "current" {
        current_user.id
    } else {
        if !allowed_role_levels.contains(&current_user.role_level) && id != current_user.id.to_string() {
            return Err((
                StatusCode::FORBIDDEN,
                Json(json!({ "error": "You do not have permission to upload for this user." })),
            ));
        }
        match Uuid::parse_str(&id) {
            Ok(uuid) => uuid,
            Err(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Invalid UUID format." })),
                ));
            }
        }
    };

    let bucket = get_env_with_default("STORAGE_BUCKET_PROFILE_PICTURES", "profile_pictures");
    let endpoint = get_env("STORAGE_ENDPOINT");
    let debug = std::env::var("IMAGE_DEBUG").is_ok();

    // Check existing profile picture
    if let Some(old_url) = fetch_profile_picture_url_from_db(&state.database, user_id)
        .await
        .map_err(|e| {
            eprintln!("DB fetch error: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to check existing profile picture" })),
            )
        })?
    {
        // Extract object key from URL
        let old_key = old_url.split('/').last().ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Invalid existing profile picture URL" })),
            )
        })?;

        // Delete old image from storage
        if let Err(e) = delete_from_storage(&state.storage, &bucket, old_key, &endpoint).await {
            eprintln!("Old image deletion failed: {e}");
            // Continue with upload despite deletion failure
        }
    }

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        eprintln!("Multipart error: {e}");
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid file data" })),
        )
    })? {
        if field.name() == Some("profile_picture") {
            // Content type validation
            let content_type = field.content_type().unwrap_or("").to_string();
            if !["image/webp", "image/jpeg", "image/png"].contains(&content_type.as_str()) {
                return Err((
                    StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    Json(json!({ "error": "Only WebP, JPEG, and PNG formats allowed" })),
                ));
            }

            // Read and validate file size
            let data = field.bytes().await.map_err(|e| {
                eprintln!("File read error: {e}");
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Failed to read file" })),
                )
            })?;

            if data.len() > MAX_FILE_SIZE {
                return Err((
                    StatusCode::PAYLOAD_TOO_LARGE,
                    Json(json!({ "error": format!("File too large (max {}MB)", MAX_FILE_SIZE / 1024 / 1024) })),
                ));
            }

            // Process image
            let processed_data = process_image(data, 300, 300, debug).await.map_err(|e| {
                eprintln!("Image processing failed: {e}");
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({ "error": format!("Image processing failed: {e}") })),
                )
            })?;

            // Generate secure filename
            let timestamp = chrono::Utc::now().timestamp();
            let object_key = format!("profile_pictures/{}_{}.webp", user_id, timestamp);

            // Upload processed image
            let file_url = upload_to_storage(
                &state.storage,
                &bucket,
                &object_key,
                &processed_data,
                &endpoint,
            )
            .await
            .map_err(|e| {
                eprintln!("Upload error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Upload failed" })),
                )
            })?;

            // Update database
            if let Err(e) = update_user_profile_picture_in_db(&state.database, user_id, &file_url).await {
                eprintln!("DB update error: {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Failed to update profile URL" })),
                ));
            }

            return Ok(Json(json!({ "url": file_url })));
        }
    }

    Err((
        StatusCode::BAD_REQUEST,
        Json(json!({ "error": "No file uploaded" })),
    ))
}
