# Models
This folder contains data models used in Axium, primarily defined as Rust structs. These models are essential for data serialization, deserialization, and validation within the application.

## Overview
The `/src/models` folder contains various struct definitions that represent key data structures, such as JWT claims and custom error types.

### Key Components
- **Serde:** Provides serialization and deserialization capabilities.
- **Utoipa:** Facilitates API documentation through the `ToSchema` derive macro.
- **Axum StatusCode:** Used for HTTP status management within custom error types.

## Usage
Import and utilize these models across your API routes, handlers, and services. For example:
```rust
use crate::models::Claims;
use crate::models::AuthError;
```

## Extending Models
You can extend the existing models by adding more fields, or create new models as needed for additional functionality. Ensure that any new models are properly documented and derive necessary traits.

## Dependencies
- [Serde](https://docs.rs/serde/latest/serde/)
- [Utoipa](https://docs.rs/utoipa/latest/utoipa/)
- [Axum](https://docs.rs/axum/latest/axum/)

## Contributing
When adding new models, ensure they are well-documented, derive necessary traits, and integrate seamlessly with the existing codebase.

## License
This project is licensed under the MIT License.
