//! Contract storage types shared across the ticketing contract.

use soroban_sdk::{contracttype, Address, BytesN, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TicketStatus {
    Valid,
    Used,
    Revoked,
    Resale,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    pub organizer: Address,
    pub name: String,
    /// Category such as "concert", "flight", "sports", "conference", etc.
    /// Kept as free text metadata rather than a fixed enum so new industries
    /// don't require a contract migration.
    pub category: String,
    /// Basis points cap on resale price relative to original sale price
    /// (e.g. 12000 = 120%). Anti-scalping enforcement.
    pub max_resale_multiplier_bps: u32,
    /// Basis points of every resale price paid to the organizer as royalty.
    pub royalty_bps: u32,
    pub tickets_issued: u64,
    pub starts_at: u64,
    pub transfer_freeze_seconds: u64,
    pub resale_cutoff_seconds: u64,
    /// When true, primary sale proceeds are held by the contract instead of
    /// paid to the organizer immediately, and can only be released once the
    /// ledger sequence reaches `escrow_release_ledger`.
    pub escrow_enabled: bool,
    /// Ledger sequence after which escrowed proceeds may be released.
    /// Ignored when `escrow_enabled` is false.
    pub escrow_release_ledger: u32,
    /// Primary sale proceeds currently held in escrow for this event.
    pub escrow_balance: i128,
    /// Per-event accepted payment token (issue #235). `None` means the
    /// event settles in the contract-wide payment token set at
    /// initialization. Lockable only while no tickets have been issued so
    /// existing sales stay denominated in the token they were paid in.
    pub payment_token: Option<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ticket {
    pub event_id: u64,
    pub owner: Address,
    pub tier: String,
    pub seat: String,
    pub status: TicketStatus,
    pub original_price: i128,
    pub resale_price: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GiftClaim {
    pub from: Address,
    pub secret_hash: BytesN<32>,
    pub expires_at: u64,
}

/// A payment token change that has been proposed but not yet applied.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PendingPaymentToken {
    pub token: Address,
    pub apply_after_ledger: u32,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    PaymentToken,
    TokenDecimals,
    PendingPaymentToken,
    Event(u64),
    Ticket(u64),
    GiftClaim(u64),
    NextTicketId,
    LastPurchaseLedger(Address),
    MinPurchaseSpacing,
}
