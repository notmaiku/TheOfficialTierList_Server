
  #FROM rust:slim AS build  
  #WORKDIR /app  
  #COPY . .  
  #RUN apt-get update && apt-get install -y musl-tools && cargo build --release  
  #
  #FROM debian:bookworm-slim  
  #RUN apt-get update && apt-get install -y ca-certificates  
  #COPY --from=build /app/target/release/totl_backend /totl_backend  
  #CMD ["/totl_backend"]  




FROM rust:slim AS build  
WORKDIR /app  
COPY . .  
RUN apt-get update && apt-get install -y --no-install-recommends musl-tools  
RUN cargo build --release --target=x86_64-unknown-linux-musl  

FROM debian:bookworm-slim  
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates  
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/totl_backend /totl_backend  
CMD ["/totl_backend"]

