FROM rust:alpine AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apk add --no-cache musl-dev musl-utils

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/url-shortening-service url-shortening-service

ENTRYPOINT [ "./url-shortening-service" ]
