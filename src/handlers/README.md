# Handlers
This folder contains the route handlers used in Axium, responsible for processing incoming HTTP requests and generating responses.

## Overview
The `/src/handlers` folder includes implementations of route handlers for API keys, usage metrics, and the homepage.

### Key Components
- **Axum Handlers:** Built using Axum's handler utilities for routing and extracting request data.
- **SQLx:** Manages database operations like fetching usage and deleting API keys.
- **UUID and Serde:** Handles unique IDs and JSON serialization.
- **Tracing:** Provides structured logging for monitoring and debugging.

## Usage
Handlers are linked to Axum routes using `route` and `handler` methods:
```rust
route("/apikeys/:id", delete(delete_apikey_by_id))
    .route("/usage/lastday", get(get_usage_last_day))
```

## Dependencies
- [Axum](https://docs.rs/axum/latest/axum/)
- [SQLx](https://docs.rs/sqlx/latest/sqlx/)
- [UUID](https://docs.rs/uuid/latest/uuid/)
- [Serde](https://docs.rs/serde/latest/serde/)
- [Tracing](https://docs.rs/tracing/latest/tracing/)

## Contributing
Ensure new handlers are well-documented, include proper error handling, and maintain compatibility with existing routes.

## License
This project is licensed under the MIT License.