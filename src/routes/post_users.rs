use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::postgres::PgPool;
use crate::models::user::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserBody {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub totp_secret: String,
    pub role_id: i32,
}

// Add a new user
pub async fn post_user(State(pool): State<PgPool>, Json(user): Json<UserBody>, ) -> impl IntoResponse {
    let row = sqlx::query!(
        "INSERT INTO users (username, email, password_hash, totp_secret, role_id) VALUES ($1, $2, $3, $4, $5) RETURNING id, username, email, password_hash, totp_secret, role_id",
        user.username,
        user.email,
        user.password_hash,
        user.totp_secret,
        user.role_id
    )
    .fetch_one(&pool) // Use `&pool` to borrow the connection pool
    .await;

    match row {
        Ok(row) => Ok(Json(User {
            id: row.id,
            username: row.username,
            email: row.email,
            password_hash: row.password_hash,
            totp_secret: row.totp_secret,
            role_id: row.role_id,
        })),
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", err),
        )),
    }
}