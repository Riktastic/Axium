use axum::Router;
use crate::routes::AppState;
use std::sync::Arc;

use crate::handlers::{login::login, protected::protected};
use crate::wrappers::authentication_route_builder::AuthenticatedRouteBuilder;

pub fn create_auth_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    AuthenticatedRouteBuilder::new(state)
        .unauthenticated_post("/login", login)
        .get("/protected", protected, vec![1, 2])
        .build()
}