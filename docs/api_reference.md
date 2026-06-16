# API Reference — pmo-core

## DeviceRegistry

```rust
pub struct DeviceRegistry { ... }

impl DeviceRegistry {
    pub fn new() -> Self
    pub fn register_device(&mut self, device: Device)
    pub fn create_group(&mut self, name: impl Into<String>) -> Uuid
    pub fn assign_model(&mut self, group_id: Uuid, model_id: Uuid) -> bool
    pub fn device_count(&self) -> usize
    pub fn group_count(&self) -> usize
    pub fn devices_in_group(&self, group_id: &Uuid) -> Vec<&Device>
}
```

## ModelRegistry

```rust
pub struct ModelRegistry { ... }

impl ModelRegistry {
    pub fn new() -> Self
    pub fn register(&mut self, bundle: ModelBundle)
    pub fn find_by_name(&self, name: &str) -> Option<&ModelBundle>
    pub fn find_by_id(&self, id: &Uuid) -> Option<&ModelBundle>
    pub fn bundle_count(&self) -> usize
    pub fn production_bundles(&self) -> Vec<&ModelBundle>
}
```

## QuotaEngine

```rust
pub struct QuotaEngine { ... }

impl QuotaEngine {
    pub fn new() -> Self
    pub fn set_limit(&mut self, device_id: Uuid, limit: QuotaLimit)
    pub fn record_inference(&mut self, device_id: Uuid)
    pub fn reset_hourly(&mut self)
    pub fn reset_daily(&mut self)
    pub fn is_allowed(&self, device_id: &Uuid) -> bool
    pub fn usage_for(&self, device_id: &Uuid) -> QuotaUsage
}
```

## PolicyEngine

```rust
pub struct PolicyEngine { ... }

impl PolicyEngine {
    pub fn new() -> Self
    pub fn load_policy(&mut self, policy: MdmPolicy)
    pub fn current_policy(&self) -> &MdmPolicy
    pub fn is_inference_allowed(&self) -> bool
    pub fn is_model_allowed(&self, model_id: &str) -> bool
    pub fn is_profiling_allowed(&self) -> bool
}
```

## ProfilingStub

```rust
pub struct ProfilingStub { ... }

impl ProfilingStub {
    pub fn new(enabled: bool) -> Self
    pub fn session(&self, label: &str) -> Option<ProfilingSession>
}

pub struct ProfilingSession { ... }

impl ProfilingSession {
    pub fn start(&mut self)
    pub fn stop(&mut self)
    pub fn elapsed_ms(&self) -> Option<f64>
    pub fn is_complete(&self) -> bool
}
```
