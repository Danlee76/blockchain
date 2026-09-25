//! Contract error codes for the ticketing contract.

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    EventNotFound = 3,
    EventAlreadyExists = 4,
    TicketNotFound = 5,
    NotOrganizer = 6,
    NotOwner = 7,
    AlreadyUsed = 8,
    Revoked = 9,
    NotForResale = 10,
    ResalePriceExceedsCap = 11,
    InvalidPrice = 12,
    InvalidRoyalty = 13,
    EscrowNotEnabled = 14,
    EventNotEnded = 15,
    PurchaseTooSoon = 16,
    NotAdmin = 17,
    EventAlreadyStarted = 18,
    InvalidEventTime = 19,
    TransfersFrozen = 20,
    ResaleClosed = 21,
    InvalidLottery = 22,
    GiftClaimNotFound = 23,
    GiftClaimExpired = 24,
    InvalidSecret = 25,
    InvalidExpiry = 26,
    EmptyBatch = 27,
    BatchTooLarge = 28,
    InvalidPaymentToken = 29,
    NoPendingPaymentToken = 30,
    TimelockNotElapsed = 31,
    TicketsAlreadyIssued = 32,
}
