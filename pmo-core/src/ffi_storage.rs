//! UniFFI bridge to the SQLite-backed persistence layer (storage.rs).
//!
//! This is the facade `pmo-macos` actually uses for its dashboard views:
//! each call opens the registry it needs from SQLite, mutates it, and
//! saves it back, mirroring the load/mutate/save pattern pmo-cli already
//! uses for every subcommand. The session-only objects in ffi.rs
//! (FfiDeviceRegistry etc.) stay as they were for Phase 1's bridge demo;
//! this module is the one with real persistence behind it.

use std::sync::Mutex;

use uuid::Uuid;

use crate::device::Device;
use crate::ffi::{
    parse_uuid, FfiDevice, FfiDeviceGroup, FfiMdmPolicy, FfiModelBundle, FfiModelVariant,
    FfiQuotaUsage,
};
use crate::model::ModelBundle;
use crate::quota::QuotaLimit;
use crate::storage::{Storage, StorageError};

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum FfiStorageError {
    #[error("database error: {0}")]
    Database(String),
    #[error("'{0}' is not a valid ID (expected a UUID)")]
    InvalidId(String),
}

impl From<StorageError> for FfiStorageError {
    fn from(e: StorageError) -> Self {
        Self::Database(e.to_string())
    }
}

fn require_uuid(raw: &str) -> Result<Uuid, FfiStorageError> {
    parse_uuid(raw).ok_or_else(|| FfiStorageError::InvalidId(raw.to_string()))
}

#[derive(uniffi::Object)]
pub struct FfiStorage {
    inner: Mutex<Storage>,
}

#[uniffi::export]
impl FfiStorage {
    /// Opens (creating if needed) the SQLite database at `path`.
    #[uniffi::constructor]
    pub fn new(path: String) -> Result<Self, FfiStorageError> {
        let storage = Storage::open(path)?;
        Ok(Self { inner: Mutex::new(storage) })
    }

    // --- Devices ---------------------------------------------------------

    pub fn all_devices(&self) -> Result<Vec<FfiDevice>, FfiStorageError> {
        let storage = self.inner.lock().unwrap();
        Ok(storage.load_device_registry()?.all_devices().iter().map(FfiDevice::from).collect())
    }

    pub fn all_groups(&self) -> Result<Vec<FfiDeviceGroup>, FfiStorageError> {
        let storage = self.inner.lock().unwrap();
        Ok(storage.load_device_registry()?.all_groups().iter().map(FfiDeviceGroup::from).collect())
    }

    pub fn register_device(
        &self,
        serial: String,
        hardware_model: String,
        os_version: String,
    ) -> Result<String, FfiStorageError> {
        let storage = self.inner.lock().unwrap();
        let mut registry = storage.load_device_registry()?;
        let device = Device { id: Uuid::new_v4(), serial, group_id: None, hardware_model, os_version };
        let id = device.id.to_string();
        registry.register_device(device);
        storage.save_device_registry(&registry)?;
        Ok(id)
    }

    /// Removes a device. Returns false if the ID is unknown.
    pub fn remove_device(&self, device_id: String) -> Result<bool, FfiStorageError> {
        let id = require_uuid(&device_id)?;
        let storage = self.inner.lock().unwrap();
        let mut registry = storage.load_device_registry()?;
        let removed = registry.remove_device(id);
        storage.save_device_registry(&registry)?;
        Ok(removed)
    }

    pub fn create_group(&self, name: String) -> Result<String, FfiStorageError> {
        let storage = self.inner.lock().unwrap();
        let mut registry = storage.load_device_registry()?;
        let id = registry.create_group(name).to_string();
        storage.save_device_registry(&registry)?;
        Ok(id)
    }

    /// Moves a device into a group (pass `None` for `group_id` to unassign it).
    /// Returns false if the device ID is unknown.
    pub fn set_device_group(
        &self,
        device_id: String,
        group_id: Option<String>,
    ) -> Result<bool, FfiStorageError> {
        let device_id = require_uuid(&device_id)?;
        let group_id = group_id.map(|g| require_uuid(&g)).transpose()?;
        let storage = self.inner.lock().unwrap();
        let mut registry = storage.load_device_registry()?;
        let moved = registry.set_device_group(device_id, group_id);
        storage.save_device_registry(&registry)?;
        Ok(moved)
    }

    // --- Models --------------------------------------------------------------

    pub fn all_models(&self) -> Result<Vec<FfiModelBundle>, FfiStorageError> {
        let storage = self.inner.lock().unwrap();
        Ok(storage.load_model_registry()?.all_bundles().iter().map(FfiModelBundle::from).collect())
    }

    pub fn register_model(
        &self,
        name: String,
        version: String,
        variant: FfiModelVariant,
        checksum: String,
        min_os_version: Option<String>,
    ) -> Result<String, FfiStorageError> {
        let storage = self.inner.lock().unwrap();
        let mut registry = storage.load_model_registry()?;
        let bundle = ModelBundle { id: Uuid::new_v4(), name, version, variant: variant.into(), checksum, min_os_version };
        let id = bundle.id.to_string();
        registry.register(bundle);
        storage.save_model_registry(&registry)?;
        Ok(id)
    }

