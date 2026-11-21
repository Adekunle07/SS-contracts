#![allow(deprecated)]
//! Events emitted by the platform treasury management contract.

use soroban_sdk::{Address, Env, String, Symbol};

/// Emitted when the contract is initialized.
pub fn initialized(env: &Env, admin: &Address) {
    let topics = (Symbol::new(env, "initialized"),);
    env.events().publish(topics, admin.clone());
}

/// Emitted each time a fee source contributes funds.
/// Topics: (FeeReceived, source); Data: amount.
pub fn fee_received(env: &Env, source: &Address, amount: i128) {
    let topics = (Symbol::new(env, "FeeReceived"), source.clone());
    env.events().publish(topics, amount);
}

/// Emitted each time the admin allocates funds out of the treasury.
/// Topics: (FundsAllocated, destination); Data: (amount, purpose).
pub fn funds_allocated(env: &Env, destination: &Address, amount: i128, purpose: &String) {
    let topics = (Symbol::new(env, "FundsAllocated"), destination.clone());
    env.events().publish(topics, (amount, purpose.clone()));
}

pub fn allocation_cooldown_started(
    env: &Env,
    ledger: u32,
    cooldown_ledgers: u32,
    unlock_ledger: u32,
) {
    let topics = (Symbol::new(env, "allocation_cooldown_started"),);
    env.events()
        .publish(topics, (ledger, cooldown_ledgers, unlock_ledger));
}
