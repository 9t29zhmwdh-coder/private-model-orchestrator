# Changelog — Private Model Orchestrator

All notable changes to this project will be documented in this file.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [0.1.3] — 2026-07-10

### Changed

- Moved the "New here? -> beginners guide" callout in README.md above the intro (previously only appeared near Requirements)

### Added

- Added the "New here?" beginner guide callout to README.de.md (was missing)

## [0.1.0] — <EARLIEST_COMMIT_DATE>

### Added

- Rust workspace with `pmo-core` and `pmo-cli` crates
- `DeviceRegistry` — register devices, create groups, assign models, query by group
- `ModelRegistry` — register versioned model bundles (`MlPackage` / `MlModelC`), find by name/ID
- `QuotaEngine` — per-device hourly and daily inference quotas with reset support
- `PolicyEngine` — MDM-sourced `MdmPolicy` enforcement (inference gate, model allowlist, profiling flag)
- `ProfilingSession` / `ProfilingStub` — wall-clock timing stubs for Core ML Profiler integration
- `OrchestratorConfig` — typed configuration with sensible defaults
- Comprehensive unit test suite (8 tests in `pmo-core::tests`)
- Bilingual README (EN / DE)
- Full documentation skeleton: ARCHITECTURE, PRIVACY, ROADMAP, CONTRIBUTING, SECURITY
- Example programs: `device_group.rs`, `model_packaging.rs`
- Scripts: `convert_model.sh`, `mdm_preflight.sh`
- MDM integration guide, AOT conversion reference, API reference
