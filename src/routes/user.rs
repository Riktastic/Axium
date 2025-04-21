use axum::{
    Router,
    routing::{get, post, delete},
    middleware::from_fn,
};
use sqlx::PgPool;

use crate::middlewares::auth::authorize;
use crate::handlers::{get_users::{get_all_users, get_users_by_id, get_current_user}, post_users::post_user, delete_users::delete_user_by_id};

pub fn create_user_routes() -> Router<PgPool> {
    Router::new()
        .route("/all", get(get_all_users).layer(from_fn(|req, next| {
            let allowed_roles = vec![2];
            authorize(req, next, allowed_roles)})))
        .route("/new", post(post_user).layer(from_fn(|req, next| {
            let allowed_roles = vec![2];
            authorize(req, next, allowed_roles)
        })))
        .route("/current", get(get_current_user).layer(from_fn(|req, next| {
            let allowed_roles = vec![1, 2]; // Or just 1 if only regular users, or 2 if only admins
            authorize(req, next, allowed_roles)
        })))
        .route("/{id}", get(get_users_by_id).layer(from_fn(|req, next| {
            let allowed_roles = vec![2];
            authorize(req, next, allowed_roles)})))
        .route("/{id}", delete(delete_user_by_id).layer(from_fn(|req, next| {
            let allowed_roles = vec![2];
            authorize(req, next, allowed_roles)})))
}