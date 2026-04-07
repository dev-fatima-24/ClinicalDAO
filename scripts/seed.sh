#!/usr/bin/env bash
set -euo pipefail

source .env

NETWORK_ARGS="--network testnet --rpc-url ${SOROBAN_RPC_URL} --source ${ADMIN_SECRET_KEY}"
ADMIN_PUB=$(soroban keys address ${ADMIN_SECRET_KEY} 2>/dev/null || \
  stellar keys address --secret-key ${ADMIN_SECRET_KEY})

echo "==> Generating test wallets..."
RESEARCHER=$(soroban keys generate researcher --no-fund 2>/dev/null && \
  soroban keys address researcher)
VOTER1=$(soroban keys generate voter1 --no-fund 2>/dev/null && \
  soroban keys address voter1)
VOTER2=$(soroban keys generate voter2 --no-fund 2>/dev/null && \
  soroban keys address voter2)
PARTICIPANT=$(soroban keys generate participant --no-fund 2>/dev/null && \
  soroban keys address participant)

echo "  Researcher : ${RESEARCHER}"
echo "  Voter1     : ${VOTER1}"
echo "  Voter2     : ${VOTER2}"
echo "  Participant: ${PARTICIPANT}"

echo "==> Funding wallets via Friendbot..."
for addr in "${RESEARCHER}" "${VOTER1}" "${VOTER2}" "${PARTICIPANT}"; do
  curl -s "https://friendbot.stellar.org?addr=${addr}" > /dev/null
done

echo "==> Minting governance tokens..."
soroban contract invoke ${NETWORK_ARGS} --id "${TOKEN_CONTRACT_ID}" \
  -- mint --to "${VOTER1}" --amount 1000
soroban contract invoke ${NETWORK_ARGS} --id "${TOKEN_CONTRACT_ID}" \
  -- mint --to "${VOTER2}" --amount 500

echo "==> Registering researcher identity..."
soroban contract invoke ${NETWORK_ARGS} --id "${IDENTITY_CONTRACT_ID:-}" \
  -- register --researcher "${RESEARCHER}" 2>/dev/null || true

echo "==> Submitting seed proposal..."
soroban contract invoke ${NETWORK_ARGS} --id "${GOVERNANCE_CONTRACT_ID}" \
  --source researcher \
  -- submit_proposal \
    --researcher "${RESEARCHER}" \
    --title "Phase II Oncology Trial" \
    --ipfs-cid "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi" \
    --funding-amount 10000000000 \
    --milestone-count 3 \
    --voting-period-secs 3600 \
    --threshold-numerator 2 \
    --threshold-denominator 3

echo ""
echo "✅ Seed complete."
