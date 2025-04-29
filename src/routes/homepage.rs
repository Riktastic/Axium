use axum::Router;
use crate::routes::AppState;
use std::sync::Arc;

use crate::handlers::homepage::homepage;
use crate::wrappers::authentication_route_builder::AuthenticatedRouteBuilder;

pub fn create_homepage_route(state: Arc<AppState>) -> Router<Arc<AppState>> {
    AuthenticatedRouteBuilder::new(state)
        .unauthenticated_get("/", homepage)
        .build()
}