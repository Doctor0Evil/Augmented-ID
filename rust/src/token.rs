//! # BfcToken V1 Implementation
//!
//! Minimalist token exposed to verifiers via Bounded Forward Channels.
//! This token contains only essential fields for verification while
//! preserving citizen privacy and neurorights constraints.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::error::{AugIdError, AugIdResult};
use crate::citizen::AugmentedCitizenId;

/// Consent state from local consent engine
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsentState {
    Confirmed,
    Denied,
    Suspended,
}

/// Interface type for identity presentation
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum InterfaceType {
    ImplantedNfc,
    EcoNfc,
    XrRig,
    MobileApp,
}

/// Capability flags for transaction limits
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapsOk {
    pub spend_cap_ok: bool,
    pub prompt_cap_ok: bool,
    pub id_check_ok: bool,
}

/// Eco and accessibility flags
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcoFlags {
    pub eco_impact_score_band: String,
    pub eaccessibility: bool,
    pub service_class_basic: String,
}

/// Minimalist token for verifier consumption (BfcToken.v1)
///
/// This token is derived from the full AugmentedCitizenID shard and VCs,
/// but exposes only the minimum information required for verification.
/// All tokens include neurorights flags as a non-negotiable header.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BfcTokenV1 {
    /// Unique token identifier
    pub tokenid: String,
    
    /// Token version (must be "v1")
    pub token_version: String,
    
    /// ISO-8601 timestamp of generation
    pub generated_at: DateTime<Utc>,
    
    /// ISO-8601 timestamp of expiry (short-lived for security)
    pub valid_until: DateTime<Utc>,
    
    /// Pseudonymous DID for payment routing
    pub walletdid: String,
    
    /// Interface type (implanted_nfc, EcoNFC, xr_rig, mobile_app)
    pub interface_type: InterfaceType,
    
    /// Consent state from local consent engine
    pub aiconsentstate: ConsentState,
    
    /// Capability flags (spend, prompt, ID check limits)
    pub caps_ok: CapsOk,
    
    /// Eco and accessibility flags
    pub eco_flags: EcoFlags,
    
    /// Neurorights header (ALWAYS present, non-negotiable)
    pub neurorights_flags: Vec<String>,
    
    /// Signature from citizen's device (BFC service)
    pub issuer_signature: String,
    
    /// HashRef to identity snapshot this token derives from
    pub snapshot_hash: String,
}

impl BfcTokenV1 {
    /// Default token validity duration (5 minutes)
    pub const DEFAULT_VALIDITY_SECS: i64 = 300;
    
    /// Required neurorights flags (must all be present)
    pub const REQUIRED_NEURORIGHTS: [&'static str; 3] = [
        "no_exclusion_basic_services",
        "no_score_from_inner_state",
        "revocable_at_will",
    ];
    
    /// Generate a new BfcTokenV1 from citizen shard and consent state
    pub fn new(
        citizen: &AugmentedCitizenId,
        consent_state: ConsentState,
        interface_type: InterfaceType,
        caps_ok: CapsOk,
        eco_flags: EcoFlags,
    ) -> AugIdResult<Self> {
        let now = Utc::now();
        let valid_until = now + chrono::Duration::seconds(Self::DEFAULT_VALIDITY_SECS);
        
        // Verify neurorights flags are complete
        let mut neurorights_flags = citizen.neurightsflags.clone();
        for flag in Self::REQUIRED_NEURORIGHTS.iter() {
            if !neurorights_flags.contains(&flag.to_string()) {
                return Err(AugIdError::NeurorightsFlagsIncomplete {
                    flag: flag.to_string(),
                });
            }
        }
        
        Ok(Self {
            tokenid: uuid::Uuid::new_v4().to_string(),
            token_version: "v1".to_string(),
            generated_at: now,
            valid_until,
            walletdid: citizen.did.clone(),
            interface_type,
            aiconsentstate: consent_state,
            caps_ok,
            eco_flags,
            neurorights_flags,
            issuer_signature: String::new(),  // Set by signing method
            snapshot_hash: citizen.organichainroot.clone(),
        })
    }
    
    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.valid_until
    }
    
    /// Validate token before acceptance by verifier
    pub fn validate(&self) -> AugIdResult<()> {
        // Check version
        if self.token_version != "v1" {
            return Err(AugIdError::InvalidOperation {
                operation: "token version mismatch".to_string(),
            });
        }
        
        // Check expiry
        if self.is_expired() {
            return Err(AugIdError::TokenExpired {
                valid_until: self.valid_until.to_rfc3339(),
                current_time: Utc::now().to_rfc3339(),
            });
        }
        
        // Check neurorights flags
        for flag in Self::REQUIRED_NEURORIGHTS.iter() {
            if !self.neurights_flags.contains(&flag.to_string()) {
                return Err(AugIdError::NeurorightsFlagsIncomplete {
                    flag: flag.to_string(),
                });
            }
        }
        
        // Check DID format
        if !self.walletdid.starts_with("bostrom1") || self.walletdid.len() != 46 {
            return Err(AugIdError::DidFormatInvalid {
                did: self.walletdid.clone(),
            });
        }
        
        // Check snapshot hash format
        if !self.snapshot_hash.starts_with("sha256:") {
            return Err(AugIdError::SnapshotHashMismatch {
                expected: "sha256:...".to_string(),
                actual: self.snapshot_hash.clone(),
            });
        }
        
        Ok(())
    }
    
    /// Sign the token with device key (placeholder for actual crypto)
    pub fn sign(&mut self, signature: String) {
        self.issuer_signature = signature;
    }
    
    /// Verify token signature (placeholder for actual crypto)
    pub fn verify_signature(&self, public_key: &[u8]) -> AugIdResult<()> {
        if self.issuer_signature.is_empty() {
            return Err(AugIdError::TokenSignatureInvalid);
        }
        // Actual implementation would use ed25519-dalek
        Ok(())
    }
    
    /// Check if consent state allows the requested operation
    pub fn consent_allows_operation(&self, operation: &str) -> bool {
        match self.aiconsentstate {
            ConsentState::Confirmed => true,  // All operations allowed
            ConsentState::Suspended => {
                // Only basic services and emergency operations allowed
                operation == "BasicService" || operation == "Emergency"
            }
            ConsentState::Denied => false,  // No operations allowed
        }
    }
}

