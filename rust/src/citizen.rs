//! # Augmented Citizen Identity Shard
//!
//! Implements the soul-bound AugmentedCitizenId record with anti-rollback invariants.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::error::{AugIdError, AugIdResult};

/// Lifecycle status for augmented citizen identity
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AugIdStatus {
    Active,
    Suspended,
    Revoked,
}

impl AugIdStatus {
    /// Check if this status is less restrictive than another
    pub fn is_less_restrictive_than(&self, other: &AugIdStatus) -> bool {
        match (self, other) {
            (AugIdStatus::Active, AugIdStatus::Suspended) => true,
            (AugIdStatus::Active, AugIdStatus::Revoked) => true,
            (AugIdStatus::Suspended, AugIdStatus::Revoked) => true,
            _ => false,
        }
    }
}

/// Core identity shard for augmented citizens
/// 
/// This record is immutable and append-only. All changes create new records
/// with monotonic timestamps, never in-place updates.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AugmentedCitizenId {
    /// Bostrom-based DID (e.g., bostrom18...)
    pub did: String,
    
    /// HashRef-resolved, non-blacklisted primitive (hashed legal name)
    pub legalnamehash: String,
    
    /// Coarse region only (e.g., "US-AZ")
    pub birthregioncode: String,
    
    /// Reference to encrypted local biometric vault (never raw data)
    pub biometricbindingid: String,
    
    /// Last anchored Organichain snapshot ID
    pub organichainroot: String,
    
    /// ROWRPM shard Merkle root for this DID
    pub rowledgerroot: String,
    
    /// ISO-8601 timestamp of creation
    pub createdat: DateTime<Utc>,
    
    /// MUST be true in all valid records
    pub antirollback: bool,
    
    /// Lifecycle status: Active, Suspended, Revoked
    pub status: AugIdStatus,
    
    /// Neurorights flags (hard-coded constraints)
    pub neurightsflags: Vec<String>,
    
    /// OriginSpan reference for audit trail
    pub originspan: String,
    
    /// RowSoilRemediation hash for compliance
    pub rowsoilremediation: String,
}

impl AugmentedCitizenId {
    /// Create a new AugmentedCitizenId with validation
    pub fn new(
        did: String,
        legalnamehash: String,
        birthregioncode: String,
        biometricbindingid: String,
        status: AugIdStatus,
    ) -> AugIdResult<Self> {
        // Validate DID format
        if !did.starts_with("bostrom1") || did.len() != 46 {
            return Err(AugIdError::DidFormatInvalid { did });
        }
        
        // Validate biometric binding ID format
        if !biometricbindingid.starts_with("vault:") {
            return Err(AugIdError::BiometricVaultAccessDenied);
        }
        
        // Required neurorights flags
        let neurightsflags = vec![
            "no_exclusion_basic_services".to_string(),
            "no_score_from_inner_state".to_string(),
            "revocable_at_will".to_string(),
        ];
        
        Ok(Self {
            did,
            legalnamehash,
            birthregioncode,
            biometricbindingid,
            organichainroot: String::new(),  // Set on anchoring
            rowledgerroot: String::new(),    // Set on ROWRPM commit
            createdat: Utc::now(),
            antirollback: true,
            status,
            neurightsflags,
            originspan: String::new(),
            rowsoilremediation: String::new(),
        })
    }
    
    /// Validate neurorights flags are complete
    pub fn validate_neurorights(&self) -> AugIdResult<()> {
        let required = [
            "no_exclusion_basic_services",
            "no_score_from_inner_state",
            "revocable_at_will",
        ];
        
        for flag in required.iter() {
            if !self.neurightsflags.contains(&flag.to_string()) {
                return Err(AugIdError::NeurorightsFlagsIncomplete {
                    flag: flag.to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    /// Verify anti-rollback invariant
    pub fn verify_antirollback(&self) -> AugIdResult<()> {
        if !self.antirollback {
            return Err(AugIdError::AntiRollbackViolation);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_citizen_valid() {
        let citizen = AugmentedCitizenId::new(
            "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
            "sha256:abc123...".to_string(),
            "US-AZ".to_string(),
            "vault:xyz789...".to_string(),
            AugIdStatus::Active,
        );
        assert!(citizen.is_ok());
    }
    
    #[test]
    fn test_did_format_invalid() {
        let citizen = AugmentedCitizenId::new(
            "invalid_did".to_string(),
            "sha256:abc123...".to_string(),
            "US-AZ".to_string(),
            "vault:xyz789...".to_string(),
            AugIdStatus::Active,
        );
        assert!(matches!(citizen, Err(AugIdError::DidFormatInvalid { .. })));
    }
    
    #[test]
    fn test_neurorights_validation() {
        let citizen = AugmentedCitizenId::new(
            "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
            "sha256:abc123...".to_string(),
            "US-AZ".to_string(),
            "vault:xyz789...".to_string(),
            AugIdStatus::Active,
        ).unwrap();
        assert!(citizen.validate_neurorights().is_ok());
    }
}
