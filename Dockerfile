FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/totl_backend
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/totl_backend/target/release/totl_backend /usr/local/bin/totl_backend

WORKDIR /usr/local/bin
CMD ["totl_backend"]