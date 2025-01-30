use axum::extract::{State, Path};
use axum::Json;
use axum::response::IntoResponse;
use sqlx::postgres::PgPool;
use crate::models::user::*;  // Import the User struct

// Get all users
pub async fn get_all_users(State(pool): State<PgPool>,) -> impl IntoResponse {
    let users = sqlx::query_as!(User, "SELECT id, username, email, password_hash, totp_secret, role_id  FROM users") // Your table name
        .fetch_all(&pool) // Borrow the connection pool
        .await;

    match users {
        Ok(users) => Ok(Json(users)), // Return all users as JSON
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error fetching users: {}", err),
        )),
    }
}

// Get a single user by id
pub async fn get_users_by_id(
        State(pool): State<PgPool>,
        Path(id): Path<i32>, // Use Path extractor here
    ) -> impl IntoResponse {

    let user = sqlx::query_as!(User, "SELECT id, username, email, password_hash, totp_secret, role_id FROM users WHERE id = $1", id)
        .fetch_optional(&pool) // Borrow the connection pool
        .await;

    match user {
        Ok(Some(user)) => Ok(Json(user)), // Return the user as JSON if found
        Ok(None) => Err((
            axum::http::StatusCode::NOT_FOUND,
            format!("User with id {} not found", id),
        )),
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error fetching user: {}", err),
        )),
    }
}

// Get a single user by username
pub async fn get_user_by_username(
    State(pool): State<PgPool>,
    Path(username): Path<String>, // Use Path extractor here for username
) -> impl IntoResponse {
    let user = sqlx::query_as!(User, "SELECT id, username, email, password_hash, totp_secret, role_id FROM users WHERE username = $1", username)
        .fetch_optional(&pool) // Borrow the connection pool
        .await;

    match user {
        Ok(Some(user)) => Ok(Json(user)), // Return the user as JSON if found
        Ok(None) => Err((
            axum::http::StatusCode::NOT_FOUND,
            format!("User with username {} not found", username),
        )),
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error fetching user: {}", err),
        )),
    }
}

// Get a single user by email
pub async fn get_user_by_email(
    State(pool): State<PgPool>,
    Path(email): Path<String>, // Use Path extractor here for email
) -> impl IntoResponse {
    let user = sqlx::query_as!(User, "SELECT id, username, email, password_hash, totp_secret, role_id FROM users WHERE email = $1", email)
        .fetch_optional(&pool) // Borrow the connection pool
        .await;

    match user {
        Ok(Some(user)) => Ok(Json(user)), // Return the user as JSON if found
        Ok(None) => Err((
            axum::http::StatusCode::NOT_FOUND,
            format!("User with email {} not found", email),
        )),
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error fetching user: {}", err),
        )),
    }
}
