use axum::{
    Router,
    routing::get,
};
use sqlx::PgPool;

use crate::handlers::homepage::homepage;

pub fn create_homepage_route() -> Router<PgPool> {
    Router::new()
        .route("/", get(homepage))
}
