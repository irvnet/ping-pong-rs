# Builder stage
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM rust:latest 
WORKDIR /app
COPY --from=builder /app/target/release/ping-pong-rs /app/ping-pong-rs
EXPOSE 8080
RUN useradd -m appuser
USER appuser
CMD ["/app/ping-pong-rs"]
