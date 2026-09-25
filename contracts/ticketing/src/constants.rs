//! Contract limits and basis-point denominators.
//!
//! Centralizes the numeric limits used by the ticketing contract so they
//! have one source of truth instead of scattered magic numbers.

/// Maximum number of tickets accepted by the batch entry points.
pub const MAX_BATCH_SIZE: u32 = 50;

/// Ledgers that must pass between proposing and applying a payment token
/// change (~1 day at 5s/ledger).
pub const PAYMENT_TOKEN_CHANGE_DELAY_LEDGERS: u32 = 17_280;

/// Basis-point denominator: 10_000 bps == 100%.
pub const BPS_DENOMINATOR: u32 = 10_000;
