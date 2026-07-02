FROM rust:alpine AS builder

RUN apk add --no-cache build-base ca-certificates

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .cargo ./.cargo

RUN cargo build --release

FROM scratch

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/url-shortening-service url-shortening-service
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

USER 10001:10001

ENTRYPOINT [ "./url-shortening-service" ]
