# Changelog: Private Model Orchestrator

All notable changes to this project will be documented in this file.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [0.5.4] - 2026-07-12

### Added

- `pmo-macos/Info.plist` and `pmo-macos/scripts/bundle.sh`: proper `.app` bundling for `pmo-macos` (bundle ID, embedded `libpmo_core.dylib` with its load path rewritten to `@executable_path/../Frameworks` since `swift build`'s linked path is an absolute path into the build machine's `target/debug`), packaged into a DMG. Ad-hoc signed, unsandboxed (see ROADMAP.md's Sandboxed App Container item, which remains separately open).
- Release workflow (`release.yml`) running `bundle.sh` and attaching the DMG to a GitHub Release on every `v*` tag push. Previously there was no packaged, installable distribution at all.
- README download section (EN + DE).
- Bundling smoke-test step in the existing `pmo-macos` CI job.

### Fixed

- Pinned `actions/checkout`, `dtolnay/rust-toolchain`, and `Swatinem/rust-cache` in `ci.yml` to a commit SHA instead of a mutable tag, per the portfolio's supply-chain integrity standard.
- Refreshed `Cargo.lock`, which still listed `pmo-core`/`pmo-cli` at version 0.5.0 despite `Cargo.toml` having moved to 0.5.3.

## [0.5.3] - 2026-07-12

### Added

- Dual-Licensing skeleton: LICENSE.COMMERCIAL, COMMERCIAL.md, and ENTERPRISE_FEATURES.md, documenting the licensing model for a future Enterprise Edition ahead of any actual feature split. The existing MIT LICENSE and all currently released code are unchanged; nothing in this repository is restricted by this addition.

## [0.5.2] - 2026-07-11

### Added

- Added the missing LICENSE file (MIT), matching what README and badges already claimed.
- Documented Dual-Licensing readiness assessment in ROADMAP.md.

## [0.5.1] - 2026-07-11

### Removed

- Removed the CLI terminal screenshot from the README hero section; only the pmo-macos GUI screenshot remains (renamed to docs/screenshot.png), consistent with the rest of the portfolio's README style for tools with a GUI.

## [0.5.0] - 2026-07-11

### Changed (SemVer correction)

- Renumbered the four Swift Integration phases from patch bumps (v0.1.7-v0.1.10) to minor bumps (v0.2.0-v0.5.0): each phase added real new functionality, not a bugfix, so each deserved a minor version per standard SemVer (and this project's own CLAUDE.md section 3). Old tags/releases deleted and recreated at the same commits under the corrected numbers.
- Replaced the stale hero screenshot (docs/screenshot.png), which still showed v0.1.0-era CLI output ("bootstrap check... land in v0.2.0") that stopped being true once Phase 2 shipped real subcommands.
- Added docs/screenshot_macos.png: a real screenshot of the running pmo-macos app showing devices registered via pmo-cli, proving the CLI and the app share the same SQLite data.
- Removed em-dashes from 10 documentation files (ARCHITECTURE.md, GETTING_STARTED.md, CONTRIBUTING.md, SKELETON.md, SECURITY.md, RELEASES.md, TEMPLATE_NOTES.md, docs/*.md) that an earlier portfolio-wide sweep didn't cover.
- Fixed GETTING_STARTED.md's "What you should see" sections: they still showed the old v0.1.0 bootstrap-check output and claimed "nothing written to disk", both no longer true now that pmo-cli persists to SQLite.


### Added

- Swift Integration Phase 4 (dashboard views): a new `FfiStorage` UniFFI object (`pmo-core/src/ffi_storage.rs`) exposes the SQLite-backed persistence layer to Swift, loading, mutating and saving the device/model/quota registries per call, mirroring the pattern `pmo-cli` already uses.
- `DeviceRegistry::remove_device` and `DeviceRegistry::set_device_group`: didn't exist yet, needed for the Devices view's add/remove and group assignment.
- `pmo-macos`'s four dashboard views now read and write real data through `FfiStorage` instead of session-only objects: Devices (list, register, remove, assign to a group), Models (list, register), Quotas (per-device usage bars, set limit, record inference, reset hourly/daily), Policy (load an MDM Configuration Profile via a file picker, display its gating rules).
- A shared `AppModel` (`@EnvironmentObject`) owns the single `FfiStorage` instance, pointed at a real SQLite file in Application Support, so every view operates on the same database.
- Verified end to end with `pmo-cli` and `pmo-macos` pointed at the same database file: devices registered via the CLI (a completely independent code path) showed up correctly in the running app, confirmed by screenshot.

### Fixed

- Discovered and worked around a Cargo feature-unification pitfall: building `pmo-cli` (`cargo build --workspace`) and `pmo-macos`'s bindings generator (`cargo build -p pmo-core --features ffi,persistence`) in sequence silently overwrite each other's shared `target/debug/libpmo_core.dylib` with different feature sets, breaking whichever binary was linked against the other's build. Documented in `pmo-macos/scripts/generate-bindings.sh`.

## [0.4.0] - 2026-07-11

### Added

- Swift Integration Phase 3 (app shell): a new `pmo-macos` Swift Package with a SwiftUI app (`PMOMacOSApp`), sidebar navigation across Devices/Models/Quotas/Policy, and live UniFFI calls into pmo-core (register a device or model bundle, set a quota limit, load a policy, all through the real bridge from Phase 1). Verified by actually launching the built app and screenshotting the running window.
- `pmo-macos/scripts/generate-bindings.sh` and `scripts/build.sh` regenerate the UniFFI Swift bindings and build+sign the app. Wired into CI as a new `pmo-macos` job on macos-latest.
- `pmo-macos/PMOMacOS.entitlements` declares the App Sandbox entitlement for the eventual real `.app` bundle. Not applied by the current build script: a bare `swift build` executable signed with `com.apple.security.app-sandbox` crashes with SIGTRAP on launch (no bundle ID / Info.plist for the sandbox to resolve), so real enforcement is deferred to the "Sandboxed App Container compliance" work in Phase 5.

### Changed

- Built the app as a Swift Package rather than a hand-authored Xcode project: fully scriptable with the Swift toolchain alone, matching how the Phase 1 UniFFI bridge is already built and tested in CI.
- README.md/README.de.md: mention the new `pmo-macos` app shell and its current limitations (session-only data, not yet an installable bundle).

## [0.3.0] - 2026-07-11

### Added

- Swift Integration Phase 2 (persistence layer): DeviceRegistry, ModelRegistry and QuotaEngine can now be saved to and loaded from a SQLite database (`rusqlite`, opt-in `persistence` cargo feature). Added `from_parts`/`all_limits`/`all_usage`/`set_usage` accessors needed to rebuild registries from persisted rows.
- `PolicyWatcher`: hot-reloads an MdmPolicy from a local JSON file on change (via `notify`), so an MDM-pushed Configuration Profile can be picked up without restarting the process.
- `pmo-cli` now has real subcommands backed by a SQLite database (`--db pmo.db` by default): `device list`/`register`, `model list`/`register`, `quota status`/`set-limit`. Data survives across separate CLI invocations.

### Changed

- README.md/README.de.md: updated the "How it runs" callout and Quickstart to reflect that pmo-cli now persists data and has real subcommands, replacing the outdated "nothing persists yet, in-memory only" wording.

## [0.2.0] - 2026-07-11

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
