//! Storage helpers for the rate-auction contract.

use soroban_sdk::{Address, BytesN, Env, Vec};

use crate::types::{Auction, Commitment, StorageKey};

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

// ── Auction ────────────────────────────────────────────────────────────────

pub fn set_auction(env: &Env, auction: &Auction) {
    let key = StorageKey::Auction(auction.invoice_id.clone());
    env.storage().persistent().set(&key, auction);
    env.storage()
        .persistent()
        .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
}

pub fn get_auction(env: &Env, invoice_id: &BytesN<32>) -> Option<Auction> {
    let key = StorageKey::Auction(invoice_id.clone());
    let data = env.storage().persistent().get(&key);
    if data.is_some() {
        env.storage()
            .persistent()
            .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
    }
    data
}

pub fn has_auction(env: &Env, invoice_id: &BytesN<32>) -> bool {
    env.storage()
        .persistent()
        .has(&StorageKey::Auction(invoice_id.clone()))
}

// ── Commitments ────────────────────────────────────────────────────────────

pub fn get_commitments(env: &Env, invoice_id: &BytesN<32>) -> Vec<Commitment> {
    let key = StorageKey::Commitments(invoice_id.clone());
    let data: Option<Vec<Commitment>> = env.storage().persistent().get(&key);
    if let Some(ref v) = data {
        // Only bump TTL when the entry exists.
        let _ = v;
        env.storage()
            .persistent()
            .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
    }
    data.unwrap_or_else(|| Vec::new(env))
}

pub fn set_commitments(env: &Env, invoice_id: &BytesN<32>, commitments: &Vec<Commitment>) {
    let key = StorageKey::Commitments(invoice_id.clone());
    env.storage().persistent().set(&key, commitments);
    env.storage()
        .persistent()
        .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
}
