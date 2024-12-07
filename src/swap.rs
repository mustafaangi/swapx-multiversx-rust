#![no_std]

use multiversx_sc::{
    require,
    storage::mappers::SingleValueMapper
};

const FEE_PERCENTAGE: u64 = 3; // 0.3% fee
const FEE_DENOMINATOR: u64 = 1000;
const LP_REWARD_PERCENTAGE: u64 = 2; // 0.2% of fees go to LP providers
const PROTOCOL_FEE_PERCENTAGE: u64 = 1; // 0.1% to protocol
const PRECISION: u64 = 1_000_000; // Precision for calculations

#[multiversx_sc::contract]
pub trait SwapContract {
    #[init]
    fn init(&self) {
        let caller = self.blockchain().get_caller();
        self.admin().set(&caller);
        self.paused().set(false);
    }

    #[only_owner]
    #[endpoint]
    fn set_paused(&self, paused: bool) {
        self.paused().set(paused);
    }

    #[only_owner]
    #[endpoint]
    fn withdraw_protocol_fees(&self, token_id: TokenIdentifier) {
        let caller = self.blockchain().get_caller();
        let fees = self.protocol_fees(&token_id).get();
        
        require!(fees > 0, "No fees to withdraw");
        
        self.send().direct_esdt(&caller, &token_id, 0, &fees);
        self.protocol_fees(&token_id).clear();
    }

    #[payable("*")]
    #[endpoint]
    fn add_liquidity(
        &self,
        token_a: TokenIdentifier,
        amount_a: BigUint,
        token_b: TokenIdentifier,
        amount_b: BigUint,
    ) {
        require!(token_a.is_valid_esdt_identifier(), "Invalid token A");
        require!(token_b.is_valid_esdt_identifier(), "Invalid token B");
        
        let caller = self.blockchain().get_caller();
        let amount_a_copy = amount_a.clone();
        
        // Transfer tokens to contract
        self.send().direct_esdt(&caller, &token_a, 0, &amount_a);
        self.send().direct_esdt(&caller, &token_b, 0, &amount_b);
        
        // Update liquidity pool
        self.token_balance(&token_a).update(|balance| *balance += amount_a);
        self.token_balance(&token_b).update(|balance| *balance += amount_b);
        
        // Update LP tokens
        self.update_lp_tokens(&caller, &amount_a_copy);
    }

    #[endpoint]
    fn deposit_token(
        &self,
        token_id: TokenIdentifier,
        amount: BigUint,
    ) {
        // Verify token is valid
        require!(token_id.is_valid_esdt_identifier(), "Invalid token ID");
        
        // Get caller address
        let caller = self.blockchain().get_caller();
        
        // Transfer tokens to contract
        self.send().direct_esdt(&caller, &token_id, 0, &amount);
        
        // Update balance in storage
        self.token_balance(&token_id).update(|balance| *balance += amount);
    }

    #[endpoint]
    fn swap_tokens(
        &self,
        token_in: TokenIdentifier,
        amount_in: BigUint,
        token_out: TokenIdentifier,
        min_amount_out: BigUint,
        slippage_rate: u64,  // New parameter
    ) {
        require!(!self.paused().get(), "Contract is paused");
        require!(slippage_rate <= 100, "Invalid slippage rate");

        // Verify tokens
        require!(token_in.is_valid_esdt_identifier(), "Invalid input token");
        require!(token_out.is_valid_esdt_identifier(), "Invalid output token");
        
        // Calculate fees
        let protocol_fee = amount_in.clone() * PROTOCOL_FEE_PERCENTAGE / FEE_DENOMINATOR;
        let lp_fee = amount_in.clone() * LP_REWARD_PERCENTAGE / FEE_DENOMINATOR;
        let amount_after_fees = &amount_in - &protocol_fee - &lp_fee;
        
        // Update protocol fees
        self.protocol_fees(&token_in).update(|fees| *fees += protocol_fee);

        // Calculate amounts with fees
        let fee_amount = amount_in.clone() * FEE_PERCENTAGE / FEE_DENOMINATOR;
        let amount_after_fee = &amount_in - &fee_amount;
        
        // Add slippage check
        let expected_rate = self.get_swap_rate(&token_in, &amount_after_fee);
        let slippage_amount = &expected_rate * slippage_rate / 100u64;
        let min_rate = &expected_rate - &slippage_amount;
        require!(min_rate >= min_amount_out, "Slippage too high");

        // Calculate output amount using constant product formula
        let rate = self.get_swap_rate(&token_in, &amount_after_fee);
        require!(rate >= min_amount_out, "Insufficient output amount");
        
        let caller = self.blockchain().get_caller();
        
        // Execute swap
        self.send().direct_esdt(&caller, &token_out, 0, &rate);
        
        // Update balances including fee
        self.token_balance(&token_in).update(|balance| *balance += amount_in);
        self.token_balance(&token_out).update(|balance| *balance -= rate);
        self.collected_fees(&token_in).update(|fees| *fees += fee_amount);
    }

