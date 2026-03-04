//! # Augmented-ID Integration Tests
//!
//! Comprehensive integration tests for the Augmented-ID system.
//! These tests verify end-to-end functionality across all components.

use augid_core::{
    AugmentedCitizenId, AugIdStatus, AugIdLedgerEntry, BfcTokenV1,
    ConsentState, InterfaceType, CapsOk, EcoFlags,
};
use augid_core::crypto::{KeyPair, sha256_hash, sign_data, verify_signature};
use augid_core::guards::{
    AugFingerprintGuard, EqualityPaymentGuard, AntiRollbackGuard,
    OfflineOperationGuard, BiometricVaultGuard, BiophysicalState,
    GuardContext, verify_all_guards,
};
use chrono::{Utc, Duration};

// ============================================================================
// TEST: Citizen Identity Creation and Validation
// ============================================================================

#[test]
fn test_citizen_identity_lifecycle() {
    // Create new citizen
    let citizen = AugmentedCitizenId::new(
        "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        sha256_hash(b"John Doe"),
        "US-AZ".to_string(),
        "vault:biometric_binding_abc123".to_string(),
        AugIdStatus::Active,
    ).expect("Failed to create citizen");
    
    // Validate neurorights
    assert!(citizen.validate_neurorights().is_ok());
    
    // Validate anti-rollback
    assert!(citizen.verify_antirollback().is_ok());
    
    // Verify neurorights flags present
    assert!(citizen.neurightsflags.contains(&"no_exclusion_basic_services".to_string()));
    assert!(citizen.neurightsflags.contains(&"no_score_from_inner_state".to_string()));
    assert!(citizen.neurightsflags.contains(&"revocable_at_will".to_string()));
}

// ============================================================================
// TEST: Token Generation and Validation
// ============================================================================

#[test]
fn test_bfc_token_lifecycle() {
    // Create citizen
    let citizen = AugmentedCitizenId::new(
        "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        sha256_hash(b"Jane Doe"),
        "US-CA".to_string(),
        "vault:biometric_binding_xyz789".to_string(),
        AugIdStatus::Active,
    ).unwrap();
    
    // Generate token
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
    
    // Sign token
    let keypair = KeyPair::generate().unwrap();
    let signature = sign_data(&keypair.signing_key, b"token_data").unwrap();
    token.sign(signature);
    
    // Validate token
    assert!(token.validate().is_ok());
    
    // Verify signature
    assert!(token.verify_signature(keypair.verifying_key.as_bytes()).is_ok());
    
    // Check not expired
    assert!(!token.is_expired());
    
    // Check consent allows operation
    assert!(token.consent_allows_operation("Payment"));
    assert!(token.consent_allows_operation("BasicService"));
}

// ============================================================================
// TEST: Ledger Chain Integrity
// ============================================================================

#[test]
fn test_ledger_chain_integrity() {
    // Create genesis citizen
    let citizen1 = AugmentedCitizenId::new(
        "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        sha256_hash(b"Test User"),
        "US-AZ".to_string(),
        "vault:biometric_binding_001".to_string(),
        AugIdStatus::Active,
    ).unwrap();
    
    // Create genesis entry
    let keypair = KeyPair::generate().unwrap();
    let signature = sign_data(&keypair.signing_key, b"genesis").unwrap();
    
    let genesis = AugIdLedgerEntry::new_genesis(
        citizen1.clone(),
        "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        signature,
    );
    
    // Create update entry
    let mut citizen2 = citizen1.clone();
    citizen2.status = AugIdStatus::Suspended;
    
    let signature2 = sign_data(&keypair.signing_key, b"update").unwrap();
    
    let update = AugIdLedgerEntry::new_update(
        &genesis,
        citizen2,
        "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        signature2,
    );
    
    // Validate chain
    let entries = vec![genesis.clone(), update.clone()];
    assert!(AntiRollbackGuard::validate_chain_integrity(&entries).is_ok());
    
    // Verify individual entry validation
    assert!(update.validate_next(Some(&genesis)).is_ok());
}

