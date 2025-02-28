# --- Stage 1: Builder Stage ---
    FROM rust:1.84-alpine AS builder

    WORKDIR /app
    
    # Install required build dependencies for Rust and SQLx
    RUN apk add --no-cache \
        pkgconfig \
        openssl-dev \
        sqlite-dev \
        build-base \
        cmake \
        curl \
        ninja-build \
        clang \
        && rm -rf /var/cache/apk/*
    
    # Cache dependencies (from Cargo.toml and Cargo.lock) to speed up future builds
    COPY Cargo.toml ./
    RUN cargo fetch
    
    # Copy the source code for the application
    COPY src src/
    
    # Set SQLX_OFFLINE to true for offline SQLx compilation
    ENV SQLX_OFFLINE=true
    
    # Copy the pre-generated SQLx metadata for offline mode
    COPY .sqlx .sqlx/
    
    # Copy the migrations folder
    COPY migrations migrations/
    
    # Build the application in release mode
    RUN cargo build --release --locked --no-default-features
    
    # Strip debug symbols to reduce binary size
    RUN strip /app/target/release/Axium
    
    # --- Stage 2: Runtime Stage ---
    FROM alpine:latest
    
    # Install runtime dependencies only (ca-certificates, openssl)
    RUN apk add --no-cache \
        ca-certificates \
        openssl \
        && rm -rf /var/cache/apk/*
    
    # Create non-root user for security purposes
    RUN adduser -D -u 1001 appuser
    
    WORKDIR /app
    
    # Copy the built binary from the builder stage
    COPY --from=builder /app/target/release/Axium .
    
    # Ensure the .env file and other app files have the correct ownership and permissions
    RUN chown -R appuser:appuser /app
    
    # Switch to the non-root user
    USER appuser
    
    # Expose the application port (default 3000)
    EXPOSE 3000
    
    # Run the application when the container starts
    CMD ["./Axium"]    