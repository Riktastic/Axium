use axum::{
    Router,
    routing::get,
    middleware::from_fn,
};
use sqlx::PgPool;

use crate::middlewares::auth::authorize;
use crate::handlers::get_usage::{get_usage_last_day, get_usage_last_week};

pub fn create_usage_routes() -> Router<PgPool> {
    Router::new()
        .route("/lastday", get(get_usage_last_day).layer(from_fn(|req, next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)})))
        .route("/lastweek", get(get_usage_last_week).layer(from_fn(|req, next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)})))
}
