use augmented_id::{
    AugmentedId,
    RightsFlags,
    DutyFlags,
    AuStatus,
    VerifiableCredential,
    VcGuard,
};
use chrono::{Duration, Utc};

fn base_rights() -> RightsFlags {
    RightsFlags {
        no_exclusion_basic_services: true,
        no_neuro_coercion: true,
        revocable_at_will: true,
        no_score_from_inner_state: true,
        augmentation_continuity: true,
        project_continuity_rust_aln_bostrom: true,
    }
}

fn base_duties() -> DutyFlags {
    DutyFlags {
        primary: augmented_id::DutyClass::PeaceKeeping,
        cooperation_neurosafe_only: true,
        civic_bounty_opt_in: true,
        civic_bounty_scope_limit_ack: true,
        no_speculative_finance: true,
        reputation_from_verified_actions: true,
        anti_stigma_commitment: true,
        public_explanation_channel: true,
    }
}

fn base_augmented_id() -> AugmentedId {
    AugmentedId {
        shard_id: aln_core::ShardId::from("test-au-id"),
        au_status: AuStatus::OrganicallyIntegratedAugmentedCitizen,
        rights: base_rights(),
        duties: base_duties(),
        issued_at: Utc::now(),
        epoch_version: 3,
    }
}

#[test]
fn augmented_id_fails_when_rights_drop() {
    let mut id = base_augmented_id();
    // Intentionally degrade a neuroright
    id.rights.no_exclusion_basic_services = false;

    let res = id.validate();
    assert!(
        res.is_err(),
        "AugmentedId::validate() should fail when neurorights are weakened"
    );
}

#[test]
fn vc_rejected_when_epoch_older_than_id() {
    let id = base_augmented_id();
    let now = Utc::now();

    let vc = VerifiableCredential {
        id: "vc-epoch-old".into(),
        subject_did: "did:bostrom:test".into(),
        issuer_did: "did:bostrom:issuer".into(),
        issued_at: now - Duration::days(1),
        expires_at: Some(now + Duration::days(30)),
        epoch: id.epoch_version - 1, // stale
        rights_snapshot: base_rights(),
        context: "payment".into(),
        signature_hex: "deadbeef".into(),
    };

    let eval = VcGuard::evaluate(&vc, &id);
    assert!(
        eval.is_err(),
        "VC with epoch lower than AugmentedId should be rejected"
    );
}

#[test]
fn vc_rejected_when_rights_snapshot_degraded() {
    let id = base_augmented_id();
    let now = Utc::now();

    let mut degraded_rights = base_rights();
    degraded_rights.no_exclusion_basic_services = false;

    let vc = VerifiableCredential {
        id: "vc-rights-degraded".into(),
        subject_did: "did:bostrom:test".into(),
        issuer_did: "did:bostrom:issuer".into(),
        issued_at: now - Duration::days(1),
        expires_at: Some(now + Duration::days(30)),
        epoch: id.epoch_version + 1, // future epoch
        rights_snapshot: degraded_rights,
        context: "payment".into(),
        signature_hex: "deadbeef".into(),
    };

    let eval = VcGuard::evaluate(&vc, &id);
    assert!(
        eval.is_err(),
        "VC with rights weaker than AugmentedId should be rejected"
    );
}

#[test]
fn vc_ok_when_epoch_and_rights_match_or_strengthen() {
    let id = base_augmented_id();
    let now = Utc::now();

    // Rights identical to ID.
    let vc = VerifiableCredential {
        id: "vc-match".into(),
        subject_did: "did:bostrom:test".into(),
        issuer_did: "did:bostrom:issuer".into(),
        issued_at: now,
        expires_at: Some(now + Duration::days(30)),
        epoch: id.epoch_version, // equal epoch
        rights_snapshot: base_rights(),
        context: "payment".into(),
        signature_hex: "deadbeef".into(),
    };

    let eval = VcGuard::evaluate(&vc, &id).expect("VC should be accepted");
    assert!(eval.accepted);
    assert!(eval.epoch_ok);
    assert!(eval.rights_ok);
}
