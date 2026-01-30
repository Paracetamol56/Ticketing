FROM oven/bun:latest AS frontend-builder
WORKDIR /app/www
COPY www/package.json www/bun.lock ./
RUN bun install --frozen-lockfile
COPY www/ ./
RUN bun run build

FROM rust:bullseye AS backend-builder
WORKDIR /app/server
COPY server/Cargo.toml server/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
COPY server/ ./
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y \
  libsqlite3-0 ca-certificates \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=backend-builder /app/server/target/release/ticketing-api ./

COPY --from=frontend-builder /app/www/build ./static
EXPOSE 8080
CMD ["./ticketing-api"]
