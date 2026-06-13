# Multi-stage build for Rust backend + React frontend

# Stage 1: Build React frontend
FROM node:24-alpine AS frontend-builder
WORKDIR /app/client
COPY client/package*.json ./
# Vite and the build plugins are devDependencies, so install everything;
# only the static build output leaves this stage.
RUN npm ci
COPY client/ ./
RUN npm run build

# Stage 2: Build Rust backend
FROM rust:1.70-slim AS backend-builder
WORKDIR /app

# Install dependencies for SQLx and OpenSSL
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    make \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files and download dependencies
COPY Cargo.toml ./
COPY Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Copy source code and build
COPY src/ ./src/
RUN touch src/main.rs && cargo build --release

# Stage 3: Runtime
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy backend binary
COPY --from=backend-builder /app/target/release/steady-pocket ./

# Copy frontend build
RUN mkdir -p /app/client
COPY --from=frontend-builder /app/client/build ./client/build

# Create directory for SQLite database
RUN mkdir -p /app/data

# Set environment variables
ENV SERVER_ADDR=0.0.0.0:8081
ENV DATABASE_URL=sqlite:/app/data/steady-pocket.db

# Expose port
EXPOSE 8081

# Run the application
CMD ["./steady-pocket"]
