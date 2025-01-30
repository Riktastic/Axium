use axum::{Extension, Json, response::IntoResponse};
use serde::{Serialize, Deserialize};
use crate::models::user::User;

#[derive(Serialize, Deserialize)]
struct UserResponse {
    id: i32, 
    username: String,
    email: String
}

pub async fn protected(Extension(user): Extension<User>) -> impl IntoResponse {
    Json(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email
    })
}