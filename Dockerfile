# ── Stage 1: Rust / Soroban contract builder ──────────────────────────────────
FROM rust:1.78-slim AS contract-builder

RUN apt-get update && apt-get install -y curl pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked soroban-cli --version 20.3.1

WORKDIR /contracts
COPY Cargo.toml Cargo.toml
COPY contracts/ contracts/
RUN cargo build --release --target wasm32-unknown-unknown

# ── Stage 2: Python API ────────────────────────────────────────────────────────
FROM python:3.11-slim AS api

WORKDIR /app

COPY api/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY api/ ./api/
COPY .env.example .env

# Copy compiled WASM artifacts for deployment scripts
COPY --from=contract-builder /contracts/target/wasm32-unknown-unknown/release/*.wasm /app/wasm/

EXPOSE 8000
CMD ["uvicorn", "api.main:app", "--host", "0.0.0.0", "--port", "8000"]
