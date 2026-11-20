//! Storage helpers for the debtor exposure cap contract.

use soroban_sdk::{Address, BytesN, Env};

use crate::types::StorageKey;

/// Ledgers below which an entry's TTL is extended (~7 days at 5s/ledger).
pub const TTL_THRESHOLD: u32 = 120_960;
/// Minimum TTL extension in ledger units (~60 days at 5s/ledger).
pub const MIN_TTL_EXTEND: u32 = 1_036_800;

fn bump(env: &Env, key: &StorageKey) {
    env.storage()
        .persistent()
        .extend_ttl(key, TTL_THRESHOLD, MIN_TTL_EXTEND);
}

/// Load the admin from instance storage.
pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage()
        .instance()
        .extend_ttl(TTL_THRESHOLD, MIN_TTL_EXTEND);
    env.storage().instance().get(&StorageKey::Admin)
}

/// Store the admin in instance storage.
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&StorageKey::Admin, admin);
    env.storage()
        .instance()
        .extend_ttl(TTL_THRESHOLD, MIN_TTL_EXTEND);
}

/// Load the default cap applied to debtors without an override.
pub fn get_default_cap(env: &Env) -> i128 {
    env.storage()
        .instance()
        .extend_ttl(TTL_THRESHOLD, MIN_TTL_EXTEND);
    env.storage()
        .instance()
        .get(&StorageKey::DefaultCap)
        .unwrap_or(0)
}

/// Store the default cap in instance storage.
pub fn set_default_cap(env: &Env, cap: i128) {
    env.storage().instance().set(&StorageKey::DefaultCap, &cap);
    env.storage()
        .instance()
        .extend_ttl(TTL_THRESHOLD, MIN_TTL_EXTEND);
}

/// Load the cap override of a debtor, if it has one.
pub fn get_debtor_cap(env: &Env, debtor_id: &BytesN<32>) -> Option<i128> {
    let key = StorageKey::DebtorCap(debtor_id.clone());
    let value = env.storage().persistent().get(&key);
    if value.is_some() {
        bump(env, &key);
    }
    value
}

/// Store a cap override for a debtor.
pub fn set_debtor_cap(env: &Env, debtor_id: &BytesN<32>, cap: i128) {
    let key = StorageKey::DebtorCap(debtor_id.clone());
    env.storage().persistent().set(&key, &cap);
    bump(env, &key);
}

/// Load the outstanding funded amount of a debtor.
pub fn get_outstanding(env: &Env, debtor_id: &BytesN<32>) -> i128 {
    let key = StorageKey::Outstanding(debtor_id.clone());
    let value = env.storage().persistent().get(&key).unwrap_or(0);
    if env.storage().persistent().has(&key) {
        bump(env, &key);
    }
    value
}

/// Store the outstanding funded amount of a debtor.
pub fn set_outstanding(env: &Env, debtor_id: &BytesN<32>, value: i128) {
    let key = StorageKey::Outstanding(debtor_id.clone());
    env.storage().persistent().set(&key, &value);
    bump(env, &key);
}

/// Load the debtor an invoice belongs to, if the invoice was listed.
pub fn get_invoice_debtor(env: &Env, invoice_id: &BytesN<32>) -> Option<BytesN<32>> {
    let key = StorageKey::InvoiceDebtor(invoice_id.clone());
    let value = env.storage().persistent().get(&key);
    if value.is_some() {
        bump(env, &key);
    }
    value
}

/// Store the debtor an invoice belongs to.
pub fn set_invoice_debtor(env: &Env, invoice_id: &BytesN<32>, debtor_id: &BytesN<32>) {
    let key = StorageKey::InvoiceDebtor(invoice_id.clone());
    env.storage().persistent().set(&key, debtor_id);
    bump(env, &key);
}
