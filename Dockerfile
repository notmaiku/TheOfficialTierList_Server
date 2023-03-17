FROM rust:latest as build

COPY . .

RUN cargo build --release


CMD ["/totl_backend"]