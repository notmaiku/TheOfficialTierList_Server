
# Build Stage - Use lightweight Rust image
FROM rust:slim as build

# Install minimal dependencies
RUN apt-get update && apt-get install -y --no-install-recommends musl-tools

WORKDIR /TOTL_BACKEND

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# Copy actual source code and build the final binary
COPY . .
RUN cargo build --release

# Runtime Stage - Use minimal Debian base
FROM debian:bookworm-slim


# Copy the built binary
COPY --from=build /TOTL_BACKEND/target/release/totl_backend /totl_backend

# Start the application
CMD ["/totl_backend"]