/// Token builder for fluent construction
pub struct BfcTokenBuilder {
    citizen: Option<AugmentedCitizenId>,
    consent_state: Option<ConsentState>,
    interface_type: Option<InterfaceType>,
    caps_ok: Option<CapsOk>,
    eco_flags: Option<EcoFlags>,
}

impl BfcTokenBuilder {
    pub fn new() -> Self {
        Self {
            citizen: None,
            consent_state: None,
            interface_type: None,
            caps_ok: None,
            eco_flags: None,
        }
    }
    
    pub fn citizen(mut self, citizen: AugmentedCitizenId) -> Self {
        self.citizen = Some(citizen);
        self
    }
    
    pub fn consent_state(mut self, state: ConsentState) -> Self {
        self.consent_state = Some(state);
        self
    }
    
    pub fn interface_type(mut self, interface: InterfaceType) -> Self {
        self.interface_type = Some(interface);
        self
    }
    
    pub fn caps_ok(mut self, caps: CapsOk) -> Self {
        self.caps_ok = Some(caps);
        self
    }
    
    pub fn eco_flags(mut self, flags: EcoFlags) -> Self {
        self.eco_flags = Some(flags);
        self
    }
    
    pub fn build(self) -> AugIdResult<BfcTokenV1> {
        BfcTokenV1::new(
            self.citizen.ok_or(AugIdError::InternalError {
                message: "citizen not set".to_string(),
            })?,
            self.consent_state.ok_or(AugIdError::InternalError {
                message: "consent_state not set".to_string(),
            })?,
            self.interface_type.ok_or(AugIdError::InternalError {
                message: "interface_type not set".to_string(),
            })?,
            self.caps_ok.ok_or(AugIdError::InternalError {
                message: "caps_ok not set".to_string(),
            })?,
            self.eco_flags.ok_or(AugIdError::InternalError {
                message: "eco_flags not set".to_string(),
            })?,
        )
    }
}

impl Default for BfcTokenBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::citizen::AugIdStatus;
    
    #[test]
    fn test_token_generation() {
        let citizen = AugmentedCitizenId::new(
            "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
            "sha256:abc123...".to_string(),
            "US-AZ".to_string(),
            "vault:xyz789...".to_string(),
            AugIdStatus::Active,
        ).unwrap();
        
        let token = BfcTokenV1::new(
            &citizen,
            ConsentState::Confirmed,
            InterfaceType::MobileApp,
            CapsOk {
                spend_cap_ok: true,
                prompt_cap_ok: true,
                id_check_ok: true,
            },
            EcoFlags {
                eco_impact_score_band: "Gold".to_string(),
                eaccessibility: true,
                service_class_basic: "Enabled".to_string(),
            },
        ).unwrap();
        
        assert_eq!(token.token_version, "v1");
        assert!(token.neurorights_flags.len() >= 3);
        assert!(!token.is_expired());
    }
    
    #[test]
    fn test_token_validation() {
        let citizen = AugmentedCitizenId::new(
            "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
            "sha256:abc123...".to_string(),
            "US-AZ".to_string(),
            "vault:xyz789...".to_string(),
            AugIdStatus::Active,
        ).unwrap();
        
        let mut token = BfcTokenV1::new(
            &citizen,
            ConsentState::Confirmed,
            InterfaceType::MobileApp,
            CapsOk {
                spend_cap_ok: true,
                prompt_cap_ok: true,
                id_check_ok: true,
            },
            EcoFlags {
                eco_impact_score_band: "Gold".to_string(),
                eaccessibility: true,
                service_class_basic: "Enabled".to_string(),
            },
        ).unwrap();
        
        token.sign("signature_placeholder".to_string());
        assert!(token.validate().is_ok());
    }
    
    #[test]
    fn test_consent_state_operation_check() {
        let citizen = AugmentedCitizenId::new(
            "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
            "sha256:abc123...".to_string(),
            "US-AZ".to_string(),
            "vault:xyz789...".to_string(),
            AugIdStatus::Active,
        ).unwrap();
        
        let token_suspended = BfcTokenV1::new(
            &citizen,
            ConsentState::Suspended,
            InterfaceType::MobileApp,
            CapsOk {
                spend_cap_ok: false,
                prompt_cap_ok: false,
                id_check_ok: true,
            },
            EcoFlags {
                eco_impact_score_band: "Gold".to_string(),
                eaccessibility: true,
                service_class_basic: "Enabled".to_string(),
            },
        ).unwrap();
        
        assert!(!token_suspended.consent_allows_operation("Payment"));
        assert!(token_suspended.consent_allows_operation("BasicService"));
        assert!(token_suspended.consent_allows_operation("Emergency"));
    }
}
