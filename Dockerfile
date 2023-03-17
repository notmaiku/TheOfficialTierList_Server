FROM rust:latest as build

WORKDIR /TOTL_BACKEND

RUN cargo build --release

COPY --from=build /TOTL_BACKEND/target/release/totl_backend /totl_backend

CMD ["/totl_backend"]