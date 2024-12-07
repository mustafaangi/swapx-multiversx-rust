#!/bin/bash

# Default values
CONTRACT="contract-address"
PROXY="https://devnet-gateway.multiversx.com"

# Check if function name is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <function-name> [arguments...]"
    exit 1
fi

FUNCTION=$1
shift  # Remove function name from arguments

# Build query command with correct syntax
CMD="mxpy --verbose contract query $CONTRACT --function=\"$FUNCTION\" --proxy=\"$PROXY\""

# Add any additional arguments
if [ $# -gt 0 ]; then
    CMD="$CMD --arguments ${@}"
fi

# Execute query
echo "Executing: $CMD"
eval $CMD