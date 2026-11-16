FROM rust:1.91-alpine3.21 AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY Cargo.toml Cargo.lock ./
COPY crates/opensec-core crates/opensec-core
COPY services/opensec-api services/opensec-api
COPY services/opensec-worker services/opensec-worker
RUN cargo build --release --bin opensec-api

FROM alpine:3.21.3
RUN apk add --no-cache curl && adduser -D -u 10001 opensec \
    && mkdir -p /var/lib/opensec/pcaps && chown 10001:10001 /var/lib/opensec/pcaps && chmod 755 /var/lib/opensec/pcaps
USER opensec
COPY --from=builder /app/target/release/opensec-api /usr/local/bin/opensec-api
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/opensec-api"]
