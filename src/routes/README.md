# Routes

This folder contains the route definitions for Axium, built using [Axum](https://docs.rs/axum/latest/axum/) and [SQLx](https://docs.rs/sqlx/latest/sqlx/).

## Overview
The `/src/routes` folder manages the routing for various API endpoints, handling operations such as CRUD functionality, usage statistics, and more. Each route is associated with its handler and protected by an authorization middleware.

### Key Components
- **Axum Router:** Sets up API routes and manages HTTP requests, see mod.rs.
- **SQLx PgPool:** Provides database connection pooling.
- **Authorization Middleware:** Ensures secure access based on user roles.

## Middleware
The `authorize` middleware is defined in `src/middlewares/auth.rs`. It takes the request, a next handler, and a vector of allowed roles. It verifies that the user has one of the required roles before forwarding the request.
It also counts the amount of sent requests and blocks requests if the user has sent too many requests.


## Handlers
Each route delegates its logic to handler functions found in the `src/handlers` folder, ensuring separation of concerns.

## Usage
Supported HTTP requests:
- unauthenticated_post/get/delete/update: For unauthenticated routes.
- post/get/delete/update: For authenticated routes. Requires a vec with the number of role levels that will be able to access the specified route.

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

Check out [the documentation of our special route builder](/documentation/authentication_route_builder.md), for more info.


## Dependencies
- [Axum](https://docs.rs/axum/latest/axum/)
- [SQLx](https://docs.rs/sqlx/latest/sqlx/)

## Contributing
Add new route files, update existing routes, or enhance the middleware and handlers. Document any changes for clarity.

## License
This project is licensed under the MIT License.