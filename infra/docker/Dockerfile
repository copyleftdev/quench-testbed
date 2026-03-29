FROM rust:1.94-slim AS builder
WORKDIR /app
COPY backend/ .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/todo-api /usr/local/bin/
EXPOSE 8080
CMD ["todo-api"]
