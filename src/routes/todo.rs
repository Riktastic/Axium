use axum::Router;
use crate::routes::AppState;
use std::sync::Arc;

use crate::handlers::{
    get_todos::{get_all_todos, get_todos_by_id},
    post_todos::post_todo,
    delete_todos::delete_todo_by_id
};
use crate::wrappers::authentication_route_builder::AuthenticatedRouteBuilder;

pub fn create_todo_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    AuthenticatedRouteBuilder::new(state)
        // Route for getting all todos
        .get("/all", get_all_todos, vec![1, 2])
        // Route for creating a new todo
        .post("/new", post_todo,vec![1, 2])
        // Route for getting a todo by ID
        .get("/{id}", get_todos_by_id, vec![1, 2])
        // Route for deleting a todo by ID
        .delete("/{id}", delete_todo_by_id, vec![1, 2])
        .build()
}