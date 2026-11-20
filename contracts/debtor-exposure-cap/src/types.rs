//! Data types for the debtor exposure cap contract.

use soroban_sdk::{contracttype, BytesN};

/// Storage key enum for instance and persistent storage.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
    /// Instance: admin allowed to manage the caps.
    Admin,
    /// Instance: fallback cap applied to debtors without an override.
    DefaultCap,
    /// Persistent: per-debtor cap override, keyed by the hashed debtor id.
    DebtorCap(BytesN<32>),
    /// Persistent: outstanding funded amount of a debtor.
    Outstanding(BytesN<32>),
    /// Persistent: debtor of an invoice, keyed by the invoice id.
    InvoiceDebtor(BytesN<32>),
}
