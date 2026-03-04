use augmented_id::{
    AugmentedId,
    RightsFlags,
    DutyFlags,
    AuStatus,
    BfcIdGuard,
    BfcIdToken,
    BfcIdView,
};
use bfc_core::consent::AiConsentState;

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
        primary: augmented_id::DutyClass::EcoCivic,
        cooperation_neurosafe_only: true,
        civic_bounty_opt_in: true,
        civic_bounty_scope_limit_ack: true,
        no_speculative_finance: true,
        reputation_from_verified_actions: true,
        anti_stigma_commitment: true,
        public_explanation_channel: true,
    }
}

fn phoenix_augmented_id() -> AugmentedId {
    AugmentedId {
        shard_id: aln_core::ShardId::from("phoenix-au-id"),
        au_status: AuStatus::OrganicallyIntegratedAugmentedCitizen,
        rights: base_rights(),
        duties: base_duties(),
        issued_at: chrono::Utc::now(),
        epoch_version: 3,
    }
}

// Phoenix ServiceClassBasic: groceries, pharmacy, water, transit.
fn phoenix_caps_ok() -> augmented_id::CapsOk {
    augmented_id::CapsOk {
        max_prompts_per_hour_ok: true,
        max_payments_per_hour_ok: true,
        max_daily_spend_ok: true,
    }
}

// Eco profile tuned to your BioPay / Phoenix eco corridors.
fn phoenix_eco_flags() -> augmented_id::EcoFlags {
    augmented_id::EcoFlags {
        eco_impact_band: "neutral-to-positive".into(),
        e_accessibility: 0.9,
        service_class_basic: true,
    }
}

#[test]
fn golden_bfc_id_view_for_phoenix_basic_services() {
    let id = phoenix_augmented_id();

    // Example DID derived from your Bostrom primary (pseudonymous).
    let wallet_did = "did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7";

    // Implanted NFC aura in a Phoenix ServiceClassBasic corridor.
    let interface_type = "implanted_nfc";

    let ai_state = AiConsentState::Confirmed;
    let caps_ok = phoenix_caps_ok();
    let eco_flags = phoenix_eco_flags();

    let token = BfcIdGuard::from_shards(
        &id,
        wallet_did,
        interface_type,
        ai_state,
        caps_ok,
        eco_flags,
    )
    .expect("BfcIdToken should be constructible for Phoenix basic services");

    // Merchant‑facing view.
    let view = augmented_id::BfcIdGuard::to_view(&token);

    // Golden expectations for Phoenix ServiceClassBasic.
    assert_eq!(view.wallet_did, wallet_did);
    assert_eq!(view.interface_type, "implanted_nfc");
    assert_eq!(view.ai_consent_state, AiConsentState::Confirmed);
    assert!(view.caps_ok.max_daily_spend_ok);
    assert!(view.caps_ok.max_payments_per_hour_ok);
    assert!(view.caps_ok.max_prompts_per_hour_ok);
    assert!(view.eco_flags.service_class_basic);
    assert!(view.eco_flags.e_accessibility >= 0.8);

    let nr = &view.neurorights_flags;
    assert!(nr.no_exclusion_basic_services);
    assert!(nr.no_neuro_coercion);
    assert!(nr.no_score_from_inner_state);
    assert!(nr.revocable_at_will);
}

#[test]
fn bfc_id_guard_rejects_when_caps_exceeded() {
    let id = phoenix_augmented_id();
    let wallet_did = "did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7";
    let interface_type = "implanted_nfc";

    let ai_state = AiConsentState::Confirmed;
    let caps_bad = augmented_id::CapsOk {
        max_prompts_per_hour_ok: false,
        max_payments_per_hour_ok: true,
        max_daily_spend_ok: true,
    };
    let eco_flags = phoenix_eco_flags();

    let res = BfcIdGuard::from_shards(
        &id,
        wallet_did,
        interface_type,
        ai_state,
        caps_bad,
        eco_flags,
    );

    assert!(
        res.is_err(),
        "BfcIdGuard must reject tokens when caps are exceeded"
    );
}

#[test]
fn bfc_id_guard_rejects_when_consent_not_confirmed() {
    let id = phoenix_augmented_id();
    let wallet_did = "did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7";
    let interface_type = "implanted_nfc";

    let ai_state = AiConsentState::Deny; // or Suspended
    let caps_ok = phoenix_caps_ok();
    let eco_flags = phoenix_eco_flags();

    let res = BfcIdGuard::from_shards(
        &id,
        wallet_did,
        interface_type,
        ai_state,
        caps_ok,
        eco_flags,
    );

    assert!(
        res.is_err(),
        "BfcIdGuard must reject tokens when AI consent is not Confirmed"
    );
}
