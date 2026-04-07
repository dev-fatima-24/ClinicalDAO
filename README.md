#  ClinicalDAO

> Decentralized Clinical Trial Funding on Stellar

ClinicalDAO is an open-source platform where researchers post clinical trial proposals, token holders vote on funding via DAO governance, and Soroban smart contracts enforce milestone-based fund releases. Trial participants are compensated directly in XLM or stablecoins — no intermediaries.

---

## Architecture

```
clinicaldao/
├── contracts/                  # Rust — Soroban smart contracts
│   ├── governance/             # DAO voting & proposal lifecycle
│   ├── escrow/                 # Milestone-gated fund releases
│   ├── token/                  # Governance token (vote weight)
│   └── identity/               # SEP-10 researcher identity hook
│
├── api/                        # Python — FastAPI backend
│   ├── routes/                 # proposals, votes, milestones, participants
│   ├── stellar/                # Horizon client, tx submission, event listener
│   └── models/                 # Pydantic schemas
│
├── frontend/                   # JavaScript — React app
│   ├── components/             # ProposalBoard, VotingDashboard, MilestonTracker
│   ├── hooks/                  # useStellarWallet, useGovernance, useEscrow
│   └── utils/                  # SEP-10 auth, Stellar SDK helpers
│
├── scripts/                    # Shell — deployment & dev tooling
│   ├── deploy.sh               # Compile + deploy contracts to testnet
│   └── seed.sh                 # Seed proposals, votes, wallets
│
└── Dockerfile                  # API + Soroban CLI container
```

---

## Tech Stack

| Layer | Technology |
|---|---|
| Smart Contracts | Rust · Soroban SDK |
| Backend | Python · FastAPI · Stellar SDK |
| Frontend | JavaScript · React · Stellar SDK JS |
| Auth | SEP-10 (Stellar Web Authentication) |
| Infrastructure | Docker · Shell |
| Network | Stellar Testnet → Mainnet |

---

## Key Features

**DAO Governance** — Researchers submit proposals on-chain. Governance token holders vote; proposals pass at a configurable M-of-N threshold.

**Milestone Escrow** — Trial funding is locked in a Soroban escrow contract and released incrementally as milestones are verified and approved by the DAO.

**SEP-10 Identity** — Researcher wallets are authenticated via the Stellar SEP-10 standard before any proposal can be submitted.

**Direct Participant Payouts** — Trial participants receive compensation in native XLM or USDC directly to their Stellar wallets via contract-triggered transfers.

---

## Prerequisites

- [Rust](https://rustup.rs/) + `wasm32-unknown-unknown` target
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)
- Python 3.11+
- Node.js 18+
- Docker & Docker Compose

---

## Quick Start

```bash
# 1. Clone the repo
git clone https://github.com/your-org/clinicaldao.git
cd clinicaldao

# 2. Copy environment config
cp .env.example .env

# 3. Start local Stellar testnet + API
docker compose up -d

# 4. Deploy contracts to testnet
./scripts/deploy.sh

# 5. Seed test data
./scripts/seed.sh

# 6. Start the frontend
cd frontend && npm install && npm run dev
```

---

## Environment Variables

```env
STELLAR_NETWORK=testnet
HORIZON_URL=https://horizon-testnet.stellar.org
SOROBAN_RPC_URL=https://soroban-testnet.stellar.org
GOVERNANCE_CONTRACT_ID=
ESCROW_CONTRACT_ID=
TOKEN_CONTRACT_ID=
ADMIN_SECRET_KEY=
API_PORT=8000
```

---

## Smart Contract Overview

| Contract | Responsibility |
|---|---|
| `governance` | Submit proposals, cast votes, finalize outcomes |
| `escrow` | Hold and release funds per milestone approval |
| `token` | Governance token issuance and vote weight |
| `identity` | SEP-10 hook for researcher wallet verification |

---

## API Endpoints

| Method | Endpoint | Description |
|---|---|---|
| GET/POST | `/proposals` | List or create trial proposals |
| GET/POST | `/votes` | Query or submit DAO votes |
| POST | `/milestones/{id}/release` | Trigger milestone fund release |
| GET | `/participants` | List participants and payout status |

---

## Governance Flow

```
Researcher (SEP-10 verified)
    │
    ▼
Submit Proposal ──► Governance Contract
                          │
                    Voting Period (configurable)
                          │
                    Threshold Met?
                     Yes │        No
                         │         └──► Proposal Rejected, funds returned
                         ▼
                  Funds Locked in Escrow
                         │
                  Milestone 1 Approved
                         │
                  Partial Release ──► Researcher Wallet
                         │
                  Milestone N Approved
                         │
                  Final Release + Participant Payouts
```

---

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/your-feature`
3. Commit your changes: `git commit -m "feat: description"`
4. Push and open a Pull Request

Please follow the [Conventional Commits](https://www.conventionalcommits.org/) standard.

---

## License

MIT © ClinicalDAO Contributors
