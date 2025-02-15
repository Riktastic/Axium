use axum::{
    Router,
    routing::{get, post, delete},
    middleware::from_fn,
};
use sqlx::PgPool;

use crate::middlewares::auth::authorize;
use crate::handlers::{get_todos::{get_all_todos, get_todos_by_id}, post_todos::post_todo, delete_todos::delete_todo_by_id};

pub fn create_todo_routes() -> Router<PgPool> {
    Router::new()
        .route("/all", get(get_all_todos).layer(from_fn(|req, next| {
            let allowed_roles = vec![1, 2];
            authorize(req, next, allowed_roles)})))
        .route("/new", post(post_todo).layer(from_fn(|req, next| {
            let allowed_roles = vec![1, 2];
            authorize(req, next, allowed_roles)
        })))
        .route("/{id}", get(get_todos_by_id).layer(from_fn(|req, next| {
            let allowed_roles = vec![1, 2];
            authorize(req, next, allowed_roles)})))
        .route("/{id}", delete(delete_todo_by_id).layer(from_fn(|req, next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)})))
}
