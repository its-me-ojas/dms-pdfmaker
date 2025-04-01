# Runtime stage
FROM ubuntu:jammy

# Install required runtime dependencies
RUN apt-get update && apt-get install -y \
    libreoffice \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY dms-pdfmaker .

# Copy the public folder directly to /usr/src/app/public
COPY ./public /app/public
RUN chmod +x dms-pdfmaker
# Set environment variables
ENV RUST_LOG=info

# Expose the port your application uses
EXPOSE 8080

# Run the binary
CMD ["./dms-pdfmaker"]
