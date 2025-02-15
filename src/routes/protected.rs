use axum::{Extension, Json, response::IntoResponse};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::user::User;
use tracing::instrument;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
struct UserResponse {
    id: Uuid, 
    username: String,
    email: String
}

#[utoipa::path(
    get,
    path = "/protected",
    tag = "protected",
    responses(
        (status = 200, description = "Protected endpoint accessed successfully", body = UserResponse),
        (status = 401, description = "Unauthorized", body = String)
    )
)]
#[instrument(skip(user))]
pub async fn protected(Extension(user): Extension<User>) -> impl IntoResponse {
    Json(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email
    })
}