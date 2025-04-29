use axum::Router;
use crate::routes::AppState;
use std::sync::Arc;

use crate::handlers::get_usage::{get_usage_last_day, get_usage_last_week};
use crate::wrappers::authentication_route_builder::AuthenticatedRouteBuilder;

pub fn create_usage_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    AuthenticatedRouteBuilder::new(state)
        // Route for getting the usage from the last day
        .get("/lastday", get_usage_last_day, vec![1, 2])
        // Route for getting the usage from the last week
        .get("/lastweek", get_usage_last_week, vec![1, 2])
        .build()
}