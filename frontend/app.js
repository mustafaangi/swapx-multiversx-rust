// Replace this with config.js import
const config = {
    contractAddress: process.env.CONTRACT_ADDRESS || "erd1...",
    proxyUrl: process.env.PROXY_URL || "https://devnet-gateway.multiversx.com"
};

class SwapApp {
    constructor() {
        this.provider = new MultiversX.ProxyProvider(config.proxyUrl);
        this.account = null;
        this.setupEventListeners();
    }

    async connectWallet() {
        try {
            const chainProvider = await MultiversX.ExtensionProvider.init();
            await chainProvider.login();
            this.account = await chainProvider.getAddress();
            document.getElementById('wallet-info').textContent = 
                `Connected: ${this.account.substring(0, 6)}...${this.account.substring(this.account.length - 4)}`;
            document.getElementById('swap-button').disabled = false;
        } catch (err) {
            console.error('Failed to connect wallet:', err);
        }
    }

    async calculateSwapRate() {
        const tokenIn = document.getElementById('token-in').value;
        const amountIn = document.getElementById('amount-in').value;
        
        if (!tokenIn || !amountIn) return;

        try {
            const tx = {
                value: "0",
                data: `getSwapRate@${tokenIn}@${amountIn}`,
                receiver: contractAddress,
                gasLimit: 5000000
            };
            
            const result = await this.provider.queryContract(tx);
            document.getElementById('swap-rate').textContent = result;
        } catch (err) {
            console.error('Failed to get swap rate:', err);
        }
    }

    async performSwap() {
        const tokenIn = document.getElementById('token-in').value;
        const amountIn = document.getElementById('amount-in').value;
        const tokenOut = document.getElementById('token-out').value;
        const minAmountOut = document.getElementById('amount-out').value;
        const slippage = document.getElementById('slippage-rate').value;

        try {
            // Add loading state
            document.getElementById('swap-button').disabled = true;
            document.getElementById('swap-button').textContent = 'Processing...';

            const tx = {
                value: "0",
                data: `swapTokens@${tokenIn}@${amountIn}@${tokenOut}@${minAmountOut}@${slippage}`,
                receiver: contractAddress,
                gasLimit: 10000000
            };
            
            const txHash = await this.provider.sendTransaction(tx);
            
            // Show success message
            this.showNotification('Swap successful!', 'success');
            
            // Refresh balances
            await this.updateBalances();
        } catch (err) {
            this.showNotification('Swap failed: ' + err.message, 'error');
        } finally {
            document.getElementById('swap-button').disabled = false;
            document.getElementById('swap-button').textContent = 'Swap';
        }
    }

    showNotification(message, type) {
        const notification = document.createElement('div');
        notification.className = `notification ${type}`;
        notification.textContent = message;
        document.body.appendChild(notification);
        setTimeout(() => notification.remove(), 3000);
    }

    async addLiquidity() {
        const tokenA = document.getElementById('token-a').value;
        const amountA = document.getElementById('amount-a').value;
        const tokenB = document.getElementById('token-b').value;
        const amountB = document.getElementById('amount-b').value;

        try {
            const tx = {
                value: "0",
                data: `addLiquidity@${tokenA}@${amountA}@${tokenB}@${amountB}`,
                receiver: contractAddress,
                gasLimit: 10000000
            };
            
            await this.provider.sendTransaction(tx);
        } catch (err) {
            console.error('Failed to add liquidity:', err);
        }
    }

    async claimRewards() {
        try {
            const tx = {
                value: "0",
                data: "claimRewards",
                receiver: contractAddress,
                gasLimit: 6000000
            };
            
            await this.provider.sendTransaction(tx);
        } catch (err) {
            console.error('Failed to claim rewards:', err);
        }
    }

    async removeLiquidity() {
        const tokenA = document.getElementById('remove-token-a').value;
        const tokenB = document.getElementById('remove-token-b').value;
        const lpAmount = document.getElementById('lp-amount').value;

        try {
            const tx = {
                value: "0",
                data: `removeLiquidity@${tokenA}@${tokenB}@${lpAmount}`,
                receiver: contractAddress,
                gasLimit: 8000000
            };
            
            await this.provider.sendTransaction(tx);
        } catch (err) {
            console.error('Failed to remove liquidity:', err);
        }
    }

    async checkContractStatus() {
        try {
            const tx = {
                value: "0",
                data: "isContractPaused",
                receiver: contractAddress,
                gasLimit: 5000000
            };
            
            const isPaused = await this.provider.queryContract(tx);
            if (isPaused) {
                document.getElementById('swap-button').disabled = true;
                document.getElementById('status-message').textContent = "Contract is paused";
            }
        } catch (err) {
            console.error('Failed to check contract status:', err);
        }
    }

    async withdrawProtocolFees(tokenId) {
        try {
            const tx = {
                value: "0",
                data: `withdrawProtocolFees@${tokenId}`,
                receiver: contractAddress,
                gasLimit: 6000000
            };
            
            await this.provider.sendTransaction(tx);
        } catch (err) {
            console.error('Failed to withdraw fees:', err);
        }
    }

    setupEventListeners() {
        document.getElementById('connect-wallet').addEventListener('click', () => this.connectWallet());
        document.getElementById('swap-button').addEventListener('click', () => this.performSwap());
        document.getElementById('amount-in').addEventListener('input', () => this.calculateSwapRate());
        document.getElementById('add-liquidity-button').addEventListener('click', () => this.addLiquidity());
        document.getElementById('claim-rewards-button').addEventListener('click', () => this.claimRewards());
        document.getElementById('remove-liquidity-button').addEventListener('click', () => this.removeLiquidity());
        this.checkContractStatus();
        setInterval(() => this.checkContractStatus(), 30000); // Check every 30 seconds
    }
}

window.onload = () => {
    window.app = new SwapApp();
};