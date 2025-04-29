use axum::Router;
use crate::routes::AppState;
use std::sync::Arc;

use crate::handlers::get_health::get_health;
use crate::wrappers::authentication_route_builder::AuthenticatedRouteBuilder;

pub fn create_health_route(state: Arc<AppState>) -> Router<Arc<AppState>> {
    AuthenticatedRouteBuilder::new(state)
        .unauthenticated_get("/health", get_health)
        .build()
}