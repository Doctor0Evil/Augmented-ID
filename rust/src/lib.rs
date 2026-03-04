//! # Augmented-ID Core Library
//!
//! A soul-bound, offline-first identity framework for augmented citizens.
//! This library implements the core data structures and validation logic
//! for the Augmented-ID system, including:
//!
//! - AugmentedCitizenId: Immutable identity shard
//! - AugIdLedgerEntry: Forward-only ledger entries
//! - BfcToken: Minimalist verifier tokens
//! - Guard validation for neurorights and anti-rollback invariants
//!
//! ## Security Notice
//!
//! This library is designed with **no fallback mechanisms**. All security
//! guarantees depend on proper implementation of the guard logic and
//! adherence to the anti-rollback invariants defined in the ALN schemas.
//!
//! ## Example
//!
//! ```rust
//! use augid_core::{AugmentedCitizenId, AugIdStatus, AugIdLedgerEntry};
//!
//! let citizen = AugmentedCitizenId::new(
//!     "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
//!     AugIdStatus::Active,
//! )?;
//! ```

pub mod citizen;
pub mod ledger;
pub mod token;
pub mod guards;
pub mod crypto;
pub mod error;

pub use citizen::AugmentedCitizenId;
pub use ledger::AugIdLedgerEntry;
pub use token::BfcTokenV1;
pub use guards::{AugFingerprintGuard, EqualityPaymentGuard, AntiRollbackGuard};
pub use error::AugIdError;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// ALN schema version this library implements
pub const ALN_SCHEMA_VERSION: &str = "1.0.0";

/// Maximum Rate-of-Harm limit (neurorights constraint)
pub const MAX_ROH_LIMIT: f64 = 0.3;

/// Default token validity duration (5 minutes)
pub const DEFAULT_TOKEN_VALIDITY_SECS: i64 = 300;
