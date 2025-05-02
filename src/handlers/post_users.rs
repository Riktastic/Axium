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
use rand::Rng;
use rand::distributions::Alphanumeric;
use chrono::Utc;
use chrono::Duration;
use tracing::error;

use crate::{core::config::{get_env_bool, get_env_with_default}, utils::auth::{generate_totp_secret, hash_password}};
use crate::utils::process_image::process_image;
use crate::database::users::{insert_user_into_db, update_user_profile_picture_in_db, fetch_profile_picture_url_from_db, fetch_user_by_email_from_db, insert_user_password_reset_code_into_db, update_user_password_in_db, fetch_current_password_reset_code_from_db, delete_all_password_reset_codes_for_user, check_user_exists_in_db, fetch_pending_user_by_email_from_db, activate_user_in_db, insert_pending_user_into_db};
use crate::storage::upload::upload_to_storage;
use crate::storage::delete::delete_from_storage;
use crate::storage::presign_url::generate_presigned_url;
use crate::models::user::{UserInsertResponse, UserInsertBody, UserProfilePictureUploadBody, UserProfilePictureUploadResponse, UserPasswordResetRequestBody, UserPasswordResetConfirmBody, UserRegisterBody, UserRegisterEmailVerifyBody, User};
use crate::routes::AppState;
use crate::mail::send::send_mail;

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
    let totp_secret = if user.totp.unwrap_or(false) {
        generate_totp_secret()
    } else {
        String::new() // or None, or whatever default you want
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
        (status = 200, description = "Profile picture uploaded successfully", body = UserProfilePictureUploadResponse),
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

    let bucket = "profile-pictures"; // or get from config/env
    let endpoint = &state.storage.endpoint_url;
    
    // Remove the endpoint prefix
    let path = old_url.strip_prefix(endpoint).unwrap_or(&old_url);
    // Remove leading slash if present
    let path = path.trim_start_matches('/');
    
    // Now, remove the bucket prefix
    let object_key = path.strip_prefix(&format!("{}/", bucket)).unwrap_or(path);
    
    // Now use object_key
    if let Err(e) = delete_from_storage(&state.storage, bucket, object_key).await {
        error!("Old image deletion failed: {e}");
        // Continue with upload despite deletion failure
    }

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        error!("Multipart error: {e}");
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
                error!("File read error: {e}");
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
                error!("Image processing failed: {e}");
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
            )
            .await
            .map_err(|e| {
                error!("Upload error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Upload failed" })),
                )
            })?;

            // Update database
            if let Err(e) = update_user_profile_picture_in_db(&state.database, user_id, &file_url).await {
                error!("DB update error: {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Failed to update profile URL" })),
                ));
            }

            // Generate pre-signed URL (valid for 15 minutes)
            let presigned_url = generate_presigned_url(
                &state.storage,
                &bucket,
                &object_key,
                900
            )
            .await
            .map_err(|e| {
                error!("Presign error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Failed to generate presigned URL" })),
                )
            })?;


            return Ok(Json(json!({
                "profile_picture_url": file_url,
                "profile_picture_presigned_url": presigned_url
            })));
        }
    }

    Err((
        StatusCode::BAD_REQUEST,
        Json(json!({ "error": "No file uploaded" })),
    ))
}

#[utoipa::path(
    post,
    path = "/reset",
    tag = "user",
    security(("jwt_token" = [])),  // If you want to secure the route with JWT authentication, add this
    request_body = UserPasswordResetRequestBody,
    responses(
        (status = 200, description = "Password reset code sent successfully", body = String),
        (status = 400, description = "Bad request, invalid email format", body = String),
        (status = 404, description = "User not found", body = String),
        (status = 500, description = "Internal server error, database or email issue", body = String)
    )
)]
pub async fn post_user_password_reset(
    State(state): State<Arc<AppState>>,
    Json(body): Json<UserPasswordResetRequestBody>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // 1. Find user by email
    let user = match fetch_user_by_email_from_db(&state.database, &body.email).await {
        Ok(Some(user)) => user,
        Ok(None) => return Ok(StatusCode::OK), // Don't reveal if email exists
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Database error"})))),
    };

    // 2. Generate code and expiry
    let code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    let expires_at = Utc::now() + Duration::hours(24);

    // 3. Store code in DB
    insert_user_password_reset_code_into_db(&state.database, user.id, &code, expires_at)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to store reset code"}))))?;

    // 4. Send email
    let subject = "Password reset request";
    let body = format!(
        "Use this code to reset your password: {}\n\nThis code will expire in 24 hours.",
        code
    );
    send_mail(&state.mail, &user.email, subject, &body)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to send email"}))))?;

    Ok(StatusCode::OK)
}

