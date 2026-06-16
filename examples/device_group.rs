//! Example: creating a device group and assigning a model.
//!
//! Run with: cargo run --example device_group

use pmo_core::{Device, DeviceRegistry, ModelBundle, ModelRegistry, ModelVariant};
use uuid::Uuid;

fn main() {
    let mut devices = DeviceRegistry::new();
    let mut models  = ModelRegistry::new();

    // Register a production model bundle
    let model_id = Uuid::new_v4();
    models.register(ModelBundle {
        id: model_id,
        name: "mistral-7b-aot".into(),
        version: "0.1.0".into(),
        variant: ModelVariant::MlModelC,
        checksum: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".into(),
        min_os_version: Some("14.0".into()),
    });

    // Create a device group for the engineering fleet
    let group_id = devices.create_group("Engineering Fleet");
    devices.assign_model(group_id, model_id);

    // Register two devices into that group
    for serial in ["C02XJ1AB0001", "C02XJ1AB0002"] {
        devices.register_device(Device {
            id: Uuid::new_v4(),
            serial: serial.into(),
            group_id: Some(group_id),
            hardware_model: "Mac14,6".into(),
            os_version: "15.0".into(),
        });
    }

    println!("Groups  : {}", devices.group_count());
    println!("Devices : {}", devices.device_count());
    println!(
        "Devices in group: {}",
        devices.devices_in_group(&group_id).len()
    );
    println!(
        "Model assigned  : {}",
        models.find_by_name("mistral-7b-aot").unwrap().name
    );
}
