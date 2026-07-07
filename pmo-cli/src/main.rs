use pmo_core::{DeviceRegistry, ModelRegistry, OrchestratorConfig, PolicyEngine, QuotaEngine};

fn main() {
    let cfg = OrchestratorConfig::default();
    println!("Private Model Orchestrator v{}", env!("CARGO_PKG_VERSION"));
    println!("Device registry : {}", cfg.device_registry_path.display());
    println!("Model registry  : {}", cfg.model_registry_path.display());
    println!("Profiling       : {}", cfg.enable_profiling);

    // Bootstrap registries (in-memory for this stub)
    let _devices = DeviceRegistry::new();
    let _models  = ModelRegistry::new();
    let _quota   = QuotaEngine::new();
    let _policy  = PolicyEngine::new();

    println!("\nAll subsystems initialised. This is a v0.1.0 bootstrap check; interactive subcommands (device list, model register, quota status) land in v0.2.0, see ROADMAP.md.");
}
