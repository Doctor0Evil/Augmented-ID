# Augmented-ID Deployment Guide

## Repository Summary

| Metric | Value |
|--------|-------|
| **Total Files** | 28 |
| **ALN Schemas** | 8 |
| **Rust Source Files** | 7 |
| **Documentation Files** | 5 |
| **Test Files** | 1 |
| **Configuration Files** | 3 |
| **Lua Client** | 1 |

---

## Repository Structure

```
Augmented-ID/
├── aln/
│   ├── schemas/
│   │   ├── id.augmented.citizen.v1.aln
│   │   ├── vc.core.envelope.v1.aln
│   │   ├── vc.health.v1.aln
│   │   ├── vc.license.v1.aln
│   │   ├── vc.eco.v1.aln
│   │   ├── vc.peacekeeper.v1.aln
│   │   ├── vc.education.v1.aln
│   │   └── vc.identity.v1.aln
│   ├── channels/
│   │   └── bfc.channel.v1.aln
│   └── guards/
│       └── augid.guard.v1.aln
├── rust/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── main.rs
│   │   ├── error.rs
│   │   ├── citizen.rs
│   │   ├── ledger.rs
│   │   ├── token.rs
│   │   ├── guards.rs
│   │   └── crypto.rs
│   ├── tests/
│   │   └── integration_test.rs
│   └── Cargo.toml
├── lua/
│   └── augid_bfc_client.lua
├── docs/
│   ├── ARCHITECTURE.md
│   ├── SECURITY_MODEL.md
│   └── OFFLINE_OPERATION.md
├── specs/
│   └── ANTI_ROLLBACK_PROOF.md
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── CHANGELOG.md
├── DEPLOYMENT.md
└── .gitignore
```

---

## Deployment Steps

### Step 1: Clone Repository

```bash
git clone https://github.com/Doctor0Evil/Augmented-ID.git
cd Augmented-ID
```

### Step 2: Build Rust Core

```bash
cd rust
cargo build --release
```

### Step 3: Compile ALN Schemas

```bash
cd ../aln
aln-compile schemas/*.aln --output ./compiled/
```

### Step 4: Run Tests

```bash
cd ../rust
cargo test --release
```

### Step 5: Deploy to Production

```bash
# Copy binaries to deployment location
cp target/release/augid_core /usr/local/bin/

# Copy ALN schemas to schema directory
cp -r ../aln/schemas /etc/augmented-id/schemas/

# Configure environment
export AUGID_ORGANICHAIN_NODE=https://organichain.aln
export AUGID_ROWPRPM_PATH=/var/lib/augmented-id/rowrpm
```

---

## System Requirements

### Minimum Requirements

| Component | Requirement |
|-----------|-------------|
| CPU | 2 cores |
| RAM | 4 GB |
| Storage | 10 GB SSD |
| OS | Linux (Ubuntu 22.04+), macOS, Windows |

### Recommended Requirements

| Component | Requirement |
|-----------|-------------|
| CPU | 4+ cores |
| RAM | 8+ GB |
| Storage | 50+ GB NVMe SSD |
| OS | Linux (Ubuntu 22.04 LTS) |

---

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `AUGID_ORGANICHAIN_NODE` | Organichain node URL | `https://organichain.aln` |
| `AUGID_ROWPRPM_PATH` | ROWRPM storage path | `/var/lib/augmented-id/rowrpm` |
| `AUGID_LOG_LEVEL` | Logging level | `info` |
| `AUGID_TOKEN_VALIDITY` | Token validity (seconds) | `300` |
| `AUGID_MAX_ROH` | Maximum Rate-of-Harm | `0.3` |

### Configuration File

```toml
# /etc/augmented-id/config.toml

[organichain]
node_url = "https://organichain.aln"
anchor_frequency = 3600  # seconds

[rowrpm]
storage_path = "/var/lib/augmented-id/rowrpm"
max_entries = 1000000

[security]
max_roh_limit = 0.3
token_validity_secs = 300
biometric_timeout_ms = 5000

[logging]
level = "info"
format = "json"
output = "/var/log/augmented-id/augid.log"
```

---

## Monitoring

### Health Checks

```bash
# Check service status
systemctl status augmented-id

# Check ledger integrity
augid_core ledger verify --chain /var/lib/augmented-id/rowrpm/chain.json

# Check guard status
augid_core guard check --citizen /etc/augmented-id/citizen.json
```

### Metrics

| Metric | Endpoint | Description |
|--------|----------|-------------|
| `augid_tokens_generated` | `/metrics` | Total tokens generated |
| `augid_validations_passed` | `/metrics` | Successful validations |
| `augid_validations_failed` | `/metrics` | Failed validations |
| `augid_guard_violations` | `/metrics` | Guard violation count |
| `augid_offline_operations` | `/metrics` | Offline operation count |

---

## Backup and Recovery

### Backup Procedure

```bash
# Backup ROWRPM shard
tar -czf rowrpm_backup_$(date +%Y%m%d).tar.gz /var/lib/augmented-id/rowrpm/

# Backup citizen snapshots
tar -czf snapshots_backup_$(date +%Y%m%d).tar.gz /var/lib/augmented-id/snapshots/

# Store backup securely
aws s3 cp rowrpm_backup_*.tar.gz s3://augmented-id-backups/
```

### Recovery Procedure

```bash
# Download backup
aws s3 cp s3://augmented-id-backups/rowrpm_backup_*.tar.gz .

# Extract backup
tar -xzf rowrpm_backup_*.tar.gz -C /var/lib/augmented-id/

# Verify integrity
augid_core ledger verify --chain /var/lib/augmented-id/rowrpm/chain.json

# Restart service
systemctl restart augmented-id
```

---

## Security Hardening

### Firewall Rules

```bash
# Allow only necessary ports
ufw allow 443/tcp  # HTTPS for Organichain
ufw allow 22/tcp   # SSH (restrict to known IPs)
ufw deny all       # Deny all other traffic
```

### Key Management

```bash
# Generate secure keys
openssl genpkey -algorithm ed25519 -out augid_key.pem
chmod 600 augid_key.pem

# Store in secure location
mv augid_key.pem /etc/augmented-id/keys/
```

### Audit Logging

```bash
# Enable audit logging
auditctl -w /var/lib/augmented-id/ -p wa -k augmented_id

# Review logs regularly
ausearch -k augmented_id --start today
```

---

## Troubleshooting

### Common Issues

| Issue | Solution |
|-------|----------|
| Token validation fails | Check token expiry and neurorights flags |
| Ledger integrity error | Run `augid_core ledger verify` |
| Biometric match fails | Check vault binding ID format |
| Offline sync fails | Verify network connectivity and snapshot validity |

### Support

- Documentation: https://docs.augmented-id.aln
- Issues: https://github.com/Doctor0Evil/Augmented-ID/issues
- Email: support@augmented-id.aln

---

## Production Checklist

- [ ] All tests pass
- [ ] Security audit completed
- [ ] Backup procedure tested
- [ ] Monitoring configured
- [ ] Alerting configured
- [ ] Documentation reviewed
- [ ] Team trained
- [ ] Rollback plan documented
```
