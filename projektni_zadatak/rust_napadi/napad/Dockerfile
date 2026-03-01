# Race Condition Demo - Dockerfile
FROM rust:alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml ./

# Create dummy main to cache dependencies
RUN mkdir src

# Copy actual source code
COPY server.rs ./src/main.rs

# Build the application
RUN cargo build --release

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache libgcc python3 py3-pip

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/race-condition-demo /app/race-condition-demo

# Copy exploit script
COPY exploit.py /app/exploit.py

# Install Python dependencies
RUN pip3 install --no-cache-dir requests --break-system-packages

# Expose port
EXPOSE 8080

# Run the application
CMD ["/app/race-condition-demo"]
