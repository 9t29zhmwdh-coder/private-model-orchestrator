//! Example: registering model variants and querying production bundles.
//!
//! Run with: cargo run --example model_packaging

use pmo_core::{ModelBundle, ModelRegistry, ModelVariant};
use uuid::Uuid;

fn main() {
    let mut registry = ModelRegistry::new();

    // Development (interpreted) variant
    registry.register(ModelBundle {
        id: Uuid::new_v4(),
        name: "mistral-7b".into(),
        version: "0.1.0-dev".into(),
        variant: ModelVariant::MlPackage,
        checksum: "dev-checksum-placeholder".into(),
        min_os_version: None,
    });

    // Production (AOT compiled) variant
    registry.register(ModelBundle {
        id: Uuid::new_v4(),
        name: "mistral-7b".into(),
        version: "0.1.0".into(),
        variant: ModelVariant::MlModelC,
        checksum: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".into(),
        min_os_version: Some("14.0".into()),
    });

    println!("Total bundles     : {}", registry.bundle_count());
    println!("Production bundles: {}", registry.production_bundles().len());

    for bundle in registry.production_bundles() {
        println!(
            "  {} v{} [{:?}] min_os={}",
            bundle.name,
            bundle.version,
            bundle.variant,
            bundle.min_os_version.as_deref().unwrap_or("any")
        );
    }
}
