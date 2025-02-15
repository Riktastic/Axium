use axum::{Extension, Json, response::IntoResponse};
use crate::models::user::{User, UserGetResponse};
use tracing::instrument;

#[utoipa::path(
    get,
    path = "/protected",
    tag = "protected",
    security(
        ("jwt_token" = [])
    ),
    responses(
        (status = 200, description = "Protected endpoint accessed successfully", body = UserGetResponse),
        (status = 401, description = "Unauthorized", body = String)
    )
)]
#[instrument(skip(user))]
pub async fn protected(Extension(user): Extension<User>) -> impl IntoResponse {
    Json(UserGetResponse {id:user.id,username:user.username,email:user.email, role_level: user.role_level, tier_level: user.tier_level, creation_date: user.creation_date
    })
}