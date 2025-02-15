use axum::{extract::{Extension, State}, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use serde_json::json;
use sqlx::postgres::PgPool;
use tracing::instrument;
use utoipa::ToSchema;

use crate::models::user::*;

#[derive(Debug, Serialize, ToSchema)]
pub struct UsageResponseLastDay {
    #[serde(rename = "requests_last_24_hours")]
    pub count: i64  // or usize depending on your count type
}

// Get usage for the last 24 hours
#[utoipa::path(
    get,
    path = "/usage/lastday",
    tag = "usage",
    responses(
        (status = 200, description = "Successfully fetched usage for the last 24 hours", body = UsageResponseLastDay),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument(skip(pool))]
pub async fn get_usage_last_day(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,  // Extract current user from the request extensions
) -> impl IntoResponse {
    let result = sqlx::query!("SELECT count(*) FROM usage WHERE user_id = $1 AND creation_date > NOW() - INTERVAL '24 hours';", user.id) 
        .fetch_one(&pool) // Borrow the connection pool
        .await;

    match result {
        Ok(row) => {
            let count = row.count.unwrap_or(0) as i64;
            Ok(Json(json!({ "requests_last_24_hours": count })))
        },
        Err(_err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not fetch the usage data." }))),
        ),
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UsageResponseLastWeek {
    #[serde(rename = "requests_last_7_days")]
    pub count: i64  // or usize depending on your count type
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
    Extension(user): Extension<User>,  // Extract current user from the request extensions
) -> impl IntoResponse {
    let result = sqlx::query!("SELECT count(*) FROM usage WHERE user_id = $1 AND creation_date > NOW() - INTERVAL '7 days';", user.id)
        .fetch_one(&pool) // Borrow the connection pool
        .await;

    match result {
        Ok(row) => {
            let count = row.count.unwrap_or(0) as i64;
            Ok(Json(json!({ "requests_last_7_days": count })))
        },
        Err(_err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not fetch the usage data." }))),
        ),
    }
}
