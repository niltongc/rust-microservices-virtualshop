# 1️⃣ Builder
FROM rust:1.93-bookworm AS builder

WORKDIR /app

# install OpenSSL to glibc
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    build-essential \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy code
COPY . .

# build
RUN cargo build --release

# Minimal Image ~34 mb
FROM gcr.io/distroless/cc:nonroot

WORKDIR /app

# Copy binaries
COPY --from=builder /app/target/release/rust-microservices /app/server

# Expose port
EXPOSE 8080

# Execute
CMD ["/app/server"]
