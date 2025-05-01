pub mod homepage;
pub mod apikey;
pub mod auth;
pub mod health;
pub mod todo;
pub mod usage;
pub mod user;

use axum::Router;
use tower_http::trace::TraceLayer;
use utoipa::openapi::security::{SecurityScheme, HttpBuilder, HttpAuthScheme};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

// Application state structure
use sqlx::PgPool;
use aws_sdk_s3::Client as S3Client; // S3 connection client
use deadpool_redis::Pool as RedisPool;  // Redis connection pool
use crate::mail::MailerState; // SmtpTransport for sending emails
use std::sync::Arc;  // For thread-safe reference counting

pub mod handlers {
    pub use crate::handlers::*;
}

pub mod models {
    pub use crate::models::*;
}

use crate::utils::global_error_handler::global_error_handler;  // Global error handler

use self::{
    todo::create_todo_routes,
    user::{create_user_root_routes, create_user_routes},
    apikey::create_apikey_routes,
    usage::create_usage_routes,
    auth::create_auth_routes,
    homepage::create_homepage_route,
    health::create_health_route,
};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AppState {
    pub database: PgPool,
    pub storage: S3Client,
    pub cache: RedisPool,
    pub mail: MailerState,
}

#[allow(dead_code)] // Not sure why, but rust-analyzer is complaining about this. While Utoipa uses it.
struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "jwt_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some("Use JWT token obtained from /login endpoint."))
                    .build()
            )
        );
    }
}

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
        handlers::get_users::get_all_users,
        handlers::get_users::get_users_by_id,
        handlers::get_apikeys::get_all_apikeys,
        handlers::get_apikeys::get_apikeys_by_id,
        handlers::get_usage::get_usage_last_day,
        handlers::get_usage::get_usage_last_week,
        handlers::get_todos::get_all_todos,
        handlers::get_todos::get_todos_by_id,
        handlers::get_health::get_health,
        handlers::post_users::post_user,
        handlers::post_users::post_user_register_verify,
        handlers::post_users::post_user_register,
        handlers::post_users::post_user_password_reset_verify,
        handlers::post_users::post_user_password_reset,
        handlers::post_users::post_user_profilepicture,
        handlers::patch_users::patch_user_profile,
        handlers::post_apikeys::post_apikey,
        handlers::post_todos::post_todo,
        handlers::rotate_apikeys::rotate_apikey,
        handlers::delete_users::delete_user_by_id,
        handlers::delete_apikeys::delete_apikey_by_id,
        handlers::delete_todos::delete_todo_by_id,
        handlers::protected::protected,
        handlers::login::login,
    ),
    components(
        schemas(
            models::apikey::ApiKey,
            models::apikey::ApiKeyInsertBody,
            models::apikey::ApiKeyInsertResponse,
            models::apikey::ApiKeyResponse,
            models::apikey::ApiKeyByIDResponse,
            models::apikey::ApiKeyGetActiveForUserResponse,
            models::apikey::ApiKeyByUserIDResponse,
            models::apikey::ApiKeyNewBody,
            models::apikey::ApiKeyRotateResponse,
            models::apikey::ApiKeyRotateResponseInfo,
            models::apikey::ApiKeyRotateBody,
            models::auth::Claims,
            models::documentation::SuccessResponse,
            models::documentation::ErrorResponse,
            models::health::HealthResponse,
            models::health::CpuUsage,
            models::health::DatabaseStatus,
            models::health::DiskUsage,
            models::health::MemoryStatus,
            models::role::Role,
            models::todo::Todo,
            models::usage::UsageResponseLastDay,
            models::usage::UsageResponseLastWeek,
            models::user::User,
            models::user::UserGetResponse,
            models::user::UserInsertBody,
            models::user::UserInsertResponse,
            models::user::UserUpdateBody,
            models::user::UserUpdateResponse,
            models::user::UserRegisterEmailVerifyBody,
            models::user::UserRegisterBody,
            models::user::UserPasswordResetCode,
            models::user::UserPasswordResetConfirmBody,
            models::user::UserPasswordResetRequestBody
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
pub fn create_routes(state: Arc<AppState>) -> Router<()> {
    // Create OpenAPI specification
    let openapi = ApiDoc::openapi();

    // Create Swagger UI
    let swagger_ui = SwaggerUi::new("/docs")
        .url("/openapi.json", openapi.clone());

    // Combine all routes and add middleware
    Router::new()
        .merge(create_homepage_route(state.clone()))
        .merge(create_auth_routes(state.clone()))
        .merge(create_user_root_routes(state.clone()))
        .merge(swagger_ui)
        .nest("/users", create_user_routes(state.clone()))
        .nest("/apikeys", create_apikey_routes(state.clone()))
        .nest("/usage", create_usage_routes(state.clone()))
        .nest("/todos", create_todo_routes(state.clone()))
        .merge(create_health_route(state.clone()))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .fallback(global_error_handler)
}