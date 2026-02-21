FROM alpine:latest
RUN sed -i 's/dl-cdn.alpinelinux.org/mirror.kartolo.sby.datautama.net.id/g' /etc/apk/repositories && \
    apk add --no-cache openssl ca-certificates curl gcompat libgcc

WORKDIR /app

# Copy pre-built binary from the host
COPY apps/rust/target/release/rustexpress /app/rustexpress

# Expose port
EXPOSE 4091

# Run the binary
CMD ["./rustexpress"]
