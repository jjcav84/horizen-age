//! # horizen-age — Privacy-Preserving Age Verification on Horizen Base L3
//!
//! Horizen adaptation of [zk-age](https://github.com/jjcav84/zk-age) —
//! replaces zkVerify submission with ZEN token staking and ZenKinetic
//! privacy gate integration.
//!
//! ## How it works
//!
//! 1. **Issuer** signs a birthdate commitment (same as zk-age)
//! 2. **User** generates a ZK proof that age >= threshold (same circuit)
//! 3. **ZenKinetic gate** scores the proof's negentropy and determines fees
//! 4. **ZEN staking** grants access — Pro tier (1,000 ZEN) required for age proofs
//! 5. **Proof settles** on Horizen Base L3 with privacy-preserving fee = 0%
//!
//! ## Quick Start
//!
//! ```rust
//! use horizen_age::AgeProofSession;
//!
//! let session = AgeProofSession::new(18, 0.95, 1_000.0);
//! let result = session.evaluate();
//! println!("Gate: {:?}", result.gate_decision);
//! println!("Fee: {} bps", result.fee_bps);
//! println!("Negentropy: {:.1} bits", result.negentropy_bits);
//! ```

pub mod session;
pub mod types;

pub use session::AgeProofSession;
pub use types::{AgeProofResult, AgeThreshold};

/// Re-export zenkinetic gate for direct access.
pub use zenkinetic::{PrivacyGate, TransactionProfile, GateDecision};
/// Re-export negentropy for scoring.
pub use negentropy::{Negentropy, RouteEnergy, Committor};
