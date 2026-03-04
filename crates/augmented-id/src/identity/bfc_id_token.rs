use serde::{Deserialize, Serialize};
use thiserror::Error;
use chrono::{DateTime, Utc};

use super::augmented_id::{AugmentedId, RightsFlags};
use bfc_core::consent::AiConsentState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapsOk {
    pub max_prompts_per_hour_ok: bool,
    pub max_payments_per_hour_ok: bool,
    pub max_daily_spend_ok: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoFlags {
    pub eco_impact_band: String,  // e.g., "low", "neutral", "high"
    pub e_accessibility: f32,     // 0.0–1.0
    pub service_class_basic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurightsFlagsView {
    pub no_exclusion_basic_services: bool,
    pub no_score_from_inner_state: bool,
    pub no_neuro_coercion: bool,
    pub revocable_at_will: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BfcIdToken {
    pub wallet_did: String,
    pub interface_type: String, // "implanted_nfc", "EcoNFC", etc.
    pub ai_consent_state: AiConsentState,
    pub caps_ok: CapsOk,
    pub eco_flags: EcoFlags,
    pub neurorights_flags: NeurightsFlagsView,
    pub issued_at: DateTime<Utc>,
    pub nonce: String,
}

/// Minimal merchant/agency view: no timestamps beyond what they need.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BfcIdView {
    pub wallet_did: String,
    pub interface_type: String,
    pub ai_consent_state: AiConsentState,
    pub caps_ok: CapsOk,
    pub eco_flags: EcoFlags,
    pub neurorights_flags: NeurightsFlagsView,
}

#[derive(Debug, Error)]
pub enum BfcIdGuardError {
    #[error("missing or degraded neurorights flags")]
    NeurorightsDegraded,
    #[error("consent not confirmed")]
    ConsentNotConfirmed,
    #[error("caps exceeded")]
    CapsExceeded,
    #[error("epoch mismatch with augmented id")]
    EpochMismatch,
}

pub struct BfcIdGuard;

impl BfcIdGuard {
    /// Construct a BfcIdToken from the qpudatashard BfcToken.v1 shard,
    /// and ensure it is consistent with AugmentedId.
    pub fn from_shards(
        id: &AugmentedId,
        wallet_did: &str,
        interface_type: &str,
        ai_state: AiConsentState,
        caps_ok: CapsOk,
        eco_flags: EcoFlags,
    ) -> Result<BfcIdToken, BfcIdGuardError> {
        let neurorights = Self::rights_to_view(&id.rights);

        if !Self::check_neurorights(&neurorights) {
            return Err(BfcIdGuardError::NeurorightsDegraded);
        }

        if ai_state != AiConsentState::Confirmed {
            return Err(BfcIdGuardError::ConsentNotConfirmed);
        }

        if !caps_ok.max_daily_spend_ok
            || !caps_ok.max_payments_per_hour_ok
            || !caps_ok.max_prompts_per_hour_ok
        {
            return Err(BfcIdGuardError::CapsExceeded);
        }

        Ok(BfcIdToken {
            wallet_did: wallet_did.to_string(),
            interface_type: interface_type.to_string(),
            ai_consent_state: ai_state,
            caps_ok,
            eco_flags,
            neurorights_flags: neurorights,
            issued_at: Utc::now(),
            nonce: Self::make_nonce(),
        })
    }

    pub fn to_view(token: &BfcIdToken) -> BfcIdView {
        BfcIdView {
            wallet_did: token.wallet_did.clone(),
            interface_type: token.interface_type.clone(),
            ai_consent_state: token.ai_consent_state,
            caps_ok: token.caps_ok.clone(),
            eco_flags: token.eco_flags.clone(),
            neurorights_flags: token.neurorights_flags.clone(),
        }
    }

    fn rights_to_view(r: &RightsFlags) -> NeurightsFlagsView {
        NeurightsFlagsView {
            no_exclusion_basic_services: r.no_exclusion_basic_services,
            no_score_from_inner_state: r.no_score_from_inner_state,
            no_neuro_coercion: r.no_neuro_coercion,
            revocable_at_will: r.revocable_at_will,
        }
    }

    fn check_neurorights(v: &NeurightsFlagsView) -> bool {
        v.no_exclusion_basic_services
            && v.no_score_from_inner_state
            && v.no_neuro_coercion
            && v.revocable_at_will
    }

    fn make_nonce() -> String {
        // Pluggable: use your Googolswarm/Bostrom randomness.
        format!("bfcid-{}", Utc::now().timestamp_nanos())
    }
}
