use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use sqlx::postgres::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::models::user::*;

// Get all users
#[utoipa::path(
    get,
    path = "/users/all",
    tag = "user",
    responses(
        (status = 200, description = "Successfully fetched all users", body = [UserResponse]),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument(skip(pool))]
pub async fn get_all_users(State(pool): State<PgPool>) -> impl IntoResponse {
    let users = sqlx::query_as!(UserResponse, "SELECT id, username, email, role_level, tier_level, creation_date FROM users")
        .fetch_all(&pool)
        .await;

    match users {
        Ok(users) => Ok(Json(users)),
        Err(_err) => Err((
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
        (status = 200, description = "Successfully fetched user by ID", body = UserResponse),
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

    let user = sqlx::query_as!(UserResponse, "SELECT id, username, email, role_level, tier_level, creation_date FROM users WHERE id = $1", uuid)
        .fetch_optional(&pool)
        .await;

    match user {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": format!("User with ID '{}' not found", id) })),
        )),
        Err(_err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not fetch the users details." })),
        )),
    }
}

// Get a single user by username
// pub async fn get_user_by_username(
//     State(pool): State<PgPool>,
//     Path(username): Path<String>,
// ) -> impl IntoResponse {
//     let user = sqlx::query_as!(User, "SELECT id, username, email, password_hash, totp_secret, role_level, tier_level, creation_date FROM users WHERE username = $1", username)
//         .fetch_optional(&pool)
//         .await;

//     match user {
//         Ok(Some(user)) => Ok(Json(user)),
//         Ok(None) => Err((
//             StatusCode::NOT_FOUND,
//             Json(json!({ "error": format!("User with username '{}' not found", username) })),
//         )),
//         Err(err) => Err((
//             StatusCode::INTERNAL_SERVER_ERROR,
//             Json(json!({ "error": "Could not fetch the users details." })),
//         )),
//     }
// }

// Get a single user by email
// pub async fn get_user_by_email(
//     State(pool): State<PgPool>,
//     Path(email): Path<String>,
// ) -> impl IntoResponse {
//     let user = sqlx::query_as!(User, "SELECT id, username, email, password_hash, totp_secret, role_level, tier_level, creation_date FROM users WHERE email = $1", email)
//         .fetch_optional(&pool)
//         .await;

//     match user {
//         Ok(Some(user)) => Ok(Json(user)),
//         Ok(None) => Err((
//             StatusCode::NOT_FOUND,
//             Json(json!({ "error": format!("User with email '{}' not found", email) })),
//         )),
//         Err(err) => Err((
//             StatusCode::INTERNAL_SERVER_ERROR,
//             Json(json!({ "error": "Could not fetch the users details." })),
//         )),
//     }
// }
