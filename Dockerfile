# Builder stage
FROM rust:1.79 AS builder

# Copy the entire workspace
COPY libs /usr/src/app/libs
COPY services /usr/src/app/services
COPY Cargo.toml Cargo.lock /usr/src/app/

# Set the working directory
WORKDIR /usr/src/app

# Build the rest_service
RUN cargo build --release

# Runtime stage
FROM alpine:3.18 AS rest_service

WORKDIR /usr/local/bin

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rest_service .

# Set the startup command
CMD ["./rest_service"]

