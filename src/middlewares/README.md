# Middleware
This folder contains middleware functions used in Axium, providing essential utilities like authentication, authorization, and usage tracking.

## Overview
The `/src/middlewares` folder includes middleware implementations for role-based access control (RBAC), JWT authentication, rate limiting, and batched usage tracking.

### Key Components
- **Axum Middleware:** Utilizes Axum's middleware layer for request handling.
- **Moka Cache:** Provides caching for rate limits.
- **SQLx:** Facilitates database interactions.
- **UUID and Chrono:** Handles unique identifiers and timestamps.

## Middleware Files
This folder includes:

- **authorize:** Middleware to enforce role-based access by validating JWT tokens and checking user roles.
- **usage tracking:** Middleware to count and store usage metrics efficiently through batched database writes.

## Usage
To apply middleware, use Axum's `layer` method:
```rust
.route("/path", get(handler).layer(from_fn(|req, next| {
    let allowed_roles = vec![1, 2];
    authorize(req, next, allowed_roles)
})))
```

## Extending Middleware
Add new middleware by creating Rust functions that implement Axum's `Next` trait. Ensure proper logging, error handling, and unit tests.

## Dependencies
- [Axum](https://docs.rs/axum/latest/axum/)
- [SQLx](https://docs.rs/sqlx/latest/sqlx/)
- [Moka Cache](https://docs.rs/moka/latest/moka/)

## Contributing
Ensure new middleware is well-documented, includes error handling, and integrates with the existing architecture.

## License
This project is licensed under the MIT License.