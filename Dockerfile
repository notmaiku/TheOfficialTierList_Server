
  #FROM rust:slim AS build  
  #WORKDIR /app  
  #COPY . .  
  #RUN apt-get update && apt-get install -y musl-tools && cargo build --release  
  #
  #FROM debian:bookworm-slim  
  #RUN apt-get update && apt-get install -y ca-certificates  
  #COPY --from=build /app/target/release/totl_backend /totl_backend  
  #CMD ["/totl_backend"]  


FROM rust:latest as build

WORKDIR /TOTL_BACKEND

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian11


COPY --from=build /TOTL_BACKEND/target/release/totl_backend /totl_backend

CMD ["/totl_backend"]
