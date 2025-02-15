// Module declarations for different route handlers
pub mod homepage;
pub mod get_todos;
pub mod get_users;
pub mod get_apikeys;
pub mod get_usage;
pub mod post_todos;
pub mod post_users;
pub mod post_apikeys;
pub mod rotate_apikeys;
pub mod get_health;
pub mod delete_users;
pub mod delete_todos;
pub mod delete_apikeys;
pub mod protected;

// Re-exporting modules to make their contents available at this level
pub use homepage::*;
pub use get_todos::*;
pub use get_users::*;
pub use get_apikeys::*;
pub use get_usage::*;
pub use rotate_apikeys::*;
pub use post_todos::*;
pub use post_users::*;
pub use post_apikeys::*;
pub use get_health::*;
pub use delete_users::*;
pub use delete_todos::*;
pub use delete_apikeys::*;
pub use protected::*;

use axum::{
    Router,
    routing::{get, post, delete}
};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::middlewares::auth::{sign_in, authorize};

// Define the OpenAPI documentation structure
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Axium",
        description = "An example API built with Rust, Axum, SQLx, and PostgreSQL.",
        version = "1.0.0",
        contact(
            url = "https://github.com/Riktastic/Axium"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    paths(
        get_all_users,
        get_users_by_id,
        get_all_apikeys,
        get_apikeys_by_id,
        get_usage_last_day,
        get_usage_last_week,
        get_all_todos,
        get_todos_by_id,
        get_health,
        post_user,
        post_apikey,
        post_todo,
        rotate_apikey,
        delete_user_by_id,
        delete_apikey_by_id,
        delete_todo_by_id,
        protected,
        //sign_in, // Add sign_in path
    ),
    components(
        schemas(
            UserResponse,
            // ApiKeyResponse,
            // ApiKeyByIDResponse,
            // Todo,
            // SignInData,
            // ...add other schemas as needed...
        )
    ),
    tags(
        (name = "user", description = "User related endpoints."),
        (name = "apikey", description = "API key related endpoints."),
        (name = "usage", description = "Usage related endpoints."),
        (name = "todo", description = "Todo related endpoints."),
        (name = "health", description = "Health check endpoint."),
    )
)]
struct ApiDoc;

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
        .route("/all", get(get_all_users).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![2];
            authorize(req, next, allowed_roles)})))
        .route("/new", post(post_user).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![2];
            authorize(req, next, allowed_roles)
        })))
        .route("/{id}", get(get_users_by_id).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![2];
            authorize(req, next, allowed_roles)})))
        .route("/{id}", delete(delete_user_by_id).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![2];
            authorize(req, next, allowed_roles)})));

    // API key-related routes
    let apikey_routes = Router::new()
        .route("/all", get(get_all_apikeys).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)})))
        .route("/new", post(post_apikey).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)
        })))
        .route("/{id}", get(get_apikeys_by_id).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)})))
        .route("/{id}", delete(delete_apikey_by_id).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)})))
        .route("/rotate/{id}", post(rotate_apikey).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)})));

    // Usage related routes
    let usage_routes = Router::new()
        .route("/lastday", get(get_usage_last_day).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)})))
        .route("/lastweek", get(get_usage_last_week).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)
        })));

    // Todo-related routes
    let todo_routes = Router::new()
        .route("/all", get(get_all_todos).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![1, 2];
            authorize(req, next, allowed_roles)})))
        .route("/new", post(post_todo).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![1, 2];
            authorize(req, next, allowed_roles)
        })))
        .route("/{id}", get(get_todos_by_id).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![1, 2];
            authorize(req, next, allowed_roles)})))
        .route("/{id}", delete(delete_todo_by_id).layer(axum::middleware::from_fn(|req,  next| {
            let allowed_roles = vec![1,2];
            authorize(req, next, allowed_roles)})));

    // Documentation:
    // Create OpenAPI specification
    let openapi = ApiDoc::openapi();

    // Create Swagger UI
    let swagger_ui = SwaggerUi::new("/swagger-ui")
        .url("/openapi.json", openapi.clone());

    // Combine all routes and add middleware
    Router::new()
        .route("/", get(homepage)) 
        .merge(auth_routes)  // Add authentication routes
        .merge(swagger_ui)
        .nest("/users", user_routes)  // Add user routes under /users
        .nest("/apikeys", apikey_routes)  // Add API key routes under /apikeys
        .nest("/usage", usage_routes)  // Add usage routes under /usage
        .nest("/todos", todo_routes)  // Add todo routes under /todos
        .route("/health", get(get_health))  // Add health check route
        .layer(axum::Extension(database_connection.clone()))  // Add database connection to all routes
        .with_state(database_connection)  // Add database connection as state
        .layer(TraceLayer::new_for_http())  // Add tracing middleware
}