# Build Stage - Use a minimal Rust image
FROM rust:alpine as build

# Install only essential dependencies
RUN apk add --no-cache musl-dev gcc

WORKDIR /TOTL_BACKEND

# Cache dependencies for faster builds
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# Copy the actual source code and build
COPY . .
RUN cargo build --release

# Runtime Stage - Use a tiny Alpine base image
FROM alpine:latest

# Install only required system dependencies
RUN apk add --no-cache ca-certificates

# Use a non-root user for security
USER nobody

# Copy only the built binary (no source code)
COPY --from=build /TOTL_BACKEND/target/release/totl_backend /totl_backend

# Start the application
CMD ["/totl_backend"]

