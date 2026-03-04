![ALN Compliance Charter](https://img.shields.io/badge/ALN%20Compliance-Enforced-brightgreen)
![KYC / DID Verified](https://img.shields.io/badge/KYC%20%2F%20DID-Verified-blue)
![Immutable Ledger](https://img.shields.io/badge/Ledger-Blockchain%20Secured-orange)
![Audit-Ready](https://img.shields.io/badge/Audit-Continuous%20Monitoring-yellow)
![Neural Networking Ready](https://img.shields.io/badge/Neural%20Networking-Governed%20Use-purple)
![BCI/EEG Local Only](https://img.shields.io/badge/BCI%20%2F%20EEG-Device%20Local%20Only-informational)
![Age Attestation](https://img.shields.io/badge/Age%20Checks-ZK%20Attestation%20Ready-success)
![Jurisdiction Profiles](https://img.shields.io/badge/Jurisdiction-HB2112%20%2B%20Global%20Profiles-blueviolet)
![On-Device Privacy](https://img.shields.io/badge/Privacy-On--Device%20Wallet-lightgrey)
![Smart City Ready](https://img.shields.io/badge/Smart%20City-Virtual%20Node%20Ready-brightgreen)

![Perplexity Logo](https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png)

Augmented-ID is a standards-based, cross-platform age-verification and DID credential wallet that automates 18+ checks with strong privacy guarantees and no repeated selfies or document uploads. It is designed to plug into any browser, app, or platform as a “yes/no” age oracle that satisfies laws like Arizona HB2112 while reducing data exposure for everybody.[1][2][3][4]


# Augmented-ID

**A Soul-Bound, Offline-First Identity Framework for Augmented Citizens**

*Version: 1.0.0 | ALN-Native | Organichain-Anchored*

---

## Overview

Augmented-ID is a cryptographically-anchored, immutable identity system designed exclusively for augmented citizens within the ALN ecosystem. It provides:

- **Soul-bound identity shards** that cannot be transferred, duplicated, or silently altered
- **100% offline capability** with no fallback mechanisms required
- **Forward-only ledger semantics** prohibiting rollbacks, reversals, and downgrades
- **Real-time biometric confirmation** that never leaves the user's device
- **Minimalist token exposure** via Bounded Forward Channels (BFC)

---

## Core Principles

| Principle | Implementation |
|-----------|----------------|
| **Immutability** | Append-only ROWRPM ledger with anti-rollback invariants |
| **Self-Sovereignty** | Citizen-owned DID with revocable delegation only |
| **Privacy** | Local biometric matching; zero-knowledge proof support |
| **Offline-First** | Signed snapshots with CodeAnchor reconciliation |
| **Neurorights** | Hard-coded constraints in all VC and token schemas |

---

## Repository Structure

```
Augmented-ID/
├── aln/
│   ├── schemas/          # ALN schema definitions
│   ├── channels/         # BFC channel specifications
│   └── guards/           # Guard and invariant definitions
├── rust/
│   ├── src/              # Rust implementation core
│   └── Cargo.toml        # Rust dependencies
├── lua/
│   └── augid_bfc_client.lua  # Lightweight client stub
├── docs/
│   ├── ARCHITECTURE.md   # System architecture overview
│   ├── SECURITY_MODEL.md # Threat model and guarantees
│   └── OFFLINE_OPERATION.md  # Offline workflow documentation
├── specs/
│   └── ANTI_ROLLBACK_PROOF.md  # Formal security properties
├── README.md
└── LICENSE
```

---

## Quick Start

### Prerequisites

- ALN Compiler v2026.1+
- Rust 1.75+ with `serde`, `ed25519-dalek`, `sha2` crates
- Organichain node access (for anchoring)

### Building the Rust Core

```bash
cd rust
cargo build --release
```

### Compiling ALN Schemas

```bash
aln-compile aln/schemas/*.aln --output ./compiled/
```

---

## Security Notice

This system is designed with **no fallback mechanisms**. All security guarantees depend on:

1. Proper key management in local biometric vaults
2. Correct implementation of ROWRPM forward-only semantics
3. Adherence to neurorights envelope constraints

**Do not modify guard logic without formal verification.**

---

## License

MIT License — See LICENSE file for terms.

---

## Contact

Repository Owner: Doctor0Evil
ALN Ecosystem: Organichain / Googolswarm / Bostrom
```
