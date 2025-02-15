<<<<<<< HEAD
# 🦖 Axium
**An example API built with Rust, Axum, SQLx, and PostgreSQL.**  
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🚀 Core Features
### **Production-Grade Foundation**  
_Jumpstart secure API development_  
- Battle-tested Rust template following industry best practices  
- Built-in scalability patterns for high-traffic environments  

### **Effortless Deployment**  
_From zero to production in minutes_  
- 🐳 Docker Compose stack with pre-configured services  
- 20-minute setup timeline with `docker-compose up` simplicity  

### **Developer-First API Experience**  
_Spec-driven development workflow_  
- Auto-generated OpenAPI 3.1 specifications  
- Interactive Swagger UI endpoint at `/docs`  
```rust
// Endpoint registration example
.route("/docs", get(serve_swagger_ui))
```

### **Enterprise-Grade Security**  
_Security by design architecture_  
- JWT authentication with Argon2id password hashing (OWASP recommended)  
- TLS 1.3/HTTP2 via AWS-LC (FIPS 140-3 compliant cryptography)  
- Role-Based Access Control (RBAC) implementation:  
```rust
.layer(middleware::from_fn(|req, next| 
    authorize(req, next, vec![1, 2]) // Admin+Mod roles
))
```

### **PostgreSQL Integration**  
_Relational data made simple_  
- SQLx-powered async database operations  
- Migration system with transactional safety  
- Connection pooling for high concurrency  

### **Performance Optimizations**  
_Engineered for speed at scale_  
- Brotli compression (11-level optimization)  
- Intelligent request caching strategies  
- Zero-copy deserialization pipelines  

### **Operational Visibility**  
_Production monitoring made easy_  
- Docker-healthcheck compatible endpoint:  
```json
{
  "status": "degraded",
  "details": {
    "database": {"status": "ok"},
    "memory": {"available_mb": 21613, "status": "normal"},
    "cpu_usage": {"available_percentage": "9.85", "status": "low"},
    "disk_usage": {"used_percentage": "74.00", "status": "ok"}
  }
}
```

### **Developer Ergonomics**  
_Code with confidence_  
- Context-aware user injection system:  
```rust
async fn create_todo(
    Extension(User { id, role, .. }): Extension<User>, // Auto-injected
    Json(payload): Json<TodoRequest>
) -> Result<impl IntoResponse> {
    // Business logic with direct user context
}
```
- Structured logging with OpenTelemetry integration  
- Compile-time configuration validation  

### **Maintenance & Compliance**  
_Future-proof codebase management_  
- Automated dependency updates via Dependabot  
- Security-focused dependency tree (cargo-audit compliant)  
- Comprehensive inline documentation:  
```rust
/// JWT middleware - Validates Authorization header
/// # Arguments
/// * `req` - Incoming request
/// * `next` - Next middleware layer
/// # Security
/// - Validates Bearer token format
/// - Checks token expiration
/// - Verifies cryptographic signature
```
=======
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
>>>>>>> 830dbdb2074fc62e056ef70d374bea3f26ac0589

## 🛠️ Technology stack
| Category              | Key Technologies               |
|-----------------------|---------------------------------|
| Web Framework         | Axum 0.8 + Tower               |
| Database              | PostgreSQL + SQLx 0.8          |
| Security              | JWT + Argon2 + Rustls           |
| Monitoring            | Tracing + Sysinfo              |

## 📂 Project structure
```
<<<<<<< HEAD
axium-api/                          # Root project directory
├── 📁 migrations/                  # Database schema migrations (SQLx)
│
├── 📁 src/                         # Application source code
│   ├── 📁 core/                    # Core application infrastructure
│   │   ├── config.rs               # Configuration loader (.env, env vars)
│   │   └── server.rs              # HTTP/HTTPS server initialization
│   │
│   ├── 📁 database/                # Database access layer
│   │   ├── connection.rs           # Connection pool management
│   │   ├── queries/                # SQL query modules
│   │   └── models.rs               # Database entity definitions
│   │
│   ├── 📁 middlewares/             # Axum middleware components
│   ├── 📁 routes/                  # API endpoint routing
│   │   └── mod.rs                  # Route aggregator
│   │
│   ├── 📁 handlers/                # Request handlers
│   │
│   ├── 📁 utils/                   # Common utilities
│   │
│   └── main.rs                     # Application entry point
│
├── 📄 .env                         # Environment configuration
├── 📄 .env.example                 # Environment template
├── 📄 Dockerfile                   # Production container build
├── 📄 docker-compose.yml           # Local development stack
└── 📄 Cargo.toml                   # Rust dependencies & metadata
```

Each folder has a detailed README.md file which explains the folder in more detail.

## 🌐 Default API endpoints

