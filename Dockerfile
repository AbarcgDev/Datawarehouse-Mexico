FROM rust:1.81 as builder

WORKDIR /app

COPY Cargo.toml .
COPY crates ./crates
COPY bin ./bin

RUN cargo generate-lockfile && cargo build --release -p main_app

# Verifica que el binario esté presente
RUN ls -l /app/target/release/

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
  libssl-dev=3.0.15-1~deb12u1 \
  ca-certificates=20230311 \
  --no-install-recommends \
  && rm -rf /var/lib/apt/lists/*WORKDIR /app

WORKDIR /app

COPY --from=builder /app/target/release/main_app .

CMD ["./main_app"]
#CMD ["sh", "-c", "while true; do sleep 1000; done"]

