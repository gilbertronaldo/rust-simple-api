FROM rust:1.67 AS builder

WORKDIR /usr/src/rust-simple-api

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

COPY Cargo.toml Cargo.lock /usr/src/rust-simple-api/
COPY src /usr/src/rust-simple-api/src

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.17 AS runtime

COPY --from=builder /usr/src/rust-simple-api/target/x86_64-unknown-linux-musl/release/rust-simple-api /usr/local/bin

EXPOSE 8000

CMD ["/usr/local/bin/rust-simple-api"]
