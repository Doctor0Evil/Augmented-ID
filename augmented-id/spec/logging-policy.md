# Logging and No-Retention Policy

## Site logs

- Store only: hash(token_id), jurisdiction_code, method_class, timestamp, decision (allow/deny). [file:1]
- Do not store: holder_pseudonymous_did, IP address, device identifiers, or any credential payload fields. [file:1]

## Verifier logs

- Maintain records needed for regulated KYC/AML obligations, separated from HB2112 age-attestation flow. [file:1]
- Do not retain full ID images or numbers solely to support future age checks; long-lived evidence is represented by the VC held in the user wallet. [file:1]
