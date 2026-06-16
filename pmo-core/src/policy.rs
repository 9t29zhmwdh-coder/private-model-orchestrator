use serde::{Deserialize, Serialize};

/// MDM-sourced policy hints.
///
/// Values originate from a custom Configuration Profile payload
/// delivered via Jamf / Apple Business Manager. See docs/mdm_integration.md.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MdmPolicy {
    /// Master switch: allow on-device inference at all.
    pub inference_allowed: bool,
    /// Restrict to listed model bundle IDs (empty = allow all registered bundles).
    pub allowed_model_ids: Vec<String>,
    /// Minimum macOS version required on the device. Enforcement is informational only.
    pub min_os_version: Option<String>,
    /// Disable profiling export (privacy-hardened fleets).
    pub disable_profiling: bool,
}

#[derive(Debug, Default)]
pub struct PolicyEngine {
    policy: MdmPolicy,
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_policy(&mut self, policy: MdmPolicy) {
        self.policy = policy;
    }

    pub fn current_policy(&self) -> &MdmPolicy {
        &self.policy
    }

    pub fn is_inference_allowed(&self) -> bool {
        self.policy.inference_allowed
    }

    pub fn is_model_allowed(&self, model_id: &str) -> bool {
        if !self.policy.inference_allowed {
            return false;
        }
        if self.policy.allowed_model_ids.is_empty() {
            return true;
        }
        self.policy.allowed_model_ids.iter().any(|id| id == model_id)
    }

    pub fn is_profiling_allowed(&self) -> bool {
        !self.policy.disable_profiling
    }
}