    #[endpoint]
    fn claim_rewards(&self) {
        let caller = self.blockchain().get_caller();
        let caller_lp_tokens = self.lp_tokens(&caller).get();
        let total_lp_tokens = self.total_lp_tokens().get();
        
        require!(caller_lp_tokens > 0, "No LP tokens owned");
        
        // Calculate reward share based on LP tokens
        let rewards = self.calculate_rewards(&caller_lp_tokens, &total_lp_tokens);
        
        // Use EGLD token identifier
        let token = TokenIdentifier::from_esdt_bytes(b"EGLD");
        self.send().direct_esdt(&caller, &token, 0, &rewards);
        
        // Reset claimed rewards
        self.user_rewards(&caller).clear();
    }

    #[endpoint]
    fn remove_liquidity(
        &self,
        token_a: TokenIdentifier,
        token_b: TokenIdentifier,
        lp_amount: BigUint,
    ) {
        let caller = self.blockchain().get_caller();
        let caller_lp_tokens = self.lp_tokens(&caller).get();
        
        require!(caller_lp_tokens >= lp_amount, "Insufficient LP tokens");
        
        let lp_amount_copy = lp_amount.clone();
        
        // Calculate share first
        let share = lp_amount_copy.clone() * PRECISION / self.total_lp_tokens().get();
        let amount_a = self.token_balance(&token_a).get() * share.clone() / PRECISION;
        let amount_b = self.token_balance(&token_b).get() * share / PRECISION;
        
        // Update LP tokens
        self.lp_tokens(&caller).update(|balance| *balance -= lp_amount.clone());
        self.total_lp_tokens().update(|total| *total -= lp_amount);
        
        // Transfer tokens back to user
        self.send().direct_esdt(&caller, &token_a, 0, &amount_a);
        self.send().direct_esdt(&caller, &token_b, 0, &amount_b);
    }

    fn calculate_rewards(
        &self,
        user_lp_tokens: &BigUint,
        total_lp_tokens: &BigUint,
    ) -> BigUint {
        let total_fees = self.total_fees().get();
        let reward_share = user_lp_tokens * LP_REWARD_PERCENTAGE / total_lp_tokens;
        reward_share * total_fees / FEE_DENOMINATOR
    }

    #[view]
    fn get_swap_rate(
        &self,
        token_in: &TokenIdentifier,
        amount_in: &BigUint,
    ) -> BigUint {
        let balance_in = self.token_balance(token_in).get();
        let balance_out = self.token_balance(token_in).get();
        
        // Improved constant product formula
        require!(balance_in > 0 && balance_out > 0, "Insufficient liquidity");
        let k = balance_in.clone() * balance_out.clone();
        let new_balance_in = balance_in + amount_in;
        let new_balance_out = k / new_balance_in;
        balance_out - new_balance_out
    }

    #[view]
    fn get_balance(&self, token_id: &TokenIdentifier) -> BigUint {
        self.token_balance(token_id).get()
    }

    #[view(isContractPaused)]
    fn is_contract_paused(&self) -> bool {
        self.paused().get()
    }

    #[storage_mapper("paused")]
    fn paused(&self) -> SingleValueMapper<bool>;

    #[storage_mapper("admin")]
    fn admin(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("protocolFees")]
    fn protocol_fees(&self, token_id: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[storage_mapper("tokenBalance")]
    fn token_balance(&self, token_id: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[storage_mapper("collectedFees")]
    fn collected_fees(&self, token_id: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[storage_mapper("lpTokens")]
    fn lp_tokens(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("totalLpTokens")]
    fn total_lp_tokens(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("userRewards")]
    fn user_rewards(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("totalFees")]
    fn total_fees(&self) -> SingleValueMapper<BigUint>;

    // Helper function to update LP tokens
    fn update_lp_tokens(&self, address: &ManagedAddress, amount: &BigUint) {
        self.lp_tokens(address).update(|balance| *balance += amount);
        self.total_lp_tokens().update(|total| *total += amount);
    }
}