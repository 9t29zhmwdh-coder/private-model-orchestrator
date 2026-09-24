<div align="center">

<img src="RayStudio.png" alt="RayStudio" width="120" />

# Private Model Orchestrator

</div>

[🇩🇪 Deutsche Version](README.de.md)

**A register for a fleet of Macs: which device, which model version, which inference quota, written down in one place.**

Installing a local model on one machine is a download. Doing it across a fleet
raises questions a download does not answer: which devices are on the old
version, who is allowed how many inference calls, what is supposed to run
where. PMO is the place to write those answers down.

```
pmo-cli device list              what is out there
pmo-cli model register           add a model bundle and its version
pmo-cli quota …                  who may run how much
```

You enter devices, model bundles and quotas yourself, with the CLI or the Mac
app. PMO does not find devices on its own, does not install or push models,
and does not run inference: it is the record, not the delivery. Nothing leaves
the machine it runs on.

**Not for you if** you are running models on your own machine. Ollama or
llama.cpp are the answer there, and this adds a registry you have no use for.
And if you need devices to report what they actually run, or models to be
rolled out, that is the job of an MDM such as Jamf or Intune; PMO only keeps
the intended state.

[![CI](https://github.com/9t29zhmwdh-coder/private-model-orchestrator/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/private-model-orchestrator/actions) [![CodeQL](https://github.com/9t29zhmwdh-coder/private-model-orchestrator/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/9t29zhmwdh-coder/private-model-orchestrator/security/code-scanning) [![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/9t29zhmwdh-coder/private-model-orchestrator/badge)](https://securityscorecards.dev/viewer/?uri=github.com/9t29zhmwdh-coder/private-model-orchestrator) [![OpenSSF Best Practices](https://www.bestpractices.dev/projects/13687/badge)](https://www.bestpractices.dev/projects/13687)

![Apple Silicon](https://img.shields.io/badge/Apple-Silicon-000000?logo=apple&logoColor=white) ![Platform](https://img.shields.io/badge/Platform-macOS-lightgrey?logo=apple&logoColor=black) ![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white) ![AI | Claude Code](https://img.shields.io/badge/AI-Claude_Code-black?logo=anthropic&logoColor=white) ![AI | Copilot](https://img.shields.io/badge/AI-Copilot-black?logo=github&logoColor=white)


> **How it runs:** `pmo-cli` reads and writes a local SQLite database (`pmo.db` by default) and exits after each subcommand; there is no installer and no background daemon. `pmo-macos` (SwiftUI, see [pmo-macos/](pmo-macos/)) reads and writes its own SQLite database in Application Support through the same UniFFI bridge; the CLI and the app see the same data if pointed at the same database file.

![pmo-macos](docs/screenshot.png)

---

> 💾 **Download:** [macOS (DMG)](https://github.com/9t29zhmwdh-coder/private-model-orchestrator/releases/latest): the packaged `pmo-macos` app, not code-signed/notarized and not yet Sandboxed App Container compliant (see ROADMAP.md), Gatekeeper will warn on first run (right-click → Open). `pmo-cli` is not packaged; build it from source, see Quickstart below.

---

> 🌱 New here? → [Step-by-step guide for beginners](GETTING_STARTED.md)

---

**In practice:** today you get a tested Rust library modeling device fleets, model bundles, quotas and MDM policy hints, a SQLite-backed persistence layer, a CLI with `device`/`model`/`quota` subcommands, and a SwiftUI app (`pmo-macos`) with real dashboard views (add/remove devices, group assignment, model bundles, per-device quota usage and reset, MDM policy file loading), all backed by the same SQLite storage layer via UniFFI. The app is packaged as a real `.app` bundle in a DMG; a notarized, Sandboxed App Container-compliant build is still on the roadmap.

## Overview

Private Model Orchestrator (PMO) is a Rust library with a CLI and a SwiftUI Mac
app around one SQLite database. It records the devices of a Mac fleet, the
model bundles meant for them, and per-device inference quotas.

## Features

| Feature | What it does |
|---------|--------------|
| **Device register** | Devices with serial, hardware model and macOS version; add, list and remove in the CLI and the app |
| **Device groups** | Create groups and move devices into them (Mac app); the library can also attach model bundles to a group |
| **Model bundles** | Name, version, variant (`.mlpackage` or `.mlmodelc`) and a checksum you supply; PMO stores the checksum, it does not verify files |
| **Quotas** | Hourly and daily limits per device, a usage counter, and manual reset in the app; nothing counts inference automatically |
| **Policy file** | Loads an MDM-style JSON policy (minimum macOS, allowed models) in the app and reloads it when the file changes |
| **Preflight script** | `scripts/mdm_preflight.sh` prints, as JSON, whether a Mac is MDM-enrolled, its macOS version and whether `coremltools` is installed |

## Requirements

- Rust 1.78+
- macOS 14+ for the Mac app
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
- [API Reference](docs/api_reference.md)
- [Roadmap](ROADMAP.md)
- [Privacy Policy](PRIVACY.md)

## Security

See [SECURITY.md](SECURITY.md) for vulnerability reporting.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

---

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Active · ![version](https://img.shields.io/github/v/release/9t29zhmwdh-coder/private-model-orchestrator?color=6b7280&style=flat-square) · **License:** MIT
