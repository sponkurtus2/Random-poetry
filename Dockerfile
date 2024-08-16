# Build stage
FROM rust:bookworm AS builder
LABEL authors="sponk"

WORKDIR /app
COPY . .
RUN cargo build --release

# Final run stage
FROM debian:bookworm-slim AS runner

# Missing dependency
RUN apt-get update && apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/random_poetry /app/random_poetry
CMD ["/app/random_poetry"]