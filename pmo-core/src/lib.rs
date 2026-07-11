//! Private Model Orchestrator: core library.
//!
//! Provides device registry, model packaging, quota management,
//! MDM policy enforcement and performance profiling stubs for
//! on-device Foundation Model deployments on Apple platforms.

#[cfg(feature = "ffi")]
uniffi::setup_scaffolding!();

pub mod device;
#[cfg(feature = "ffi")]
pub mod ffi;
pub mod model;
pub mod policy;
#[cfg(feature = "persistence")]
pub mod policy_watch;
pub mod profiler;
pub mod quota;
#[cfg(feature = "persistence")]
pub mod storage;

pub use device::{Device, DeviceGroup, DeviceRegistry};
pub use model::{ModelBundle, ModelRegistry, ModelVariant};
pub use policy::{MdmPolicy, PolicyEngine};
#[cfg(feature = "persistence")]
pub use policy_watch::{PolicyWatchError, PolicyWatcher};
pub use profiler::{ProfilingSession, ProfilingStub};
pub use quota::{QuotaEngine, QuotaLimit, QuotaUsage};
#[cfg(feature = "persistence")]
pub use storage::{Storage, StorageError};

/// Top-level orchestrator configuration.
#[derive(Debug, Clone)]
pub struct OrchestratorConfig {
    pub device_registry_path: std::path::PathBuf,
    pub model_registry_path: std::path::PathBuf,
    pub enable_profiling: bool,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            device_registry_path: std::path::PathBuf::from("devices.db"),
            model_registry_path: std::path::PathBuf::from("models.db"),
            enable_profiling: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_profiling_disabled() {
        let cfg = OrchestratorConfig::default();
        assert!(!cfg.enable_profiling);
    }

    #[test]
    fn default_config_paths_are_relative() {
        let cfg = OrchestratorConfig::default();
        assert!(!cfg.device_registry_path.is_absolute());
        assert!(!cfg.model_registry_path.is_absolute());
    }

    #[test]
    fn device_registry_registers_and_counts() {
        use uuid::Uuid;
        let mut reg = DeviceRegistry::new();
        reg.register_device(Device {
            id: Uuid::new_v4(),
            serial: "C02XJ1ABCD12".into(),
            group_id: None,
            hardware_model: "MacBookPro18,3".into(),
            os_version: "14.5".into(),
        });
        assert_eq!(reg.device_count(), 1);
    }

    #[test]
    fn quota_engine_blocks_when_limit_reached() {
        use uuid::Uuid;
        let id = Uuid::new_v4();
        let mut engine = QuotaEngine::new();
        engine.set_limit(id, QuotaLimit { daily_max: Some(2), hourly_max: None });
        assert!(engine.is_allowed(&id));
        engine.record_inference(id);
        engine.record_inference(id);
        assert!(!engine.is_allowed(&id));
    }

    #[test]
    fn policy_engine_blocks_when_inference_disabled() {
        let mut engine = PolicyEngine::new();
        engine.load_policy(MdmPolicy {
            inference_allowed: false,
            ..Default::default()
        });
        assert!(!engine.is_inference_allowed());
        assert!(!engine.is_model_allowed("any-model"));
    }

    #[test]
    fn model_registry_finds_bundle_by_name() {
        use uuid::Uuid;
        let mut reg = ModelRegistry::new();
        reg.register(ModelBundle {
            id: Uuid::new_v4(),
            name: "mistral-7b".into(),
            version: "0.1.0".into(),
            variant: ModelVariant::MlModelC,
            checksum: "abc123".into(),
            min_os_version: Some("14.0".into()),
        });
        assert!(reg.find_by_name("mistral-7b").is_some());
        assert!(reg.find_by_name("unknown").is_none());
    }

    #[test]
    fn profiling_session_measures_elapsed() {
        let mut session = ProfilingSession::new("test-inference");
        assert!(!session.is_complete());
        session.start();
        session.stop();
        assert!(session.is_complete());
        assert!(session.elapsed_ms().unwrap() >= 0.0);
    }
}
