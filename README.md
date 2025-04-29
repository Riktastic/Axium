# 🦖 Axium
**An example API built with Rust, Axum, SQLx, and PostgreSQL.**  
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> ⚠️ **Warning:** This project is under active development. Pushed changes have been tested. But it might not yet be production ready.

## Summary
Axium is a high-performance, security-focused API boilerplate built using Rust, Axum, SQLx, S3 and PostgreSQL. It provides a ready-to-deploy solution with modern best practices, including JWT authentication, role-based access control (RBAC), structured logging, and enterprise-grade security. With a focus on developer experience, Axium offers auto-generated API documentation, efficient database interactions, and an ergonomic code structure for ease of maintenance and scalability.

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
  - [🤝 Contributing](#-contributing)
    - [📝 How to Contribute](#-how-to-contribute)
    - [🔍 Code Style](#-code-style)
    - [🛠️ Reporting Bugs](#️-reporting-bugs)
    - [💬 Discussion](#-discussion)
    - [🧑‍💻 Code of Conduct](#-code-of-conduct)
    - [🎉 Thanks for Contributing!](#-thanks-for-contributing)

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
    "details": {
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
            }
        ],
        "memory": {
            "available_mb": 17785,
            "status": "normal"
        },
        "network": {
            "status": "ok"
        }
    },
    "status": "degraded"
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
- Security-focused dependency tree (cargo-audit compliant)  
- Comprehensive inline documentation

## 🛠️ Technology stack
| Category              | Key Technologies               |
|-----------------------|---------------------------------|
| Web Framework         | Axum 0.8 + Tower               |
| Database              | PostgreSQL + SQLx 0.8          |
| Security              | JWT + Argon2 + Rustls           |
| Monitoring            | Tracing + Sysinfo              |

## 📂 Project structure
```
axium/                              # Root project directory
├── 📁 migrations/                  # Database schema migrations (SQLx)
│
├── 📁 src/                         # Application source code
│   ├── 📁 core/                    # Core application infrastructure
│   │   ├── config.rs               # Configuration loader (.env, env vars)
│   │   └── server.rs               # HTTP/HTTPS server initialization
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
│   ├── 📁 wrappers/                   # Wrapper implementations
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

| Method | Endpoint                        | Auth Required | Administrator only | Description                                                      |
|--------|---------------------------------|---------------|-------------------|------------------------------------------------------------------|
| POST   | `/login`                        | 🚫            | 🚫                | Authenticate user and get JWT token                              |
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

Make sure to have a PostgreSQL database and a S3 storage available. Both can be easily locally installed. We recommend [MinIO](https://min.io/) for developing the API locally 

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
Create a .env file in the root of the project or configure the application using environment variables.

Make sure to change the `JWT_SECRET_KEY`.

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

# For running Axium standalone:
DATABASE_URL="postgres://dbuser:1234@localhost/axium"

# For docker:
DATABASE_USER=dbuser
DATABASE_PASSWORD=1234
DATABASE_DB=axium

# Maximum number of connections in the database pool
DATABASE_MAX_CONNECTIONS=20

# Minimum number of connections in the database pool
DATABASE_MIN_CONNECTIONS=5


# ==============================
# ☁️ STORAGE (S3/MINIO) CONFIGURATION
# ==============================

# Endpoint (e.g., http://127.0.0.1:9000)
STORAGE_ENDPOINT="http://127.0.0.1:9000"

# Region (e.g., us-east-1)
STORAGE_REGION="us-east-1"

# Access key
STORAGE_ACCESS_KEY="minioadmin"

# Secret key
STORAGE_SECRET_KEY="minioadmin"

# Bucket name for storing profile pictures. ! Make sure that this bucket has been created.
STORAGE_BUCKET_PROFILE_PICTURES="profile-pictures"


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
JWT_SECRET_KEY="Change me!"

# JWT issuer
JWT_ISSUER="your_issuer"  # Set this to your desired issuer value

# JWT audience
JWT_AUDIENCE="your_audience"  # Set this to your desired audience value

# Allow authentication via HTTP cookies (true/false)
JWT_ALLOW_COOKIE_AUTH=true

# Force authentication via HTTP cookies only (true/false)
JWT_FORCE_COOKIE_AUTH=false

# Name of the cookie used to store the JWT token
JWT_COOKIE_NAME="auth_token"

# Maximum age of the JWT token in seconds
JWT_COOKIE_MAX_AGE=604800 # 7 days in seconds

# SameSite attribute for the JWT cookie (Lax, Strict, or None), SERVER_HTTPS_ENABLED must be true.
JWT_COOKIE_SAMESITE="Lax" 


# ==============================
# 🌐 CORS CONFIGURATION
# ==============================

# Allowed origin for CORS requests (comma-separated for multiple origins)
# Example: "http://127.0.0.1:3000,http://localhost:3000"
CORS_ALLOW_ORIGIN="*"

# Allowed HTTP methods for CORS (comma-separated)
# Example: "GET,POST,PUT,DELETE,OPTIONS"
CORS_ALLOW_METHODS="GET,POST,PUT,DELETE,OPTIONS"

# Allowed headers for CORS (comma-separated)
# Example: "Authorization,Content-Type,Origin"
CORS_ALLOW_HEADERS="Authorization,Content-Type,Origin"

# Allow credentials (true/false)
CORS_ALLOW_CREDENTIALS=true

# Max age (in seconds) for preflight request caching
CORS_MAX_AGE=3600
```

## 🤝 Contributing

We welcome contributions to the Axium project! Whether it's fixing bugs, improving documentation, or adding new features, your help is greatly appreciated. Please follow these guidelines to ensure a smooth contribution process.

### 📝 How to Contribute

1. **Fork the Repository**  
   Start by forking the repository to your own GitHub account.

2. **Clone Your Fork**  
   Clone your forked repository to your local machine:
   ```bash
   git clone https://github.com/your-username/Axium.git
   cd Axium
   ```

3. **Create a New Branch**  
   Create a new branch for your feature or bug fix:
   ```bash
   git checkout -b feature-name
   ```

4. **Make Your Changes**  
   Make the necessary changes to the code or documentation. Make sure to write tests for new features and adhere to the existing code style.

5. **Commit Your Changes**  
   Commit your changes with a clear, descriptive message:
   ```bash
   git commit -m "Add feature XYZ or fix issue ABC"
   ```

6. **Push to Your Fork**  
   Push your changes to your fork:
   ```bash
   git push origin feature-name
   ```

7. **Open a Pull Request**  
   Open a pull request against the `main` branch of the original repository. In the description, provide details about the changes you made, the problem they solve, and any testing you performed.

### 🔍 Code Style

- Follow the **Rust style guidelines** outlined in the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/).
- Use **cargo fmt** to automatically format your code:
  ```bash
  cargo fmt
  ```
- Write **meaningful commit messages** that describe the changes you've made.

### 🛠️ Reporting Bugs

If you encounter a bug or issue, please check if it has already been reported in the [GitHub issues](https://github.com/Riktastic/Axium/issues). If not, create a new issue, providing the following information:

- A clear description of the problem.
- Steps to reproduce the issue.
- Expected vs. actual behavior.
- Any relevant logs or error messages.

### 💬 Discussion

Feel free to open discussions in the [Discussions](https://github.com/Riktastic/Axium/discussions) section for general questions, ideas, or advice on how to improve the project.

### 🧑‍💻 Code of Conduct

Please be respectful and follow the [Code of Conduct](https://www.contributor-covenant.org/) while interacting with other contributors. Let's maintain a positive and welcoming environment.

### 🎉 Thanks for Contributing!

Your contributions help make Axium better for everyone! 🙏
