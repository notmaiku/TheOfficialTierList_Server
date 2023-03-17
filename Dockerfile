FROM rust:latest as build

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

CMD cargo run 