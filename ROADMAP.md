# Roadmap — Private Model Orchestrator

## v0.1.0 — Initial Import ✅

- Rust workspace: `pmo-core`, `pmo-cli`
- Device registry (register, group, assign)
- Model registry (bundle, variant, checksum)
- Quota engine (daily/hourly caps, reset)
- MDM policy engine (Configuration Profile hints)
- Performance profiling stubs
- Bilingual README (EN / DE)
- Full documentation skeleton

## v0.2.0 — Persistence Layer

- [ ] SQLite backend for device and model registries (`rusqlite`)
- [ ] Quota persistence across restarts
- [ ] Policy hot-reload from local file (watch via `notify`)
- [ ] `pmo-cli` subcommands: `device list`, `model register`, `quota status`

## v0.3.0 — Swift Integration

- [ ] UniFFI bindings for `pmo-core` → Swift
- [ ] SwiftUI macOS app (`pmo-macos`): device dashboard, quota overview
- [ ] Keychain-based policy storage for MDM-managed devices
- [ ] Sandboxed App Container compliance (notarization)

## v0.4.0 — AOT Pipeline

- [ ] `scripts/convert_model.sh` — production-ready `coremltools` wrapper
- [ ] Bundle signing with Developer ID (code-sign step in CI)
- [ ] `.mlmodelc` integrity verification at load time
- [ ] Multi-variant bundle distribution (`.mlpackage` dev, `.mlmodelc` prod)

## v1.0.0 — Enterprise GA

- [ ] Full Jamf Pro integration (Configuration Profile schema published)
- [ ] Apple Business Manager fleet provisioning guide
- [ ] Core ML Profiler integration (replace `ProfilingStub`)
- [ ] Audit export (JSON, CSV) for compliance reporting
- [ ] Performance regression benchmarking in CI

## Out of Scope

- Cloud inference endpoints — PMO is strictly on-device
- iOS / iPadOS deployment — macOS fleet only in v1
- Model training — inference orchestration only
