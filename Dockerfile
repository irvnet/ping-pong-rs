# BUILDER STAGE
FROM rust:latest AS builder

# Set the working directory
WORKDIR /app

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# RUNTIME STAGE
FROM rust:latest 

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/ping-pong-rs /app/ping-pong-rs

# Expose the default port (can be overridden)
EXPOSE 8080

# Set default environment variables 
ENV HOST=0.0.0.0
ENV PORT=8080
ENV RUST_LOG=info

RUN useradd -m appuser
USER appuser
CMD ["/app/ping-pong-rs"]
