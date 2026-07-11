use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Supported Core ML packaging variants.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModelVariant {
    /// Interpreted .mlpackage (development / flexible compute unit)
    MlPackage,
    /// Ahead-of-time compiled .mlmodelc (production / ANE-optimised)
    MlModelC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelBundle {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub variant: ModelVariant,
    /// SHA-256 hex digest of the bundle archive
    pub checksum: String,
    /// Minimum macOS version for deployment
    pub min_os_version: Option<String>,
}

#[derive(Debug, Default)]
pub struct ModelRegistry {
    bundles: Vec<ModelBundle>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Rebuilds a registry from previously persisted bundles.
    pub fn from_parts(bundles: Vec<ModelBundle>) -> Self {
        Self { bundles }
    }

    pub fn register(&mut self, bundle: ModelBundle) {
        self.bundles.push(bundle);
    }

    pub fn find_by_name(&self, name: &str) -> Option<&ModelBundle> {
        self.bundles.iter().find(|b| b.name == name)
    }

    pub fn find_by_id(&self, id: &Uuid) -> Option<&ModelBundle> {
        self.bundles.iter().find(|b| &b.id == id)
    }

    pub fn bundle_count(&self) -> usize {
        self.bundles.len()
    }

    pub fn production_bundles(&self) -> Vec<&ModelBundle> {
        self.bundles.iter().filter(|b| b.variant == ModelVariant::MlModelC).collect()
    }

    pub fn all_bundles(&self) -> &[ModelBundle] {
        &self.bundles
    }
}
