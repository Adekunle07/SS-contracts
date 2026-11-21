use soroban_sdk::contracterror;

/// Errors that can occur while enforcing the per-debtor exposure cap.
#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    /// Contract has already been initialized.
    AlreadyInitialized = 1,
    /// Contract has not been initialized.
    NotInitialized = 2,
    /// The caller is not the configured admin.
    Unauthorized = 3,
    /// The amount is not a positive number.
    InvalidAmount = 4,
    /// The cap is not a positive number.
    InvalidCap = 5,
    /// The invoice is not registered with the contract.
    InvoiceNotFound = 6,
    /// The invoice has already been listed.
    InvoiceAlreadyListed = 7,
    /// The operation would raise the outstanding amount above the cap.
    DebtorCapExceeded = 8,
    /// The outstanding amount or a projected total overflowed.
    ArithmeticOverflow = 9,
}
