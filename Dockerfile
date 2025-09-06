# Stage 1: Build the application
FROM rust:1.77-slim as builder
WORKDIR /usr/src/app

# Install sqlx-cli
RUN cargo install sqlx-cli

# Copy project files
COPY . .

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:bullseye-slim
WORKDIR /usr/src/app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/thai_nlem_api .
# Copy migrations for sqlx-cli (if you want to run migrations on deploy)
# COPY --from=builder /usr/src/app/migrations ./migrations
# COPY --from=builder /usr/local/bin/sqlx /usr/local/bin/sqlx

# Expose the port the app runs on
EXPOSE 3000

# Set the entrypoint
CMD ["./thai_nlem_api"]