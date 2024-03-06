FROM rust:1.49-slim-buster as build

WORKDIR /TOTL_BACKEND

COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=build /TOTL_BACKEND/target/release/totl_backend /totl_backend

CMD ["/totl_backend"]

