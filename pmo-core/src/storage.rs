//! SQLite persistence for the device, model and quota registries.
//!
//! Each registry stays a plain in-memory struct (see device.rs, model.rs,
//! quota.rs); this module only knows how to serialize one to a SQLite
//! connection and rebuild one from it. Callers own the load/save timing:
//! `pmo-cli` loads on startup and saves after every mutating subcommand.

use std::path::Path;

use rusqlite::Connection;
use uuid::Uuid;

use crate::device::{Device, DeviceGroup, DeviceRegistry};
use crate::model::{ModelBundle, ModelRegistry, ModelVariant};
use crate::quota::{QuotaEngine, QuotaLimit, QuotaUsage};

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("database error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("stored UUID is malformed: {0}")]
    InvalidUuid(String),
    #[error("stored model variant is unknown: {0}")]
    InvalidVariant(String),
}

pub struct Storage {
    conn: Connection,
}

impl Storage {
    /// Opens (creating if needed) a SQLite database at `path` and runs migrations.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        let conn = Connection::open(path)?;
        let storage = Self { conn };
        storage.migrate()?;
        Ok(storage)
    }

    /// Opens an in-memory database. Used by tests and by callers that only
    /// want the schema without touching disk.
    pub fn open_in_memory() -> Result<Self, StorageError> {
        let conn = Connection::open_in_memory()?;
        let storage = Self { conn };
        storage.migrate()?;
        Ok(storage)
    }

    fn migrate(&self) -> Result<(), StorageError> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS devices (
                id TEXT PRIMARY KEY,
                serial TEXT NOT NULL,
                group_id TEXT,
                hardware_model TEXT NOT NULL,
                os_version TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS device_groups (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                model_id TEXT
            );
            CREATE TABLE IF NOT EXISTS model_bundles (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                version TEXT NOT NULL,
                variant TEXT NOT NULL,
                checksum TEXT NOT NULL,
                min_os_version TEXT
            );
            CREATE TABLE IF NOT EXISTS quota_limits (
                device_id TEXT PRIMARY KEY,
                daily_max INTEGER,
                hourly_max INTEGER
            );
            CREATE TABLE IF NOT EXISTS quota_usage (
                device_id TEXT PRIMARY KEY,
                daily_count INTEGER NOT NULL,
                hourly_count INTEGER NOT NULL
            );
            ",
        )?;
        Ok(())
    }

    fn parse_uuid(s: &str) -> Result<Uuid, StorageError> {
        Uuid::parse_str(s).map_err(|_| StorageError::InvalidUuid(s.to_string()))
    }

    // --- Devices ---------------------------------------------------------

    pub fn save_device_registry(&self, registry: &DeviceRegistry) -> Result<(), StorageError> {
        self.conn.execute("DELETE FROM devices", [])?;
        self.conn.execute("DELETE FROM device_groups", [])?;
        for d in registry.all_devices() {
            self.conn.execute(
                "INSERT INTO devices (id, serial, group_id, hardware_model, os_version) VALUES (?1, ?2, ?3, ?4, ?5)",
                (
                    d.id.to_string(),
                    &d.serial,
                    d.group_id.map(|g| g.to_string()),
                    &d.hardware_model,
                    &d.os_version,
                ),
            )?;
        }
        for g in registry.all_groups() {
            self.conn.execute(
                "INSERT INTO device_groups (id, name, model_id) VALUES (?1, ?2, ?3)",
                (g.id.to_string(), &g.name, g.model_id.map(|m| m.to_string())),
            )?;
        }
        Ok(())
    }

    pub fn load_device_registry(&self) -> Result<DeviceRegistry, StorageError> {
        let mut devices = Vec::new();
        let mut stmt = self.conn.prepare(
            "SELECT id, serial, group_id, hardware_model, os_version FROM devices",
        )?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let group_id: Option<String> = row.get(2)?;
            devices.push(Device {
                id: Self::parse_uuid(&id)?,
                serial: row.get(1)?,
                group_id: group_id.map(|g| Self::parse_uuid(&g)).transpose()?,
                hardware_model: row.get(3)?,
                os_version: row.get(4)?,
            });
        }

        let mut groups = Vec::new();
        let mut stmt = self.conn.prepare("SELECT id, name, model_id FROM device_groups")?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let model_id: Option<String> = row.get(2)?;
            groups.push(DeviceGroup {
                id: Self::parse_uuid(&id)?,
                name: row.get(1)?,
                model_id: model_id.map(|m| Self::parse_uuid(&m)).transpose()?,
            });
        }

        Ok(DeviceRegistry::from_parts(devices, groups))
    }

    // --- Models ------------------------------------------------------------

    pub fn save_model_registry(&self, registry: &ModelRegistry) -> Result<(), StorageError> {
        self.conn.execute("DELETE FROM model_bundles", [])?;
        for b in registry.all_bundles() {
            let variant = match b.variant {
                ModelVariant::MlPackage => "mlpackage",
                ModelVariant::MlModelC => "mlmodelc",
            };
            self.conn.execute(
                "INSERT INTO model_bundles (id, name, version, variant, checksum, min_os_version) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                (b.id.to_string(), &b.name, &b.version, variant, &b.checksum, &b.min_os_version),
            )?;
        }
        Ok(())
    }

    pub fn load_model_registry(&self) -> Result<ModelRegistry, StorageError> {
        let mut bundles = Vec::new();
        let mut stmt = self.conn.prepare(
            "SELECT id, name, version, variant, checksum, min_os_version FROM model_bundles",
        )?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let variant_str: String = row.get(3)?;
            let variant = match variant_str.as_str() {
                "mlpackage" => ModelVariant::MlPackage,
                "mlmodelc" => ModelVariant::MlModelC,
                other => return Err(StorageError::InvalidVariant(other.to_string())),
            };
            bundles.push(ModelBundle {
                id: Self::parse_uuid(&id)?,
                name: row.get(1)?,
                version: row.get(2)?,
                variant,
                checksum: row.get(4)?,
                min_os_version: row.get(5)?,
            });
        }
        Ok(ModelRegistry::from_parts(bundles))
    }

    // --- Quota ---------------------------------------------------------------

    pub fn save_quota_engine(&self, engine: &QuotaEngine) -> Result<(), StorageError> {
        self.conn.execute("DELETE FROM quota_limits", [])?;
        self.conn.execute("DELETE FROM quota_usage", [])?;
        for (device_id, limit) in engine.all_limits() {
            self.conn.execute(
                "INSERT INTO quota_limits (device_id, daily_max, hourly_max) VALUES (?1, ?2, ?3)",
                (
                    device_id.to_string(),
                    limit.daily_max.map(|v| v as i64),
                    limit.hourly_max.map(|v| v as i64),
                ),
            )?;
        }
        for (device_id, usage) in engine.all_usage() {
            self.conn.execute(
                "INSERT INTO quota_usage (device_id, daily_count, hourly_count) VALUES (?1, ?2, ?3)",
                (device_id.to_string(), usage.daily_count as i64, usage.hourly_count as i64),
            )?;
        }
        Ok(())
    }

    pub fn load_quota_engine(&self) -> Result<QuotaEngine, StorageError> {
        let mut limits = std::collections::HashMap::new();
        let mut stmt = self.conn.prepare("SELECT device_id, daily_max, hourly_max FROM quota_limits")?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let device_id: String = row.get(0)?;
            let daily_max: Option<i64> = row.get(1)?;
            let hourly_max: Option<i64> = row.get(2)?;
            limits.insert(
                Self::parse_uuid(&device_id)?,
                QuotaLimit { daily_max: daily_max.map(|v| v as u64), hourly_max: hourly_max.map(|v| v as u64) },
            );
        }

        let mut usage = std::collections::HashMap::new();
        let mut stmt = self.conn.prepare("SELECT device_id, daily_count, hourly_count FROM quota_usage")?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let device_id: String = row.get(0)?;
            let daily_count: i64 = row.get(1)?;
            let hourly_count: i64 = row.get(2)?;
            usage.insert(
                Self::parse_uuid(&device_id)?,
                QuotaUsage { daily_count: daily_count as u64, hourly_count: hourly_count as u64 },
            );
        }

        Ok(QuotaEngine::from_parts(limits, usage))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::Device;

    #[test]
    fn device_registry_round_trips_through_sqlite() {
        let storage = Storage::open_in_memory().unwrap();

        let mut registry = DeviceRegistry::new();
        let group_id = registry.create_group("Engineering Fleet");
        registry.register_device(Device {
            id: Uuid::new_v4(),
            serial: "C02XJ1ABCD12".into(),
            group_id: Some(group_id),
            hardware_model: "MacBookPro18,3".into(),
            os_version: "14.5".into(),
        });

        storage.save_device_registry(&registry).unwrap();
        let reloaded = storage.load_device_registry().unwrap();

        assert_eq!(reloaded.device_count(), 1);
        assert_eq!(reloaded.group_count(), 1);
        assert_eq!(reloaded.all_devices()[0].serial, "C02XJ1ABCD12");
        assert_eq!(reloaded.all_devices()[0].group_id, Some(group_id));
    }

    #[test]
    fn model_registry_round_trips_through_sqlite() {
        let storage = Storage::open_in_memory().unwrap();

        let mut registry = ModelRegistry::new();
        registry.register(ModelBundle {
            id: Uuid::new_v4(),
            name: "mistral-7b".into(),
            version: "0.1.0".into(),
            variant: ModelVariant::MlModelC,
            checksum: "abc123".into(),
            min_os_version: Some("14.0".into()),
        });

        storage.save_model_registry(&registry).unwrap();
        let reloaded = storage.load_model_registry().unwrap();

        assert_eq!(reloaded.bundle_count(), 1);
        assert!(reloaded.find_by_name("mistral-7b").is_some());
        assert_eq!(reloaded.find_by_name("mistral-7b").unwrap().variant, ModelVariant::MlModelC);
    }

    #[test]
    fn quota_engine_round_trips_through_sqlite() {
        let storage = Storage::open_in_memory().unwrap();

        let device_id = Uuid::new_v4();
        let mut engine = QuotaEngine::new();
        engine.set_limit(device_id, QuotaLimit { daily_max: Some(10), hourly_max: None });
        engine.record_inference(device_id);
        engine.record_inference(device_id);

        storage.save_quota_engine(&engine).unwrap();
        let reloaded = storage.load_quota_engine().unwrap();

        let usage = reloaded.usage_for(&device_id);
        assert_eq!(usage.daily_count, 2);
        assert!(reloaded.is_allowed(&device_id));
    }

    #[test]
    fn reopening_the_same_database_file_persists_across_restarts() {
        let dir = std::env::temp_dir().join(format!("pmo-test-{}", Uuid::new_v4()));
        let db_path = dir.join("pmo.db");
        std::fs::create_dir_all(&dir).unwrap();

        {
            let storage = Storage::open(&db_path).unwrap();
            let mut registry = DeviceRegistry::new();
            registry.register_device(Device {
                id: Uuid::new_v4(),
                serial: "RESTART-TEST".into(),
                group_id: None,
                hardware_model: "Mac mini".into(),
                os_version: "14.5".into(),
            });
            storage.save_device_registry(&registry).unwrap();
        }

        {
            let storage = Storage::open(&db_path).unwrap();
            let reloaded = storage.load_device_registry().unwrap();
            assert_eq!(reloaded.device_count(), 1);
            assert_eq!(reloaded.all_devices()[0].serial, "RESTART-TEST");
        }

        std::fs::remove_dir_all(&dir).ok();
    }
}
