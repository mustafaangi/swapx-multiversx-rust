```markdown


# MultiversX Token Swap Protocol

A decentralized token swap protocol built on MultiversX blockchain with automated market maker (AMM) functionality, liquidity provision, and fee distribution system.

## Features

- Automated Market Maker (AMM)
- Liquidity Pool Management
- Fee Distribution System:
  - Protocol fees (0.1%)
  - LP rewards (0.2%)
- Slippage Protection
- Admin Controls

## Prerequisites

- Rust (latest stable)
- MultiversX SDK
- NodeJS & npm
- MultiversX wallet (devnet)

## Quick Start

1. Clone and install dependencies:
```bash
git clone <your-repo-url>
cd multiverse-final
cargo build
```

2. Set up environment:
```bash
cp .env.example .env    # Configure environment variables
cp walletKey.pem.example walletKey.pem    # Add your wallet key
```

3. Deploy contract:
```bash
./deploy.sh    # Deploys to devnet
```

4. Start frontend:
```bash
cd frontend
npm install
npm start
```

## Smart Contract Functions

### User Functions
- `swap_tokens(token_in, amount_in, token_out, min_amount_out, slippage_rate)`
- `add_liquidity(token_a, amount_a, token_b, amount_b)`
- `remove_liquidity(token_a, token_b, lp_amount)`
- `claim_rewards()`

### View Functions
- `get_swap_rate(token_in, amount_in)`
- `get_balance(token_id)`
- `isContractPaused()`

### Admin Functions
- `set_paused(paused)`
- `withdraw_protocol_fees(token_id)`

## Development

### Build Contract
```bash
cargo build --release
cd wasm && cargo build --target wasm32-unknown-unknown --release
```

### Testing
```bash
cargo test
```

### Query Contract
```bash
./scripts/query.sh isContractPaused
./scripts/query.sh getBalance token_id
```

## Security

- Private keys are never committed to git
- Use .env for sensitive data
- Environment-specific configurations
- Gas limits set per operation
- Slippage protection implemented

## Environment Variables

```env
WALLET_PEM_FILE=walletKey.pem
PROXY_URL=https://devnet-gateway.multiversx.com
CHAIN=D
GAS_LIMIT=60000000
MIN_EGLD=0.1
```

## File Structure
```
├── src/
│   └── swap.rs            # Main contract logic
├── wasm/
│   └── src/lib.rs         # WASM bindings
├── frontend/
│   ├── app.js
│   ├── index.html
│   └── styles.css
├── scripts/
│   ├── deploy.sh
│   └── query.sh
└── README.md
```

## Contributing

1. Fork repository
2. Create feature branch
3. Commit changes
4. Create pull request

