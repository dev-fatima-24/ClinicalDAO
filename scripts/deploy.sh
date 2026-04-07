#!/usr/bin/env bash
set -euo pipefail

source .env

NETWORK_ARGS="--network testnet --rpc-url ${SOROBAN_RPC_URL} --source ${ADMIN_SECRET_KEY}"

echo "==> Building contracts..."
cargo build --release --target wasm32-unknown-unknown

WASM_DIR="target/wasm32-unknown-unknown/release"

deploy() {
  local name=$1
  local wasm="${WASM_DIR}/${name}.wasm"
  echo "  Deploying ${name}..."
  soroban contract deploy ${NETWORK_ARGS} --wasm "${wasm}"
}

TOKEN_ID=$(deploy token)
echo "TOKEN_CONTRACT_ID=${TOKEN_ID}"

IDENTITY_ID=$(deploy identity)
echo "IDENTITY_CONTRACT_ID=${IDENTITY_ID}"

GOVERNANCE_ID=$(deploy governance)
echo "GOVERNANCE_CONTRACT_ID=${GOVERNANCE_ID}"

ESCROW_ID=$(deploy escrow)
echo "ESCROW_CONTRACT_ID=${ESCROW_ID}"

# Initialize contracts
ADMIN_PUB=$(soroban keys address ${ADMIN_SECRET_KEY} 2>/dev/null || \
  stellar keys address --secret-key ${ADMIN_SECRET_KEY})

echo "==> Initializing token..."
soroban contract invoke ${NETWORK_ARGS} --id "${TOKEN_ID}" \
  -- initialize --admin "${ADMIN_PUB}" --name "ClinicalDAO" --symbol "CDAO"

echo "==> Initializing identity..."
soroban contract invoke ${NETWORK_ARGS} --id "${IDENTITY_ID}" \
  -- initialize --admin "${ADMIN_PUB}"

echo "==> Initializing governance..."
soroban contract invoke ${NETWORK_ARGS} --id "${GOVERNANCE_ID}" \
  -- initialize --admin "${ADMIN_PUB}" --token-contract "${TOKEN_ID}"

echo "==> Initializing escrow..."
soroban contract invoke ${NETWORK_ARGS} --id "${ESCROW_ID}" \
  -- initialize --admin "${ADMIN_PUB}"

# Write contract IDs back to .env
sed -i "s/^GOVERNANCE_CONTRACT_ID=.*/GOVERNANCE_CONTRACT_ID=${GOVERNANCE_ID}/" .env
sed -i "s/^ESCROW_CONTRACT_ID=.*/ESCROW_CONTRACT_ID=${ESCROW_ID}/" .env
sed -i "s/^TOKEN_CONTRACT_ID=.*/TOKEN_CONTRACT_ID=${TOKEN_ID}/" .env

echo ""
echo "✅ Deployment complete."
echo "   Governance : ${GOVERNANCE_ID}"
echo "   Escrow     : ${ESCROW_ID}"
echo "   Token      : ${TOKEN_ID}"
echo "   Identity   : ${IDENTITY_ID}"
