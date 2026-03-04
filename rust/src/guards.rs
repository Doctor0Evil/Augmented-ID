//! # Guard Validation Logic
//!
//! Implements runtime guard checks for neurorights, anti-rollback,
//! biometric vault security, and offline operation constraints.
//!
//! Guards are the enforcement layer that ensures all Augmented-ID
//! operations comply with the invariants defined in ALN schemas.

use crate::error::{AugIdError, AugIdResult};
use crate::citizen::AugmentedCitizenId;
use crate::token::{BfcTokenV1, ConsentState};
use crate::ledger::AugIdLedgerEntry;

/// Biophysical state metrics (from local consent engine)
#[derive(Clone, Debug)]
pub struct BiophysicalState {
    pub stress_level: f64,      // St: 0.0 - 1.0
    pub lifeforce_level: f64,   // Lt: 0.0 - 1.0
    pub roh_current: f64,       // Rate-of-Harm: must be <= 0.3
}

impl BiophysicalState {
    /// Check if state is within HealthyEngagementBand
    pub fn is_healthy_engagement(&self) -> bool {
        self.stress_level >= 0.3 && self.stress_level <= 0.7 &&
        self.lifeforce_level >= 0.3 && self.lifeforce_level <= 0.7
    }
    
    /// Check if Rate-of-Harm is within limits
    pub fn is_roh_compliant(&self) -> bool {
        self.roh_current <= 0.3
    }
}

/// ============================================================================
/// GUARD: AugFingerprintGuard
/// PURPOSE: Enforce neurorights flags and biophysical state safety bands
/// ============================================================================

pub struct AugFingerprintGuard;

impl AugFingerprintGuard {
    /// Required neurorights flags
    pub const REQUIRED_NEURORIGHTS: [&'static str; 3] = [
        "no_exclusion_basic_services",
        "no_score_from_inner_state",
        "revocable_at_will",
    ];
    
    /// Validate neurorights flags are complete
    pub fn validate_neurorights(neurorights_flags: &[String]) -> AugIdResult<()> {
        for flag in Self::REQUIRED_NEURORIGHTS.iter() {
            if !neurorights_flags.contains(&flag.to_string()) {
                return Err(AugIdError::NeurorightsFlagsIncomplete {
                    flag: flag.to_string(),
                });
            }
        }
        Ok(())
    }
    
    /// Validate biophysical state for CONFIRMED consent
    pub fn validate_biophysical_state(state: &BiophysicalState) -> AugIdResult<()> {
        if !state.is_healthy_engagement() {
            return Err(AugIdError::BiophysicalStateUnsafe);
        }
        
        if !state.is_roh_compliant() {
            return Err(AugIdError::RohLimitExceeded {
                roh: state.roh_current,
                max_roh: 0.3,
            });
        }
        
        Ok(())
    }
    
    /// Validate consent state with biometric verification requirement
    pub fn validate_consent_with_biometric(
        consent_state: &ConsentState,
        biometric_verified: bool,
    ) -> AugIdResult<()> {
        match consent_state {
            ConsentState::Confirmed => {
                if !biometric_verified {
                    return Err(AugIdError::BiometricMatchFailed);
                }
            }
            ConsentState::Suspended | ConsentState::Denied => {
                // No biometric required for denied/suspended states
            }
        }
        Ok(())
    }
    
    /// Full guard check for identity verification operations
    pub fn verify_identity_operation(
        citizen: &AugmentedCitizenId,
        biophysical_state: &BiophysicalState,
        biometric_verified: bool,
        consent_state: &ConsentState,
    ) -> AugIdResult<()> {
        // Check neurorights
        Self::validate_neurorights(&citizen.neurightsflags)?;
        
        // Check anti-rollback
        citizen.verify_antirollback()?;
        
        // Check biophysical state for CONFIRMED consent
        if *consent_state == ConsentState::Confirmed {
            Self::validate_biophysical_state(biophysical_state)?;
            Self::validate_consent_with_biometric(consent_state, biometric_verified)?;
        }
        
        Ok(())
    }
}

/// ============================================================================
/// GUARD: EqualityPaymentGuard
/// PURPOSE: Enforce payment caps and neurorights during transaction processing
/// ============================================================================

pub struct EqualityPaymentGuard;

impl EqualityPaymentGuard {
    /// Validate payment transaction
    pub fn validate_payment(
        token: &BfcTokenV1,
        transaction_amount: f64,
        spend_cap: f64,
        prompts_last_hour: u32,
        prompt_cap: u32,
        service_class: &str,
    ) -> AugIdResult<()> {
        // Check consent state
        if !token.consent_allows_operation("Payment") {
            return Err(AugIdError::ConsentStateInvalid {
                state: format!("{:?}", token.aiconsentstate),
            });
        }
        
        // Check spend cap
        if transaction_amount > spend_cap {
            return Err(AugIdError::InvalidOperation {
                operation: format!(
                    "transaction amount {} exceeds spend cap {}",
                    transaction_amount, spend_cap
                ),
            });
        }
        
        // Check prompt cap
        if prompts_last_hour > prompt_cap {
            return Err(AugIdError::InvalidOperation {
                operation: format!(
                    "prompts {} exceeds cap {}",
                    prompts_last_hour, prompt_cap
                ),
            });
        }
        
        // Basic services cannot require payment
        if service_class == "BasicService" && transaction_amount > 0.0 {
            return Err(AugIdError::InvalidOperation {
                operation: "basic services cannot require payment".to_string(),
            });
        }
        
        // Verify neurorights flags
        AugFingerprintGuard::validate_neurorights(&token.neurorights_flags)?;
        
        Ok(())
    }
}

