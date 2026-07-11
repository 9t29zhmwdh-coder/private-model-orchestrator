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

**Phase 3: Swift Package scaffold** ✅
- [x] New SwiftUI target `pmo-macos/`, macOS 14+. Built as a Swift Package (`Package.swift`), not a hand-authored `.xcodeproj`: more scriptable, and `swift build`/`swift run` need nothing beyond the Swift toolchain already required for the UniFFI bridge. `PMOMacOS.entitlements` (App Sandbox) exists but is deliberately not applied yet, see below.
- [x] Embed the Rust static library and generated Swift bindings as a build phase (`pmo-macos/scripts/generate-bindings.sh` + `Package.swift` linker settings), matching the `pmo-core/scripts` pattern from Phase 1
- [x] App shell: sidebar navigation (Devices, Models, Quotas, Policy), dark theme. Verified by actually launching the built app and screenshotting the running window, not just compiling it

**Known gap carried into Phase 5:** applying `com.apple.security.app-sandbox` to the raw `swift build` executable and launching it crashes with SIGTRAP (verified locally): a bare Mach-O binary with no `.app` bundle (no `Info.plist`/bundle ID) cannot establish a WindowServer connection under the sandbox. Real sandbox compliance needs proper `.app` bundling, which is what Phase 5's "Sandboxed App Container compliance" bullet is for. `scripts/build.sh` signs ad-hoc without the entitlement for now so the app actually runs.

**Phase 4: Dashboard views** ✅
- [x] Devices: list, add/remove, group assignment. Backed by a new `FfiStorage` UniFFI facade (`pmo-core/src/ffi_storage.rs`) that loads/mutates/saves the SQLite-backed registries per call, the same pattern `pmo-cli` already uses
- [x] Models: bundle list, variant/checksum display
- [x] Quotas: per-device usage bars, reset action (hourly and daily, matching `QuotaEngine::reset_hourly`/`reset_daily`)
- [x] Policy: load an MDM Configuration Profile (JSON file picker), display gating rules (inference/profiling allowed, minimum OS version, allowed model IDs)

All four views now share one `FfiStorage` instance (`AppModel`, injected via `@EnvironmentObject`) pointed at a real SQLite file in Application Support, replacing the session-only `FfiDeviceRegistry`/`FfiModelRegistry`/`FfiQuotaEngine` objects from Phase 3's demo. Those Phase 1 objects still exist and are still covered by the Swift round-trip test; `FfiStorage` is the one the app actually uses now. Added `DeviceRegistry::remove_device`/`set_device_group` to pmo-core (needed for "add/remove" and "group assignment", didn't exist before).

**Phase 5: CI, branding, release**
- [x] New GitHub Actions job: builds `pmo-macos` (`swift build` via `pmo-macos/scripts/build.sh`, ad-hoc signed, not notarized, matching the other desktop tools). Landed early, alongside Phase 3, since the build script and CI job were the natural place to prove the app actually compiles in CI.
- [ ] Update README.md/README.de.md with a real docs/screenshot.png once the dashboard views (Phase 4) have something worth screenshotting
- [ ] Sandboxed App Container compliance: proper `.app` bundling (Info.plist, bundle ID) so `PMOMacOS.entitlements` can actually be applied without crashing on launch (see the Phase 3 gap note above)

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

## Dual-Licensing Readiness

Assessed 2026-07-11 as a Dual-Licensing candidate (Community MIT + Commercial/Enterprise tier), and the closest to ready of any repo assessed so far in this portfolio: PMO already models real fleet concepts (device groups, per-device quotas, MDM policy gating) rather than just planning them. Not ready yet; blocked on:

- [ ] No live Jamf Pro API integration yet (v1.0.0 item above): today policy is loaded from a manually-picked JSON file, not pulled from an MDM server
- [ ] No Apple Business Manager fleet provisioning automation yet, still a documentation-only guide item
- [ ] No audit export for compliance reporting yet (v1.0.0 item above)
- [ ] No packaged, installable distribution yet: the macOS app cannot be sandbox-compliant until Phase 5's `.app` bundling gap is closed, so even the Community edition is not yet "finished" by this portfolio's own definition (installable, runnable) let alone the Commercial edition

Once Jamf Pro integration, ABM provisioning and audit export (v1.0.0) land, revisit: candidate Enterprise-only features would be live MDM server integration, ABM fleet provisioning automation, and compliance audit export, with the core device/model/quota registries, policy engine and desktop app staying Community/MIT.
