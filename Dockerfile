FROM rust:latest as build

WORKDIR /TOTL_BACKEND

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian11




COPY --from=build /TOTL_BACKEND/release/totl_backend /totl_backend

CMD ["/totl_backend"]
