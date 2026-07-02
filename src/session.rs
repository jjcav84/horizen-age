//! Age proof session — evaluates a ZK age proof through the ZenKinetic gate.

use crate::types::{AgeProofResult, AgeThreshold};
use zenkinetic::{PrivacyGate, TransactionProfile};
use zenkinetic::staking::StakeTier;

/// Configuration for a Horizen age verification session.
///
/// Adapts zk-age's `ProofPotential` to the Horizen ecosystem:
/// - ZEN staking replaces zkVerify fees
/// - ZenKinetic gate scores privacy alignment
/// - Negentropy scoring remains the same (via negentropy crate)
#[derive(Debug, Clone)]
pub struct AgeProofSession {
    /// Age threshold to prove (e.g., 18, 21)
    pub threshold: AgeThreshold,
    /// Issuer trust score (0..1) — same as zk-age
    pub issuer_trust: f64,
    /// ZEN tokens staked by the user
    pub zen_staked: f64,
    /// Proof age in seconds (for recency decay)
    pub proof_age_secs: f64,
    /// Proof generation latency in ms
    pub proof_latency_ms: u64,
    /// Circuit constraint count (zk-age default: 17)
    pub constraint_count: u64,
}

impl Default for AgeProofSession {
    fn default() -> Self {
        Self {
            threshold: AgeThreshold::Over18,
            issuer_trust: 0.95,
            zen_staked: 1_000.0, // Pro tier
            proof_age_secs: 0.0,
            proof_latency_ms: 800,
            constraint_count: 17, // zk-age circuit: 17 non-linear constraints
        }
    }
}

impl AgeProofSession {
    pub fn new(threshold: u64, issuer_trust: f64, zen_staked: f64) -> Self {
        assert!(issuer_trust.is_finite() && (0.0..=1.0).contains(&issuer_trust), "issuer_trust must be in [0,1]");
        assert!(zen_staked.is_finite() && zen_staked >= 0.0, "zen_staked must be non-negative and finite");
        Self {
            threshold: AgeThreshold::Custom(threshold),
            issuer_trust,
            zen_staked,
            ..Default::default()
        }
    }

    /// Evaluate the age proof through the ZenKinetic privacy gate.
    ///
    /// This replaces zk-age's `ProofPotential::energy()` + zkVerify submission
    /// with a ZenKinetic gate evaluation that determines fees based on
    /// privacy alignment and ZEN staking.
    pub fn evaluate(&self) -> AgeProofResult {
        let threshold_val = self.threshold.value();

        // Build a ZenKinetic transaction profile for the age proof
        let profile = TransactionProfile {
            has_zk_proof: true,
            constraint_count: self.constraint_count,
            anonymity_set_bits: 0, // age proofs don't have anonymity sets
            proof_age_secs: self.proof_age_secs,
            proof_latency_ms: self.proof_latency_ms,
            verify_latency_ms: 27, // Horizen L3 verify is fast
            zen_staked: self.zen_staked,
        };

        let gate = PrivacyGate::evaluate(&profile);

        // Check ZEN staking access — Pro tier required for age proofs
        let stake_tier = StakeTier::from_staked(self.zen_staked);
        let access_granted = stake_tier.grants_confidential_transfer();

        // Negentropy: N = constraints × log₂(threshold)
        let negentropy_bits =
            negentropy::Negentropy::from_constraints(self.constraint_count, threshold_val).bits();

        AgeProofResult {
            gate_decision: format!("{:?}", gate.decision),
            fee_bps: gate.discounted_fee_bps,
            negentropy_bits,
            alignment: gate.alignment,
            committor: gate.committor,
            stake_tier: stake_tier.label().to_string(),
            threshold: self.threshold.label(),
            access_granted,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_age_proof_aligned() {
        let session = AgeProofSession::default();
        let result = session.evaluate();

        assert_eq!(result.gate_decision, "Allow");
        assert!(result.negentropy_bits > 0.0);
        assert!(result.access_granted);
    }

    #[test]
    fn test_insufficient_stake_denied() {
        let session = AgeProofSession {
            zen_staked: 50.0, // Below Basic tier
            ..Default::default()
        };
        let result = session.evaluate();

        assert!(!result.access_granted);
    }

    #[test]
    fn test_higher_threshold_more_negentropy() {
        let low = AgeProofSession::new(13, 0.9, 1_000.0).evaluate();
        let high = AgeProofSession::new(25, 0.9, 1_000.0).evaluate();

        assert!(high.negentropy_bits > low.negentropy_bits);
    }

    #[test]
    fn test_stale_proof_lower_energy() {
        let fresh = AgeProofSession::default().evaluate();
        let stale = AgeProofSession {
            proof_age_secs: 7200.0,
            ..Default::default()
        }
        .evaluate();

        assert!(stale.alignment < fresh.alignment);
    }

    #[test]
    fn test_negentropy_formula() {
        // 17 constraints, threshold 18: N = 17 * log2(18) ≈ 70.9 bits
        let session = AgeProofSession::new(18, 0.9, 1_000.0);
        let result = session.evaluate();
        let expected = 17.0 * (18.0f64).log2();
        assert!((result.negentropy_bits - expected).abs() < 0.01);
    }
}
