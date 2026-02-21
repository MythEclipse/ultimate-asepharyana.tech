FROM ubuntu:24.04
RUN apt-get update && apt-get install -y openssl ca-certificates curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app

WORKDIR /app

# Copy pre-built binary from the host
COPY apps/rust/target/release/rustexpress /app/rustexpress

# Expose port
EXPOSE 4091

# Run the binary
CMD ["./rustexpress"]
