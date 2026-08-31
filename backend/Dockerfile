# Stage 1: Build the Rust application
FROM rust:1-slim AS builder

WORKDIR /app

# Copy dependency files first for caching
COPY Cargo.toml Cargo.lock ./

# Copy source directory
COPY src ./src

# Build release binary
RUN cargo build --release

# Stage 2: Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install SSL certificates for outbound HTTPS requests
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy compiled binary from builder
COPY --from=builder /app/target/release/message_backend /app/backend-bin

EXPOSE 7878

CMD ["/app/backend-bin"]
