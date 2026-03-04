use serde::{Deserialize, Serialize};
use thiserror::Error;
use chrono::{DateTime, Utc};

use aln_core::ShardId;
use qpudatashards::shard::{ShardEnvelope, ShardPayload};

#[derive(Debug, Error)]
pub enum AugmentedIdError {
    #[error("shard not found: {0}")]
    ShardNotFound(String),
    #[error("invalid status for augmented citizen: {0}")]
    InvalidStatus(String),
    #[error("required neurorights flag missing: {0}")]
    MissingRight(&'static str),
    #[error("duty profile invalid: {0}")]
    InvalidDuty(&'static str),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

/// Direct mapping of auorgintegratedcitizencompat2026.aln
/// (simplified to core fields; ALN generator should produce this).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RightsFlags {
    pub no_exclusion_basic_services: bool,
    pub no_neuro_coercion: bool,
    pub revocable_at_will: bool,
    pub no_score_from_inner_state: bool,
    pub augmentation_continuity: bool,
    pub project_continuity_rust_aln_bostrom: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DutyClass {
    PeaceKeeping,
    CivicSupport,
    EcoCivic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DutyFlags {
    pub primary: DutyClass,
    pub cooperation_neurosafe_only: bool,
    pub civic_bounty_opt_in: bool,
    pub civic_bounty_scope_limit_ack: bool,
    pub no_speculative_finance: bool,
    pub reputation_from_verified_actions: bool,
    pub anti_stigma_commitment: bool,
    pub public_explanation_channel: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuStatus {
    OrganicallyIntegratedAugmentedCitizen,
    // Other status enums if present in ALN…
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AugmentedId {
    pub shard_id: ShardId,
    pub au_status: AuStatus,
    pub rights: RightsFlags,
    pub duties: DutyFlags,
    pub issued_at: DateTime<Utc>,
    pub epoch_version: u32,
}

impl AugmentedId {
    /// Load from a generic qpudatashard envelope.
    pub fn from_shard(envelope: &ShardEnvelope) -> Result<Self, AugmentedIdError> {
        if envelope.schema != "au.org.integrated.citizen.compat.2026" {
            return Err(AugmentedIdError::ShardNotFound(envelope.schema.clone()));
        }

        let payload = match &envelope.payload {
            ShardPayload::Json(json) => json,
            _ => {
                return Err(AugmentedIdError::Serde(serde_json::Error::custom(
                    "non-JSON payload",
                )))
            }
        };

        let id: AugmentedId = serde_json::from_value(payload.clone())?;
        id.validate()?;
        Ok(id)
    }

    /// Hard invariants from your neurorights/duty headers.
    pub fn validate(&self) -> Result<(), AugmentedIdError> {
        match self.au_status {
            AuStatus::OrganicallyIntegratedAugmentedCitizen => {}
            _ => {
                return Err(AugmentedIdError::InvalidStatus(format!(
                    "{:?}",
                    self.au_status
                )))
            }
        }

        if !self.rights.no_exclusion_basic_services {
            return Err(AugmentedIdError::MissingRight(
                "no_exclusion_basic_services",
            ));
        }
        if !self.rights.no_neuro_coercion {
            return Err(AugmentedIdError::MissingRight("no_neuro_coercion"));
        }
        if !self.rights.revocable_at_will {
            return Err(AugmentedIdError::MissingRight("revocable_at_will"));
        }
        if !self.rights.no_score_from_inner_state {
            return Err(AugmentedIdError::MissingRight("no_score_from_inner_state"));
        }

        if !self.duties.no_speculative_finance {
            return Err(AugmentedIdError::InvalidDuty("no_speculative_finance"));
        }
        if !self.duties.reputation_from_verified_actions {
            return Err(AugmentedIdError::InvalidDuty(
                "reputation_from_verified_actions",
            ));
        }

        Ok(())
    }

    /// Helper: succinct human‑readable preamble for logs/UI.
    pub fn preamble(&self) -> String {
        format!(
            "As an organically‑integrated augmented citizen (epoch {}), \
             I retain neurorights and non‑exclusion from basic services, \
             and earn reputation only from verified peacekeeping, civic, \
             and ecopositive actions.",
            self.epoch_version
        )
    }
}
