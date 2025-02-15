# Routes

This folder contains the route definitions for Axium, built using [Axum](https://docs.rs/axum/latest/axum/) and [SQLx](https://docs.rs/sqlx/latest/sqlx/).

## Overview
The `/src/routes` folder manages the routing for various API endpoints, handling operations such as CRUD functionality, usage statistics, and more. Each route is associated with its handler and protected by an authorization middleware.

### Key Components
- **Axum Router:** Sets up API routes and manages HTTP requests, see mod.rs.
- **SQLx PgPool:** Provides database connection pooling.
- **Authorization Middleware:** Ensures secure access based on user roles.

## Middleware
The `authorize` middleware is defined in `src/middlewares/auth.rs`. It takes the request, a next handler, and a vector of allowed roles. It verifies that the user has one of the required roles before forwarding the request. Usage example:
```rust
.route("/path", get(handler).layer(from_fn(|req, next| {
    let allowed_roles = vec![1, 2];
    authorize(req, next, allowed_roles)
})))
```
Ensure that the `authorize` function is imported and applied to each route that requires restricted access.
The `authorize` middleware ensures users have appropriate roles before accessing certain routes.

## Handlers
Each route delegates its logic to handler functions found in the `src/handlers` folder, ensuring separation of concerns.

## Usage
Integrate these routes into your main application router by nesting them appropriately:
```rust
let app = Router::new()
    .nest("/todos", create_todo_routes())
    .nest("/usage", create_usage_routes());
```

## Dependencies
- [Axum](https://docs.rs/axum/latest/axum/)
- [SQLx](https://docs.rs/sqlx/latest/sqlx/)

## Contributing
Add new route files, update existing routes, or enhance the middleware and handlers. Document any changes for clarity.

## License
This project is licensed under the MIT License.