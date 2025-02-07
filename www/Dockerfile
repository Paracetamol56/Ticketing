FROM oven/bun:latest AS builder
WORKDIR /app
COPY package.json bun.lockb ./
RUN bun install --frozen-lockfile
COPY . .
RUN bun run build

FROM httpd:2.4-alpine
WORKDIR /usr/local/apache2/htdocs
COPY --from=builder /app/build/ ./
EXPOSE 80
CMD ["httpd", "-D", "FOREGROUND"]
