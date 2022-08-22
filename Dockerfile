FROM rust:slim-buster as builder
WORKDIR /usr/src/bitly
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libsqlite3-0 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/bitly /usr/local/bin/bitly
EXPOSE 4242
CMD ["bitly"]
