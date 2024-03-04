FROM rust:latest as build

WORKDIR /TOTL_BACKEND

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=build /TOTL_BACKEND/target/release/totl_backend /totl_backend

CMD ["/totl_backend"]
