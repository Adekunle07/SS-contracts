use soroban_sdk::contracterror;

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    /// Contract has already been initialized.
    AlreadyInit = 1,
    /// Contract has not been initialized yet.
    NotInit = 2,
    /// Caller is not the authorized party for this operation.
    Unauthorized = 3,
    /// An address argument is the zero / invalid address.
    InvalidAddress = 4,
    /// `start_rate_bps` must be > 0 and ≤ `max_rate_bps`.
    InvalidRateRange = 5,
    /// `step_bps` must be > 0.
    InvalidStep = 6,
    /// `step_ledgers` must be > 0.
    InvalidStepLedgers = 7,
    /// `face_value` must be > 0.
    InvalidFaceValue = 8,
    /// An auction for this invoice_id already exists.
    AuctionExists = 9,
    /// No auction found for this invoice_id.
    AuctionNotFound = 10,
    /// The auction is not in the Open state.
    AuctionNotOpen = 11,
    /// Commitment amount must be > 0.
    InvalidAmount = 12,
    /// Commitment would exceed the remaining funding needed.
    ExceedsFaceValue = 13,
    /// Cannot cancel an auction that already has commitments.
    HasCommitments = 14,
    /// A checked arithmetic operation overflowed.
    Overflow = 15,
    /// Auction has expired at the max rate; use `close_expired` to settle.
    AuctionExpired = 16,
}
