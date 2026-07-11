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

    /// Removes a device by ID. Returns false if no device with that ID exists.
    pub fn remove_device(&mut self, id: Uuid) -> bool {
        let len_before = self.devices.len();
        self.devices.retain(|d| d.id != id);
        self.devices.len() != len_before
    }

    /// Moves a device into a group (or out of any group, if `group_id` is `None`).
    /// Returns false if the device ID is unknown.
    pub fn set_device_group(&mut self, device_id: Uuid, group_id: Option<Uuid>) -> bool {
        if let Some(d) = self.devices.iter_mut().find(|d| d.id == device_id) {
            d.group_id = group_id;
            return true;
        }
        false
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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_device() -> Device {
        Device {
            id: Uuid::new_v4(),
            serial: "C02XJ1ABCD12".into(),
            group_id: None,
            hardware_model: "MacBookPro18,3".into(),
            os_version: "14.5".into(),
        }
    }

    #[test]
    fn remove_device_deletes_a_known_device() {
        let mut reg = DeviceRegistry::new();
        let device = sample_device();
        let id = device.id;
        reg.register_device(device);
        assert_eq!(reg.device_count(), 1);

        assert!(reg.remove_device(id));
        assert_eq!(reg.device_count(), 0);
    }

    #[test]
    fn remove_device_returns_false_for_unknown_id() {
        let mut reg = DeviceRegistry::new();
        assert!(!reg.remove_device(Uuid::new_v4()));
    }

    #[test]
    fn set_device_group_moves_a_device_between_groups() {
        let mut reg = DeviceRegistry::new();
        let device = sample_device();
        let id = device.id;
        reg.register_device(device);
        let group_id = reg.create_group("Engineering Fleet");

        assert!(reg.set_device_group(id, Some(group_id)));
        assert_eq!(reg.devices_in_group(&group_id).len(), 1);

        assert!(reg.set_device_group(id, None));
        assert_eq!(reg.devices_in_group(&group_id).len(), 0);
    }

    #[test]
    fn set_device_group_returns_false_for_unknown_device() {
        let mut reg = DeviceRegistry::new();
        let group_id = reg.create_group("Engineering Fleet");
        assert!(!reg.set_device_group(Uuid::new_v4(), Some(group_id)));
    }
}
