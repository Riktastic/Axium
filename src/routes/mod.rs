// Module declarations for different route handlers
pub mod get_todos;
pub mod get_users;
pub mod post_todos;
pub mod post_users;
pub mod get_health;
pub mod protected;

// Re-exporting modules to make their contents available at this level
pub use get_todos::*;
pub use get_users::*;
pub use post_todos::*;
pub use post_users::*;
pub use get_health::*;
pub use protected::*;

use axum::{
    Router,
    routing::{get, post},
};

use sqlx::PgPool;

use crate::middlewares::auth::{sign_in, authorize};

/// Function to create and configure all routes
pub fn create_routes(database_connection: PgPool) -> Router {
    // Authentication routes
    let auth_routes = Router::new()
        .route("/signin", post(sign_in))
        .route("/protected", get(protected).route_layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![1, 2];
            authorize(req, next, allowed_roles)
        })));

    // User-related routes
    let user_routes = Router::new()
        .route("/all", get(get_all_users))
        .route("/{id}", get(get_users_by_id))
        .route("/", post(post_user));

    // Todo-related routes
    let todo_routes = Router::new()
        .route("/all", get(get_all_todos))
        .route("/", post(post_todo).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![1, 2];
            authorize(req, next, allowed_roles)
        })))
        .route("/{id}", get(get_todos_by_id));

    // Combine all routes and add middleware
    Router::new()
        .merge(auth_routes)  // Add authentication routes
        .nest("/users", user_routes)  // Add user routes under /users
        .nest("/todos", todo_routes)  // Add todo routes under /todos
        .route("/health", get(get_health))  // Add health check route
        .layer(axum::Extension(database_connection.clone()))  // Add database connection to all routes
        .with_state(database_connection)  // Add database connection as state
}
