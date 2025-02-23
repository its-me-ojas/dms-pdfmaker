# Build stage
FROM rust:1.80-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .

# Build dependencies first (for better caching)
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install required runtime dependencies
RUN apt-get update && apt-get install -y \
    libreoffice \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/dms-pdfmaker /usr/local/bin/dms-pdfmaker

# Copy the public folder directly to /usr/src/app/public
COPY --from=builder /usr/src/app/public /usr/src/app/public

# Create directories for the application
RUN mkdir -p /usr/src/app/data

# Set environment variables
ENV RUST_LOG=info

# Expose the port your application uses
EXPOSE 8080

# Run the binary
CMD ["dms-pdfmaker"]
