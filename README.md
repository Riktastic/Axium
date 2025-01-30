# 🦀 Axum API Quickstart
**An example API built with Rust, Axum, SQLx, and PostgreSQL**  
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🚀 Core Features
- **Rust API template** - Production-ready starter template with modern practices,
- **PostgreSQL integration** - Full database support with SQLx migrations,
- **Easy to secure** - HTTP/2 with secure TLS defaults (AWS-LC, FIPS 140-3),
- **Easy to configure** - `.env` and environment variables,
- **JWT authentication** - Secure token-based auth with Argon2 password hashing,
- **Optimized for performance** - Brotli compression,
- **Comprehensive health monitoring**  
  Docker-compatible endpoint with system metrics:  
  ```json
  {
    "details": {
      "cpu_usage": {"available_percentage": "9.85", "status": "low"},
      "database": {"status": "ok"},
      "disk_usage": {"status": "ok", "used_percentage": "74.00"},
      "memory": {"available_mb": 21613, "status": "normal"}
    },
    "status": "degraded"
  }
  ```
- **Granular access control** - Role-based endpoint protection:  
  ```rust
  .route("/", post(post_todo).layer(axum::middleware::from_fn(|req, next| {
      let allowed_roles = vec![1, 2];
      authorize(req, next, allowed_roles)
  })))
  ```
- **User context injection** - Automatic user profile handling in endpoints:  
  ```rust
  pub async fn post_todo(
      Extension(user): Extension<User>,  // Injected user
      Json(todo): Json<TodoBody>
  ) -> impl IntoResponse {
      if todo.user_id != user.id {
          return Err((StatusCode::FORBIDDEN, Json(json!({ 
              "error": "Cannot create todos for others" 
          }))));
      }
  ```
- **Observability** - Integrated tracing,
- **Documented codebase** - Extensive inline comments for easy modification and readability,
- **Latest dependencies** - Regularly updated Rust ecosystem crates,

## 🛠️ Technology stack
| Category              | Key Technologies               |
|-----------------------|---------------------------------|
| Web Framework         | Axum 0.8 + Tower               |
| Database              | PostgreSQL + SQLx 0.8          |
| Security              | JWT + Argon2 + Rustls           |
| Monitoring            | Tracing + Sysinfo              |

## 📂 Project structure
```
rustapi/
├── migrations/      # SQL schema migrations. Creates the required tables and inserts demo data.
├── src/
│   ├── core/        # Core modules: for reading configuration files, starting the server and configuring HTTPS/
│   ├── database/    # Database connectivity, getters and setters for the database.
│   ├── middlewares/ # Currently just the authentication system.
│   ├── models/      # Data structures
│   └── routes/      # API endpoints
│        └── mod.rs      # API endpoint router.
│   └── .env         # Configuration file.
└── Dockerfile       # Builds a docker container for the application.
└── compose.yaml     # Docker-compose.yaml. Runs container for the application (also includes a PostgreSQL-container).
```

## 🌐 Default API endpoints

| Method | Endpoint               | Auth Required | Allowed Roles | Description                          |
|--------|------------------------|---------------|---------------|--------------------------------------|
| POST   | `/signin`              | No            |           | Authenticate user and get JWT token  |
| GET    | `/protected`           | Yes           | 1, 2          | Test endpoint for authenticated users |
| GET    | `/health`              | No            |           | System health check with metrics     |
|        |                        |               |               |                                      |
| **User routes**         |                        |               |               |                                      |
| GET    | `/users/all`           | No*           |           | Get all users                        |
| GET    | `/users/{id}`          | No*           |           | Get user by ID                       |
| POST   | `/users/`              | No*           |           | Create new user                      |
|        |                        |               |               |                                      |
| **Todo routes**         |                        |               |               |                                      |
| GET    | `/todos/all`           | No*           |           | Get all todos                        |
| POST   | `/todos/`              | Yes           | 1, 2          | Create new todo                      |
| GET    | `/todos/{id}`          | No*           |           | Get todo by ID                       |

**Key:**  
🔒 = Requires JWT in `Authorization: Bearer <token>` header  
\* Currently unprotected - recommend adding authentication for production  
**Roles:** 1 = User, 2 = Administrator

**Security notes:**  
- All POST endpoints expect JSON payloads
- User creation endpoint should be protected in production
- Consider adding rate limiting to authentication endpoints
**Notes:**  
- 🔒 = Requires JWT in `Authorization: Bearer <token>` header  
- Roles: `1` = Regular User, `2` = Administrator  
- *Marked endpoints currently unprotected - recommend adding middleware for production use
- All POST endpoints expect JSON payloads


## 📦 Installation & Usage
```bash
# Clone and setup
git clone https://github.com/Riktastic/rustapi.git
cd rustapi && cp .env.example .env

# Database setup
sqlx database create && sqlx migrate run

# Start server
cargo run --release
```

### 🔐 Default accounts

**Warning:** These accounts should only be used for initial testing. Always change or disable them in production environments.

| Email               | Password | Role           |
|---------------------|----------|----------------|
| `user@test.com`     | `test`   | User           |
| `admin@test.com`    | `test`   | Administrator  |

⚠️ **Security recommendations:**
1. Rotate passwords immediately after initial setup
2. Disable default accounts before deploying to production
3. Implement proper user management endpoints


### ⚙️ Configuration
Create a .env file in the root of the project or configure the application using environment variables.

```env
# ==============================
# 📌 DATABASE CONFIGURATION
# ==============================

# PostgreSQL connection URL (format: postgres://user:password@host/database)
DATABASE_URL="postgres://postgres:1234@localhost/database_name"

# Maximum number of connections in the database pool
DATABASE_MAX_CONNECTIONS=20

# Minimum number of connections in the database pool
DATABASE_MIN_CONNECTIONS=5

# ==============================
# 🌍 SERVER CONFIGURATION
# ==============================

# IP address the server will bind to (0.0.0.0 allows all network interfaces)
SERVER_IP="0.0.0.0"

# Port the server will listen on
SERVER_PORT="3000"

# Enable tracing for debugging/logging (true/false)
SERVER_TRACE_ENABLED=true

# ==============================
# 🔒 HTTPS CONFIGURATION
# ==============================

# Enable HTTPS (true/false)
SERVER_HTTPS_ENABLED=false

# Enable HTTP/2 when using HTTPS (true/false)
SERVER_HTTPS_HTTP2_ENABLED=true

# Path to the SSL certificate file (only used if SERVER_HTTPS_ENABLED=true)
SERVER_HTTPS_CERT_FILE_PATH=cert.pem

# Path to the SSL private key file (only used if SERVER_HTTPS_ENABLED=true)
SERVER_HTTPS_KEY_FILE_PATH=key.pem

# ==============================
# 🚦 RATE LIMIT CONFIGURATION
# ==============================

# Maximum number of requests allowed per period
SERVER_RATE_LIMIT=5

# Time period (in seconds) for rate limiting
SERVER_RATE_LIMIT_PERIOD=1

# ==============================
# 📦 COMPRESSION CONFIGURATION
# ==============================

# Enable Brotli compression (true/false)
SERVER_COMPRESSION_ENABLED=true

# Compression level (valid range: 0-11, where 11 is the highest compression)
SERVER_COMPRESSION_LEVEL=6

# ==============================
# 🔑 AUTHENTICATION CONFIGURATION
# ==============================

# Argon2 salt for password hashing (must be kept secret!)
AUTHENTICATION_ARGON2_SALT="dMjQgtSmoQIH3Imi"
```