    // --- Quota -----------------------------------------------------------------

    pub fn quota_usage(&self, device_id: String) -> Result<FfiQuotaUsage, FfiStorageError> {
        let id = require_uuid(&device_id)?;
        let storage = self.inner.lock().unwrap();
        Ok(storage.load_quota_engine()?.usage_for(&id).into())
    }

    pub fn quota_is_allowed(&self, device_id: String) -> Result<bool, FfiStorageError> {
        let id = require_uuid(&device_id)?;
        let storage = self.inner.lock().unwrap();
        Ok(storage.load_quota_engine()?.is_allowed(&id))
    }

    pub fn set_quota_limit(
        &self,
        device_id: String,
        daily_max: Option<u64>,
        hourly_max: Option<u64>,
    ) -> Result<(), FfiStorageError> {
        let id = require_uuid(&device_id)?;
        let storage = self.inner.lock().unwrap();
        let mut engine = storage.load_quota_engine()?;
        engine.set_limit(id, QuotaLimit { daily_max, hourly_max });
        storage.save_quota_engine(&engine)?;
        Ok(())
    }

    pub fn record_inference(&self, device_id: String) -> Result<(), FfiStorageError> {
        let id = require_uuid(&device_id)?;
        let storage = self.inner.lock().unwrap();
        let mut engine = storage.load_quota_engine()?;
        engine.record_inference(id);
        storage.save_quota_engine(&engine)?;
        Ok(())
    }

    /// Resets every device's hourly usage counter (quota dashboard "reset" action).
    pub fn reset_hourly_quota(&self) -> Result<(), FfiStorageError> {
        let storage = self.inner.lock().unwrap();
        let mut engine = storage.load_quota_engine()?;
        engine.reset_hourly();
        storage.save_quota_engine(&engine)?;
        Ok(())
    }

    /// Resets every device's daily (and hourly) usage counter.
    pub fn reset_daily_quota(&self) -> Result<(), FfiStorageError> {
        let storage = self.inner.lock().unwrap();
        let mut engine = storage.load_quota_engine()?;
        engine.reset_daily();
        storage.save_quota_engine(&engine)?;
        Ok(())
    }
}

/// Loads an MDM Configuration Profile from a local JSON file. Read-only:
/// the dashboard's Policy view uses this to display gating rules, hot-reload
/// (`PolicyWatcher::poll_reload`) is wired in a later pass once the app has
/// a background timer to drive it.
#[uniffi::export]
pub fn load_policy_file(path: String) -> Result<FfiMdmPolicy, FfiStorageError> {
    let contents = std::fs::read_to_string(&path)
        .map_err(|e| FfiStorageError::Database(format!("failed to read {path}: {e}")))?;
    let policy: crate::policy::MdmPolicy = serde_json::from_str(&contents)
        .map_err(|e| FfiStorageError::Database(format!("invalid policy JSON in {path}: {e}")))?;
    Ok(FfiMdmPolicy::from(&policy))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_db_path() -> std::path::PathBuf {
        std::env::temp_dir().join(format!("pmo-ffi-storage-test-{}.db", Uuid::new_v4()))
    }

    #[test]
    fn device_lifecycle_round_trips_through_the_facade() {
        let path = temp_db_path();
        let storage = FfiStorage::new(path.to_string_lossy().into_owned()).unwrap();

        let device_id = storage
            .register_device("C02XJ1ABCD12".into(), "MacBookPro18,3".into(), "14.5".into())
            .unwrap();
        assert_eq!(storage.all_devices().unwrap().len(), 1);

        let group_id = storage.create_group("Engineering Fleet".into()).unwrap();
        assert!(storage.set_device_group(device_id.clone(), Some(group_id)).unwrap());

        assert!(storage.remove_device(device_id).unwrap());
        assert_eq!(storage.all_devices().unwrap().len(), 0);

        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn quota_lifecycle_round_trips_through_the_facade() {
        let path = temp_db_path();
        let storage = FfiStorage::new(path.to_string_lossy().into_owned()).unwrap();

        let device_id = storage.register_device("SERIAL".into(), "Mac mini".into(), "14.5".into()).unwrap();
        storage.set_quota_limit(device_id.clone(), Some(2), None).unwrap();
        assert!(storage.quota_is_allowed(device_id.clone()).unwrap());

        storage.record_inference(device_id.clone()).unwrap();
        storage.record_inference(device_id.clone()).unwrap();
        assert!(!storage.quota_is_allowed(device_id.clone()).unwrap());

        storage.reset_daily_quota().unwrap();
        assert!(storage.quota_is_allowed(device_id).unwrap());

        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn invalid_device_id_is_rejected() {
        let path = temp_db_path();
        let storage = FfiStorage::new(path.to_string_lossy().into_owned()).unwrap();
        assert!(storage.remove_device("not-a-uuid".into()).is_err());
        std::fs::remove_file(&path).ok();
    }
}
