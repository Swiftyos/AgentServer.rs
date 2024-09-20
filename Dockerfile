# Build stage
FROM rust:1.81-alpine AS builder_base
WORKDIR /usr/src/app
RUN apk add --no-cache musl-dev

# Use ARG to determine the target architecture
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "amd64" ]; then \
        rustup target add x86_64-unknown-linux-musl; \
    elif [ "$TARGETARCH" = "arm64" ]; then \
        rustup target add aarch64-unknown-linux-musl; \
    fi

COPY . .

FROM builder_base AS builder
# Use the appropriate target based on the architecture
RUN if [ "$TARGETARCH" = "amd64" ]; then \
        cargo build --release --target x86_64-unknown-linux-musl && \
        strip target/x86_64-unknown-linux-musl/release/rustysrv; \
    elif [ "$TARGETARCH" = "arm64" ]; then \
        cargo build --release --target aarch64-unknown-linux-musl && \
        strip target/aarch64-unknown-linux-musl/release/rustysrv; \
    fi

# Runtime stage
FROM alpine:3.18
RUN apk add --no-cache ca-certificates
# Copy the binary from the appropriate location based on architecture
COPY --from=builder /usr/src/app/target/*/release/rustysrv /usr/local/bin/app
EXPOSE 3000
CMD ["app"]