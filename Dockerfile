FROM rust:latest AS builder

# Set working directory
WORKDIR /app

# Copy Cargo.lock and Cargo.toml for dependency resolution
COPY Cargo.lock ./
COPY Cargo.toml ./

# Install dependencies and cache them in the builder stage
RUN cargo install --locked --path .

# Copy source code (excluding Cargo files)
COPY --from=source . .

# Exclude Cargo files from the final image
EXCLUDE .git .cargo build.rs dev-dependencies.toml target.toml Cargo.lock Cargo.toml

# Build the application in the final stage
FROM rust:latest

WORKDIR /app

# Copy the excluded files and build the application
COPY --from=builder --exclude=$EXCLUDE . .
RUN cargo build --release

# Final image stage
FROM scratch

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/chat-core ./

# Set the entrypoint
ENTRYPOINT ["/app/chat-core"]
