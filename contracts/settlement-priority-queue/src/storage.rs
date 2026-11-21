//! Storage helpers for the settlement-priority-queue contract.

use soroban_sdk::{Address, BytesN, Env, Vec};

use crate::types::{InvoiceOwed, PayeeClass, StorageKey};

/// Ledgers below which an entry's TTL is extended (~7 days at 5 s/ledger).
const TTL_THRESHOLD: u32 = 120_960;
/// Minimum TTL extension in ledger units (~60 days at 5 s/ledger).
const TTL_EXTEND_TO: u32 = 1_036_800;

// ── Admin ──────────────────────────────────────────────────────────────────

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&StorageKey::Admin, admin);
    env.storage()
        .instance()
        .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
}

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage()
        .instance()
        .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
    env.storage().instance().get(&StorageKey::Admin)
}

// ── Priority order ─────────────────────────────────────────────────────────

pub fn set_priority(env: &Env, order: &Vec<PayeeClass>) {
    env.storage().instance().set(&StorageKey::Priority, order);
    env.storage()
        .instance()
        .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
}

pub fn get_priority(env: &Env) -> Option<Vec<PayeeClass>> {
    env.storage()
        .instance()
        .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
    env.storage().instance().get(&StorageKey::Priority)
}

// ── Invoice owed amounts ───────────────────────────────────────────────────

pub fn set_invoice(env: &Env, invoice_id: &BytesN<32>, owed: &InvoiceOwed) {
    let key = StorageKey::Distribution(invoice_id.clone());
    env.storage().persistent().set(&key, owed);
    env.storage()
        .persistent()
        .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
}

pub fn get_invoice(env: &Env, invoice_id: &BytesN<32>) -> Option<InvoiceOwed> {
    let key = StorageKey::Distribution(invoice_id.clone());
    let data = env.storage().persistent().get(&key);
    if data.is_some() {
        env.storage()
            .persistent()
            .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
    }
    data
}

pub fn has_invoice(env: &Env, invoice_id: &BytesN<32>) -> bool {
    env.storage()
        .persistent()
        .has(&StorageKey::Distribution(invoice_id.clone()))
}
