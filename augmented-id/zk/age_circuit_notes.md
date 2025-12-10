# ZK Age-Over-Threshold Circuit

- Input: commitment to date-of-birth or birth-year inside the VC, plus public threshold and jurisdiction code. [file:1]
- Prove: that `current_time - dob >= threshold_years` without leaking the exact age or date. [file:1]
- Implementation: small-range circuit over BLS12-381 using Rust libraries, targeting sub-200ms proof generation on midrange devices and compact proof size for web transport. [file:1]
- Fallback: if ZK is not available, rely on selective disclosure where the issuer encodes `age_over` as a boolean with minimal additional metadata. [file:1]