/// ============================================================================
/// GUARD: AntiRollbackGuard
/// PURPOSE: Enforce forward-only ledger semantics across all operations
/// ============================================================================

pub struct AntiRollbackGuard;

impl AntiRollbackGuard {
    /// Validate ledger entry chain integrity
    pub fn validate_chain_integrity(entries: &[AugIdLedgerEntry]) -> AugIdResult<()> {
        for i in 1..entries.len() {
            let prev = &entries[i - 1];
            let curr = &entries[i];
            
            // Verify prev_entry_id linkage
            if curr.prev_entry_id.as_deref() != Some(&prev.entry_id) {
                return Err(AugIdError::BackwardsLink);
            }
            
            // Verify timestamp monotonicity
            if curr.timestamp <= prev.timestamp {
                return Err(AugIdError::TimestampNotMonotonic);
            }
            
            // Verify anti-rollback flag
            if !curr.citizen.antirollback {
                return Err(AugIdError::AntiRollbackViolation);
            }
        }
        Ok(())
    }
    
    /// Validate status transition (forward-only)
    pub fn validate_status_transition(
        previous_status: &str,
        new_status: &str,
        governance_approved: bool,
    ) -> AugIdResult<()> {
        // Once Revoked, never return to Active or Suspended
        if previous_status == "Revoked" && (new_status == "Active" || new_status == "Suspended") {
            return Err(AugIdError::StatusDowngradeForbidden {
                from: previous_status.to_string(),
                to: new_status.to_string(),
            });
        }
        
        // Status can tighten without governance, but cannot loosen
        let is_less_restrictive = match (previous_status, new_status) {
            ("Revoked", "Active") | ("Revoked", "Suspended") => true,
            ("Suspended", "Active") => true,
            _ => false,
        };
        
        if is_less_restrictive && !governance_approved {
            return Err(AugIdError::StatusDowngradeForbidden {
                from: previous_status.to_string(),
                to: new_status.to_string(),
            });
        }
        
        Ok(())
    }
}

/// ============================================================================
/// GUARD: OfflineOperationGuard
/// PURPOSE: Enforce offline operation security constraints
/// ============================================================================

pub struct OfflineOperationGuard;

impl OfflineOperationGuard {
    /// Validate offline snapshot
    pub fn validate_offline_snapshot(
        snapshot_hash: &str,
        snapshot_valid_from: DateTime<Utc>,
        snapshot_valid_until: DateTime<Utc>,
    ) -> AugIdResult<()> {
        // Check hash format
        if !snapshot_hash.starts_with("sha256:") {
            return Err(AugIdError::SnapshotHashMismatch {
                expected: "sha256:...".to_string(),
                actual: snapshot_hash.to_string(),
            });
        }
        
        // Check validity window
        let now = Utc::now();
        if now < snapshot_valid_from || now > snapshot_valid_until {
            return Err(AugIdError::OfflineSnapshotExpired);
        }
        
        Ok(())
    }
    
    /// Detect conflicting offline operations
    pub fn detect_conflicting_operations(
        operations: &[(String, DateTime<Utc>)],
    ) -> AugIdResult<()> {
        // Check for duplicate operation IDs within validity window
        let mut seen = std::collections::HashSet::new();
        for (op_id, timestamp) in operations {
            if !seen.insert(op_id.clone()) {
                return Err(AugIdError::ConflictingOfflineOperations);
            }
        }
        Ok(())
    }
}

/// ============================================================================
/// GUARD: BiometricVaultGuard
/// PURPOSE: Enforce biometric vault security constraints (local-only matching)
/// ============================================================================

pub struct BiometricVaultGuard;

impl BiometricVaultGuard {
    /// Validate biometric binding ID format
    pub fn validate_biometric_binding_id(binding_id: &str) -> AugIdResult<()> {
        if !binding_id.starts_with("vault:") {
            return Err(AugIdError::BiometricVaultAccessDenied);
        }
        
        if binding_id.len() < 11 {  // "vault:" + at least some hash
            return Err(AugIdError::BiometricVaultAccessDenied);
        }
        
        Ok(())
    }
    
    /// Verify no raw biometric data in record fields
    pub fn verify_no_raw_biometric(field_names: &[&str]) -> AugIdResult<()> {
        let forbidden = ["biometric_raw", "biometric_template", "biometric_data"];
        
        for field in field_names {
            if forbidden.contains(field) {
                return Err(AugIdError::RawBiometricTransmissionAttempt);
            }
        }
        Ok(())
    }
    
