FROM  rust:1.84.0-slim as build

WORKDIR /usr/src/rnsh

COPY . .

RUN cargo install --path

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/rnsh /usr/local/bin/rnsh


