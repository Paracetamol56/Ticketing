FROM rust:bullseye AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
COPY . .
RUN cargo build --release

FROM debian:bullseye
RUN apt-get update && apt-get install -y libsqlite3-0 ca-certificates
COPY --from=builder /app/target/release/ticketing-api /usr/src/ticketing-api
EXPOSE 8080
CMD ["/usr/src/ticketing-api"]
