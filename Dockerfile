FROM rust:1.93.1 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --release --bin blazing-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/blazing-server /usr/local/bin/blazing-server

EXPOSE 3000
ENV RUST_LOG=info
CMD ["blazing-server"]
