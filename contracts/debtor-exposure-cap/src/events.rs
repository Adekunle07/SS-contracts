//! Event definitions for the debtor exposure cap contract.

use soroban_sdk::{BytesN, Env, Symbol};

/// Emitted when the admin sets a per-debtor cap override.
pub fn debtor_cap_set(env: &Env, debtor_id: &BytesN<32>, cap: i128) {
    env.events()
        .publish((Symbol::new(env, "debtor_cap_set"), debtor_id.clone()), cap);
}

/// Emitted when a funding brings a debtor's outstanding amount exactly to the
/// cap, i.e. the cap is now fully consumed.
///
/// The event is published on the successful path only: a funding that would
/// push the outstanding amount above the cap reverts with
/// `Error::DebtorCapExceeded`, and Soroban discards any event emitted by a
/// reverting call.
pub fn debtor_cap_hit(env: &Env, debtor_id: &BytesN<32>, outstanding: i128, cap: i128) {
    env.events().publish(
        (Symbol::new(env, "debtor_cap_hit"), debtor_id.clone()),
        (outstanding, cap),
    );
}
