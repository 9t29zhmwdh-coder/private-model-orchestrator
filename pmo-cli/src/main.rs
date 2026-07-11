use clap::{Parser, Subcommand, ValueEnum};
use pmo_core::{Device, ModelBundle, ModelVariant, OrchestratorConfig, QuotaLimit, Storage};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "pmo-cli", version, about = "Private Model Orchestrator CLI")]
struct Cli {
    /// Path to the SQLite database file.
    #[arg(long, global = true, default_value = "pmo.db")]
    db: std::path::PathBuf,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Device registry operations.
    Device {
        #[command(subcommand)]
        action: DeviceAction,
    },
    /// Model bundle operations.
    Model {
        #[command(subcommand)]
        action: ModelAction,
    },
    /// Inference quota operations.
    Quota {
        #[command(subcommand)]
        action: QuotaAction,
    },
}

#[derive(Subcommand)]
enum DeviceAction {
    /// List all registered devices.
    List,
    /// Register a new device.
    Register {
        #[arg(long)]
        serial: String,
        #[arg(long)]
        hardware_model: String,
        #[arg(long)]
        os_version: String,
    },
}

#[derive(Subcommand)]
enum ModelAction {
    /// Register a model bundle.
    Register {
        #[arg(long)]
        name: String,
        #[arg(long)]
        version: String,
        #[arg(long, value_enum)]
        variant: VariantArg,
        #[arg(long)]
        checksum: String,
        #[arg(long)]
        min_os_version: Option<String>,
    },
    /// List all registered model bundles.
    List,
}

#[derive(Clone, ValueEnum)]
enum VariantArg {
    MlPackage,
    MlModelC,
}

impl From<VariantArg> for ModelVariant {
    fn from(v: VariantArg) -> Self {
        match v {
            VariantArg::MlPackage => ModelVariant::MlPackage,
            VariantArg::MlModelC => ModelVariant::MlModelC,
        }
    }
}

#[derive(Subcommand)]
enum QuotaAction {
    /// Show quota usage and remaining allowance for a device.
    Status {
        #[arg(long)]
        device: String,
    },
    /// Set daily/hourly inference limits for a device.
    SetLimit {
        #[arg(long)]
        device: String,
        #[arg(long)]
        daily: Option<u64>,
        #[arg(long)]
        hourly: Option<u64>,
    },
}

fn parse_device_id(raw: &str) -> Uuid {
    Uuid::parse_str(raw).unwrap_or_else(|_| {
        eprintln!("error: '{raw}' is not a valid device ID (expected a UUID)");
        std::process::exit(1);
    })
}

fn open_storage(path: &std::path::Path) -> Storage {
    Storage::open(path).unwrap_or_else(|e| {
        eprintln!("error: failed to open database at {}: {e}", path.display());
        std::process::exit(1);
    })
}

fn main() {
    let cli = Cli::parse();

    let Some(command) = cli.command else {
        let cfg = OrchestratorConfig::default();
        println!("Private Model Orchestrator v{}", env!("CARGO_PKG_VERSION"));
        println!("Database: {}", cli.db.display());
        println!("Profiling: {}", cfg.enable_profiling);
        println!("\nRun `pmo-cli --help` to see available subcommands (device, model, quota).");
        return;
    };

    let storage = open_storage(&cli.db);

    match command {
        Command::Device { action } => match action {
            DeviceAction::List => {
                let registry = storage.load_device_registry().expect("failed to load devices");
                if registry.all_devices().is_empty() {
                    println!("No devices registered yet.");
                }
                for d in registry.all_devices() {
                    println!("{}  {}  {}  os {}", d.id, d.serial, d.hardware_model, d.os_version);
                }
            }
            DeviceAction::Register { serial, hardware_model, os_version } => {
                let mut registry = storage.load_device_registry().expect("failed to load devices");
                let device = Device { id: Uuid::new_v4(), serial, group_id: None, hardware_model, os_version };
                let id = device.id;
                registry.register_device(device);
                storage.save_device_registry(&registry).expect("failed to save devices");
                println!("Registered device {id}");
            }
        },
        Command::Model { action } => match action {
            ModelAction::List => {
                let registry = storage.load_model_registry().expect("failed to load models");
                if registry.all_bundles().is_empty() {
                    println!("No model bundles registered yet.");
                }
                for b in registry.all_bundles() {
                    println!("{}  {} v{}  {:?}  checksum {}", b.id, b.name, b.version, b.variant, b.checksum);
                }
            }
            ModelAction::Register { name, version, variant, checksum, min_os_version } => {
                let mut registry = storage.load_model_registry().expect("failed to load models");
                let bundle = ModelBundle {
                    id: Uuid::new_v4(),
                    name,
                    version,
                    variant: variant.into(),
                    checksum,
                    min_os_version,
                };
                let id = bundle.id;
                registry.register(bundle);
                storage.save_model_registry(&registry).expect("failed to save models");
                println!("Registered model bundle {id}");
            }
        },
        Command::Quota { action } => match action {
            QuotaAction::Status { device } => {
                let device_id = parse_device_id(&device);
                let engine = storage.load_quota_engine().expect("failed to load quota data");
                let usage = engine.usage_for(&device_id);
                let allowed = engine.is_allowed(&device_id);
                println!(
                    "Device {device_id}: {} inferences today, {} this hour. Currently {}.",
                    usage.daily_count,
                    usage.hourly_count,
                    if allowed { "allowed" } else { "blocked by quota" }
                );
            }
            QuotaAction::SetLimit { device, daily, hourly } => {
                let device_id = parse_device_id(&device);
                let mut engine = storage.load_quota_engine().expect("failed to load quota data");
                engine.set_limit(device_id, QuotaLimit { daily_max: daily, hourly_max: hourly });
                storage.save_quota_engine(&engine).expect("failed to save quota data");
                println!("Set limit for device {device_id}: daily {daily:?}, hourly {hourly:?}");
            }
        },
    }
}
