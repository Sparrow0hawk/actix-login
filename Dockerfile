FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .
# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release --bin actix_login && mv ./target/release/actix_login ./actix_login

# Runtime image
FROM debian:bookworm-slim as main

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app
COPY ./static /app/static
# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/actix_login /app/actix_login

# Run the app
CMD ./actix_login --port 8080 --host 0.0.0.0
