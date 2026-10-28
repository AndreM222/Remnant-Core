# ----------------------------------------------------
# Stage 1: Builder
# Compiles the Rust bnary in a full Rust environment
# ----------------------------------------------------
FROM rust:latest AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy dependency files first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Caching cargo dependencies in Docker — fake both bin and lib targets
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && echo "// dummy lib for dependency caching" > src/lib.rs
RUN cargo build --release

RUN rm -rf src

# Copy real source code and build
COPY src ./src
RUN touch src/main.rs src/lib.rs && cargo build --release

# ----------------------------------------------------
# Stage 2: Runtime
# Minimal image containing only compiled Binary
# No Rust toolchain, no source code, no build tools
# ----------------------------------------------------
FROM  debian:bookworm-slim AS runtime

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN  useradd --create-home --shell /bin/bash remnant
USER remnant
WORKDIR /home/remnant

# Copy only the compiled binary from the builder stage
COPY --from=builder /app/target/release/remnant-core ./remnant-core

# Expose the application port
EXPOSE 8080

# Health check so Railway knows when the container is ready
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

CMD ["./remnant-core"]
