use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: Uuid,
    pub serial: String,
    pub group_id: Option<Uuid>,
    pub hardware_model: String,
    pub os_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceGroup {
    pub id: Uuid,
    pub name: String,
    pub model_id: Option<Uuid>,
}

#[derive(Debug, Default)]
pub struct DeviceRegistry {
    devices: Vec<Device>,
    groups: Vec<DeviceGroup>,
}

impl DeviceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Rebuilds a registry from previously persisted devices and groups.
    pub fn from_parts(devices: Vec<Device>, groups: Vec<DeviceGroup>) -> Self {
        Self { devices, groups }
    }

    pub fn register_device(&mut self, device: Device) {
        self.devices.push(device);
    }

    pub fn create_group(&mut self, name: impl Into<String>) -> Uuid {
        let id = Uuid::new_v4();
        self.groups.push(DeviceGroup { id, name: name.into(), model_id: None });
        id
    }

    pub fn assign_model(&mut self, group_id: Uuid, model_id: Uuid) -> bool {
        if let Some(g) = self.groups.iter_mut().find(|g| g.id == group_id) {
            g.model_id = Some(model_id);
            return true;
        }
        false
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    pub fn devices_in_group(&self, group_id: &Uuid) -> Vec<&Device> {
        self.devices.iter().filter(|d| d.group_id.as_ref() == Some(group_id)).collect()
    }

    pub fn all_devices(&self) -> &[Device] {
        &self.devices
    }

    pub fn all_groups(&self) -> &[DeviceGroup] {
        &self.groups
    }
}
