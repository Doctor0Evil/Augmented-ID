# Contributing to Augmented-ID

Thank you for your interest in contributing to Augmented-ID. This document outlines the process for contributing code, documentation, and other improvements.

---

## Code of Conduct

All contributors must adhere to the following principles:

1. **Neurorights First**: All contributions must preserve and enhance neurorights protections
2. **Security First**: No contribution may weaken security guarantees
3. **Immutability**: No contribution may enable rollback, reversal, or downgrade
4. **Privacy**: No contribution may expose raw biometric data or enable transmission
5. **Offline-Capable**: All features must work offline without fallback mechanisms

---

## Getting Started

### Prerequisites

- Rust 1.75+
- ALN Compiler v2026.1+
- Git

### Setup

```bash
# Clone the repository
git clone https://github.com/Doctor0Evil/Augmented-ID.git
cd Augmented-ID

# Build the Rust core
cd rust
cargo build --release

# Run tests
cargo test

# Compile ALN schemas
cd ../aln
aln-compile schemas/*.aln --output ./compiled/
```

---

## Contribution Guidelines

### Code Style

#### Rust

- Follow Rust API Guidelines
- Use `cargo fmt` before committing
- Run `cargo clippy` for linting
- All public functions must have documentation
- All errors must use `AugIdError` enum

#### ALN

- Follow ALN Schema Style Guide
- All records must include `antirollback` field
- All guards must enforce neurorights flags
- No external function calls allowed

### Testing Requirements

All contributions must include:

1. **Unit Tests**: For individual functions
2. **Integration Tests**: For component interactions
3. **Guard Tests**: For all security invariants
4. **Offline Tests**: For offline operation verification

### Security Review

All contributions undergo security review:

1. **Static Analysis**: Automated security scanning
2. **Guard Verification**: All guards must pass
3. **Neurorights Check**: All neurorights must be preserved
4. **Anti-Rollback Check**: No rollback vectors introduced

---

## Pull Request Process

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### PR Requirements

- [ ] All tests pass
- [ ] Code is formatted
- [ ] Documentation updated
- [ ] Security review completed
- [ ] Neurorights preserved
- [ ] No external dependencies added without approval

---

## Reporting Issues

### Security Issues

**DO NOT** report security issues via public GitHub issues. Instead:

1. Email: security@augmented-id.aln
2. Include: Description, reproduction steps, impact assessment
3. Wait: For response before public disclosure

### Bug Reports

For non-security bugs:

1. Check existing issues first
2. Include: Steps to reproduce, expected behavior, actual behavior
3. Include: Environment details (OS, Rust version, ALN version)
4. Include: Test case if possible

### Feature Requests

For new features:

1. Describe the use case
2. Explain why it's needed
3. Provide ALN schema sketch if applicable
4. Consider security implications

---

## Architecture Decision Records (ADRs)

Major changes require an ADR:

```markdown
# ADR-XXX: [Title]

## Status
[Proposed | Accepted | Deprecated | Superseded]

## Context
[What is the issue we're addressing?]

## Decision
[What is the change we're making?]

## Consequences
[What are the implications of this change?]

## Security Impact
[How does this affect security guarantees?]

## Neurorights Impact
[How does this affect neurorights protections?]
```

---

## Release Process

### Version Numbering

Augmented-ID follows semantic versioning:

- **MAJOR**: Breaking changes to schema or guards
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

### Release Checklist

- [ ] All tests pass
- [ ] Security audit completed
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version numbers updated
- [ ] Release notes written
- [ ] Tags created

---

## License

By contributing, you agree that your contributions will be licensed under the MIT License with the Neurorights Addendum.

---

## Contact

- Repository Owner: Doctor0Evil
- Email: contact@augmented-id.aln
- Discord: [Augmented-ID Community]
- Documentation: https://docs.augmented-id.aln
```
