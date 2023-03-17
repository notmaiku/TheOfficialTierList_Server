FROM rust:latest as build

COPY ./ ./

RUN cargo build --release


CMD ["./target/release/totl_backend"]