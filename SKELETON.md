# Private Model Orchestrator: Professional Repo Skeleton

**Generated:** 2026-06-16 | **Release:** v0.1.0 | **Stack:** Rust (pmo-core, pmo-cli)

---

## Canonical File Tree

```
private-model-orchestrator/
├── Cargo.toml                              ← workspace root
├── pmo-core/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                          ← public API + unit tests
│       ├── device.rs                       ← Device, DeviceGroup, DeviceRegistry
│       ├── model.rs                        ← ModelBundle, ModelVariant, ModelRegistry
│       ├── quota.rs                        ← QuotaLimit, QuotaEngine
│       ├── policy.rs                       ← MdmPolicy, PolicyEngine
│       └── profiler.rs                     ← ProfilingSession, ProfilingStub
├── pmo-cli/
│   ├── Cargo.toml
│   └── src/main.rs
├── examples/
│   ├── device_group.rs
│   └── model_packaging.rs
├── scripts/
│   ├── convert_model.sh                    ← coremltools AOT wrapper
│   └── mdm_preflight.sh                   ← device readiness check
├── docs/
│   ├── mdm_integration.md
│   ├── aot_conversion.md
│   └── api_reference.md
├── .github/
│   ├── workflows/ci.yml
│   ├── ISSUE_TEMPLATE/bug_report.md
│   ├── ISSUE_TEMPLATE/feature_request.md
│   └── PULL_REQUEST_TEMPLATE.md
├── README.md
├── README.de.md
├── ARCHITECTURE.md
├── PRIVACY.md
├── ROADMAP.md
├── CONTRIBUTING.md
├── SECURITY.md
├── CODE_OF_CONDUCT.md
├── CHANGELOG.md
├── RELEASES.md
├── SKELETON.md                             ← this file
└── TEMPLATE_NOTES.md
```

---

## File Contents

### `Cargo.toml` (workspace root)

```toml
[workspace]
members = ["pmo-core", "pmo-cli"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT"
authors = ["Rafael Yilmaz"]
repository = "https://github.com/9t29zhmwdh-coder/private-model-orchestrator"

[workspace.dependencies]
serde       = { version = "1", features = ["derive"] }
serde_json  = "1"
thiserror   = "1"
uuid        = { version = "1", features = ["v4", "serde"] }
chrono      = { version = "0.4", features = ["serde"] }
tokio       = { version = "1", features = ["rt-multi-thread", "macros"] }
```

---

### `pmo-core/src/lib.rs`

```rust
//! Private Model Orchestrator: core library.

pub mod device;
pub mod model;
pub mod policy;
pub mod profiler;
pub mod quota;

pub use device::{Device, DeviceGroup, DeviceRegistry};
pub use model::{ModelBundle, ModelRegistry, ModelVariant};
pub use policy::{MdmPolicy, PolicyEngine};
pub use profiler::{ProfilingSession, ProfilingStub};
pub use quota::{QuotaEngine, QuotaLimit, QuotaUsage};

#[derive(Debug, Clone)]
pub struct OrchestratorConfig {
    pub device_registry_path: std::path::PathBuf,
    pub model_registry_path: std::path::PathBuf,
    pub enable_profiling: bool,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            device_registry_path: std::path::PathBuf::from("devices.db"),
            model_registry_path: std::path::PathBuf::from("models.db"),
            enable_profiling: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_profiling_disabled() {
        let cfg = OrchestratorConfig::default();
        assert!(!cfg.enable_profiling);
    }

    #[test]
    fn quota_engine_blocks_when_limit_reached() {
        use uuid::Uuid;
        let id = Uuid::new_v4();
        let mut engine = QuotaEngine::new();
        engine.set_limit(id, QuotaLimit { daily_max: Some(2), hourly_max: None });
        engine.record_inference(id);
        engine.record_inference(id);
        assert!(!engine.is_allowed(&id));
    }

    #[test]
    fn policy_engine_blocks_when_inference_disabled() {
        let mut engine = PolicyEngine::new();
        engine.load_policy(MdmPolicy { inference_allowed: false, ..Default::default() });
        assert!(!engine.is_inference_allowed());
    }

    #[test]
    fn model_registry_finds_bundle_by_name() {
        use uuid::Uuid;
        let mut reg = ModelRegistry::new();
        reg.register(ModelBundle {
            id: Uuid::new_v4(),
            name: "mistral-7b".into(),
            version: "0.1.0".into(),
            variant: ModelVariant::MlModelC,
            checksum: "abc123".into(),
            min_os_version: Some("14.0".into()),
        });
        assert!(reg.find_by_name("mistral-7b").is_some());
    }
}
```

---

### `README.md` (EN): see `/tmp/pmo/README.md`

