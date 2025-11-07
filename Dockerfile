# Build stage
FROM rust:1.88-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build for release
RUN cargo build --release --bin api-server

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 app

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/api-server /usr/local/bin/api-server

# Change ownership
RUN chown -R app:app /app

USER app

EXPOSE 8080

CMD ["api-server"]
