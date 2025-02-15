use axum::{
    Router,
    routing::get,
};
use sqlx::PgPool;

use crate::handlers::get_health::get_health;

pub fn create_health_route() -> Router<PgPool> {
    Router::new()
        .route("/health", get(get_health))
}
