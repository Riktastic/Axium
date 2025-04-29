use std::sync::Arc;
use axum::{
    Router,
    body::Body,
    http::Request,
    middleware::Next,
    extract::State,
    routing::{get, post, delete},
};
use crate::routes::AppState;
use crate::middlewares::auth::authorize;
use axum::middleware::from_fn_with_state;

/// Builder for constructing routers with role-based authentication middleware.
///
/// # Example
///
/// ```
/// let state = Arc::new(AppState::new(...));
/// let router = AuthenticatedRouteBuilder::new(state)
///     .unauthenticated_post("/login", login_handler)
///     .get("/admin", admin_handler, vec!) // Only admins
///     .get("/user", user_handler, vec!) // Users and admins
///     .build();
/// ```
///
/// # Role Levels
/// - `1`: User
/// - `2`: Administrator
///
/// # Pros
/// - Cleaner, DRY route definitions with built-in role checks.
/// - Centralizes authentication/authorization logic.
/// - Consistent use of application state.
///
/// # Cons
/// - Slightly more complex than vanilla Axum routers.
/// - Less flexibility if you need per-route custom middleware logic.
/// - All handlers must accept `Arc<AppState>`.
pub struct AuthenticatedRouteBuilder {
    router: Router<Arc<AppState>>,
    state: Arc<AppState>,
}

#[allow(dead_code)]
impl AuthenticatedRouteBuilder {
    /// Create a new builder with the given shared application state.
    pub fn new(state: Arc<AppState>) -> Self {
        Self {
            router: Router::new(),
            state,
        }
    }

    /// Add a GET route with required role levels.
    ///
    /// `allowed_roles` is a vector of role levels (e.g., `[1]` for users, `[2]` for admins).
    #[allow(dead_code)]
    pub fn get<H, T>(mut self, path: &str, handler: H, allowed_roles: Vec<i32>) -> Self
    where
        H: axum::handler::Handler<T, Arc<AppState>> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        let allowed_roles = Arc::new(allowed_roles);
        self.router = self.router.route(
            path,
            get(handler).layer(from_fn_with_state(
                self.state.clone(),
                move |State(state): State<Arc<AppState>>, req: Request<Body>, next: Next| {
                    let allowed_roles = Arc::clone(&allowed_roles);
                    async move { authorize(allowed_roles, state, req, next).await }
                },
            )),
        );
        self
    }

    /// Add a POST route with required role levels.
    #[allow(dead_code)]
    pub fn post<H, T>(mut self, path: &str, handler: H, allowed_roles: Vec<i32>) -> Self
    where
        H: axum::handler::Handler<T, Arc<AppState>> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        let allowed_roles = Arc::new(allowed_roles);
        self.router = self.router.route(
            path,
            post(handler).layer(from_fn_with_state(
                self.state.clone(),
                move |State(state): State<Arc<AppState>>, req: Request<Body>, next: Next| {
                    let allowed_roles = Arc::clone(&allowed_roles);
                    async move { authorize(allowed_roles, state, req, next).await }
                },
            )),
        );
        self
    }

    /// Add a DELETE route with required role levels.
    #[allow(dead_code)]
    pub fn delete<H, T>(mut self, path: &str, handler: H, allowed_roles: Vec<i32>) -> Self
    where
        H: axum::handler::Handler<T, Arc<AppState>> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        let allowed_roles = Arc::new(allowed_roles);
        self.router = self.router.route(
            path,
            delete(handler).layer(from_fn_with_state(
                self.state.clone(),
                move |State(state): State<Arc<AppState>>, req: Request<Body>, next: Next| {
                    let allowed_roles = Arc::clone(&allowed_roles);
                    async move { authorize(allowed_roles, state, req, next).await }
                },
            )),
        );
        self
    }

    /// Add a PATCH route with required role levels.
    #[allow(dead_code)]
    pub fn patch<H, T>(mut self, path: &str, handler: H, allowed_roles: Vec<i32>) -> Self
    where
        H: axum::handler::Handler<T, Arc<AppState>> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        let allowed_roles = Arc::new(allowed_roles);
        self.router = self.router.route(
            path,
            axum::routing::patch(handler).layer(from_fn_with_state(
                self.state.clone(),
                move |State(state): State<Arc<AppState>>, req: Request<Body>, next: Next| {
                    let allowed_roles = Arc::clone(&allowed_roles);
                    async move { authorize(allowed_roles, state, req, next).await }
                },
            )),
        );
        self
    }

    // --- Unauthenticated routes below ---

    /// Add a GET route without authentication.
    #[allow(dead_code)]
    pub fn unauthenticated_get<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, Arc<AppState>> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        self.router = self.router.route(path, get(handler));
        self
    }

    /// Add a POST route without authentication.
    #[allow(dead_code)]
    pub fn unauthenticated_post<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, Arc<AppState>> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        self.router = self.router.route(path, post(handler));
        self
    }

    /// Add a DELETE route without authentication.
    #[allow(dead_code)]
    pub fn unauthenticated_delete<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, Arc<AppState>> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        self.router = self.router.route(path, delete(handler));
        self
    }

    /// Add a PATCH route without authentication.
    #[allow(dead_code)]
    pub fn unauthenticated_patch<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, Arc<AppState>> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        self.router = self.router.route(path, axum::routing::patch(handler));
        self
    }

    /// Finalize the builder and return the constructed router.
    ///
    /// Note: The returned router still expects `Arc<AppState>` to be provided at the top level.
    pub fn build(self) -> Router<Arc<AppState>> {
        self.router
    }
}