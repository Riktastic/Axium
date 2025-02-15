# --- Stage 1: Builder Stage ---
    FROM rust:1.75-slim-bookworm AS builder

    WORKDIR /app
    
    # Install required build dependencies
    RUN apt-get update && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        && rm -rf /var/lib/apt/lists/*
    
    # Cache dependencies
    COPY Cargo.toml Cargo.lock ./
    RUN cargo fetch --locked
    
    # Copy source code
    COPY src src/
    COPY build.rs build.rs
    
    # Build the application in release mode
    RUN cargo build --release --locked
    
    # Strip debug symbols to reduce binary size
    RUN strip /app/target/release/Axium
    
    
    # --- Stage 2: Runtime Stage ---
    FROM debian:bookworm-slim
    
    # Install runtime dependencies only
    RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        openssl \
        && rm -rf /var/lib/apt/lists/*
    
    # Create non-root user
    RUN useradd --no-log-init -r -m -u 1001 appuser
    
    WORKDIR /app
    
    # Copy built binary from builder stage
    COPY --from=builder /app/target/release/Axium .
    
    # Copy environment file (consider secrets management for production)
    COPY .env .env
    
    # Change ownership to non-root user
    RUN chown -R appuser:appuser /app
    
    USER appuser
    
    # Expose the application port
    EXPOSE 3000
    
    # Run the application
    CMD ["./Axium"]    