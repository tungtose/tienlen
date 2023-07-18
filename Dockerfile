FROM rust:1.71.0 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path ./server
FROM debian:bullseye-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/tienlen-server /usr/local/bin/tienlen-server
CMD ["tienlen-server"]
