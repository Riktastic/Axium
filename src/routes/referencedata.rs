use axum::Router;
use crate::routes::AppState;
use std::sync::Arc;
use tracing::instrument; // For logging

use crate::handlers::get_referencedata::get_referencedata;
use crate::wrappers::authentication_route_builder::AuthenticatedRouteBuilder;

pub fn create_referencedata_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    AuthenticatedRouteBuilder::new(state)
        // Route for getting the usage from the last day
        .unauthenticated_get("/referencedata/{id}", get_referencedata)
        .build()
}