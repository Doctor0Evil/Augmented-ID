//! # Augmented-ID Error Types
//!
//! Comprehensive error handling for all Augmented-ID operations.

use thiserror::Error;

/// Core error enumeration for Augmented-ID operations
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AugIdError {
    // Anti-rollback violations
    #[error("Anti-rollback violation: record must have antirollback=true")]
    AntiRollbackViolation,
    
    #[error("Backwards link detected: prev_entry_id does not match previous entry")]
    BackwardsLink,
    
    #[error("Status downgrade forbidden: cannot transition from {from} to {to}")]
    StatusDowngradeForbidden { from: String, to: String },
    
    #[error("Timestamp not monotonic: new entry timestamp must be greater than previous")]
    TimestampNotMonotonic,
    
    // Validation errors
    #[error("DID format invalid: expected bostrom1[a-z0-9]{{38}}, got {did}")]
    DidFormatInvalid { did: String },
    
    #[error("Neurorights flags incomplete: missing required flag {flag}")]
    NeurorightsFlagsIncomplete { flag: String },
    
    #[error("Signature verification failed")]
    SignatureVerificationFailed,
    
    #[error("Snapshot hash mismatch: expected {expected}, got {actual}")]
    SnapshotHashMismatch { expected: String, actual: String },
    
    // Biometric errors
    #[error("Biometric match failed")]
    BiometricMatchFailed,
    
    #[error("Biometric vault access denied")]
    BiometricVaultAccessDenied,
    
    #[error("Raw biometric data transmission attempt detected")]
    RawBiometricTransmissionAttempt,
    
    // Token errors
    #[error("Token expired: valid until {valid_until}, current time {current_time}")]
    TokenExpired { valid_until: String, current_time: String },
    
    #[error("Token signature invalid")]
    TokenSignatureInvalid,
    
    #[error("Consent state invalid for operation: {state}")]
    ConsentStateInvalid { state: String },
    
    // Ledger errors
    #[error("ROWRPM chain integrity violation")]
    RowrpmChainIntegrityViolation,
    
    #[error("Organichain anchor mismatch")]
    OrganichainAnchorMismatch,
    
    #[error("Entry not found: {entry_id}")]
    EntryNotFound { entry_id: String },
    
    // Guard errors
    #[error("Guard validation failed: {guard_name} - {reason}")]
    GuardValidationFailed { guard_name: String, reason: String },
    
    #[error("Rate-of-Harm limit exceeded: {roh} > {max_roh}")]
    RohLimitExceeded { roh: f64, max_roh: f64 },
    
    #[error("Biophysical state outside HealthyEngagementBand")]
    BiophysicalStateUnsafe,
    
    // Offline operation errors
    #[error("Offline snapshot expired")]
    OfflineSnapshotExpired,
    
    #[error("Conflicting offline operations detected")]
    ConflictingOfflineOperations,
    
    #[error("Network reconciliation failed")]
    NetworkReconciliationFailed,
    
    // Cryptography errors
    #[error("Key generation failed")]
    KeyGenerationFailed,
    
    #[error("Encryption failed")]
    EncryptionFailed,
    
    #[error("Decryption failed")]
    DecryptionFailed,
    
    // General errors
    #[error("Invalid operation: {operation}")]
    InvalidOperation { operation: String },
    
    #[error("Internal error: {message}")]
    InternalError { message: String },
}

/// Result type alias for Augmented-ID operations
pub type AugIdResult<T> = Result<T, AugIdError>;
