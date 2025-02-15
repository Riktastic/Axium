use axum::{
    Router,
    routing::post,
    routing::get,
};
use sqlx::PgPool;

use crate::handlers::{signin::signin, protected::protected};
use crate::middlewares::auth::authorize;
use axum::middleware::from_fn;

pub fn create_auth_routes() -> Router<PgPool> {
    Router::new()
        .route("/signin", post(signin))
        .route("/protected", get(protected).layer(from_fn(|req,  next| {
            let allowed_roles = vec![1, 2];
            authorize(req, next, allowed_roles)
        })))
}
