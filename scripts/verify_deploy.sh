
#!/bin/bash

# Configuration
CONTRACT="erd1qqqqqqqqqqqqqpgqg7q8gekyl7ltg5exdv5ngc79khn6hjwh07kscx9dtc"
PROXY="https://devnet-gateway.multiversx.com"
MAX_RETRIES=10
SLEEP_TIME=5

# Function to check contract deployment
check_contract() {
    echo "Checking contract deployment..."
    
    for i in $(seq 1 $MAX_RETRIES); do
        echo "Attempt $i of $MAX_RETRIES..."
        
        # Query contract status
        result=$(mxpy contract query "$CONTRACT" \
            --function="isContractPaused" \
            --proxy="$PROXY" 2>&1)
        
        # Check if query was successful
        if [[ $result != *"invalid contract code"* ]]; then
            echo "Contract deployed successfully!"
            echo "Contract address: $CONTRACT"
            echo "Explorer URL: https://devnet-explorer.multiversx.com/accounts/$CONTRACT"
            exit 0
        fi
        
        echo "Contract not ready yet, waiting $SLEEP_TIME seconds..."
        sleep $SLEEP_TIME
    done
    
    echo "Contract deployment verification failed after $MAX_RETRIES attempts"
    exit 1
}

# Run verification
check_contract