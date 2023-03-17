FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown

WORKDIR /TOTL_BACKEND

RUN cargo build --release

COPY --from=build /TOTL_BACKEND/target/release/totl_backend /totl_backend

CMD ["/totl_backend"]