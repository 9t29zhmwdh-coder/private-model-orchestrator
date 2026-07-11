//! UniFFI bridge: exposes pmo-core to Swift as a set of thread-safe
//! objects. Each wraps the corresponding plain Rust type behind a
//! `Mutex`, since UniFFI object methods take `&self` (the generated
//! Swift class holds a shared reference) while the underlying
//! registries use `&mut self`.

use std::sync::Mutex;
use uuid::Uuid;

use crate::device::{Device, DeviceGroup, DeviceRegistry};
use crate::model::{ModelBundle, ModelRegistry, ModelVariant};
use crate::policy::{MdmPolicy, PolicyEngine};
use crate::quota::{QuotaEngine, QuotaLimit, QuotaUsage};

#[derive(uniffi::Record, Clone)]
pub struct FfiDevice {
    pub id: String,
    pub serial: String,
    pub group_id: Option<String>,
    pub hardware_model: String,
    pub os_version: String,
}

impl From<&Device> for FfiDevice {
    fn from(d: &Device) -> Self {
        Self {
            id: d.id.to_string(),
            serial: d.serial.clone(),
            group_id: d.group_id.map(|g| g.to_string()),
            hardware_model: d.hardware_model.clone(),
            os_version: d.os_version.clone(),
        }
    }
}

#[derive(uniffi::Record, Clone)]
pub struct FfiDeviceGroup {
    pub id: String,
    pub name: String,
    pub model_id: Option<String>,
}

impl From<&DeviceGroup> for FfiDeviceGroup {
    fn from(g: &DeviceGroup) -> Self {
        Self {
            id: g.id.to_string(),
            name: g.name.clone(),
            model_id: g.model_id.map(|m| m.to_string()),
        }
    }
}

#[derive(uniffi::Record, Clone)]
pub struct FfiModelBundle {
    pub id: String,
    pub name: String,
    pub version: String,
    pub variant: FfiModelVariant,
    pub checksum: String,
    pub min_os_version: Option<String>,
}

impl From<&ModelBundle> for FfiModelBundle {
    fn from(b: &ModelBundle) -> Self {
        Self {
            id: b.id.to_string(),
            name: b.name.clone(),
            version: b.version.clone(),
            variant: b.variant.clone().into(),
            checksum: b.checksum.clone(),
            min_os_version: b.min_os_version.clone(),
        }
    }
}

#[derive(uniffi::Enum, Clone)]
pub enum FfiModelVariant {
    MlPackage,
    MlModelC,
}

impl From<ModelVariant> for FfiModelVariant {
    fn from(v: ModelVariant) -> Self {
        match v {
            ModelVariant::MlPackage => Self::MlPackage,
            ModelVariant::MlModelC => Self::MlModelC,
        }
    }
}

impl From<FfiModelVariant> for ModelVariant {
    fn from(v: FfiModelVariant) -> Self {
        match v {
            FfiModelVariant::MlPackage => Self::MlPackage,
            FfiModelVariant::MlModelC => Self::MlModelC,
        }
    }
}

#[derive(uniffi::Record, Clone)]
pub struct FfiQuotaUsage {
    pub daily_count: u64,
    pub hourly_count: u64,
}

impl From<QuotaUsage> for FfiQuotaUsage {
    fn from(u: QuotaUsage) -> Self {
        Self { daily_count: u.daily_count, hourly_count: u.hourly_count }
    }
}

#[derive(uniffi::Record, Clone, Default)]
pub struct FfiMdmPolicy {
    pub inference_allowed: bool,
    pub allowed_model_ids: Vec<String>,
    pub min_os_version: Option<String>,
    pub disable_profiling: bool,
}

impl From<&MdmPolicy> for FfiMdmPolicy {
    fn from(p: &MdmPolicy) -> Self {
        Self {
            inference_allowed: p.inference_allowed,
            allowed_model_ids: p.allowed_model_ids.clone(),
            min_os_version: p.min_os_version.clone(),
            disable_profiling: p.disable_profiling,
        }
    }
}

impl From<FfiMdmPolicy> for MdmPolicy {
    fn from(p: FfiMdmPolicy) -> Self {
        Self {
            inference_allowed: p.inference_allowed,
            allowed_model_ids: p.allowed_model_ids,
            min_os_version: p.min_os_version,
            disable_profiling: p.disable_profiling,
        }
    }
}

pub(crate) fn parse_uuid(s: &str) -> Option<Uuid> {
    Uuid::parse_str(s).ok()
}

#[derive(uniffi::Object, Default)]
pub struct FfiDeviceRegistry {
    inner: Mutex<DeviceRegistry>,
}

#[uniffi::export]
impl FfiDeviceRegistry {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a new device, returns its generated ID.
    pub fn register_device(&self, serial: String, hardware_model: String, os_version: String) -> String {
        let device = Device {
            id: Uuid::new_v4(),
            serial,
            group_id: None,
            hardware_model,
            os_version,
        };
        let id = device.id.to_string();
        self.inner.lock().unwrap().register_device(device);
        id
    }

    /// Creates a device group, returns its generated ID.
    pub fn create_group(&self, name: String) -> String {
        self.inner.lock().unwrap().create_group(name).to_string()
    }

