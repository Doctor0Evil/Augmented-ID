# Augmented-ID Threat Model v1

## Attacker classes

- Underage user
  - Goal: bypass age gates by VPN, credential sharing, spoofing, or social engineering. [file:1]
- Compromised site or relying party
  - Goal: exfiltrate identity, reconstruct profiles, or link behavior across origins using tokens and metadata. [file:1]
- Compromised verifier
  - Goal: retain or resell PII contrary to HB2112 no-retention and minimal-disclosure rules. [file:1]
- Network attacker
  - Goal: harvest tokens in transit, replay assertions, or downgrade cryptography to weaken origin/nonce binding. [file:1]
- Platform/OS/browser
  - Goal: correlate Augmented-ID activity across sites using extension calls, IPC metadata, or wallet telemetry. [file:1]

## Assets

- Age credentials
  - HB2112-compliant verifiable credentials encoding “age_over” plus jurisdiction and method metadata. [file:1]
- Wallet keys
  - Master seed, per-origin keypairs, and issuer-trust anchors used for DID derivation and token signatures. [file:1]
- Unlock factors
  - Local PINs, device biometrics, and optional BCI-based unlock flags that gate access to wallet keys. [file:1]
- Logs
  - Non-identifying audit records proving that a compliant method guarded each access without exposing PII. [file:1]
- Issuer keys
  - Verifier signing keys and associated policy that bind age attestations to HB2112-permitted methods. [file:1]
- Policy metadata
  - Local rules for consent, parental controls, jurisdiction mapping, and per-site preferences that constrain issuance. [file:1]
