//! Data types for the settlement-priority-queue contract.

use soroban_sdk::{contracttype, BytesN, Vec};

/// Storage key enum for instance and persistent storage.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
    /// Instance: registry admin.
    Admin,
    /// Instance: the ordered list of payee classes used by every distribution.
    Priority,
    /// Persistent: per-invoice distribution record keyed by invoice id.
    Distribution(BytesN<32>),
}

/// The five payee classes that may participate in a distribution.
///
/// The order of this enum's discriminants has no semantic meaning — priority
/// is determined entirely by the Vec<PayeeClass> stored via `set_priority`.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum PayeeClass {
    /// Platform operating fee taken first.
    PlatformFee = 0,
    /// Insurance / loss-reserve pool.
    InsurancePool = 1,
    /// Senior tranche investors (lowest risk, paid ahead of junior).
    SeniorInvestors = 2,
    /// Junior tranche investors (higher yield, bear shortfall first).
    JuniorInvestors = 3,
    /// Remaining proceeds returned to the invoice issuer.
    IssuerResidual = 4,
}

/// Total number of distinct PayeeClass variants. Used for validation.
pub const PAYEE_CLASS_COUNT: u32 = 5;

/// The amount that each PayeeClass is owed for a given invoice distribution.
/// All five fields must be set when registering an invoice.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvoiceOwed {
    pub platform_fee: i128,
    pub insurance_pool: i128,
    pub senior_investors: i128,
    pub junior_investors: i128,
    pub issuer_residual: i128,
}

impl InvoiceOwed {
    /// Total amount owed across all classes.
    pub fn total(&self) -> Option<i128> {
        self.platform_fee
            .checked_add(self.insurance_pool)?
            .checked_add(self.senior_investors)?
            .checked_add(self.junior_investors)?
            .checked_add(self.issuer_residual)
    }

    /// Amount owed to `class`.
    pub fn owed_for(&self, class: PayeeClass) -> i128 {
        match class {
            PayeeClass::PlatformFee => self.platform_fee,
            PayeeClass::InsurancePool => self.insurance_pool,
            PayeeClass::SeniorInvestors => self.senior_investors,
            PayeeClass::JuniorInvestors => self.junior_investors,
            PayeeClass::IssuerResidual => self.issuer_residual,
        }
    }
}

/// Per-class allocation produced by a distribution run.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassAllocation {
    pub class: PayeeClass,
    /// Amount paid to this class.
    pub amount: i128,
    /// Shortfall: how much of `owed` could not be covered (0 when fully paid).
    pub shortfall: i128,
}

/// Full result of a `distribute_by_priority` or `preview_distribution` call.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributionResult {
    /// Per-class breakdown in priority order.
    pub allocations: Vec<ClassAllocation>,
    /// Sum of all `amount` fields; must equal the distributed `amount`.
    pub total_paid: i128,
}
