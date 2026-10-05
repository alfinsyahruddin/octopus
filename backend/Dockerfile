FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

ENV OPENSSL_STATIC=1
RUN cargo build --release

FROM alpine:3.21 AS runner
RUN apk add --no-cache ca-certificates
WORKDIR /app

COPY --from=builder /app/target/release/backend /app/backend
EXPOSE 8000
CMD ["/app/backend"]