### `README.de.md` (DE): see `/tmp/pmo/README.de.md`

### `ARCHITECTURE.md`: see `/tmp/pmo/ARCHITECTURE.md`

### `PRIVACY.md`: see `/tmp/pmo/PRIVACY.md`

### `ROADMAP.md`: see `/tmp/pmo/ROADMAP.md`

### `CONTRIBUTING.md`: see `/tmp/pmo/CONTRIBUTING.md`

### `SECURITY.md`: see `/tmp/pmo/SECURITY.md`

### `CHANGELOG.md`: see `/tmp/pmo/CHANGELOG.md`

### `RELEASES.md`: see `/tmp/pmo/RELEASES.md`

---

### `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: "-D warnings"

jobs:
  check:
    name: Check (${{ matrix.os }})
    strategy:
      matrix:
        os: [macos-latest, ubuntu-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - uses: Swatinem/rust-cache@v2
      - run: cargo check --workspace
      - run: cargo fmt --all -- --check
      - run: cargo clippy --workspace -- -D warnings
      - run: cargo test --workspace

  audit:
    name: Security audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install cargo-audit --locked
      - run: cargo audit
```

> **Note:** Push to `.github/workflows/ci.yml` requires the `workflows` OAuth scope.
> Run `gh auth refresh -s workflows` once, then push this file.

---

## Migration Checklist

### Step 1: Create GitHub repo

```bash
gh repo create 9t29zhmwdh-coder/private-model-orchestrator \
  --public \
  --description "Privacy-first orchestration of Foundation Models for Apple device fleets"
```

### Step 2: Push all files via Git Tree API (single commit)

```bash
export PATH="/usr/bin:/opt/homebrew/bin:/usr/local/bin:$PATH"
OWNER="9t29zhmwdh-coder"
REPO="private-model-orchestrator"
GH="/opt/homebrew/bin/gh"
B64="/usr/bin/base64"
TR="/usr/bin/tr"

# Collect all files into blobs
FILES=(
  "Cargo.toml" "pmo-core/Cargo.toml" "pmo-core/src/lib.rs"
  "pmo-core/src/device.rs" "pmo-core/src/model.rs"
  "pmo-core/src/quota.rs" "pmo-core/src/policy.rs" "pmo-core/src/profiler.rs"
  "pmo-cli/Cargo.toml" "pmo-cli/src/main.rs"
  "examples/device_group.rs" "examples/model_packaging.rs"
  "scripts/convert_model.sh" "scripts/mdm_preflight.sh"
  "docs/mdm_integration.md" "docs/aot_conversion.md" "docs/api_reference.md"
  "README.md" "README.de.md" "ARCHITECTURE.md" "PRIVACY.md"
  "ROADMAP.md" "CONTRIBUTING.md" "SECURITY.md" "CODE_OF_CONDUCT.md"
  "CHANGELOG.md" "RELEASES.md" "SKELETON.md" "TEMPLATE_NOTES.md"
  ".github/workflows/ci.yml"
  ".github/ISSUE_TEMPLATE/bug_report.md"
  ".github/ISSUE_TEMPLATE/feature_request.md"
  ".github/PULL_REQUEST_TEMPLATE.md"
)

TREE_ITEMS="["
FIRST=true
for f in "${FILES[@]}"; do
  BLOB_SHA=$($GH api "repos/$OWNER/$REPO/git/blobs" \
    --method POST \
    --input - <<< "{\"content\":\"$($B64 < "/tmp/pmo/$f" | $TR -d '\n')\",\"encoding\":\"base64\"}" \
    --jq '.sha')
  [ "$FIRST" = "true" ] && FIRST=false || TREE_ITEMS+=","
  TREE_ITEMS+="{\"path\":\"$f\",\"mode\":\"100644\",\"type\":\"blob\",\"sha\":\"$BLOB_SHA\"}"
done
TREE_ITEMS+="]"
```

### Step 3: Create tree, commit, branch

```bash
TREE_SHA=$(echo "{\"tree\":$TREE_ITEMS}" | \
  $GH api "repos/$OWNER/$REPO/git/trees" --method POST --input - --jq '.sha')

COMMIT_SHA=$(echo "{\"message\":\"scaffold: Add project template files\",\"tree\":\"$TREE_SHA\",\"parents\":[]}" | \
  $GH api "repos/$OWNER/$REPO/git/commits" --method POST --input - --jq '.sha')

$GH api "repos/$OWNER/$REPO/git/refs" \
  --method POST -f ref="refs/heads/main" -f sha="$COMMIT_SHA"
```

### Step 4: Set default branch

```bash
$GH api "repos/$OWNER/$REPO" --method PATCH -f default_branch="main"
```

### Step 5: Validate

```bash
$GH api "repos/$OWNER/$REPO/contents/SKELETON.md" --jq '.name'
$GH api "repos/$OWNER/$REPO/contents/pmo-core/src/lib.rs" --jq '.name'
```

### Step 6: Run cargo check locally

```bash
cd /tmp/pmo && cargo check --workspace && cargo test --workspace
```

### Step 7: Push CI workflow (requires workflows scope)

```bash
gh auth refresh -s workflows
# then re-push .github/workflows/ci.yml via API or git push
```

### Step 8: Add topics

```bash
$GH api "repos/$OWNER/$REPO/topics" \
  --method PUT \
  -f "names[]=rust" -f "names[]=coreml" -f "names[]=apple-silicon" \
  -f "names[]=mdm" -f "names[]=privacy" -f "names[]=enterprise"
```

### Step 9: Tag initial commit

```bash
INIT_SHA=$($GH api "repos/$OWNER/$REPO/git/ref/heads/main" --jq '.object.sha')
$GH api "repos/$OWNER/$REPO/git/refs" \
  --method POST -f ref="refs/tags/v0.1.0" -f sha="$INIT_SHA"
```

### Step 10: Create release

```bash
$GH api "repos/$OWNER/$REPO/releases" \
  --method POST \
  -f tag_name="v0.1.0" \
  -f name="v0.1.0: Initial import" \
  -f body="Initial import (earliest commit date: <EARLIEST_COMMIT_DATE>)

Privacy-first orchestration of Foundation Models for Apple device fleets.
Device registry, model packaging, quota management, MDM policy stubs, AOT conversion reference." \
  -F prerelease=true --jq '.html_url'
```

---

## Release Metadata (JSON)

```json
{
  "repo": "private-model-orchestrator",
  "owner": "9t29zhmwdh-coder",
  "tag": "v0.1.0",
  "name": "v0.1.0: Initial import",
  "earliest_commit_date": "<EARLIEST_COMMIT_DATE>",
  "prerelease": true,
  "body": "Initial import (earliest commit date: <EARLIEST_COMMIT_DATE>)\n\nPrivacy-first orchestration of Foundation Models for Apple device fleets.\nDevice registry, model packaging, quota management, MDM policy stubs, AOT conversion reference.",
  "topics": ["rust", "coreml", "apple-silicon", "mdm", "privacy", "enterprise"],
  "license": "MIT",
  "stack": "Rust (pmo-core, pmo-cli)",
  "platform": "macOS 14+",
  "generated": "2026-06-16"
}
```

---

## PR Description (EN)

```markdown
## Summary

- Add full project skeleton for `private-model-orchestrator`
- Rust workspace with `pmo-core` (device registry, model packaging, quota, MDM policy, profiling stubs) and `pmo-cli`
- 8 unit tests covering all core subsystems
- Bilingual documentation: README (EN / DE), ARCHITECTURE, PRIVACY, ROADMAP
- CI matrix (macOS / Ubuntu / Windows) + security audit job
- MDM integration guide, AOT conversion reference, API reference
- Examples: `device_group.rs`, `model_packaging.rs`
- Scripts: `convert_model.sh`, `mdm_preflight.sh`

## Test Plan

- [ ] `cargo check --workspace` passes
- [ ] `cargo test --workspace`: all 8 tests green
- [ ] `cargo clippy --workspace -- -D warnings` clean
- [ ] README renders correctly on GitHub
- [ ] CI workflow triggers on push
```

## PR-Beschreibung (DE)

```markdown
## Zusammenfassung

- Vollständiges Projekt-Skeleton für `private-model-orchestrator`
- Rust-Workspace mit `pmo-core` (Geräteregister, Modell-Packaging, Quota, MDM-Policy, Profiling-Stubs) und `pmo-cli`
- 8 Unit-Tests für alle Kern-Subsysteme
- Zweisprachige Dokumentation: README (EN / DE), ARCHITECTURE, PRIVACY, ROADMAP
- CI-Matrix (macOS / Ubuntu / Windows) + Security-Audit-Job
- MDM-Integrationshandbuch, AOT-Konvertierungsreferenz, API-Referenz
- Beispiele: `device_group.rs`, `model_packaging.rs`
- Skripte: `convert_model.sh`, `mdm_preflight.sh`

## Testplan

- [ ] `cargo check --workspace` erfolgreich
- [ ] `cargo test --workspace`: alle 8 Tests grün
- [ ] `cargo clippy --workspace -- -D warnings` sauber
- [ ] README wird auf GitHub korrekt dargestellt
- [ ] CI-Workflow wird bei Push ausgelöst
```

---

*private-model-orchestrator, RayStudio · Rafael Yilmaz · MIT License · 2026*
