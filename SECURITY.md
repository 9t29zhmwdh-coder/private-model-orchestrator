# Security Policy — Private Model Orchestrator

## Supported Versions / Unterstützte Versionen

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅        |

## Reporting a Vulnerability / Sicherheitslücke melden

**Do not open a public GitHub issue for security vulnerabilities.**

Report via [GitHub Security Advisory](https://github.com/9t29zhmwdh-coder/private-model-orchestrator/security/advisories/new)
or contact the maintainer via the GitHub profile.

Include: description, steps to reproduce, potential impact, suggested fix.
Response within 7 business days.

**Keine öffentlichen GitHub-Issues für Sicherheitslücken.**

Bitte über [GitHub Security Advisory](https://github.com/9t29zhmwdh-coder/private-model-orchestrator/security/advisories/new)
oder direkt über das GitHub-Profil des Maintainers melden.

---

## Security Design / Sicherheitsarchitektur

| Property | Detail |
|----------|--------|
| Network | Zero outbound connections |
| Storage | Local only; no cloud sync |
| Model bundles | SHA-256 checksum verified at load |
| Policy source | MDM Configuration Profile (signed, verified by OS) |
| Profiling data | Never exported automatically |
| Dependencies | Minimal; audited with `cargo audit` |

## Supply Chain

All Rust dependencies are pinned in `Cargo.lock`. Run `cargo audit` before deployment.

**Last updated: 2026-06-16**
