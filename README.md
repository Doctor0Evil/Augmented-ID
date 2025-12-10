![ALN Compliance Charter](https://img.shields.io/badge/ALN%20Compliance-Enforced-brightgreen)
![KYC/DID Verified](https://img.shields.io/badge/KYC%20%2F%20DID-Verified-blue)
![Immutable Ledger](https://img.shields.io/badge/Ledger-Blockchain%20Secured-orange)
![Audit-Ready](https://img.shields.io/badge/Audit-Continuous%20Monitoring-yellow)
![Neural Networking Ready](https://img.shields.io/badge/Neural%20Networking-Governed%20Use-purple)
![BCI/EEG Local Only](https://img.shields.io/badge/BCI%20%2F%20EEG-Device%20Local%20Only-informational)
![Age Attestation](https://img.shields.io/badge/Age%20Checks-ZK%20Attestation%20Ready-success)
![Jurisdiction Profiles](https://img.shields.io/badge/Jurisdiction-HB2112%20%2B%20Global%20Profiles-blueviolet)
![On-Device Privacy](https://img.shields.io/badge/Privacy-On%2DDevice%20Wallet-lightgrey)
![Smart City Ready](https://img.shields.io/badge/Smart%20City-Virtual%20Node%20Ready-brightgreen)

<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" style="height:64px;margin-right:32px"/>

Augmented-ID is a standards-based, cross-platform age-verification and DID credential wallet that automates 18+ checks with strong privacy guarantees and no repeated selfies or document uploads. It is designed to plug into any browser, app, or platform as a “yes/no” age oracle that satisfies laws like Arizona HB2112 while reducing data exposure for everybody.[1][2][3][4]

***

# Augmented-ID

Augmented-ID is a Web5-style decentralized identity and age-verification layer that issues, stores, and presents cryptographic “age over X in jurisdiction Y” credentials. It focuses on removing friction and invasive checks from age gates while giving regulators and platforms stronger, auditable assurances than current selfie/ID upload flows.[2][5][6][1]

## Core features

- **Automated age verification**  
  - One-time onboarding using compliant methods (digital ID or approved third-party checks).  
  - After onboarding, all age checks are performed automatically in the background or via one-click consent; no repeated uploads or CAPTCHAs.[3][1]

- **Decentralized identity (DID) wallet**  
  - Stores W3C DID-based verifiable credentials proving “age ≥ threshold” and jurisdiction attributes.  
  - Uses pairwise pseudonymous identifiers so each site or app sees a different DID, preventing cross-site tracking.[1]

- **Zero-knowledge and minimal disclosure**  
  - Sites receive only a signed assertion like “adult in AZ” bound to their domain and a short expiry, not date of birth or legal name.  
  - Optional zero-knowledge proofs allow age-over-threshold verification without revealing any underlying age data.[7][1]

- **Regulation-aware by design**  
  - Maps directly onto HB2112’s “digital identification” / “commercially reasonable” verification methods while honoring the statute’s explicit ban on retention of identifying information.[2][3]
  - Extensible policy engine for other age-verification laws and sectoral rules worldwide.[5]

- **Augmented- and accessibility-friendly**  
  - Supports AI-augmented and BCI users by treating neural/biometric signals strictly as local unlock factors; only standard cryptographic tokens ever leave the device.[8][9]
  - Replaces hostile “are you human?” copy with neutral, rights-focused language and simplifies flows for neurodivergent and disabled users.[10]

## How it works

1. **Enrollment**  
   - The user chooses a compliant verifier (e.g., government ID-based, or transactional-data-based service) in their jurisdiction.  
   - The verifier checks age according to local law, then issues a signed age credential (a verifiable credential) to the user’s Augmented-ID wallet and deletes raw ID inputs as required by HB2112-type no-retention rules.[6][2]

2. **Local storage and unlocking**  
   - Credentials are encrypted and stored on the user’s device (or user-controlled secure storage), never on-chain.  
   - Optional local factors (PIN, device biometrics, or privacy-preserving BCI unlock) are used only to access the wallet, not as remote identifiers.[9][8]

3. **On access to restricted content**  
   - A site or app integrates a small client library or extension hook that, when harmful or age-gated material is requested, asks the wallet:  
     - `isAdult(jurisdiction, site_origin)`  
   - The wallet evaluates policy and, if appropriate, returns a signed, short-lived “adult” token bound to `site_origin` and a nonce.  
   - The site verifies the signature with known issuer keys and decides allow/deny without ever seeing personal identity data.[11][1]

4. **Audit and compliance**  
   - Sites and verifiers keep only non-identifying event logs (e.g., hashed token IDs, status, jurisdiction), enabling proof of compliance to regulators without any retained PII, in line with HB2112 and similar statutes.[6][2]

## Integration targets

Augmented-ID is built to be environment-agnostic:

- **Web browsers** – extensions or native browser APIs (where available) to expose the `isAdult` call to scripts and service workers.[1]
- **Mobile and desktop apps** – SDKs wrapping the same protocol for native platforms.  
- **Smart TVs, consoles, and kiosks** – embedded or WebView-based integrations that use local wallets or hardware tokens.[12]
- **Smart city and public systems** – portals, public Wi-Fi, and digital kiosks use the same credential to enforce age gates without learning or storing citizen identity details.[10][12]

## Security and privacy principles

- **No central identity store** – all age credentials live in user-held wallets; issuers only keep what is required for their own regulatory compliance.[5][1]
- **No biometric/BCI data in transit** – EEG/BCI and other biosignals never leave the device; they are used only to decrypt or unlock keys locally.[8][9]
- **Short-lived, domain-bound tokens** – each access proof is unique, time-limited, and tied to a specific origin, preventing replay and token sharing.[1]
- **Compliance-first logging** – logs are designed to demonstrate that a lawful check occurred without creating new surveillance or breach risks.[2][6]

## Goals

Augmented-ID aims to:

- Make underage access to restricted content significantly harder by relying on cryptographically strong, once-verified credentials instead of weak UI gates.[13][1]
- Reduce the privacy and dignity costs of compliance for adults, especially those who are AI-augmented or otherwise marginalized by today’s age-gating designs.[3][10]
- Provide regulators and operators with a practical, interoperable path that is strictly as protective as laws like HB2112 require—while being measurably safer and less intrusive than ID-upload or selfie-based systems.[14][2]
