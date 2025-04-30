use axum::{response::IntoResponse, http::StatusCode, Json};
use serde_json::json;

pub async fn global_error_handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "status": "error",
            "details": "Not Found"
        })),
    )
}