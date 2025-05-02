# 🦖 Axium
**An example API built with Rust, Axum, SQLx, S3, Redis, and PostgreSQL.** 

![GitHub last commit](https://img.shields.io/github/last-commit/Riktastic/Axium)
![GitHub commit activity](https://img.shields.io/github/commit-activity/w/Riktastic/Axium)
[![Coverage Status](https://coveralls.io/repos/github/Riktastic/Axium/badge.svg?branch=main)](https://coveralls.io/github/Riktastic/Axium?branch=main)
![GitHub License](https://img.shields.io/github/license/Riktastic/Axium)

> ⚠️ **Warning:** This project is under active development. Pushed changes have been tested. But it might not yet be production ready.

## Summary
Axium is a high-performance, security-focused API boilerplate built using Rust, Axum, SQLx, S3, Redis, and PostgreSQL. 

It provides a ready-to-deploy solution with modern best practices, including JWT authentication, role-based access control (RBAC), structured logging, and enterprise-grade security. With a focus on developer experience, Axium offers auto-generated API documentation, efficient database interactions, and an ergonomic code structure for ease of maintenance and scalability.

The project uses its own routing wrapper built on top of Axum.
The wrapper was created to simplify the integration of the RBAC and unify the passing of connections to external services (database, Redis, S3). 

## Table of Contents
- [🦖 Axium](#-axium)
  - [Summary](#summary)
  - [Table of Contents](#table-of-contents)
  - [🚀 Core Features](#-core-features)
    - [**Effortless Deployment**](#effortless-deployment)
    - [**Developer-First API Experience**](#developer-first-api-experience)
    - [**Enterprise-Grade Security**](#enterprise-grade-security)
    - [**PostgreSQL Integration**](#postgresql-integration)
    - [**Performance Optimizations**](#performance-optimizations)
    - [**Operational Visibility**](#operational-visibility)
    - [**Developer Ergonomics**](#developer-ergonomics)
    - [**Maintenance \& Compliance**](#maintenance--compliance)
  - [🛠️ Technology stack](#️-technology-stack)
  - [📂 Project structure](#-project-structure)
  - [🌐 Default API endpoints](#-default-api-endpoints)
    - [**Notes:**](#notes)
  - [📦 Installation \& usage](#-installation--usage)
  - [Integration](#integration)
    - [🔐 Authentication](#-authentication)
    - [👤 Default accounts](#-default-accounts)
      - [Administrative password resets](#administrative-password-resets)
    - [⚙️ Configuration](#️-configuration)

## 🚀 Core Features
### **Effortless Deployment**  
_From zero to production in minutes_  
- 🐳 Docker Compose stack with pre-configured services  
- 20-minute setup timeline with `docker-compose up` simplicity  

### **Developer-First API Experience**  
_Spec-driven development workflow_  
- Auto-generated OpenAPI 3.1 specifications  
- Interactive Swagger UI endpoint at `/docs`
- Custom wrapper for a simpler implementation of RBAC (which extends Axum) following the DRY principle (Don't repeat yourself)

### **Enterprise-Grade Security**  
_Security by design architecture_  
- JWT authentication with Argon2id password hashing (OWASP recommended)  
- TLS 1.3/HTTP2 via AWS-LC (FIPS 140-3 compliant cryptography)
- Key rotation & expiration
- Custom Role-Based Access Control (RBAC) implementation, ([read more](/documentation/authentication_route_builder.md)):  
```rust
.get("/all", get_all_apikeys, vec![1, 2])          // Admins and users
))
```

### **PostgreSQL Integration**  
_Relational data made simple_  
- SQLx-powered async database operations  
- Migration system with transactional safety  
- Connection pooling for high concurrency
- Lower stress on the database by checking the Redis cache first

### **Performance Optimizations**  
_Engineered for speed at scale_  
- Brotli compression (11-level optimization)  
- Intelligent request caching strategies  

### **Operational Visibility**  
_Production monitoring made easy_  
- Docker-healthcheck / OpenTelemetry compatible endpoint:  
```json
{
    "details": {
        "cache": {
            "status": "ok"
        },
        "cpu_usage": {
            "available_percentage": "2.48",
            "status": "low",
            "usage_percentage": "97.52"
        },
        "database": {
            "status": "ok"
        },
        "disk_usage": {
            "status": "ok",
            "used_percentage": "85.00"
        },
        "important_processes": [
            {
                "name": "postgres",
                "status": "running"
            },
            {
                "name": "minio",
                "status": "running"
            }
        ],
        "memory": {
            "available_mb": 17785,
            "status": "normal"
        },
        "network": {
            "status": "ok"
        }
        "storage": {
            "status": "ok"
        },
    },
    "status": "degraded"
}
```

### **Developer Ergonomics**  
_Code with confidence_  
- Context-aware user injection system:  
```rust
pub async fn get_users_by_id(
    State(state): State<Arc<AppState>>, // Database connection + storage connection
    Path(id): Path<String>, // Path variables
    Extension(current_user): Extension<User>, // Current user
) -> impl IntoResponse {

    // Business logic with direct user context
}
```
- Structured logging with OpenTelemetry integration  
- Compile-time configuration validation  

### **Maintenance & Compliance**  
_Future-proof codebase management_  
- Security-focused dependency tree (cargo-audit compliant)  
- Comprehensive inline documentation

## 🛠️ Technology stack
| Category              | Key Technologies               |
|-----------------------|---------------------------------|
| Web Framework         | Axum 0.8 + Tower               |
| Database              | PostgreSQL + SQLx 0.8          |
| Storage               | S3 / MinIO                      |
| Security              | JWT + Argon2 + Rustls + TOPTP    |
| Monitoring            | Tracing + Sysinfo              |

## 📂 Project structure
```
axium/                              # Root project directory
├── migrations/                     # Database schema migrations
├── src/                            # Application source code
│   ├── core/                       # Core application infrastructure
│   ├── database/                   # Database access layer (SQLx)
│   ├── middlewares/                # Middleware components
│   ├── routes/                     # API endpoint routing
│   ├── handlers/                   # Request handlers
│   ├── utils/                      # Common utilities
│   ├── wrappers/                   # Wrapper implementations
│   ├── cache/                      # Caching mechanisms (Redis)
│   ├── storage/                    # Storage service integrations (S3 / MinIO)
│   └── main.rs                     # Application entry point
├── documentation/                  # Project documentation
├── Bruno.json                      # API testing configuration for Bruno
├── .env.example                    # Environment template
├── Dockerfile                      # Production container build
├── docker-compose.yml              # Local development stack
└── Cargo.toml                      # Rust dependencies & metadata
```

## 🌐 Default API endpoints

| Method | Endpoint                        | Auth Required | Administrator only | Description                                                      |
|--------|---------------------------------|---------------|-------------------|------------------------------------------------------------------|
| POST   | `/login`                        | 🚫            | 🚫                | Authenticate user and get JWT token                              |
| POST   | `/register`           | 🚫            | 🚫                | Create an user account.    |
| POST   | `/register/verify`   | 🚫            | 🚫                | Confirm the acount creation using the activation code sent to the user's email.           |
| POST   | `/reset`           | 🚫            | 🚫                | Request a password reset code to be sent to the user's email.    |
| POST   | `/reset/verify`   | 🚫            | 🚫                | Confirm password reset with code and set new password.           |
| GET    | `/protected`                    | ✅            | 🚫                | Test endpoint for authenticated users                            |
| GET    | `/health`                       | 🚫            | 🚫                | System health check with metrics                                 |
|        |                                 |               |                   |                                                                  |
| **Apikey routes**                        |               |                   |                                                                  |
| GET    | `/apikeys/all`                  | ✅            | 🚫                | Get all apikeys of the current user.                             |
| POST   | `/apikeys/`                     | ✅            | 🚫                | Create a new apikey.                                             |
| GET    | `/apikeys/{id}`                 | ✅            | 🚫                | Get an apikey by ID.                                             |
| DELETE | `/apikeys/{id}`                 | ✅            | 🚫                | Delete an apikey by ID.                                          |
| POST   | `/apikeys/rotate/{id}`          | ✅            | 🚫                | Rotates an API key, disables the old one (grace period 24 hours), returns a new one. |
|        |                                 |               |                   |                                                                  |
| **User routes**                          |               |                   |                                                                  |
| GET    | `/users/all`                    | ✅            | ✅                | Get all users.                                                   |
| POST   | `/users/`                       | ✅            | ✅                | Create a new user.                                               |
| POST   | `/users/{id}/profile-picture`   | ✅            | 🚫/✅ (see below)  | Upload or update a user's profile picture. Will be converted to WebP, cropped to 300x300, max 10 MB, Admins can upload for others. |
| PATCH  | `/users/{id}`                   | ✅            | 🚫/✅ (see below)  | Update user profile fields (self or admin for others).           |
| GET    | `/users/current`                | ✅            | 🚫                | Get the current user.                                            |
| GET    | `/users/{id}`                   | ✅            | ✅                | Get a user by ID.                                                |
| DELETE | `/users/{id}`                   | ✅            | ✅                | Delete a user by ID.                                             |
|        |                                 |               |                   |                                                                  |
| **Usage routes**                         |               |                   |                                                                  |
| GET    | `/usage/lastweek`               | ✅            | 🚫                | Amount of API calls within the last week of the current user.    |
| GET    | `/usage/lastday`                | ✅            | 🚫                | Amount of API calls within last day of the current user.         |
|        |                                 |               |                   |                                                                  |
| **Todo routes**                          |               |                   |                                                                  |
| GET    | `/todos/all`                    | ✅            | 🚫                | Get all todos of the current user.                               |
| POST   | `/todos/`                       | ✅            | 🚫                | Create a new todo.                                               |
| GET    | `/todos/{id}`                   | ✅            | 🚫                | Get a todo by ID.                                                |
| DELETE | `/todos/{id}`                   | ✅            | 🚫                | Delete a todo by ID.                                             |

---

### **Notes:**
- **POST `/users/{id}/profile-picture`** and **PATCH `/users/{id}`**:  
  - Regular users can update their own profile or profile picture.
  - Admins can update or upload for any user.
  - Marked as "🚫/✅ (see below)" to indicate both self and admin access.
- If you want to clarify this further, you can add a footnote or a new column for "Self or Admin".

## 📦 Installation & usage
To get started with Axium, you'll need to install it on your system. We provide detailed installation guides for different environments:

- **Docker setup**: Follow the instructions in [Docker setup guide](/documentation/installation_docker.md) to run Axium using Docker Compose.
- **Ubuntu setup**: For users on Ubuntu or other Debian-based systems, refer to the [Ubuntu setup Guide](/documentation/installation_ubuntu.md).
- **Windows setup**: Windows users can find their setup instructions in the [Windows setup guide](/documentation/installation_windows.md).

Make sure to have a PostgreSQL database and a S3 storage available. Both can be easily locally installed. We recommend [MinIO](https://min.io/), as the S3 storage for developing the API locally 

These guides cover cloning the repository, setting up the environment, configuring the database, and running the application.

## Integration
You can easily integrate Axium with your applications. Here is a detailed guide of integrating the authentication process in SolidJS:
- [SolidJS](/documentation/integration_solidjs.md).

We might add some more examples in the future. The SolidJS-example can be easily adapted for other JavaScript/TypeScript frameworks.


### 🔐 Authentication
To authenticate, send a POST request to the `/login` endpoint with a JSON body in the following format:

```json
{
  "email": "admin@test.com",
  "password": "test",
  "totp": "12234"  // Optional: only required if your account uses 2FA
}
```

Depending on the server configuration, after a successful login:

- You will receive a JWT token in the response body, and/or,

- The server will set a secure, HTTP-only cookie containing your authentication token.

If you receive a JWT in the response body:
- **Send it in the Authorization header for future requests:** `Authorization: Bearer <your_token_here>`
- **If you receive a cookie:** Your browser will automatically send it with each request. No manual action is needed.


### 👤 Default accounts

**Warning:** These accounts should only be used for initial testing. Always change or disable them in production environments.

| Email               | Password | Role           |
|---------------------|----------|----------------|
| `user@test.com`     | `test`   | User           |
| `admin@test.com`    | `test`   | Administrator  |

⚠️ **Security recommendations:**
1. Rotate passwords immediately after initial setup.
2. Disable default accounts before deploying to production.
3. Implement proper user management endpoints.

#### Administrative password resets  
*For emergency access recovery only*  

1. **Database Access**  
   Connect to PostgreSQL using privileged credentials:  
   ```bash  
   psql -U admin_user -d axium_db -h localhost  
   ```

2. **Secure Hash Generation**  
   Use the integrated CLI tool (never user online generators):  
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
Most configuration options can be set using the `.env` file and the database tables that are being created during the first run (check out: `/migrations`). 

As this project is a template we encourage you to tinker it to your hearts desire. First place to start in most cases is the `handlers` folder. Here you can define per endpoint what should happen. Afterwards you can easily integrate your new handler in one of the `/routes`.