| Method | Endpoint               | Auth Required | Administrator only | Description                          |
|--------|------------------------|---------------|-------------------|--------------------------------------|
| POST   | `/signin`              | 🚫            | 🚫                | Authenticate user and get JWT token  |
| GET    | `/protected`           | ✅            | 🚫                | Test endpoint for authenticated users |
| GET    | `/health`              | 🚫            | 🚫                | System health check with metrics     |
|        |                        |               |                   |                                      |
| **Apikey routes**         |                        |               |                   |                                      |
| GET    | `/apikeys/all`         | ✅            | ✅                | Get all apikeys of the current user. |
| POST   | `/apikeys/`            | ✅            | ✅                | Create a new apikey.                 |
| GET    | `/apikeys/{id}`        | ✅            | ✅                | Get an apikey by ID.                 |
| DELETE | `/apikeys/{id}`        | ✅            | 🚫                | Delete an apikey by ID.              |
| POST   | `/apikeys/rotate/{id}` | ✅            | 🚫                | Rotates an API key, disables the old one (grace period 24 hours), returns a new one. |
|        |                        |               |                   |                                      |
| **User routes**           |                        |               |                   |                                      |
| GET    | `/users/all`           | ✅            | ✅                | Get all users.                       |
| POST   | `/users/`              | ✅            | ✅                | Create a new user.                   |
| GET    | `/users/{id}`          | ✅            | ✅                | Get a user by ID.                    |
| DELETE | `/users/{id}`          | ✅            | ✅                | Delete a user by ID.                 |
|        |                        |               |                   |                                      |
| **Todo routes**           |                        |               |                   |                                      |
| GET    | `/todos/all`           | ✅            | 🚫                | Get all todos of the current user.   |
| POST   | `/todos/`              | ✅            | 🚫                | Create a new todo.                   |
| GET    | `/todos/{id}`          | ✅            | 🚫                | Get a todo by ID.                    |
| DELETE | `/todos/{id}`          | ✅            | 🚫                | Delete a todo by ID.                 |
=======
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

>>>>>>> 830dbdb2074fc62e056ef70d374bea3f26ac0589

## 📦 Installation & Usage
```bash
# Clone and setup
git clone https://github.com/Riktastic/Axium.git
cd Axium && cp .env.example .env

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
<<<<<<< HEAD
1. Rotate passwords immediately after initial setup.
2. Disable default accounts before deploying to production.
3. Implement proper user management endpoints.
=======
1. Rotate passwords immediately after initial setup
2. Disable default accounts before deploying to production
3. Implement proper user management endpoints
>>>>>>> 830dbdb2074fc62e056ef70d374bea3f26ac0589

#### Administrative password resets  
*For emergency access recovery only*  

1. **Database Access**  
   Connect to PostgreSQL using privileged credentials:  
   ```bash  
   psql -U admin_user -d axium_db -h localhost  
   ```

2. **Secure Hash Generation**  
   Use the integrated CLI tool (never online generators):  
   ```bash  
   cargo run --bin argon2-cli -- "new_password"  
   # Output: $argon2id$v=19$m=19456,t=2,p=1$b2JqZWN0X2lkXzEyMzQ1$R7Zx7Y4W...
   ```

3. **Database Update**  
   ```sql  
   UPDATE users  
   SET 
       password_hash = '$argon2id...',  
       updated_at = NOW()  
   WHERE email = 'user@example.com';  
   ```

4. **Verification**  
   - Immediately test new credentials  
   - Force user password change on next login  

### ⚙️ Configuration
Create a .env file in the root of the project or configure the application using environment variables.

<<<<<<< HEAD
=======
### ⚙️ Configuration
Create a .env file in the root of the project or configure the application using environment variables.

>>>>>>> 830dbdb2074fc62e056ef70d374bea3f26ac0589
```env
# ==============================
# ⚙️ GENERAL CONFIGURATION
# ==============================
ENVIRONMENT="development" # "production"

# ==============================
# 🌍 SERVER CONFIGURATION
# ==============================

# IP address the server will bind to (0.0.0.0 allows all network interfaces)
SERVER_IP="0.0.0.0"

# Port the server will listen on
SERVER_PORT="3000"

# Enable tracing for debugging/logging (true/false)
SERVER_TRACE_ENABLED=true

# Amount of threads used to run the server
SERVER_WORKER_THREADS=2


# ==============================
# 🛢️ DATABASE CONFIGURATION
# ==============================

# PostgreSQL connection URL (format: postgres://user:password@host/database)
DATABASE_URL="postgres://postgres:1234@localhost/database_name"

# Maximum number of connections in the database pool
DATABASE_MAX_CONNECTIONS=20

# Minimum number of connections in the database pool
DATABASE_MIN_CONNECTIONS=5


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

# JWT secret key.
JWT_SECRET_KEY="fgr4fe34w2rfTwfe3444234edfewfw4e#f$#wferg23w2DFSdf"
```
