use soroban_sdk::contracterror;

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    /// Contract has already been initialized.
    AlreadyInit = 1,
    /// Contract has not been initialized yet.
    NotInit = 2,
    /// Caller is not the admin.
    Unauthorized = 3,
    /// An address argument is the zero / invalid address.
    InvalidAddress = 4,
    /// The priority order is missing one or more PayeeClass variants.
    MissingPayeeClass = 5,
    /// The priority order contains duplicate PayeeClass variants.
    DuplicatePayeeClass = 6,
    /// The order vec does not contain exactly PAYEE_CLASS_COUNT entries.
    InvalidOrderLength = 7,
    /// No priority order has been configured yet.
    PriorityNotSet = 8,
    /// An invoice with this id has not been registered.
    InvoiceNotFound = 9,
    /// An invoice with this id is already registered.
    InvoiceExists = 10,
    /// Distribution amount must be greater than zero.
    InvalidAmount = 11,
    /// An owed amount for a payee class is negative.
    InvalidOwed = 12,
    /// A checked arithmetic operation overflowed.
    Overflow = 13,
    /// The invoice has already been distributed; call again is a no-op guard.
    AlreadyDistributed = 14,
}
