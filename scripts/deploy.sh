
#!/bin/bash

# Build contract
cargo build --release

# Build WASM
cd wasm
cargo build --target wasm32-unknown-unknown --release
cd ..

# Copy WASM file
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/swap_contract_wasm.wasm output/swap-contract.wasm

# Deploy contract
mxpy contract deploy \
    --bytecode=output/swap-contract.wasm \
    --pem=walletKey.pem \
    --proxy=https://devnet-gateway.multiversx.com \
    --chain=D \
    --gas-limit=60000000 \
    --recall-nonce