    /// Assigns a model bundle to a group. Returns false if the group ID is unknown or malformed.
    pub fn assign_model(&self, group_id: String, model_id: String) -> bool {
        let (Some(g), Some(m)) = (parse_uuid(&group_id), parse_uuid(&model_id)) else {
            return false;
        };
        self.inner.lock().unwrap().assign_model(g, m)
    }

    pub fn device_count(&self) -> u32 {
        self.inner.lock().unwrap().device_count() as u32
    }

    pub fn group_count(&self) -> u32 {
        self.inner.lock().unwrap().group_count() as u32
    }

    /// Lists every device registered so far (dashboard "Devices" view).
    pub fn all_devices(&self) -> Vec<FfiDevice> {
        self.inner.lock().unwrap().all_devices().iter().map(FfiDevice::from).collect()
    }

    /// Lists every device in a group. Returns an empty list if the group ID is malformed.
    pub fn devices_in_group(&self, group_id: String) -> Vec<FfiDevice> {
        let Some(id) = parse_uuid(&group_id) else { return Vec::new() };
        self.inner.lock().unwrap().devices_in_group(&id).into_iter().map(FfiDevice::from).collect()
    }

    /// Lists every device group (dashboard group picker).
    pub fn all_groups(&self) -> Vec<FfiDeviceGroup> {
        self.inner.lock().unwrap().all_groups().iter().map(FfiDeviceGroup::from).collect()
    }

    /// Removes a device. Returns false if the ID is unknown or malformed.
    pub fn remove_device(&self, device_id: String) -> bool {
        let Some(id) = parse_uuid(&device_id) else { return false };
        self.inner.lock().unwrap().remove_device(id)
    }
}

#[derive(uniffi::Object, Default)]
pub struct FfiModelRegistry {
    inner: Mutex<ModelRegistry>,
}

#[uniffi::export]
impl FfiModelRegistry {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a model bundle, returns its generated ID.
    pub fn register(
        &self,
        name: String,
        version: String,
        variant: FfiModelVariant,
        checksum: String,
        min_os_version: Option<String>,
    ) -> String {
        let bundle = ModelBundle {
            id: Uuid::new_v4(),
            name,
            version,
            variant: variant.into(),
            checksum,
            min_os_version,
        };
        let id = bundle.id.to_string();
        self.inner.lock().unwrap().register(bundle);
        id
    }

    pub fn find_by_name(&self, name: String) -> Option<FfiModelBundle> {
        self.inner.lock().unwrap().find_by_name(&name).map(FfiModelBundle::from)
    }

    pub fn bundle_count(&self) -> u32 {
        self.inner.lock().unwrap().bundle_count() as u32
    }

    /// Lists every registered model bundle (dashboard "Models" view).
    pub fn all_bundles(&self) -> Vec<FfiModelBundle> {
        self.inner.lock().unwrap().all_bundles().iter().map(FfiModelBundle::from).collect()
    }
}

#[derive(uniffi::Object, Default)]
pub struct FfiQuotaEngine {
    inner: Mutex<QuotaEngine>,
}

#[uniffi::export]
impl FfiQuotaEngine {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets daily/hourly caps for a device. Pass `None` for either to leave it unlimited.
    pub fn set_limit(&self, device_id: String, daily_max: Option<u64>, hourly_max: Option<u64>) -> bool {
        let Some(id) = parse_uuid(&device_id) else { return false };
        self.inner.lock().unwrap().set_limit(id, QuotaLimit { daily_max, hourly_max });
        true
    }

    pub fn record_inference(&self, device_id: String) -> bool {
        let Some(id) = parse_uuid(&device_id) else { return false };
        self.inner.lock().unwrap().record_inference(id);
        true
    }

    pub fn reset_hourly(&self) {
        self.inner.lock().unwrap().reset_hourly();
    }

    pub fn reset_daily(&self) {
        self.inner.lock().unwrap().reset_daily();
    }

    pub fn is_allowed(&self, device_id: String) -> bool {
        let Some(id) = parse_uuid(&device_id) else { return false };
        self.inner.lock().unwrap().is_allowed(&id)
    }

    pub fn usage_for(&self, device_id: String) -> Option<FfiQuotaUsage> {
        let id = parse_uuid(&device_id)?;
        Some(self.inner.lock().unwrap().usage_for(&id).into())
    }
}

#[derive(uniffi::Object, Default)]
pub struct FfiPolicyEngine {
    inner: Mutex<PolicyEngine>,
}

#[uniffi::export]
impl FfiPolicyEngine {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_policy(&self, policy: FfiMdmPolicy) {
        self.inner.lock().unwrap().load_policy(policy.into());
    }

    pub fn current_policy(&self) -> FfiMdmPolicy {
        FfiMdmPolicy::from(self.inner.lock().unwrap().current_policy())
    }

    pub fn is_inference_allowed(&self) -> bool {
        self.inner.lock().unwrap().is_inference_allowed()
    }

    pub fn is_model_allowed(&self, model_id: String) -> bool {
        self.inner.lock().unwrap().is_model_allowed(&model_id)
    }

    pub fn is_profiling_allowed(&self) -> bool {
        self.inner.lock().unwrap().is_profiling_allowed()
    }
}
