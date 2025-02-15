use axum::{
    Router,
    routing::{get, post, delete},
    middleware::from_fn,
};
use sqlx::PgPool;

use crate::middlewares::auth::authorize;
use crate::handlers::{get_apikeys::{get_all_apikeys, get_apikeys_by_id}, post_apikeys::post_apikey, rotate_apikeys::rotate_apikey, delete_apikeys::delete_apikey_by_id};

pub fn create_apikey_routes() -> Router<PgPool> {
    Router::new()
        .route("/all", get(get_all_apikeys).layer(from_fn(|req, next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)})))
        .route("/new", post(post_apikey).layer(from_fn(|req, next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)
        })))
        .route("/{id}", get(get_apikeys_by_id).layer(from_fn(|req, next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)})))
        .route("/{id}", delete(delete_apikey_by_id).layer(from_fn(|req, next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)})))
        .route("/rotate/{id}", post(rotate_apikey).layer(from_fn(|req, next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)})))
}