# Database
This folder contains the database interaction layer for Axium, handling database connections, migrations, and queries related to API keys and usage metrics.

## Overview
The `/src/database` folder includes functions for inserting, retrieving, modifying, and deleting API keys, along with usage tracking and database connection management.

### Key Components
- **SQLx:** Asynchronous database operations for PostgreSQL.
- **Chrono:** Date and time manipulation.
- **UUID:** Handling unique identifiers for users and keys.
- **Dotenvy:** Securely loads environment variables.
- **ThisError:** Provides structured error handling.

## Usage
Database functions are called by route handlers for secure data operations. Ensure environment variables like `DATABASE_URL` are properly configured before running the API.

## Dependencies
- [SQLx](https://docs.rs/sqlx/latest/sqlx/)
- [Chrono](https://docs.rs/chrono/latest/chrono/)
- [UUID](https://docs.rs/uuid/latest/uuid/)
- [Dotenvy](https://docs.rs/dotenvy/latest/dotenvy/)
- [ThisError](https://docs.rs/thiserror/latest/thiserror/)

## Contributing
Ensure database queries are secure, optimized, and well-documented. Validate all user inputs before performing database operations.

## License
This project is licensed under the MIT License.

