FROM rust:latest as build

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

RUN cargo clean && cargo build

CMD cargo run 