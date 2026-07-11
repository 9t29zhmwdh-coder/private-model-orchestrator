<div align="center">

<img src="RayStudio.png" alt="RayStudio" width="120" />

# Private Model Orchestrator

</div>

[🇩🇪 Deutsche Version](README.de.md)

**Privacy-first orchestration of Foundation Models for Apple device fleets.**

Deploy, version and serve on-device AI models at enterprise scale. Zero data leaves the device.

[![CI](https://github.com/9t29zhmwdh-coder/private-model-orchestrator/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/private-model-orchestrator/actions) ![Apple Silicon](https://img.shields.io/badge/Apple-Silicon-000000?logo=apple&logoColor=white) ![Platform](https://img.shields.io/badge/Platform-macOS-lightgrey?logo=apple&logoColor=black) ![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white) ![AI | Claude Code](https://img.shields.io/badge/AI-Claude_Code-black?logo=anthropic&logoColor=white) ![AI | Copilot](https://img.shields.io/badge/AI-Copilot-black?logo=github&logoColor=white) ![AI | Ollama](https://img.shields.io/badge/AI-Ollama-black?logo=ollama&logoColor=white)


> **How it runs:** PMO is a command-line tool, not a background service or GUI. `pmo-cli` reads and writes a local SQLite database (`pmo.db` by default) and exits after each subcommand; there is no installer and no background daemon. The Swift/macOS dashboard is on the roadmap, not shipped yet.

![Private Model Orchestrator](docs/screenshot.png)

---

> 🌱 New here? → [Step-by-step guide for beginners](GETTING_STARTED.md)

---

**In practice:** today you get a tested Rust library modeling device fleets, model bundles, quotas and MDM policy hints, a SQLite-backed persistence layer, a CLI with `device`/`model`/`quota` subcommands, and a UniFFI bridge ready for a future Swift consumer. The SwiftUI dashboard app itself is on the roadmap, not shipped yet.

## Overview

Private Model Orchestrator (PMO) is an enterprise toolkit for managing Foundation Model deployments across Apple device fleets. All inference happens entirely on-device via Core ML; no telemetry, no cloud egress.

## Features

| Feature | Description |
|---------|-------------|
| **AOT Conversion** | Reference pipeline for `.mlpackage` → `.mlmodelc` ahead-of-time compilation |
| **Model Packaging** | Versioned, checksum-verified model bundles with variant tracking |
| **Device Groups** | Fleet segmentation with per-group model assignments |
| **Quota Management** | Per-device hourly/daily inference quotas with reset support |
| **MDM Integration** | Configuration Profile hints for Jamf / Apple Business Manager |
| **Performance Profiling** | Instrumented stubs ready for Core ML Profiler integration |

## Requirements

- Rust 1.78+
- macOS 14+ (for Core ML AOT features)
- Jamf Pro or Apple Business Manager (optional, for MDM integration)

## Quickstart

```bash
# Build
cargo build --workspace

# Run CLI (prints a status summary if no subcommand is given)
cargo run --bin pmo-cli

# Register a device and a model bundle, then check its quota
cargo run --bin pmo-cli -- device register --serial C02XJ1ABCD12 --hardware-model "MacBookPro18,3" --os-version 14.5
cargo run --bin pmo-cli -- device list
cargo run --bin pmo-cli -- model register --name mistral-7b --version 0.1.0 --variant ml-model-c --checksum abc123
cargo run --bin pmo-cli -- quota status --device <device-id-from-the-list-above>

# Test
cargo test --workspace
```

By default, `pmo-cli` reads and writes `pmo.db` in the current directory; pass `--db <path>` to use a different location.

## Uninstall / Cleanup

Delete the `target/` build directory and the `pmo.db` SQLite file (or whichever path you passed to `--db`).

## Documentation

- [Architecture](ARCHITECTURE.md)
- [MDM Integration Guide](docs/mdm_integration.md)
- [AOT Conversion Reference](docs/aot_conversion.md)
- [API Reference](docs/api_reference.md)
- [Roadmap](ROADMAP.md)
- [Privacy Policy](PRIVACY.md)

## Security

See [SECURITY.md](SECURITY.md) for vulnerability reporting.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

---

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Active · ![version](https://img.shields.io/github/v/release/9t29zhmwdh-coder/private-model-orchestrator?color=6b7280&style=flat-square) · **License:** MIT
