FROM --platform=$BUILDPLATFORM rust as server-sources

WORKDIR /usr/src/bitly
RUN cargo init
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir -p .cargo && cargo vendor > .cargo/config

FROM rust as builder
WORKDIR /usr/src/bitly
COPY src/ src/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY --from=server-sources /usr/src/bitly/.cargo .cargo
COPY --from=server-sources /usr/src/bitly/vendor vendor
RUN cargo build --release --offline

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libsqlite3-0 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/bitly/target/release/bitly /usr/local/bin/bitly
EXPOSE 4242
CMD ["bitly"]
