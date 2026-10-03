FROM rust:1.80-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM alpine:3.20 AS runner
RUN apk add --no-cache ca-certificates
WORKDIR /app

COPY --from=builder /app/target/release/backend /app/backend
EXPOSE 8000
CMD ["/app/backend"]
