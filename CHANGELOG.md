# Changelog: Private Model Orchestrator

All notable changes to this project will be documented in this file.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [0.1.7] - 2026-07-11

### Added

- Swift Integration Phase 1 (UniFFI bridge): pmo-core now exposes DeviceRegistry, ModelRegistry, QuotaEngine and PolicyEngine as thread-safe UniFFI objects behind an opt-in `ffi` cargo feature. Added `all_devices`/`all_groups`/`all_bundles` accessors needed for the future dashboard list views.
- A `uniffi-bindgen` binary target (`cargo run -p pmo-core --features ffi --bin uniffi-bindgen`) generates the Swift bindings.
- `scripts/test-swift-bridge.sh` builds the bridge, generates bindings and runs a Swift round-trip test (`pmo-core/tests/swift_bridge/main.swift`) that exercises all four UniFFI objects against real data. Wired into CI as a new `swift-bridge` job on macos-latest.

## [0.1.6] - 2026-07-11

### Changed

- Detailed the v0.3.0 "Swift Integration" roadmap milestone into concrete phases (UniFFI bridge, persistence, Xcode scaffold, dashboard views, CI/branding). Persistence, previously its own v0.2.0 milestone, is now folded into v0.3.0 so pmo-macos ships with a working SQLite backend instead of losing all data on restart.
- Removed em-dashes from ROADMAP.md headings and bullets

## [0.1.5] - 2026-07-11

### Fixed

- Corrected README hero section: only the title image and title stay centered, tagline, description and badges are now left aligned like the rest of the document

## [0.1.4] - 2026-07-10

### Fixed

- Removed em-dashes from CHANGELOG.md, replaced with colons/plain hyphens

## [0.1.3] - 2026-07-10

### Changed

- Moved the "New here? -> beginners guide" callout in README.md above the intro (previously only appeared near Requirements)

### Added

- Added the "New here?" beginner guide callout to README.de.md (was missing)

## [0.1.0] - <EARLIEST_COMMIT_DATE>

### Added

- Rust workspace with `pmo-core` and `pmo-cli` crates
- `DeviceRegistry`: register devices, create groups, assign models, query by group
- `ModelRegistry`: register versioned model bundles (`MlPackage` / `MlModelC`), find by name/ID
- `QuotaEngine`: per-device hourly and daily inference quotas with reset support
- `PolicyEngine`: MDM-sourced `MdmPolicy` enforcement (inference gate, model allowlist, profiling flag)
- `ProfilingSession` / `ProfilingStub`: wall-clock timing stubs for Core ML Profiler integration
- `OrchestratorConfig`: typed configuration with sensible defaults
- Comprehensive unit test suite (8 tests in `pmo-core::tests`)
- Bilingual README (EN / DE)
- Full documentation skeleton: ARCHITECTURE, PRIVACY, ROADMAP, CONTRIBUTING, SECURITY
- Example programs: `device_group.rs`, `model_packaging.rs`
- Scripts: `convert_model.sh`, `mdm_preflight.sh`
- MDM integration guide, AOT conversion reference, API reference
