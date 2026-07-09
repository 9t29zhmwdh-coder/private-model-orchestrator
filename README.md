<div align="center">

<img src="RayStudio.png" alt="RayStudio" width="80" />

# Private Model Orchestrator

**Privacy-first orchestration of Foundation Models for Apple device fleets.**

Deploy, version and serve on-device AI models at enterprise scale. Zero data leaves the device.

[![CI](https://github.com/9t29zhmwdh-coder/private-model-orchestrator/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/private-model-orchestrator/actions) ![Apple Silicon](https://img.shields.io/badge/Apple-Silicon-000000?logo=apple&logoColor=white) ![Platform](https://img.shields.io/badge/Platform-macOS-lightgrey?logo=apple&logoColor=black) ![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white) ![AI | Claude Code](https://img.shields.io/badge/AI-Claude_Code-black?logo=anthropic&logoColor=white) ![AI | Copilot](https://img.shields.io/badge/AI-Copilot-black?logo=github&logoColor=white) ![AI | Ollama](https://img.shields.io/badge/AI-Ollama-black?logo=ollama&logoColor=white)

</div>

> **How it runs:** PMO is a command-line tool, not a background service or GUI. `pmo-cli` runs once and exits after printing its status; there is no installer, and nothing persists yet (v0.1.0 registries are in-memory, see [ROADMAP.md](ROADMAP.md)).

![Private Model Orchestrator](docs/screenshot.png)

---

**In practice:** today you get a tested Rust library modeling device fleets, model bundles, quotas and MDM policy hints, plus a CLI that verifies the subsystems initialise correctly. Persistence, interactive subcommands and the Swift/macOS dashboard are on the roadmap, not shipped yet.

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

> 🌱 New here? → [Step-by-step guide for beginners](GETTING_STARTED.md)

## Quickstart

```bash
# Build
cargo build --workspace

# Run CLI
cargo run --bin pmo-cli

# Test
cargo test --workspace
```

## Uninstall / Cleanup

Delete the `target/` build directory. `pmo-cli` does not write any files in v0.1.0 (registries are in-memory), so there is nothing else to clean up.

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
