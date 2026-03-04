# Augmented-ID Security Model

## Threat Model

This document defines the threat model for Augmented-ID, including assumed attacker capabilities, protected assets, and security guarantees.

---

## Assumptions

### Trusted Components

1. **Citizen Device Secure Enclave**: Hardware-backed key storage is trusted
2. **Local Consent Engine**: Runs on citizen's device under their control
3. **ALN Compiler**: Schema validation is correct and uncompromised
4. **Organichain Consensus**: Ledger consensus is honest majority

### Untrusted Components

1. **Verifier Terminals**: May be compromised or malicious
2. **Network Channels**: All communications may be intercepted
3. **Legacy Gateways**: May attempt to exceed delegation scope
4. **Other Citizens**: May attempt identity theft or fraud

---

## Attacker Capabilities

| Capability | Mitigation |
|------------|------------|
| Network eavesdropping | All messages encrypted, minimal token exposure |
| Terminal compromise | Tokens short-lived, neurorights flags enforced |
| Key theft (device) | Secure enclave, biometric binding required |
| Ledger manipulation | ROWRPM forward-only, Organichain anchoring |
| Coercion attacks | Consent state can be SUSPENDED, basic services protected |
| Replay attacks | Token expiry, nonce-based signatures |
| Rollback attacks | Anti-rollback guards, monotonic timestamps |

---

## Protected Assets

### High Value

1. **Citizen DID Private Keys**: Never leave secure enclave
2. **Biometric Templates**: Never leave device, never transmitted
3. **Identity History**: Immutable via ROWRPM anchoring
4. **Neurorights Constraints**: Hard-coded, cannot be bypassed

### Medium Value

1. **Verifiable Credentials**: Revocable, time-limited
2. **Transaction History**: Auditable via ROWRPM
3. **Delegation Credentials**: Scoped, logged, revocable

### Low Value

1. **BfcToken.v1**: Short-lived, minimal data
2. **Public DID**: Pseudonymous, no sensitive data

---

## Security Guarantees

### Cryptographic Guarantees

1. **Unforgeability**: Ed25519 signatures cannot be forged
2. **Integrity**: SHA-256 hashes detect any tampering
3. **Confidentiality**: AES-256-GCM encryption for sensitive data
4. **Non-Repudiation**: All operations signed by DID keys

### Ledger Guarantees

1. **Immutability**: ROWRPM append-only, no deletions
2. **Forward-Only**: Timestamps monotonically increasing
3. **Anti-Rollback**: Status can tighten, never loosen without governance
4. **Anchoring**: Organichain provides external integrity check

### Neurorights Guarantees

1. **No Exclusion**: Basic services cannot be denied
2. **No Inner-State Scoring**: Mental/biophysical state not scored
3. **Revocable Delegations**: Citizen can reclaim control anytime
4. **Local Biometric Only**: Raw data never transmitted

---

## Attack Scenarios and Mitigations

### Scenario 1: Stolen Device

**Attack**: Attacker obtains citizen's device

**Mitigations**:
- Secure enclave requires biometric to access keys
- Biometric vault locked to original user
- Citizen can revoke all delegations via governance event
- New device can be provisioned with new keys

**Residual Risk**: Medium (depends on secure enclave strength)

---

### Scenario 2: Compromised Verifier

**Attack**: Verifier terminal is malicious

**Mitigations**:
- Tokens expire in 5 minutes
- Minimal data exposure (BfcToken.v1)
- Neurorights flags always present and validated
- All requests logged to citizen's ROWRPM

**Residual Risk**: Low (limited data exposure)

---

### Scenario 3: Coercion Attack

**Attack**: Citizen forced to consent under duress

**Mitigations**:
- Consent engine checks biophysical state (St/Lt levels)
- High stress triggers SUSPENDED state automatically
- Basic services cannot be denied even in SUSPENDED state
- Panic gesture can trigger silent alert

**Residual Risk**: Medium (physical coercion hard to prevent)

---

### Scenario 4: Ledger Rollback

**Attack**: Attacker attempts to rewrite identity history

**Mitigations**:
- ROWRPM forward-only by design
- Organichain anchoring provides external check
- Anti-rollback guards at schema and application level
- Merkle root verification on every read

**Residual Risk**: Very Low (computationally infeasible)

---

### Scenario 5: Biometric Database Breach

**Attack**: Attacker attempts to steal biometric data

**Mitigations**:
- No central biometric database exists
- Templates stored only in local device vault
- Raw biometric data never transmitted
- Only signed "ok/failed" proof is shared

**Residual Risk**: Very Low (no central target)

---

## Security Audit Checklist

### Code Level

- [ ] All signatures verified before acceptance
- [ ] All tokens validated for expiry
- [ ] All neurorights flags checked
- [ ] All anti-rollback invariants enforced
- [ ] No raw biometric data in any record

### Deployment Level

- [ ] Secure enclave properly configured
- [ ] Key rotation procedures documented
- [ ] Incident response plan in place
- [ ] Audit logging enabled and monitored
- [ ] Rate limiting on all endpoints

### Operational Level

- [ ] Regular security assessments scheduled
- [ ] Vulnerability disclosure process defined
- [ ] Backup and recovery procedures tested
- [ ] Governance event procedures documented
- [ ] User education materials available

---

## Compliance Considerations

| Regulation | Status | Notes |
|------------|--------|-------|
| GDPR | Partial | Right to erasure conflicts with immutability |
| CCPA | Partial | Similar GDPR conflicts |
| EU AI Act | Compliant | No subliminal or manipulative AI |
| Neuro-Rights | Compliant | Hard-coded neurorights enforcement |
| NIST 800-63B | Compliant | Biometric authentication meets AAL2 |

---

## Security Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Token validation time | < 5ms | Benchmark tests |
| Biometric false accept rate | < 0.001% | Vendor specifications |
| Biometric false reject rate | < 1% | Vendor specifications |
| Ledger integrity checks | 100% | Every read operation |
| Neurorights flag coverage | 100% | Schema validation |

---

## Incident Response

### Severity Levels

1. **Critical**: Key compromise, ledger manipulation
2. **High**: Biometric vault breach, coercion detected
3. **Medium**: Token replay, delegation abuse
4. **Low**: Expired tokens, validation failures

### Response Procedures

1. **Detection**: Automated monitoring and user reports
2. **Containment**: Revoke affected credentials, suspend operations
3. **Eradication**: Patch vulnerabilities, rotate keys
4. **Recovery**: Restore from backup, re-anchor to Organichain
5. **Lessons Learned**: Document and update security model
```
