# AuthenticatedRouteBuilder

A builder pattern for constructing Axum routers with **role-based authentication middleware**.  
Allows you to easily define authenticated and unauthenticated routes, with per-route role checks.

Source code: [Authentication_route_builder.rs](/src/wrappers/authentication_route_builder.rs)

---

## Features

- **Role-based middleware**: Attach a middleware to routes that checks for allowed user roles before passing to the handler.
- **Consistent state management**: All routes expect `Arc`, making handler and middleware extraction uniform.
- **Builder pattern**: Chain route definitions fluently.
- **Separation of concerns**: Auth logic is encapsulated, reducing boilerplate in route definitions.

---

## Example Usage

Supported HTTP requests:
- unauthenticated_post/get/delete/patch: For unauthenticated routes.
- post/get/delete/patch: For authenticated routes. Requires a vec with the number of role levels that will be able to access the specified route.

```rust
// In your routes/auth.rs or similar

use crate::routes::AppState;
use crate::handlers::{login::login, protected::protected};
use crate::middlewares::auth_route_builder::AuthenticatedRouteBuilder;
use std::sync::Arc;

pub fn create_auth_routes(state: Arc) -> Router> {
    AuthenticatedRouteBuilder::new(state)
        .unauthenticated_post("/login", login)
        .get("/protected", protected, vec![1, 2]) // 1=user, 2=admin
        .build()
}
```

P:S: You can ignore this custom wrapper. But to be still be able to set up RBAC and pass an appstate you will have to setup each route like this:
```rust
pub fn create_apikey_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let allowed_roles = Arc::new(vec![1, 2]);

    Router::new()
        .route(
            "/protected",
            get(protected).layer(from_fn_with_state(
                state.clone(),
                {
                    let allowed_roles = allowed_roles.clone();
                    move |State(state): State<Arc<AppState>>, req: Request<Body>, next: Next| {
                        let allowed_roles = allowed_roles.clone();
                        async move { authorize(allowed_roles, state, req, next).await } // Authorization middleware
                    }
                },
            )),
        )
```


Then, at your top-level router:

```rust
pub fn create_routes(state: Arc) -> Router {
    Router::new()
        .merge(create_auth_routes(state.clone()))
        // ... other merges ...
        .with_state(state)
}
```

---

## **Pros vs. Vanilla Axum Router**

**Pros:**
- Centralizes authentication/authorization logic.
- Reduces repetitive code for role checks.
- Keeps route definitions clean and readable.
- Easy to add or change role requirements.

**Cons:**
- Slightly more abstraction; may feel "magical" to new Rust/Axum users.
- Less flexible if you want per-route custom middleware (but you can always `.layer()` after building).
- All handlers must accept `Arc` as state.

---

## **Role Levels**

- `1`: User (default for most endpoints)
- `2`: Administrator (for admin-only routes)
- You can extend this system for more roles if needed.

---

## **Summary**

This builder pattern is a **powerful, DRY, and idiomatic way** to manage authentication and role-based authorization in Axum, while keeping your codebase maintainable and secure.  
If you need to add more roles, simply update your `authorize` logic and role documentation!

---
Answer from Perplexity: pplx.ai/share