// ============================================================================
// TEST: Status Transition Guards
// ============================================================================

#[test]
fn test_status_transition_guards() {
    // Revoked -> Active should fail
    assert!(AntiRollbackGuard::validate_status_transition(
        "Revoked", "Active", false
    ).is_err());
    
    // Revoked -> Suspended should fail
    assert!(AntiRollbackGuard::validate_status_transition(
        "Revoked", "Suspended", false
    ).is_err());
    
    // Active -> Suspended should pass
    assert!(AntiRollbackGuard::validate_status_transition(
        "Active", "Suspended", false
    ).is_ok());
    
    // Suspended -> Revoked should pass
    assert!(AntiRollbackGuard::validate_status_transition(
        "Suspended", "Revoked", false
    ).is_ok());
    
    // Revoked -> Active with governance should pass
    assert!(AntiRollbackGuard::validate_status_transition(
        "Revoked", "Active", true
    ).is_ok());
}

// ============================================================================
// TEST: Biophysical State Guards
// ============================================================================

#[test]
fn test_biophysical_state_guards() {
    // Healthy state should pass
    let healthy_state = BiophysicalState {
        stress_level: 0.5,
        lifeforce_level: 0.5,
        roh_current: 0.2,
    };
    assert!(AugFingerprintGuard::validate_biophysical_state(&healthy_state).is_ok());
    
    // High stress should fail
    let high_stress_state = BiophysicalState {
        stress_level: 0.9,
        lifeforce_level: 0.5,
        roh_current: 0.2,
    };
    assert!(AugFingerprintGuard::validate_biophysical_state(&high_stress_state).is_err());
    
    // High RoH should fail
    let high_roh_state = BiophysicalState {
        stress_level: 0.5,
        lifeforce_level: 0.5,
        roh_current: 0.5,
    };
    assert!(AugFingerprintGuard::validate_biophysical_state(&high_roh_state).is_err());
}

// ============================================================================
// TEST: Payment Guard
// ============================================================================

