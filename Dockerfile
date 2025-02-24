# Stage 1: Build the application
FROM rust:1.65 as builder
WORKDIR /app

# Copy the Cargo manifests to leverage Docker cache for dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --locked
RUN rm -rf src

# Copy the source code and build the actual application
COPY . .
RUN cargo build --release --locked

# Stage 2: Create a minimal runtime image
FROM debian:buster-slim
WORKDIR /app

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y libpq-dev openssl && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/decentralized_identity /app/decentralized_identity

# Set an environment variable (optional)
ENV RUST_LOG=info

# Expose the API port
EXPOSE 3030

# Define the command to run the application
CMD ["./decentralized_identity"]
