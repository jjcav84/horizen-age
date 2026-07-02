<p align="center">
  <a href="https://www.orkidlabs.com"><img src="assets/logo.png" alt="Orkid Labs" width="220" /></a>
</p>

# horizen-age — Privacy-Preserving Age Verification on Horizen Base L3

> *Prove you're old enough without revealing your age. Pay zero fees for
> preserving privacy.*
>
> **By [Orkid Labs](https://www.orkidlabs.com)** — privacy-first crypto engineering

Horizen adaptation of [zk-age](https://github.com/jjcav84/zk-age) —
replaces zkVerify submission with ZEN token staking and
[ZenKinetic](https://github.com/jjcav84/zenkinetic) privacy gate
integration.

[![License: MIT](https://img.shields.io/badge/License-MIT-a78bfa.svg)](LICENSE)
[![Horizen](https://img.shields.io/badge/Horizen-Base%20L3-ff6b35.svg)](https://horizen.org)
[![negentropy](https://img.shields.io/badge/powered%20by-negentropy-a78bfa.svg)](https://github.com/jjcav84/negentropy)

## How it works

1. **Issuer** signs a birthdate commitment (same as zk-age)
2. **User** generates a ZK proof that age >= threshold (same circuit)
3. **ZenKinetic gate** scores the proof's negentropy — privacy-preserving = 0% fee
4. **ZEN staking** grants access — Pro tier (1,000 ZEN) required
5. **Proof settles** on Horizen Base L3 as a confidential transaction

## Quick Start

```rust
use horizen_age::AgeProofSession;

let session = AgeProofSession::new(18, 0.95, 1_000.0);
let result = session.evaluate();

println!("Gate: {:?}", result.gate_decision);    // Allow
println!("Fee: {} bps", result.fee_bps);          // 0
println!("Negentropy: {:.1} bits", result.negentropy_bits);  // 70.9
println!("Access: {}", result.access_granted);    // true
```

## ZEN Token Utility

| Stake Tier | Min ZEN | Fee Discount | Age Proof Access |
|-----------|---------|-------------|-----------------|
| Basic | 100 | 25% off | — |
| Pro | 1,000 | 50% off | ✓ |
| Max | 10,000 | 75% off | ✓ |

## Architecture

```
horizen-age
├── depends on → negentropy (physics scoring)
├── depends on → zenkinetic (privacy gate + ZEN staking)
├── adapts → zk-age (ZK age verification circuit)
└── deploys on → Horizen Base L3
```

## Origin

This is the Horizen-native adaptation of [zk-age](https://github.com/jjcav84/zk-age).
The ZK circuit and proof generation are the same; the chain integration
changes from zkVerify to Horizen Base L3 with ZEN staking and ZenKinetic
privacy gating.

## Thrive Horizen Boost Program (#39) — Grant Plan

### Ecosystem value proposition

horizen-age brings privacy-preserving age verification to Horizen Base L3. This is the Horizen-native adaptation of [zk-age](https://github.com/jjcav84/zk-age) — an existing, working project migrating to Horizen infrastructure. The ZK circuit stays the same; the chain integration changes from zkVerify to Horizen Base L3 with ZEN staking and ZenKinetic privacy gating.

### Milestone roadmap

Progressive achievement over 120 days, following Thrive's Horizen Boost Program milestone structure.

**Application Requirements (10% unlocked at approval)**:
- ✅ Comprehensive audit of the existing project (zk-age) with a detailed privacy enhancement plan
- ✅ Clear migration and deployment strategy to Horizen infrastructure
- ✅ Demonstrated traction and user base on the current platform with verifiable metrics
- ✅ Privacy feature roadmap and technical implementation timeline

**Milestone 1 (20% unlocked) — 30 days post approval**:
- Successful deployment and integration with Horizen privacy features
- Privacy capabilities successfully integrated into the existing application
- User migration plan executed with active user transition

**Milestone 2 (30% unlocked) — 75 days post approval**:
- Privacy features improving user experience
- Integration with other Horizen ecosystem projects
- Growth metrics (choose one):
  - TVL: $250K+ in ZEN locked in smart contracts, staking, or liquidity pools
  - Volume: 50K+ transactions demonstrating privacy preservation
  - Unique Wallets: 1,000+ verified users utilizing privacy features

**Milestone 3 (40% unlocked) — 120 days post approval**:
- Become a successful case study for the Horizen ecosystem
- Scale metrics (choose one):
  - TVL: $500K+ in ZEN locked in smart contracts, staking, or liquidity pools
  - Volume: 100K+ transactions demonstrating privacy preservation
  - Unique Wallets: 2,500+ verified users utilizing privacy features

## Ecosystem

Part of the negentropy-powered privacy stack for Horizen:

- [negentropy](https://github.com/jjcav84/negentropy) — shared physics engine
- [zenkinetic](https://github.com/jjcav84/zenkinetic) — thermodynamic privacy gate
- [horizen-age](https://github.com/jjcav84/horizen-age) — **this repo**
- [horizen-attest](https://github.com/jjcav84/horizen-attest) — ZK attestations
- [horizen-ballot](https://github.com/jjcav84/horizen-ballot) — anonymous voting

## About

Built by [Orkid Labs](https://www.orkidlabs.com) — a privacy-first crypto
engineering lab building thermodynamic infrastructure for decentralized
systems.

## License

MIT — see [LICENSE](LICENSE).
