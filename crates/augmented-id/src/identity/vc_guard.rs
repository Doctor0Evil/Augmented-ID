use serde::{Deserialize, Serialize};
use thiserror::Error;
use chrono::{DateTime, Utc};

use super::augmented_id::{AugmentedId, RightsFlags};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableCredential {
    pub id: String,
    pub subject_did: String,
    pub issuer_did: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub epoch: u32,
    pub rights_snapshot: RightsFlags,
    pub context: String, // e.g., "payment", "civic", "eco-reward"
    pub signature_hex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VcEvaluation {
    pub accepted: bool,
    pub reason: String,
    pub epoch_ok: bool,
    pub rights_ok: bool,
}

#[derive(Debug, Error)]
pub enum VcGuardError {
    #[error("VC expired at {0}")]
    Expired(DateTime<Utc>),
    #[error("epoch out of range: vc={vc_epoch}, id={id_epoch}")]
    EpochMismatch { vc_epoch: u32, id_epoch: u32 },
    #[error("rights degradation detected")]
    RightsDegraded,
    #[error("signature invalid")]
    InvalidSignature,
}

pub struct VcGuard;

impl VcGuard {
    /// Evaluate VC against the user's AugmentedId.
    pub fn evaluate(vc: &VerifiableCredential, id: &AugmentedId) -> Result<VcEvaluation, VcGuardError> {
        let now = Utc::now();

        if let Some(exp) = vc.expires_at {
            if exp < now {
                return Err(VcGuardError::Expired(exp));
            }
        }

        if vc.epoch < id.epoch_version {
            return Err(VcGuardError::EpochMismatch {
                vc_epoch: vc.epoch,
                id_epoch: id.epoch_version,
            });
        }

        // Rights must not be weaker than the ID's rights.
        if !Self::rights_superset(&vc.rights_snapshot, &id.rights) {
            return Err(VcGuardError::RightsDegraded);
        }

        // Signature check is delegated to your DID/Googolswarm stack.
        if !Self::verify_signature(vc) {
            return Err(VcGuardError::InvalidSignature);
        }

        Ok(VcEvaluation {
            accepted: true,
            reason: "ok".to_string(),
            epoch_ok: true,
            rights_ok: true,
        })
    }

    fn rights_superset(vc: &RightsFlags, id: &RightsFlags) -> bool {
        (!id.no_exclusion_basic_services || vc.no_exclusion_basic_services)
            && (!id.no_neuro_coercion || vc.no_neuro_coercion)
            && (!id.revocable_at_will || vc.revocable_at_will)
            && (!id.no_score_from_inner_state || vc.no_score_from_inner_state)
            && (!id.augmentation_continuity || vc.augmentation_continuity)
            && (!id.project_continuity_rust_aln_bostrom
                || vc.project_continuity_rust_aln_bostrom)
    }

    fn verify_signature(_vc: &VerifiableCredential) -> bool {
        // Placeholder: wire to Bostrom/Googolswarm VC verification.
        true
    }
}
