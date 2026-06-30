//! Types for Horizen age verification.

use serde::{Deserialize, Serialize};

/// Age threshold to prove (e.g., 18, 21, 65).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgeThreshold {
    Over18,
    Over21,
    Over65,
    Custom(u64),
}

impl AgeThreshold {
    pub fn value(&self) -> u64 {
        match self {
            Self::Over18 => 18,
            Self::Over21 => 21,
            Self::Over65 => 65,
            Self::Custom(v) => *v,
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::Over18 => "18+".to_string(),
            Self::Over21 => "21+".to_string(),
            Self::Over65 => "65+".to_string(),
            Self::Custom(v) => format!("{}+", v),
        }
    }
}

/// Result of a Horizen age proof evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeProofResult {
    /// ZenKinetic gate decision
    pub gate_decision: String,
    /// Fee in basis points (after ZEN stake discount)
    pub fee_bps: u32,
    /// Negentropy extracted by the proof (bits)
    pub negentropy_bits: f64,
    /// Privacy alignment score (0..1)
    pub alignment: f64,
    /// Committor probability (proof validity confidence)
    pub committor: f64,
    /// ZEN stake tier
    pub stake_tier: String,
    /// Age threshold proven
    pub threshold: String,
    /// Whether ZEN staking grants access to this proof type
    pub access_granted: bool,
}
