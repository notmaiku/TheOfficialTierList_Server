FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown

WORKDIR /TOTL_BACKEND
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian11


COPY --from=build /TOTL_BACKEND/target/release/totl_backend /totl_backend

CMD ["/totl_backend"]