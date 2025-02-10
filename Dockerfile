
# Build Stage - Use a minimal Rust image
FROM rust:alpine as build

# Install only essential dependencies
RUN apk add --no-cache musl-dev gcc

WORKDIR /TOTL_BACKEND

# Cache dependencies for faster builds
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# Copy actual source code and build the final binary
COPY . .
RUN cargo build --release

# Runtime Stage - Use a small Alpine image
FROM alpine:latest

# Install CA certificates (needed for HTTPS)
RUN apk add --no-cache ca-certificates && update-ca-certificates

# Set a non-root user for security
USER nobody

# Copy only the built binary
COPY --from=build /TOTL_BACKEND/target/release/totl_backend /totl_backend

# Railway provides a default PORT environment variable; no need to expose a port
CMD ["/totl_backend"]

