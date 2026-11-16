FROM rust:1.85.0-alpine3.21 AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY Cargo.toml ./
COPY services/opensec-api/Cargo.toml services/opensec-api/Cargo.toml
COPY services/opensec-api/src services/opensec-api/src
COPY services/opensec-worker/Cargo.toml services/opensec-worker/Cargo.toml
COPY services/opensec-worker/src services/opensec-worker/src
RUN cargo build --release --bin opensec-worker

FROM alpine:3.21.3
RUN adduser -D -u 10001 opensec
USER opensec
COPY --from=builder /app/target/release/opensec-worker /usr/local/bin/opensec-worker
ENTRYPOINT ["/usr/local/bin/opensec-worker"]
