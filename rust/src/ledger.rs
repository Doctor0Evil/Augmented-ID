//! # Augmented-ID Ledger Entry
//!
//! Implements forward-only, append-only ledger entries with anti-rollback enforcement.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::citizen::{AugmentedCitizenId, AugIdStatus};
use crate::error::{AugIdError, AugIdResult};

/// Ledger entry for Augmented-ID operations
/// 
/// Each entry links to its predecessor, creating an immutable chain
/// that mechanically prevents rollback and history erasure.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AugIdLedgerEntry {
    /// Unique entry identifier
    pub entry_id: String,
    
    /// Reference to previous entry (None for genesis)
    pub prev_entry_id: Option<String>,
    
    /// Citizen identity record
    pub citizen: AugmentedCitizenId,
    
    /// ISO-8601 timestamp
    pub timestamp: DateTime<Utc>,
    
    /// DID of entity authorizing this entry
    pub author_did: String,
    
    /// Ed25519 signature over entry contents
    pub signature: String,
}

impl AugIdLedgerEntry {
    /// Create a new genesis ledger entry (no previous entry)
    pub fn new_genesis(citizen: AugmentedCitizenId, author_did: String, signature: String) -> Self {
        Self {
            entry_id: uuid::Uuid::new_v4().to_string(),
            prev_entry_id: None,
            citizen,
            timestamp: Utc::now(),
            author_did,
            signature,
        }
    }
    
    /// Create a new ledger entry linking to previous entry
    pub fn new_update(
        prev_entry: &AugIdLedgerEntry,
        citizen: AugmentedCitizenId,
        author_did: String,
        signature: String,
    ) -> Self {
        Self {
            entry_id: uuid::Uuid::new_v4().to_string(),
            prev_entry_id: Some(prev_entry.entry_id.clone()),
            citizen,
            timestamp: Utc::now(),
            author_did,
            signature,
        }
    }
    
    /// Validate this entry against the previous entry (forward-only enforcement)
    pub fn validate_next(&self, previous: Option<&AugIdLedgerEntry>) -> AugIdResult<()> {
        // Verify anti-rollback flag
        if !self.citizen.antirollback {
            return Err(AugIdError::AntiRollbackViolation);
        }
        
        // Verify previous entry linkage
        if let Some(prev) = previous {
            if self.prev_entry_id.as_deref() != Some(&prev.entry_id) {
                return Err(AugIdError::BackwardsLink);
            }
            
            // Verify timestamp is monotonic
            if self.timestamp <= prev.timestamp {
                return Err(AugIdError::TimestampNotMonotonic);
            }
            
            // Verify status transition rules (no downgrade without governance)
            match (&prev.citizen.status, &self.citizen.status) {
                (AugIdStatus::Revoked, AugIdStatus::Active) |
                (AugIdStatus::Revoked, AugIdStatus::Suspended) => {
                    return Err(AugIdError::StatusDowngradeForbidden {
                        from: format!("{:?}", prev.citizen.status),
                        to: format!("{:?}", self.citizen.status),
                    });
                }
                _ => {}
            }
        } else {
            // Genesis entry must have no prev_entry_id
            if self.prev_entry_id.is_some() {
                return Err(AugIdError::BackwardsLink);
            }
        }
        
        // Verify neurorights flags
        self.citizen.validate_neurorights()?;
        
        Ok(())
    }
    
    /// Verify entry signature
    pub fn verify_signature(&self, public_key: &[u8]) -> AugIdResult<()> {
        // Implementation would use ed25519-dalek for actual verification
        // This is a placeholder for the signature verification logic
        if self.signature.is_empty() {
            return Err(AugIdError::SignatureVerificationFailed);
        }
        Ok(())
    }
}

/// Ledger chain manager for maintaining ROWRPM integrity
pub struct LedgerChain {
    entries: Vec<AugIdLedgerEntry>,
    chain_root: String,
}

impl LedgerChain {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            chain_root: String::new(),
        }
    }
    
    /// Append a new entry to the chain with validation
    pub fn append(&mut self, entry: AugIdLedgerEntry) -> AugIdResult<()> {
        let previous = self.entries.last();
        entry.validate_next(previous)?;
        
        self.entries.push(entry);
        self.update_chain_root();
        
        Ok(())
    }
    
    /// Get the latest entry
    pub fn latest(&self) -> Option<&AugIdLedgerEntry> {
        self.entries.last()
    }
    
    /// Update the Merkle root of the chain
    fn update_chain_root(&mut self) {
        // Implementation would compute Merkle root of all entries
        self.chain_root = format!("merkle:{}", self.entries.len());
    }
    
    /// Get the current chain root
    pub fn chain_root(&self) -> &str {
        &self.chain_root
    }
    
    /// Verify chain integrity
    pub fn verify_integrity(&self) -> AugIdResult<()> {
        for i in 1..self.entries.len() {
            let prev = &self.entries[i - 1];
            let curr = &self.entries[i];
            curr.validate_next(Some(prev))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::citizen::AugmentedCitizenId;
    
    #[test]
    fn test_genesis_entry() {
        let citizen = AugmentedCitizenId::new(
            "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
            "sha256:abc123...".to_string(),
            "US-AZ".to_string(),
            "vault:xyz789...".to_string(),
            AugIdStatus::Active,
        ).unwrap();
        
        let entry = AugIdLedgerEntry::new_genesis(
            citizen,
            "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
            "signature_placeholder".to_string(),
        );
        
        assert!(entry.validate_next(None).is_ok());
        assert!(entry.prev_entry_id.is_none());
    }
    
    #[test]
    fn test_status_downgrade_forbidden() {
        // Create genesis with Revoked status
        let mut citizen = AugmentedCitizenId::new(
            "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
            "sha256:abc123...".to_string(),
            "US-AZ".to_string(),
            "vault:xyz789...".to_string(),
            AugIdStatus::Revoked,
        ).unwrap();
        
        let genesis = AugIdLedgerEntry::new_genesis(
            citizen.clone(),
            "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
            "signature_placeholder".to_string(),
        );
        
        // Try to create update with Active status (should fail)
        citizen.status = AugIdStatus::Active;
        let update = AugIdLedgerEntry::new_update(
            &genesis,
            citizen,
            "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
            "signature_placeholder".to_string(),
        );
        
        assert!(matches!(
            update.validate_next(Some(&genesis)),
            Err(AugIdError::StatusDowngradeForbidden { .. })
        ));
    }
}
