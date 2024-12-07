#!/bin/bash

# Clean previous builds
rm -rf target
rm -rf wasm/target
rm -rf output

# Create output directory
mkdir -p output

# Build main contract
cargo build --release

# Build and copy wasm
cd wasm
cargo build --target wasm32-unknown-unknown --release
cd ..

# Create output directory and copy wasm
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
