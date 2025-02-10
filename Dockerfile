# Use the latest stable Rust slim image
FROM rust:latest as build

# Set working directory
WORKDIR /TOTL_BACKEND

# Copy only Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the rest of the source code and build
COPY . .
RUN cargo build --release

# Use a more recent slim Debian base image
FROM debian:bookworm-slim

# Set a non-root user for better security
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

USER nobody

# Copy the built binary from the builder stage
COPY --from=build /TOTL_BACKEND/target/release/totl_backend /totl_backend

# Set the entrypoint
CMD ["/totl_backend"]
