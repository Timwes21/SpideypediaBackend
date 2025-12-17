# =========================
# 1. Builder image (Rust 1.89.0)
# =========================
FROM rust:1.89.0-slim AS builder

# Create a new empty shell project
WORKDIR /app

# Install build dependencies (you can add more as needed)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Copy manifest first to leverage Docker layer caching
COPY Cargo.toml Cargo.lock ./

# This build step will cache dependencies
RUN mkdir src && echo "fn main() { println!(\"dummy build\"); }" > src/main.rs
RUN cargo build --release && rm -rf src

# Now copy the actual source
COPY . .

# Build your real binary in release mode
RUN cargo build --release

# =========================
# 2. Runtime image
# =========================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 👇 Replace `myapp` with the actual name of your binary (from Cargo.toml [package].name)
COPY --from=builder /app/target/release/myapp /usr/local/bin/myapp

# Run as non-root user (optional but recommended)
RUN useradd -m appuser
USER appuser

CMD ["myapp"]
