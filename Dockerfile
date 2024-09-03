# Stage 1: Build the Rust application
FROM rust:latest as builder

# Create and set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs file to cache dependencies
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

# Build the dependencies
RUN cargo build --release

# Copy the actual source code
COPY src ./src

# Build the actual application
RUN cargo build --release

# Stage 2: Create a smaller runtime image
FROM debian:buster-slim

# Install required packages
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/khunpk /app/

# Expose the port the app will run on
EXPOSE 8080

# Command to run the application
CMD ["/app/khunpk"]