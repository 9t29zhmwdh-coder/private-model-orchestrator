# Roadmap: Private Model Orchestrator

## v0.1.0: Initial Import ✅

- Rust workspace: `pmo-core`, `pmo-cli`
- Device registry (register, group, assign)
- Model registry (bundle, variant, checksum)
- Quota engine (daily/hourly caps, reset)
- MDM policy engine (Configuration Profile hints)
- Performance profiling stubs
- Bilingual README (EN / DE)
- Full documentation skeleton

## v0.3.0: Swift Integration (includes persistence)

Persistence was originally planned as its own v0.2.0 milestone, but was
pulled forward into this milestone: a dashboard with no persistence would
lose all data on every restart, so `pmo-macos` gets a working SQLite
backend from day one instead of being a throwaway in-memory demo.

**Phase 1: UniFFI bridge** ✅
- [x] Add the `uniffi` crate to `pmo-core`, expose `DeviceRegistry`, `ModelRegistry`, `QuotaEngine` and `PolicyEngine` as UniFFI objects
- [x] Generate Swift bindings, verify with a minimal Swift test program that real data round-trips from Rust

**Phase 2: Persistence layer** ✅
- [x] SQLite backend for device and model registries (`rusqlite`)
- [x] Quota persistence across restarts
- [x] Policy hot-reload from a local file (watch via `notify`)
- [x] `pmo-cli` subcommands: `device list`/`register`, `model list`/`register`, `quota status`/`set-limit`

**Phase 3: Xcode project scaffold**
- [ ] New SwiftUI target `pmo-macos/`, macOS 14+, App Sandbox entitlements
- [ ] Embed the Rust static library and generated Swift bindings as a build phase / SPM binary target
- [ ] App shell: sidebar navigation (Devices, Models, Quotas, Policy), dark theme matching the other RayStudio Tauri apps

**Phase 4: Dashboard views**
- [ ] Devices: list, add/remove, group assignment
- [ ] Models: bundle list, variant/checksum display
- [ ] Quotas: per-device usage bars, reset action
- [ ] Policy: load an MDM Configuration Profile, display gating rules

**Phase 5: CI, branding, release**
- [ ] New GitHub Actions job: `xcodebuild` for `pmo-macos`, ad-hoc signed (not notarized, matching the other desktop tools)
- [ ] Update README.md/README.de.md: remove the "CLI-only, no GUI" callout, add a screenshot
- [ ] Sandboxed App Container compliance

## v0.4.0: AOT Pipeline

- [ ] `scripts/convert_model.sh`: production-ready `coremltools` wrapper
- [ ] Bundle signing with Developer ID (code-sign step in CI)
- [ ] `.mlmodelc` integrity verification at load time
- [ ] Multi-variant bundle distribution (`.mlpackage` dev, `.mlmodelc` prod)

## v1.0.0: Enterprise GA

- [ ] Full Jamf Pro integration (Configuration Profile schema published)
- [ ] Apple Business Manager fleet provisioning guide
- [ ] Core ML Profiler integration (replace `ProfilingStub`)
- [ ] Audit export (JSON, CSV) for compliance reporting
- [ ] Performance regression benchmarking in CI

## Out of Scope

- Cloud inference endpoints: PMO is strictly on-device
- iOS / iPadOS deployment: macOS fleet only in v1
- Model training: inference orchestration only
