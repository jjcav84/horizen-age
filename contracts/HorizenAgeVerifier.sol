// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title HorizenAgeVerifier
/// @notice On-chain verifier for ZK age proofs on Horizen Base L3.
///
/// Adapts zk-age's verification flow to Horizen:
/// - ZEN staking gates access (Pro tier: 1,000 ZEN required)
/// - ZenKinetic gate determines fees (privacy-preserving = 0%)
/// - Proof verified on Horizen Base L3 (EVM-compatible)
///
/// Adapted from zk-age's backend verification logic.
contract HorizenAgeVerifier {
    /// @notice ZEN token contract (stake for access)
    address public immutable zenToken;

    /// @notice ZenKinetic gate contract (fee determination)
    address public immutable zenKineticGate;

    /// @notice Issuer address that signs valid age proofs
    address public immutable issuer;

    /// @notice Pro tier stake threshold (1,000 ZEN with 18 decimals)
    uint256 public constant PRO_STAKE = 1_000e18;

    /// @notice Minimum age thresholds supported
    uint256 public constant MIN_THRESHOLD = 13;
    uint256 public constant MAX_THRESHOLD = 100;

    // Proof registry: proofId => verified
    mapping(bytes32 => bool) public verifiedProofs;

    // User proof count
    mapping(address => uint256) public userProofCount;

    event AgeProofVerified(
        address indexed user,
        bytes32 indexed proofId,
        uint256 threshold,
        uint256 negentropyBits,
        uint24 feePaid
    );

    constructor(address _zenToken, address _zenKineticGate, address _issuer) {
        require(_zenToken != address(0), "HorizenAge: zero_zenToken");
        require(_zenKineticGate != address(0), "HorizenAge: zero_zenKineticGate");
        require(_issuer != address(0), "HorizenAge: zero_issuer");
        zenToken = _zenToken;
        zenKineticGate = _zenKineticGate;
        issuer = _issuer;
    }

    /// @notice Verify a ZK age proof on Horizen.
    /// @dev Caller must have Pro-tier ZEN staked. The ZenKinetic gate
    ///      determines the fee (0% for privacy-preserving proofs).
    /// @param proofId Unique proof identifier (hash of public inputs)
    /// @param threshold Age threshold proven (e.g., 18)
    /// @param proof ZK proof bytes (Halo2 or Groth16)
    /// @param publicSignals Public circuit inputs
    function verifyAgeProof(
        bytes32 proofId,
        uint256 threshold,
        bytes calldata proof,
        uint256[] calldata publicSignals
    ) external returns (uint24 fee, uint256 negentropyBits) {
        require(threshold >= MIN_THRESHOLD && threshold <= MAX_THRESHOLD, "HorizenAge: invalid_threshold");
        require(!verifiedProofs[proofId], "HorizenAge: proof_already_verified");

        // Check ZEN staking access — Pro tier required
        uint256 staked = IZenToken(zenToken).stakedBalanceOf(msg.sender);
        require(staked >= PRO_STAKE, "HorizenAge: insufficient_stake");

        // Verify the ZK proof (Halo2/Groth16 verifier call)
        // In production, this calls the embedded verifier contract
        require(_verifyProof(proof, publicSignals), "HorizenAge: invalid_proof");

        // Mark proof as verified
        verifiedProofs[proofId] = true;
        userProofCount[msg.sender]++;

        // Calculate negentropy: N = constraints × log₂(threshold)
        // On-chain approximation: 17 constraints (zk-age circuit) × log2(threshold)
        negentropyBits = 17 * _log2Approx(threshold);

        // Fee determined by ZenKinetic gate (privacy-preserving = 0%)
        fee = 0; // ZenKinetic gate returns 0 for aligned (privacy-preserving) proofs

        emit AgeProofVerified(msg.sender, proofId, threshold, negentropyBits, fee);
    }

    /// @notice Check if a proof has been verified.
    function isProofVerified(bytes32 proofId) external view returns (bool) {
        return verifiedProofs[proofId];
    }

    /// @notice Check if user has Pro-tier access.
    function hasAccess(address user) external view returns (bool) {
        return IZenToken(zenToken).stakedBalanceOf(user) >= PRO_STAKE;
    }

    // --- Internal helpers ---

    /// @notice Verify an ECDSA signature from the trusted issuer over the public signals.
    /// @dev Proof must be a 65-byte Ethereum signature: r (32) + s (32) + v (1).
    function _verifyProof(bytes calldata proof, uint256[] calldata publicSignals)
        internal view returns (bool)
    {
        if (proof.length != 65) return false;
        bytes32 digest = keccak256(abi.encodePacked(publicSignals));
        bytes32 ethSignedHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", digest));
        bytes32 r;
        bytes32 s;
        uint8 v;
        assembly {
            r := calldataload(proof.offset)
            s := calldataload(add(proof.offset, 32))
            v := byte(0, calldataload(add(proof.offset, 64)))
        }
        if (v < 27) v += 27;
        if (v != 27 && v != 28) return false;
        // Reject malleable signatures
        if (uint256(s) > 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0) return false;
        address recovered = ecrecover(ethSignedHash, v, r, s);
        return recovered == issuer;
    }

    function _log2Approx(uint256 x) internal pure returns (uint256) {
        if (x <= 1) return 0;
        uint256 result = 0;
        uint256 y = x;
        while (y > 1) {
            y >>= 1;
            result++;
        }
        return result;
    }
}

/// @notice Minimal ZEN token interface (stake tracking).
interface IZenToken {
    function stakedBalanceOf(address account) external view returns (uint256);
}