#[utoipa::path(
    post,
    path = "/reset/verify",
    tag = "user",
    security(("jwt_token" = [])),  // If you want to secure the route with JWT authentication, add this
    request_body = UserPasswordResetConfirmBody,
    responses(
        (status = 200, description = "Password reset successful", body = String),
        (status = 400, description = "Bad request, invalid code or email", body = String),
        (status = 404, description = "User not found", body = String),
        (status = 400, description = "Invalid or expired reset code", body = String),
        (status = 500, description = "Internal server error, database issue", body = String)
    )
)]
#[instrument(skip(state, body))]
pub async fn post_user_password_reset_verify(
    State(state): State<Arc<AppState>>,
    Json(body): Json<UserPasswordResetConfirmBody>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // 1. Validate new password (example: at least 8 chars)
    if body.new_password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Password must be at least 8 characters long." }))
        ));
    }

    // 2. Find user by email
    let user = match fetch_user_by_email_from_db(&state.database, &body.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            // Don't reveal if email exists or not
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid code or email." }))
            ));
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Database error." }))
            ));
        }
    };

    // 3. Fetch and verify reset code
    #[allow(unused_variables)] // Currently not needed for further processing
    let reset_code = match fetch_current_password_reset_code_from_db(&state.database, user.id).await {
        Ok(Some(code)) => {
            // Check if the reset code from the database matches the provided code
            if code.code == body.code {
                // The reset code is valid
                // Proceed with the next steps
            } else {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Invalid or expired code." }))
                ));
            }
        },
        _ => {
            // If no code was found or there's an error, return an invalid code response
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid or expired code." }))
            ));
        }
    };
        

    // 4. Hash new password
    let new_password_hash = hash_password(&body.new_password)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to hash password." }))))?;

    // 5. Update user's password
    update_user_password_in_db(&state.database, user.id, &new_password_hash).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to update password." }))))?;

    // 6. Invalidate the reset code
    delete_all_password_reset_codes_for_user(&state.database, user.id).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to invalidate reset code." }))))?;

    Ok(StatusCode::OK)
}

#[utoipa::path(
    post,
    path = "/register",
    tag = "user",
    request_body = UserRegisterBody,
    responses(
        (status = 200, description = "Registration successful, verification email sent", body = String),
        (status = 400, description = "Invalid input", body = String),
        (status = 409, description = "User/email already exists", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
pub async fn post_user_register(
    State(state): State<Arc<AppState>>,
    Json(user): Json<UserRegisterBody>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
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

    // Check if user/email exists
    if check_user_exists_in_db(&state.database, &user.email, &user.username)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {e}") }))
        )
    })?
    {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({ "error": "User or email already exists." }))
        ));
    }

    // Hash password
    let hashed_password = hash_password(&user.password)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to hash password." }))))?;

    // Generate TOTP secret if requested
    let totp_secret = if user.totp.unwrap_or(false) {
        Some(generate_totp_secret())
    } else {
        None
    };

    // Generate verification code
    let code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    let expires_at = Utc::now() + Duration::hours(24);

    // Insert user in "pending" state
    insert_pending_user_into_db(
        &state.database,
        &user.username,
        &user.email,
        &hashed_password,
        &code,
        expires_at,
        user.first_name.as_deref(),
        user.last_name.as_deref(),
        user.country_code.as_deref(),
        user.language_code.as_deref(),
        user.birthday,
        user.description.as_deref(),
        totp_secret.as_deref()
    ).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to create user." }))))?;

    // Send verification email
    let subject = "Verify your email";
    let body = format!(
        "Welcome! Please verify your email by using this code: {}\n\nThis code will expire in 24 hours.",
        code
    );
    send_mail(&state.mail, &user.email, subject, &body)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to send verification email." }))))?;

    Ok(StatusCode::OK)
}

#[utoipa::path(
    post,
    path = "/register/verify",
    tag = "user",
    request_body = UserRegisterEmailVerifyBody,
    responses(
        (status = 200, description = "Email verified successfully", body = String),
        (status = 400, description = "Invalid code or email", body = String),
        (status = 404, description = "User not found", body = String),
        (status = 410, description = "Verification code expired", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
pub async fn post_user_register_verify(
    State(state): State<Arc<AppState>>,
    Json(body): Json<UserRegisterEmailVerifyBody>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // 1. Find user by email
    let user = match fetch_pending_user_by_email_from_db(&state.database, &body.email).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err((StatusCode::NOT_FOUND, Json(json!({ "error": "User not found." })))),
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Database error." })))),
    };

    // 2. Check code and expiry
    if user.verification_code.as_deref() != Some(body.code.as_str()) {
        return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid code." }))));
    }

    if user.verification_expires_at.is_none() || Utc::now() > user.verification_expires_at.unwrap() {
        return Err((StatusCode::GONE, Json(json!({ "error": "Verification code expired." }))));
    }

    // 3. Activate user
    activate_user_in_db(&state.database, user.id).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to activate user." }))))?;

    Ok(StatusCode::OK)
}
