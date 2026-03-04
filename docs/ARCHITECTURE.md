# Augmented-ID System Architecture

## Overview

Augmented-ID is a soul-bound, offline-first identity framework designed exclusively for augmented citizens within the ALN ecosystem. This document describes the complete system architecture, component interactions, and data flows.

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         AUGMENTED CITIZIN DEVICE                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │ Biometric   │  │ Consent     │  │ BFC         │  │ Identity    │    │
│  │ Vault       │  │ Engine      │  │ Service     │  │ Shard       │    │
│  │ (Local)     │  │ (Local)     │  │ (Local)     │  │ (ROWRPM)    │    │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘    │
│         │                │                │                │            │
│         └────────────────┴────────────────┴────────────────┘            │
│                              │                                          │
│                    ┌─────────▼─────────┐                                │
│                    │  BfcToken.v1      │                                │
│                    │  (Minimal View)   │                                │
│                    └─────────┬─────────┘                                │
└──────────────────────────────┼──────────────────────────────────────────┘
                               │ NFC / BFC Channel
                               ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           VERIFIER / RELYING PARTY                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │ Token       │  │ Guard       │  │ VC          │  │ Legacy      │    │
│  │ Validator   │  │ Validator   │  │ Resolver    │  │ Gateway     │    │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘    │
└─────────────────────────────────────────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           ALN LEDGER LAYER                               │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │ Organichain │  │ ROWRPM      │  │ CodeAnchor  │  │ Googolswarm │    │
│  │ (Anchor)    │  │ (Shard)     │  │ (Snapshot)  │  │ (Address)   │    │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘    │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Core Components

### 1. AugmentedCitizenID Shard

The soul-bound identity anchor stored in ROWRPM:

- **Immutable**: Append-only, no in-place updates
- **Anchored**: Linked to Organichain via `organichainroot`
- **Auditable**: Full history via `rowledgerroot`
- **Neurorights-Bound**: Hard-coded constraints in every record

### 2. Biometric Vault

Local secure storage for biometric templates:

- **Device-Bound**: Keys stored in secure enclave
- **Local-Only**: Matching happens on-device
- **No Transmission**: Raw biometric data never leaves device
- **Signed Proof**: Only "ok/failed" proof is transmitted

### 3. Consent Engine

Local decision-making for identity exposure:

- **Biophysical-Aware**: Considers St/Lt (Stress/LifeForce) levels
- **RoH Compliant**: Enforces Rate-of-Harm ≤ 0.3
- **Revocable**: All delegations revocable at will
- **Transparent**: Full audit log of consent decisions

### 4. BFC Service

Bounded Forward Channel for secure communication:

- **Envelope-Bound**: All messages bounded by mobility/topology
- **Schema-Declared**: Every message declares expected schema
- **Short-Lived**: Tokens expire in 5 minutes
- **Minimalist**: Only essential fields exposed

### 5. Verifier Gateway

Legacy system integration layer:

- **Rust-Based**: High-performance validation
- **Guard-Enforced**: All guards run before acceptance
- **Delegation-Logging**: All delegations logged to ROWRPM
- **Reclamation-Safe**: Citizens can reclaim ownership anytime

---

## Data Flows

### Identity Verification Flow

```
1. Citizen presents BfcToken.v1 via NFC
2. Verifier validates token structure and expiry
3. Verifier checks neurorights flags present
4. Verifier validates signature against DID
5. Verifier checks snapshot hash against Organichain
6. Decision: Accept or Reject
```

### Payment Authorization Flow

```
1. Citizen initiates payment via BFC
2. Consent engine checks biophysical state
3. Biometric vault verifies match locally
4. BfcToken.v1 generated with caps_ok flags
5. EqualityPaymentGuard validates caps
6. Transaction logged to ROWRPM
7. Payment processed via wallet DID
```

### Offline Operation Flow

```
1. Device stores signed snapshot with CodeAnchor
2. Offline operations logged locally
3. On reconnection, reconcile with Organichain
4. Conflicts detected and rejected
5. New entries anchored to chain
```

---

## Security Layers

| Layer | Mechanism | Purpose |
|-------|-----------|---------|
| **Cryptographic** | Ed25519 signatures, SHA-256 hashes | Data integrity and authenticity |
| **Ledger** | ROWRPM forward-only, anti-rollback | History immutability |
| **Biometric** | Local vault, no transmission | Privacy protection |
| **Consent** | Biophysical state checks | User safety |
| **Neurorights** | Hard-coded flags in all tokens | Rights enforcement |
| **Network** | BFC envelopes, mobility bounds | Communication security |

---

## Integration Points

### With Organichain

- Snapshot anchoring via `organichainroot`
- CodeAnchor references for offline validation
- Periodic reconciliation on reconnection

### With ROWRPM

- All identity operations logged as RowEvents
- Merkle root tracking via `rowledgerroot`
- Chain integrity verification

### With Legacy Systems

- Rust gateway translates legacy requests to BFC
- Delegation credentials logged to citizen's ROWRPM
- Revocation via new ROWRPM entry (no external calls)

---

## Performance Characteristics

| Operation | Latency (Offline) | Latency (Online) |
|-----------|-------------------|------------------|
| Token Generation | < 10ms | < 10ms |
| Token Validation | < 5ms | < 5ms |
| Biometric Match | < 100ms | < 100ms |
| Ledger Append | N/A | < 1s |
| Organichain Anchor | N/A | < 5s |

---

## Scalability Considerations

- **Per-Citizen Shards**: Each citizen has independent ROWRPM shard
- **No Global State**: Verifiers only need citizen's snapshot
- **Offline-First**: Network load minimized by local validation
- **Minimal Tokens**: BfcToken.v1 is < 1KB serialized

---

## Future Extensions

1. **Zero-Knowledge Proofs**: For attribute verification without disclosure
2. **Multi-Sig Governance**: For exceptional status transitions
3. **Cross-Chain Bridges**: For interoperability with non-ALN systems
4. **Hardware Security Modules**: For enhanced key protection
```
