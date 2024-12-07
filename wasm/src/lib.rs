#![no_std]

extern crate alloc;

use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

multiversx_sc_wasm_adapter::endpoints! {
    swap_contract
    (
        init => init
        deposit_token => deposit_token
        swap_tokens => swap_tokens
        get_swap_rate => get_swap_rate
        get_balance => get_balance
        claim_rewards => claim_rewards
        remove_liquidity => remove_liquidity
        add_liquidity => add_liquidity
        set_paused => set_paused
        withdraw_protocol_fees => withdraw_protocol_fees
        isContractPaused => is_contract_paused
        callBack => callBack
    )
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}