    /// Full biometric vault guard check
    pub fn verify_biometric_operation(
        binding_id: &str,
        field_names: &[&str],
        match_location: &str,
    ) -> AugIdResult<()> {
        Self::validate_biometric_binding_id(binding_id)?;
        Self::verify_no_raw_biometric(field_names)?;
        
        if match_location != "local_device" {
            return Err(AugIdError::RawBiometricTransmissionAttempt);
        }
        
        Ok(())
    }
}

/// ============================================================================
/// COMPOSITE: Verify All Guards
/// PURPOSE: Run all relevant guards for a given operation type
/// ============================================================================

pub fn verify_all_guards(operation_type: &str, context: &GuardContext) -> AugIdResult<()> {
    match operation_type {
        "identity_verification" => {
            AugFingerprintGuard::verify_identity_operation(
                &context.citizen,
                &context.biophysical_state,
                context.biometric_verified,
                &context.consent_state,
            )?;
            AntiRollbackGuard::validate_chain_integrity(&context.ledger_entries)?;
        }
        "payment" => {
            EqualityPaymentGuard::validate_payment(
                &context.token,
                context.transaction_amount,
                context.spend_cap,
                context.prompts_last_hour,
                context.prompt_cap,
                &context.service_class,
            )?;
            AugFingerprintGuard::validate_neurorights(&context.token.neurorights_flags)?;
        }
        "offline_operation" => {
            OfflineOperationGuard::validate_offline_snapshot(
                &context.snapshot_hash,
                context.snapshot_valid_from,
                context.snapshot_valid_until,
            )?;
            OfflineOperationGuard::detect_conflicting_operations(&context.offline_operations)?;
        }
        "biometric_auth" => {
            BiometricVaultGuard::verify_biometric_operation(
                &context.biometric_binding_id,
                &context.field_names,
                &context.match_location,
            )?;
            AugFingerprintGuard::validate_consent_with_biometric(
                &context.consent_state,
                context.biometric_verified,
            )?;
        }
        _ => {
            return Err(AugIdError::InvalidOperation {
                operation: operation_type.to_string(),
            });
        }
    }
    
    Ok(())
}

/// Context structure for guard validation
pub struct GuardContext {
    pub citizen: AugmentedCitizenId,
    pub token: BfcTokenV1,
    pub biophysical_state: BiophysicalState,
    pub biometric_verified: bool,
    pub consent_state: ConsentState,
    pub ledger_entries: Vec<AugIdLedgerEntry>,
    pub transaction_amount: f64,
    pub spend_cap: f64,
    pub prompts_last_hour: u32,
    pub prompt_cap: u32,
    pub service_class: String,
    pub snapshot_hash: String,
    pub snapshot_valid_from: DateTime<Utc>,
    pub snapshot_valid_until: DateTime<Utc>,
    pub offline_operations: Vec<(String, DateTime<Utc>)>,
    pub biometric_binding_id: String,
    pub field_names: Vec<String>,
    pub match_location: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::citizen::AugIdStatus;
    use crate::token::{CapsOk, EcoFlags, InterfaceType};
    
    #[test]
    fn test_aug_fingerprint_guard_neurorights() {
        let flags = vec![
            "no_exclusion_basic_services".to_string(),
            "no_score_from_inner_state".to_string(),
            "revocable_at_will".to_string(),
        ];
        assert!(AugFingerprintGuard::validate_neurorights(&flags).is_ok());
    }
    
    #[test]
    fn test_aug_fingerprint_guard_biophysical() {
        let healthy_state = BiophysicalState {
            stress_level: 0.5,
            lifeforce_level: 0.5,
            roh_current: 0.2,
        };
        assert!(AugFingerprintGuard::validate_biophysical_state(&healthy_state).is_ok());
        
        let unhealthy_state = BiophysicalState {
            stress_level: 0.9,
            lifeforce_level: 0.5,
            roh_current: 0.2,
        };
        assert!(AugFingerprintGuard::validate_biophysical_state(&unhealthy_state).is_err());
    }
    
    #[test]
    fn test_anti_rollback_guard_status_transition() {
        // Revoked -> Active should fail
        assert!(AntiRollbackGuard::validate_status_transition(
            "Revoked", "Active", false
        ).is_err());
        
        // Active -> Suspended should pass
        assert!(AntiRollbackGuard::validate_status_transition(
            "Active", "Suspended", false
        ).is_ok());
    }
    
    #[test]
    fn test_equality_payment_guard_basic_service() {
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
        
        // Basic service with payment should fail
        assert!(EqualityPaymentGuard::validate_payment(
            &token, 10.0, 100.0, 5, 10, "BasicService"
        ).is_err());
        
        // Basic service without payment should pass
        assert!(EqualityPaymentGuard::validate_payment(
            &token, 0.0, 100.0, 5, 10, "BasicService"
        ).is_ok());
    }
}
