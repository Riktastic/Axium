# Middleware
This folder contains middleware functions used in Axium, providing essential utilities like authentication, authorization, and usage tracking.

## Overview
The `/src/middlewares` folder includes middleware implementations for role-based access control (RBAC), JWT authentication, rate limiting, and batched usage tracking.

## Extending Middleware
Add new middleware by creating Rust functions that implement Axum's `Next` trait. Ensure proper logging, error handling, and unit tests.

## Contributing
Ensure new middleware is well-documented, includes error handling, and integrates with the existing architecture.

## License
This project is licensed under the MIT License.