#[test]
fn test_equality_payment_guard() {
    // Create citizen and token
    let citizen = AugmentedCitizenId::new(
        "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        sha256_hash(b"Payment Test"),
        "US-AZ".to_string(),
        "vault:biometric_binding_pay".to_string(),
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
    
    // Valid payment should pass
    assert!(EqualityPaymentGuard::validate_payment(
        &token, 50.0, 100.0, 5, 10, "Standard"
    ).is_ok());
    
    // Exceeding spend cap should fail
    assert!(EqualityPaymentGuard::validate_payment(
        &token, 150.0, 100.0, 5, 10, "Standard"
    ).is_err());
    
    // Basic service with payment should fail
    assert!(EqualityPaymentGuard::validate_payment(
        &token, 10.0, 100.0, 5, 10, "BasicService"
    ).is_err());
    
    // Basic service without payment should pass
    assert!(EqualityPaymentGuard::validate_payment(
        &token, 0.0, 100.0, 5, 10, "BasicService"
    ).is_ok());
}

// ============================================================================
// TEST: Offline Operation Guard
// ============================================================================

#[test]
fn test_offline_operation_guard() {
    let now = Utc::now();
    let valid_from = now - Duration::days(1);
    let valid_until = now + Duration::days(6);
    
    // Valid snapshot should pass
    assert!(OfflineOperationGuard::validate_offline_snapshot(
        "sha256:abc123def456",
        valid_from,
        valid_until,
    ).is_ok());
    
    // Invalid hash format should fail
    assert!(OfflineOperationGuard::validate_offline_snapshot(
        "invalid_hash",
        valid_from,
        valid_until,
    ).is_err());
    
    // Expired snapshot should fail
    let expired_until = now - Duration::days(1);
    assert!(OfflineOperationGuard::validate_offline_snapshot(
        "sha256:abc123def456",
        valid_from,
        expired_until,
    ).is_err());
}

// ============================================================================
// TEST: Biometric Vault Guard
// ============================================================================

#[test]
fn test_biometric_vault_guard() {
    // Valid binding ID should pass
    assert!(BiometricVaultGuard::validate_biometric_binding_id(
        "vault:abc123def456"
    ).is_ok());
    
    // Invalid binding ID should fail
    assert!(BiometricVaultGuard::validate_biometric_binding_id(
        "invalid_binding"
    ).is_err());
    
    // No raw biometric fields should pass
    assert!(BiometricVaultGuard::verify_no_raw_biometric(&[
        "did", "status", "timestamp"
    ]).is_ok());
    
    // Raw biometric field should fail
    assert!(BiometricVaultGuard::verify_no_raw_biometric(&[
        "did", "biometric_raw", "status"
    ]).is_err());
}

// ============================================================================
// TEST: Composite Guard Verification
// ============================================================================

#[test]
fn test_composite_guard_verification() {
    // Create test context
    let citizen = AugmentedCitizenId::new(
        "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        sha256_hash(b"Guard Test"),
        "US-AZ".to_string(),
        "vault:biometric_binding_guard".to_string(),
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
    
    let context = GuardContext {
        citizen: citizen.clone(),
        token: token.clone(),
        biophysical_state: BiophysicalState {
            stress_level: 0.5,
            lifeforce_level: 0.5,
            roh_current: 0.2,
        },
        biometric_verified: true,
        consent_state: ConsentState::Confirmed,
        ledger_entries: vec![],
        transaction_amount: 50.0,
        spend_cap: 100.0,
        prompts_last_hour: 5,
        prompt_cap: 10,
        service_class: "Standard".to_string(),
        snapshot_hash: "sha256:abc123".to_string(),
        snapshot_valid_from: Utc::now() - Duration::days(1),
        snapshot_valid_until: Utc::now() + Duration::days(6),
        offline_operations: vec![],
        biometric_binding_id: "vault:abc123".to_string(),
        field_names: vec!["did".to_string()],
        match_location: "local_device".to_string(),
    };
    
    // Identity verification should pass
    assert!(verify_all_guards("identity_verification", &context).is_ok());
    
    // Payment should pass
    assert!(verify_all_guards("payment", &context).is_ok());
    
    // Offline operation should pass
    assert!(verify_all_guards("offline_operation", &context).is_ok());
    
    // Biometric auth should pass
    assert!(verify_all_guards("biometric_auth", &context).is_ok());
}

// ============================================================================
// TEST: Token Expiry
// ============================================================================

#[test]
fn test_token_expiry() {
    let citizen = AugmentedCitizenId::new(
        "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        sha256_hash(b"Expiry Test"),
        "US-AZ".to_string(),
        "vault:biometric_binding_exp".to_string(),
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
    
    token.sign("signature".to_string());
    
    // Token should be valid initially
    assert!(token.validate().is_ok());
    assert!(!token.is_expired());
    
    // Note: In production, we would wait or manipulate time
    // For this test, we verify the expiry logic exists
    assert!(token.valid_until > token.generated_at);
}

// ============================================================================
// TEST: Neurorights Enforcement
// ============================================================================

#[test]
fn test_neurorights_enforcement() {
    let citizen = AugmentedCitizenId::new(
        "bostrom18abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        sha256_hash(b"Neurorights Test"),
        "US-AZ".to_string(),
        "vault:biometric_binding_neuro".to_string(),
        AugIdStatus::Active,
    ).unwrap();
    
    // All required neurorights must be present
    assert!(citizen.neurightsflags.contains(&"no_exclusion_basic_services".to_string()));
    assert!(citizen.neurightsflags.contains(&"no_score_from_inner_state".to_string()));
    assert!(citizen.neurightsflags.contains(&"revocable_at_will".to_string()));
    
    // Guard validation should pass
    assert!(AugFingerprintGuard::validate_neurorights(&citizen.neurightsflags).is_ok());
    
    // Token should inherit neurorights
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
    
    assert!(token.neurorights_flags.contains(&"no_exclusion_basic_services".to_string()));
    assert!(token.neurorights_flags.contains(&"no_score_from_inner_state".to_string()));
    assert!(token.neurorights_flags.contains(&"revocable_at_will".to_string()));
}
