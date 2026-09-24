# Architecture: Private Model Orchestrator

## System Overview

PMO is a Rust workspace. The core library (`pmo-core`) holds the register (devices, groups, model bundles, quotas, policy) and its SQLite storage; the CLI uses it directly, the SwiftUI app through a UniFFI bridge. PMO does not talk to the devices it lists: every entry is made by hand.

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
          │ in-process                │ UniFFI bridge
          ▼                           ▼
┌─────────────────┐       ┌─────────────────────────┐
│    pmo-cli      │       │  pmo-macos (SwiftUI)    │
│ (runs and exits)│       │  via UniFFI bridge       │
└─────────────────┘       └─────────────────────────┘
          │                           │
          └─────────────┬─────────────┘
                        ▼
       ┌────────────────────────────────┐
       │  SQLite register               │
       │  (pmo.db or Application        │
       │   Support), entered by hand    │
       └────────────────────────────────┘
```

## Module Responsibilities

### `device`
- `Device`: serial, hardware model, OS version, group membership
- `DeviceGroup`: named fleet segment with optional model assignment
- `DeviceRegistry`: in-memory CRUD, lookup by group

### `model`
- `ModelVariant`: `MlPackage` (interpreted) vs `MlModelC` (AOT compiled)
- `ModelBundle`: versioned bundle descriptor with a stored checksum (not verified against any file)
- `ModelRegistry`: register, look up by name / ID, filter by variant

### `quota`
- `QuotaLimit`: daily/hourly caps per device
- `QuotaUsage`: mutable usage counters (reset on schedule)
- `QuotaEngine`: enforce limits, record inference, expose usage

### `policy`
- `MdmPolicy`: deserialized from a Configuration Profile JSON payload
- `PolicyEngine`: load policy, gate inference / model access / profiling

### `profiler`
- `ProfilingSession`: a plain wall-clock timer with start/stop/elapsed; it does not call Core ML or Instruments
- `ProfilingStub`: returns `None` when the policy disables profiling

## Data Flow

```
you ──▶ pmo-cli  ──▶ pmo-core ──▶ pmo.db (SQLite, current directory or --db)
you ──▶ pmo-macos ──▶ UniFFI (FfiStorage) ──▶ pmo-core ──▶ SQLite in Application Support
```

Both write the same schema, so the CLI and the app see the same data when they
use the same file. Nothing is sent to or received from the devices in the
register. `PolicyEngine` and `QuotaEngine` answer "would this be allowed", for a
program that embeds `pmo-core` and asks; PMO itself runs no inference and
enforces nothing on a device.

## Configuration Profile Schema (MDM Integration)

See `docs/mdm_integration.md` for the full Jamf-compatible payload. The `MdmPolicy` struct maps directly from a JSON payload of type `com.raystudio.pmo.policy`.

## External Dependencies

| Crate | Purpose |
|-------|---------|
| `serde` / `serde_json` | Policy JSON deserialization |
| `uuid` | Stable device / model / group IDs |
| `chrono` | Timestamp-based quota windows |
| `thiserror` | Structured error types |

No network calls. No file I/O in `pmo-core` (persistence layer is the caller's responsibility).
