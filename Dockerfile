FROM rust:alpine AS builder

RUN apk add --no-cache build-base

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

FROM scratch

WORKDIR /app

COPY --from=builder /app/target/release/url-shortening-service url-shortening-service

ENTRYPOINT [ "./url-shortening-service" ]
