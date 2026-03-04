use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AugIdStatus {
    Active,
    Suspended,
    Revoked,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AugmentedCitizenId {
    pub did: String,               // bostrom18...
    pub legalnamehash: String,     // HashRef-resolved, non-blacklisted
    pub birthregioncode: String,   // e.g. "US-AZ"
    pub biometricbindingid: String,
    pub organichainroot: String,
    pub rowledgerroot: String,
    pub createdat: String,         // ISO-8601
    pub antirollback: bool,        // must be true
    pub status: AugIdStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AugIdLedgerEntry {
    pub entry_id: String,
    pub citizen: AugmentedCitizenId,
    pub prev_entry_id: Option<String>, // forward-only link
    pub timestamp: String,
    pub author_did: String,            // e.g. gateway or citizen
    pub signature: String,             // Ed25519/compatible
}

#[derive(Debug)]
pub enum AugIdError {
    AntiRollbackViolation,
    BackwardsLink,
    StatusDowngradeForbidden,
}

impl AugIdLedgerEntry {
    /// Enforce forward-only, soul-bound evolution rules.
    pub fn validate_next(
        &self,
        previous: Option<&AugIdLedgerEntry>,
    ) -> Result<(), AugIdError> {
        if !self.citizen.antirollback {
            return Err(AugIdError::AntiRollbackViolation);
        }
        if let Some(prev) = previous {
            if self.prev_entry_id.as_deref() != Some(&prev.entry_id) {
                return Err(AugIdError::BackwardsLink);
            }
            // Example invariant: once Revoked, never go back to Active.
            use AugIdStatus::*;
            match (&prev.citizen.status, &self.citizen.status) {
                (Revoked, Active) | (Revoked, Suspended) => {
                    return Err(AugIdError::StatusDowngradeForbidden)
                }
                _ => {}
            }
        }
        Ok(())
    }
}
