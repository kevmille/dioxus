# Use a specific Rust version for stability
FROM rust:1 AS chef
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

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

# Create the final bundle folder. Bundle always executes in release mode with optimizations enabled
RUN dx bundle --platform web

FROM chef AS runtime
COPY --from=builder /app/target/dx/toki/release/web/ /usr/local/app
# Copy public directory files (robots.txt, sitemap.xml, etc.)
COPY --from=builder /app/public/ /usr/local/app/

# set our port and make sure to listen for all connections
ENV PORT=8080
ENV IP=0.0.0.0

# expose the port 8080
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]
