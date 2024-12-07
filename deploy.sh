#!/bin/bash

# Exit on error
set -e 

echo "Starting deployment process..."

# Configuration
PROXY="https://devnet-gateway.multiversx.com"
MIN_EGLD="0.1" # Minimum EGLD required for deployment

# Check wallet balance
check_balance() {
    echo "Checking wallet balance..."
    
    # Convert PEM to address
    WALLET=$(mxpy wallet convert walletKey.pem --in-format=pem --out-format=address-bech32)
    
    if [ -z "$WALLET" ]; then
        echo "Error: Could not extract wallet address from PEM file"
        echo "Please ensure walletKey.pem exists and is valid"
        exit 1
    fi
    
    echo "Wallet address: $WALLET"
    
    # Query balance
    BALANCE=$(mxpy wallet balance "$WALLET" --proxy="$PROXY" || echo "0")
    
    if [ -z "$BALANCE" ] || [ "$BALANCE" = "0" ]; then
        echo "Error: Could not query wallet balance"
        echo "Please ensure you have funds at: $WALLET"
        echo "You can get test EGLD from: https://r3d4.fr/faucet"
        exit 1
    fi
    
    echo "Wallet balance: $BALANCE EGLD"
}

# Check balance before proceeding
check_balance

# Function to clean directory with proper permissions
clean_directory() {
    if [ -d "$1" ]; then
        echo "Cleaning directory: $1"
        sudo rm -rf "$1" || {
            echo "Failed to remove directory: $1"
            echo "Attempting without sudo..."
            rm -rf "$1" || {
                echo "Warning: Could not clean $1"
            }
        }
    fi
}

# Clean directories
echo "Cleaning build directories..."
clean_directory "target"
clean_directory "wasm/target"
clean_directory "output"

# Create output directory
echo "Creating output directory..."
mkdir -p output

# Build contract
echo "Building contract..."
cargo build --release || {
    echo "Contract build failed"
    exit 1
}

# Build wasm
echo "Building WASM..."
cd wasm
cargo build --release --target wasm32-unknown-unknown || {
    echo "WASM build failed"
    exit 1
}
cd ..

# Copy wasm file
echo "Copying WASM file..."
cp wasm/target/wasm32-unknown-unknown/release/swap_contract_wasm.wasm output/swap-contract.wasm || {
    echo "Failed to copy WASM file"
    exit 1
}

# Deploy contract and save address
echo "Deploying contract..."
DEPLOY_OUTPUT=$(mxpy contract deploy \
    --bytecode=output/swap-contract.wasm \
    --pem=walletKey.pem \
    --proxy=$PROXY \
    --chain=D \
    --gas-limit=60000000 \
    --recall-nonce \
    --send)

# Extract and save contract address
CONTRACT_ADDRESS=$(echo "$DEPLOY_OUTPUT" | grep -o 'erd1[a-zA-Z0-9]\{58\}')
echo "Contract deployed at: $CONTRACT_ADDRESS"
echo "$CONTRACT_ADDRESS" > contract_address.txt

# Wait for deployment to be processed
echo "Waiting for deployment to be processed..."
sleep 10

# Verify deployment
echo "Verifying deployment..."
mxpy contract query "$CONTRACT_ADDRESS" \
    --function="isContractPaused" \
    --proxy="https://devnet-gateway.multiversx.com"

echo "Deployment completed successfully!"

