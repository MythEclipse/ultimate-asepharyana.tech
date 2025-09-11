# Use a Rust base image
FROM rust:latest as builder

# Set the working directory in the container
WORKDIR /app

# Copy Cargo.toml and Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Copy the apps/rust directory
COPY src ./src
COPY build_utils ./build_utils
COPY migrations ./migrations

# Build the Rust application
RUN cargo build --release --manifest-path Cargo.toml --target-dir target

# Use a smaller base image for the final stage
FROM debian:stable-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/rust /usr/local/bin/rust_app

# Expose the port Rust application runs on
EXPOSE 4091

# Command to run the application
CMD ["/usr/local/bin/rust_app"]
