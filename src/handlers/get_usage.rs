use axum::{extract::{Extension, State}, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use sqlx::postgres::PgPool;
use tracing::instrument;

use crate::models::user::*;
use crate::models::usage::*;
use crate::database::usage::fetch_usage_count_from_db;

// Get usage for the last 24 hours
#[utoipa::path(
    get,
    path = "/usage/lastday",
    tag = "usage",
    security(
        ("jwt_token" = [])
    ),
    responses(
        (status = 200, description = "Successfully fetched usage for the last 24 hours", body = UsageResponseLastDay),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument(skip(pool))]
pub async fn get_usage_last_day(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    match fetch_usage_count_from_db(&pool, user.id, "24 hours").await {
        Ok(count) => Ok(Json(json!({ "requests_last_24_hours": count }))),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not fetch the usage data." }))
        )),
    }
}

// Get usage for the last 7 days
#[utoipa::path(
    get,
    path = "/usage/lastweek",
    tag = "usage",
    responses(
        (status = 200, description = "Successfully fetched usage for the last 7 days", body = UsageResponseLastDay),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument(skip(pool))]
pub async fn get_usage_last_week(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    match fetch_usage_count_from_db(&pool, user.id, "7 days").await {
        Ok(count) => Ok(Json(json!({ "requests_last_7_days": count }))),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not fetch the usage data." }))
        )),
    }
}
