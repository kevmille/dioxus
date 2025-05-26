# Use a specific Rust version for stability
FROM rust:1.76-slim AS chef
RUN cargo install cargo-chef
WORKDIR /app

# ---------------------------
# Dependency Caching Stage
# ---------------------------
FROM chef AS planner
# Copy only dependency files first
COPY Cargo.toml Cargo.lock ./
# Generate recipe without full source copy
RUN cargo chef prepare --recipe-path recipe.json

# ---------------------------
# Build Stage
# ---------------------------
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Install dependencies (cached unless Cargo.toml/lock changes)
RUN cargo chef cook --release --recipe-path recipe.json

# Copy remaining source files AFTER dependencies are built
COPY src ./src
COPY assets ./assets
COPY .dockerignore ./

# Install dx-cli (cached in separate layer)
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
    && cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

# Build application (cached unless source changes)
RUN dx bundle --platform web --release

# ---------------------------
# Runtime Stage (Ultra-slim)
# ---------------------------
FROM alpine:3.19 AS runtime
RUN apk add --no-cache libgcc
WORKDIR /app

# Copy only built artifacts
COPY --from=builder /app/target/dx/toki/release/web/ /app
COPY --from=builder /app/assets ./assets

# Runtime config
ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

ENTRYPOINT ["/app/server"]
