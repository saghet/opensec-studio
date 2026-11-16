FROM rust:1.91-alpine3.21 AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY Cargo.toml Cargo.lock ./
COPY crates/opensec-core crates/opensec-core
COPY services/opensec-api services/opensec-api
COPY services/opensec-worker services/opensec-worker
RUN cargo build --release --bin opensec-worker

FROM alpine:3.21.3
RUN adduser -D -u 10001 opensec
USER opensec
COPY --from=builder /app/target/release/opensec-worker /usr/local/bin/opensec-worker
ENTRYPOINT ["/usr/local/bin/opensec-worker"]
