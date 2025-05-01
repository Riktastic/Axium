use axum::Router;
use crate::routes::AppState;
use std::sync::Arc;

use crate::handlers::{
    get_users::{get_all_users, get_users_by_id},
    post_users::{post_user, post_user_profilepicture, post_user_password_reset, post_user_password_reset_confirm},
    patch_users::patch_user_profile,
    delete_users::delete_user_by_id
};
use crate::wrappers::authentication_route_builder::AuthenticatedRouteBuilder;

pub fn create_user_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    AuthenticatedRouteBuilder::new(state)
        // Route for getting all users (requires role 2)
        .get("/all", get_all_users, vec![2])
        // Route for creating a new user (requires role 2)
        .post("/new", post_user, vec![2])
        // Route for requesting a password reset (unauthenticated)
        .unauthenticated_post("/password-reset", post_user_password_reset)
        // Route for confirming password reset (unauthenticated)
        .unauthenticated_post("/password-reset/confirm", post_user_password_reset_confirm)
        // Route for getting user by email (requires role 2)
        // Route for adding profile pictures.
        .post("/{id}/profile-picture", post_user_profilepicture, vec![1, 2]) 
        // Route for getting user by ID (requires roles 1 or 2)
        .get("/{id}", get_users_by_id, vec![1, 2])
        // Route for updating user profile fields (requires roles 1 or 2)
        .patch("/{id}", patch_user_profile, vec![1, 2])
        // Route for deleting a user by ID (requires role 2)
        .delete("/{id}", delete_user_by_id, vec![2])
        .build()
}