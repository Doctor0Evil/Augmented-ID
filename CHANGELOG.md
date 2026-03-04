# Changelog

All notable changes to Augmented-ID will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Initial repository structure
- Core ALN schemas for AugmentedCitizenID
- Verifiable Credential schemas (Health, License, Eco, Peacekeeper, Education, Identity)
- Bounded Forward Channel (BFC) specification
- Guard definitions for neurorights and anti-rollback
- Rust core implementation (citizen, ledger, token, guards, crypto)
- Lua client stub for BFC operations
- Comprehensive documentation (Architecture, Security Model, Offline Operation)
- Formal anti-rollback security proofs
- Integration test suite

### Security
- All neurorights flags hard-coded in schemas
- Anti-rollback guards at schema and application level
- Biometric vault security (local-only matching)
- Offline operation security guarantees
- No external dependencies with known vulnerabilities

---

## [1.0.0] - 2026-03-05

### Added
- **Initial Release** of Augmented-ID framework

#### Core Components
- `AugmentedCitizenID` shard with soul-bound properties
- `VCCoreEnvelope` for all Verifiable Credentials
- `BoundedForwardChannel` for secure communication
- `BfcTokenV1` for minimalist verifier tokens
- `AugFingerprintGuard` for neurorights enforcement
- `EqualityPaymentGuard` for transaction security
- `AntiRollbackGuard` for ledger integrity
- `OfflineOperationGuard` for offline security
- `BiometricVaultGuard` for biometric privacy

#### Verifiable Credentials
- `au.vc.health.v1` - Health attestations
- `au.vc.license.v1` - License/credential proofs
- `au.vc.eco.v1` - Eco-contribution records
- `au.vc.peacekeeper.v1` - Civic contribution credentials
- `au.vc.education.v1` - Education credentials
- `au.vc.identity.v1` - Base identity attestations

#### Rust Implementation
- Full `augid_core` library
- Command-line interface binary
- Cryptographic utilities (Ed25519, SHA-256, AES-256-GCM)
- Comprehensive test suite (unit + integration)

#### Documentation
- Architecture overview
- Security model and threat analysis
- Offline operation guide
- Formal anti-rollback proofs
- Contributing guidelines
- Changelog

### Security Guarantees
- **Immutability**: Append-only ROWRPM ledger
- **Forward-Only**: Monotonic timestamps, no rollback
- **Neurorights**: Hard-coded in all tokens and VCs
- **Biometric Privacy**: Local-only matching, no transmission
- **Offline-Capable**: 100% offline operation without fallbacks

### Known Limitations
- Zero-knowledge proof support planned for future release
- Cross-chain bridges not yet implemented
- Hardware security module integration planned
- Formal machine-checked proofs (future work)

---

## Version History

| Version | Release Date | Status |
|---------|--------------|--------|
| 1.0.0 | 2026-03-05 | Released |
| 0.9.0 | 2026-02-15 | Beta |
| 0.5.0 | 2026-01-01 | Alpha |

---

## Upcoming (Planned)

### Version 1.1.0
- Zero-knowledge proof support for attribute verification
- Multi-signature governance for exceptional status transitions
- Enhanced biometric vault security (TEE integration)

### Version 1.2.0
- Cross-chain bridge specifications
- Hardware security module support
- Formal machine-checked proofs (Coq/TLA+)

### Version 2.0.0
- Full decentralized governance model
- Community-driven VC schema library
- Interoperability with non-ALN identity systems

---

## Migration Guide

### From 0.9.0 to 1.0.0
- No breaking changes
- All schemas backward compatible
- Update Rust dependencies to latest versions

### From 0.5.0 to 0.9.0
- Neurorights flags now mandatory in all tokens
- Anti-rollback guards enforced at compile time
- Biometric vault binding ID format changed to `vault:*`

---

## Security Advisories

| Advisory ID | Date | Severity | Description |
|-------------|------|----------|-------------|
| AUGID-2026-001 | 2026-03-01 | Low | Token expiry validation edge case (fixed in 1.0.0) |

---

## Contributors

- Doctor0Evil (Repository Owner)
- ALN Ecosystem Contributors
- Organichain Foundation
- Community Contributors

For full contributor list, see GitHub contributors page.
```
