FROM rust:1.66 as builder
WORKDIR /usr/src/app
COPY . . 
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /usr/src/app/totl_backend /totl_backend

WORKDIR /totl_backend

CMD ["totl_backend"]