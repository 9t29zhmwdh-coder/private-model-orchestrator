<div align="center">

<img src="RayStudio.png" alt="RayStudio" width="80" />

# Private Model Orchestrator

**Privacy-first orchestration of Foundation Models for Apple device fleets.**

Deploy, version and serve on-device AI models at enterprise scale — zero data leaves the device.

[![CI](https://github.com/9t29zhmwdh-coder/private-model-orchestrator/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/private-model-orchestrator/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-green)](LICENSE)
[![Platform: macOS 14+](https://img.shields.io/badge/Platform-macOS%2014+-lightgrey)](#)

</div>

---

## Overview

Private Model Orchestrator (PMO) is an enterprise toolkit for managing Foundation Model deployments across Apple device fleets. All inference happens entirely on-device via Core ML — no telemetry, no cloud egress.

## Features

| Feature | Description |
|---------|-------------|
| **AOT Conversion** | Reference pipeline for `.mlpackage` → `.mlmodelc` ahead-of-time compilation |
| **Model Packaging** | Versioned, checksum-verified model bundles with variant tracking |
| **Device Groups** | Fleet segmentation with per-group model assignments |
| **Quota Management** | Per-device hourly/daily inference quotas with reset support |
| **MDM Integration** | Configuration Profile hints for Jamf / Apple Business Manager |
| **Performance Profiling** | Instrumented stubs ready for Core ML Profiler integration |

## Quickstart

```bash
# Build
cargo build --workspace

# Run CLI
cargo run --bin pmo-cli

# Test
cargo test --workspace
```

## Documentation

- [Architecture](ARCHITECTURE.md)
- [MDM Integration Guide](docs/mdm_integration.md)
- [AOT Conversion Reference](docs/aot_conversion.md)
- [API Reference](docs/api_reference.md)
- [Roadmap](ROADMAP.md)
- [Privacy Policy](PRIVACY.md)

## Requirements

- Rust 1.78+
- macOS 14+ (for Core ML AOT features)
- Jamf Pro or Apple Business Manager (optional — for MDM integration)

## Security

See [SECURITY.md](SECURITY.md) for vulnerability reporting.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

---

<div align="center">

**RayStudio · Rafael Yilmaz · MIT License · 2026**

</div>
