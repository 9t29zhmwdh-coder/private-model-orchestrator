# Architecture — Private Model Orchestrator

## System Overview

PMO is structured as a Rust workspace. The core library (`pmo-core`) contains all domain logic and is consumption-agnostic: it can be embedded in a CLI, a macOS SwiftUI app via UniFFI, or a headless daemon.

```
┌─────────────────────────────────────────────────────────────────┐
│                         pmo-core                                 │
│                                                                   │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌────────────┐  │
│  │  device  │──▶│  quota   │   │  policy  │   │  profiler  │  │
│  └──────────┘   └──────────┘   └──────────┘   └────────────┘  │
│       │               │               │               │          │
│       └───────────────┴───────────────┴───────────────┘         │
│                               │                                   │
│                          ┌────┴────┐                             │
│                          │  model  │                             │
│                          └─────────┘                             │
└─────────────────────────────────────────────────────────────────┘
          │ in-process                │ (future) UniFFI / C-ABI
          ▼                           ▼
┌─────────────────┐       ┌─────────────────────────┐
│    pmo-cli      │       │  pmo-macos (SwiftUI)    │
│  (Unix daemon)  │       │  via UniFFI bridge       │
└─────────────────┘       └─────────────────────────┘
          │                           │
          └─────────────┬─────────────┘
                        ▼
       ┌────────────────────────────────┐
       │    Apple Device Fleet          │
       │  (MDM: Jamf / ABM)            │
       │  Core ML · ANE · GPU          │
       └────────────────────────────────┘
```

## Module Responsibilities

### `device`
- `Device` — serial, hardware model, OS version, group membership
- `DeviceGroup` — named fleet segment with optional model assignment
- `DeviceRegistry` — in-memory CRUD, lookup by group

### `model`
- `ModelVariant` — `MlPackage` (interpreted) vs `MlModelC` (AOT compiled)
- `ModelBundle` — versioned, checksum-verified bundle descriptor
- `ModelRegistry` — register, look up by name / ID, filter by variant

### `quota`
- `QuotaLimit` — daily/hourly caps per device
- `QuotaUsage` — mutable usage counters (reset on schedule)
- `QuotaEngine` — enforce limits, record inference, expose usage

### `policy`
- `MdmPolicy` — deserialized from a Configuration Profile JSON payload
- `PolicyEngine` — load policy, gate inference / model access / profiling

### `profiler`
- `ProfilingSession` — wall-clock timer stub with start/stop/elapsed
- `ProfilingStub` — factory that returns `None` when profiling is disabled

## Data Flow — Inference Request

```
Device                    pmo-cli / pmo-macos
  │                              │
  │  inference request           │
  │──────────────────────────────▶
  │                              │
  │                      PolicyEngine::is_inference_allowed()
  │                      PolicyEngine::is_model_allowed(model_id)
  │                              │ denied ──▶ reject
  │                              │
  │                      QuotaEngine::is_allowed(device_id)
  │                              │ denied ──▶ reject
  │                              │
  │                      ProfilingStub::session("inference")
  │                      [→ Core ML / ANE execution here]
  │                      ProfilingSession::stop()
  │                              │
  │                      QuotaEngine::record_inference(device_id)
  │                              │
  │◀──────────────── result / error ─────────────────────
```

## Configuration Profile Schema (MDM Integration)

See `docs/mdm_integration.md` for the full Jamf-compatible payload. The `MdmPolicy` struct maps directly from a JSON payload of type `com.raystudio.pmo.policy`.

## AOT Conversion Reference

See `docs/aot_conversion.md` for the `coremltools`-based pipeline that produces `.mlmodelc` bundles for ANE-optimised deployment.

## External Dependencies

| Crate | Purpose |
|-------|---------|
| `serde` / `serde_json` | Policy JSON deserialization |
| `uuid` | Stable device / model / group IDs |
| `chrono` | Timestamp-based quota windows |
| `thiserror` | Structured error types |

No network calls. No file I/O in `pmo-core` (persistence layer is the caller's responsibility).
