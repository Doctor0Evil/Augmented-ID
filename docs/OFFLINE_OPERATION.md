# Augmented-ID Offline Operation Guide

## Overview

Augmented-ID is designed for 100% offline capability with no fallback mechanisms required. This document describes how offline operations work, their security guarantees, and reconciliation procedures.

---

## Offline Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     OFFLINE MODE OPERATION                       │
│                                                                  │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐         │
│  │ Local       │    │ Local       │    │ Local       │         │
│  │ Identity    │    │ Consent     │    │ Biometric   │         │
│  │ Snapshot    │    │ Engine      │    │ Vault       │         │
│  │ (Cached)    │    │ (Active)    │    │ (Active)    │         │
│  └──────┬──────┘    └──────┬──────┘    └──────┬──────┘         │
│         │                  │                  │                 │
│         └──────────────────┴──────────────────┘                 │
│                          │                                      │
│                  ┌───────▼───────┐                              │
│                  │ BfcToken.v1   │                              │
│                  │ Generation    │                              │
│                  └───────┬───────┘                              │
│                          │                                      │
│                  ┌───────▼───────┐                              │
│                  │ NFC / BFC     │                              │
│                  │ Presentation  │                              │
│                  └───────┬───────┘                              │
│                          │                                      │
│                          ▼                                      │
│                  ┌───────────────┐                              │
│                  │ Verifier      │                              │
│                  │ (Offline)     │                              │
│                  └───────────────┘                              │
└─────────────────────────────────────────────────────────────────┘
```

---

## Offline Capabilities

### What Works Offline

| Feature | Status | Notes |
|---------|--------|-------|
| Identity Presentation | ✅ Full | BfcToken.v1 generation and validation |
| Biometric Authentication | ✅ Full | Local matching only |
| Consent Decisions | ✅ Full | Local consent engine active |
| Payment Authorization | ✅ Full | Caps enforcement local |
| VC Presentation | ✅ Full | Cached credentials |
| Transaction Logging | ✅ Local | Queued for reconciliation |

### What Requires Network

| Feature | Status | Notes |
|---------|--------|-------|
| Organichain Anchoring | ❌ Deferred | Queued until reconnection |
| Delegation Revocation | ⚠️ Local | Effective locally, anchored later |
| New VC Issuance | ❌ Deferred | Requires issuer online |
| Cross-Citizen Verification | ❌ Deferred | Requires ledger access |

---

## Offline Snapshot Management

### Snapshot Structure

```json
{
  "snapshot_hash": "sha256:abc123...",
  "snapshot_valid_from": "2026-03-05T00:00:00Z",
  "snapshot_valid_until": "2026-03-12T00:00:00Z",
  "citizen_shard": { ... },
  "cached_vcs": [ ... ],
  "codeanchor_ref": "anchor:xyz789..."
}
```

### Snapshot Validity

- **Default Validity**: 7 days from creation
- **Maximum Validity**: 30 days (configurable)
- **Auto-Refresh**: When network available, snapshot refreshed
- **Expiry Handling**: Expired snapshots rejected by verifiers

### Snapshot Storage

- **Location**: Device secure storage
- **Encryption**: AES-256-GCM with device-bound key
- **Integrity**: SHA-256 hash verified on every access
- **Backup**: Encrypted backup to citizen's cloud vault (optional)

---

## Offline Transaction Flow

### Step 1: Pre-Offline Preparation

```
1. Device connects to network
2. Latest snapshot fetched from Organichain
3. Snapshot cached with validity window
4. CodeAnchor reference stored
5. Network disconnection detected
```

### Step 2: Offline Operation

```
1. Citizen initiates operation (payment, ID check, etc.)
2. Biometric match performed locally
3. Consent engine evaluates biophysical state
4. BfcToken.v1 generated from cached snapshot
5. Token presented to verifier via NFC/BFC
6. Verifier validates token offline
7. Transaction logged to local queue
```

### Step 3: Reconnection and Reconciliation

```
1. Device detects network availability
2. Local transaction queue uploaded
3. Organichain anchor verified
4. Conflicts detected and resolved
5. New entries anchored to chain
6. Snapshot refreshed
7. Local queue cleared
```

---

## Conflict Detection and Resolution

### Conflict Types

| Type | Detection | Resolution |
|------|-----------|------------|
| Double-Spend | Duplicate transaction ID | Reject second transaction |
| Status Conflict | Divergent status updates | Latest timestamp wins |
| Delegation Conflict | Overlapping delegations | Most restrictive applies |
| Snapshot Conflict | Hash mismatch | Re-fetch from Organichain |

### Conflict Resolution Rules

1. **Timestamp Priority**: Later timestamps take precedence
2. **Citizen Authority**: Citizen-signed operations override delegated
3. **Governance Override**: Governance events override all
4. **Audit Trail**: All conflicts logged for review

---

## Offline Security Guarantees

### What Is Protected Offline

| Asset | Protection | Mechanism |
|-------|------------|-----------|
| Private Keys | Full | Secure enclave, biometric lock |
| Biometric Templates | Full | Local vault, no transmission |
| Identity History | Full | Snapshot hash verification |
| Neurorights | Full | Hard-coded in token schema |

### What Is Limited Offline

| Feature | Limitation | Reason |
|---------|------------|--------|
| Revocation Propagation | Delayed | Requires network |
| New Credential Issuance | Blocked | Requires issuer |
| Cross-Verification | Blocked | Requires ledger |
| Governance Events | Queued | Requires consensus |

---

## Offline Verifier Requirements

### Minimum Verifier Setup

```rust
// Verifier must implement:
1. Token structure validation
2. Expiry checking
3. Neurorights flag verification
4. Signature verification (cached keys)
5. Snapshot hash validation (cached anchors)
```

### Verifier Cache Management

- **Key Cache**: Citizen DIDs and public keys (30-day TTL)
- **Anchor Cache**: Organichain snapshot hashes (7-day TTL)
- **Revocation Cache**: Known revoked DIDs (30-day TTL)
- **Sync Frequency**: Every 24 hours when online

---

## Offline Operation Limits

### Transaction Limits

| Operation | Offline Limit | Online Limit |
|-----------|---------------|--------------|
| Payment Amount | $100/day | Configurable |
| Transaction Count | 10/day | Configurable |
| VC Presentations | Unlimited | Unlimited |
| Identity Checks | Unlimited | Unlimited |

### Rationale

- **Payment Limits**: Reduce fraud risk during offline period
- **Transaction Count**: Detect anomalous behavior
- **VC/ID Unlimited**: Essential services must remain accessible

---

## Reconnection Procedures

### Automatic Reconnection

```
1. Network detected
2. TLS handshake with Organichain node
3. Authenticate with citizen DID
4. Upload queued transactions
5. Download new anchor points
6. Verify chain integrity
7. Refresh local snapshot
8. Clear local queue
```

### Manual Reconnection (If Automatic Fails)

```
1. Citizen initiates manual sync
2. QR code or NFC tap to trusted node
3. Encrypted data transfer
4. Same verification as automatic
5. Confirmation to citizen
```

### Reconnection Failure Handling

- **Retry Schedule**: Exponential backoff (1m, 5m, 15m, 1h, 6h)
- **Maximum Queue**: 1000 transactions (oldest dropped)
- **Citizen Notification**: Alert after 24 hours offline
- **Emergency Mode**: Basic services only after 7 days

---

## Offline Testing Checklist

- [ ] Token generation works without network
- [ ] Biometric match succeeds offline
- [ ] Consent engine evaluates correctly
- [ ] Verifier validates tokens offline
- [ ] Transaction queue persists across reboots
- [ ] Reconciliation succeeds on reconnection
- [ ] Conflicts detected and resolved correctly
- [ ] Expired snapshots rejected
- [ ] Payment limits enforced offline
- [ ] Neurorights flags present in all tokens
```
