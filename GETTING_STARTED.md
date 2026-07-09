# Getting Started with Private Model Orchestrator

This guide is for people who have never used Rust, the terminal, or Git before. It walks you through everything step by step, so you can build and run Private Model Orchestrator (PMO) from scratch.

> Note: PMO is a command-line tool. There is no graphical app window — you interact with it entirely through the terminal, and it currently supports macOS (Core ML AOT features require macOS 14+). The Windows and Linux sections below cover getting the Rust toolchain installed and the project building; running the actual on-device AI features requires macOS.

---

## Windows

### 1. Open a terminal

Right-click the Start button and choose **Terminal** (or **Windows PowerShell** on older versions of Windows).

### 2. Check if Rust is already installed

Type these two commands, one at a time, and press Enter after each:

```powershell
rustc --version
cargo --version
```

If you see version numbers (e.g. `rustc 1.78.0`), Rust is installed — skip to step 3.

If instead you see something like `rustc is not recognized as an internal or external command`, Rust is either not installed or not available on your PATH (the list of places Windows looks for programs).

**Install Rust:**

1. Go to [https://rustup.rs](https://rustup.rs)
2. Download `rustup-init.exe`
3. Run it and follow the on-screen prompts (the default options are fine)
4. Close your terminal window completely and open a new one (this refreshes the PATH)
5. Run `rustc --version` again to confirm it works

### 3. Get the code (no Git knowledge needed)

1. Go to the repository page: [https://github.com/9t29zhmwdh-coder/private-model-orchestrator](https://github.com/9t29zhmwdh-coder/private-model-orchestrator)
2. Click the green **Code** button
3. Click **Download ZIP**
4. Extract the ZIP file somewhere you can find it (e.g. your Desktop)

If you do have Git installed and prefer to use it:

```bash
git clone https://github.com/9t29zhmwdh-coder/private-model-orchestrator.git
```

### 4. Build the project

In your terminal, navigate into the extracted (or cloned) folder, then run:

```powershell
cargo build --release
```

This will download dependencies and compile the project. It can take a few minutes the first time.

### 5. Run it

```powershell
.\target\release\pmo-cli.exe
```

### What you should see

PMO's CLI is a one-shot check, not a background service — it prints its version and status, initializes its internal subsystems in memory, and then exits. There's nothing to configure and no credentials or network access needed. Expect output similar to:

```
Private Model Orchestrator v0.1.0
Device registry : ...
Model registry  : ...
Profiling       : false

All subsystems initialised. This is a v0.1.0 bootstrap check; interactive subcommands (device list, model register, quota status) land in v0.2.0, see ROADMAP.md.
```

That's it — no files are written and nothing persists in this version.

### Troubleshooting

| Issue | Fix |
|---|---|
| `rustc`/`cargo` "is not recognized" even after installing | Close the terminal completely and open a new one. Rust's installer updates your PATH, but already-open terminals don't see the change until restarted. |
| `cargo build` fails with linker errors (e.g. `link.exe not found`) | You're missing the C++ Build Tools. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and select the "Desktop development with C++" workload, then try again. |
| Nothing happens / window closes immediately when double-clicking `pmo-cli.exe` | Run it from an open terminal window (as shown above) instead of double-clicking, so you can see its printed output before the window would close. |

---

## Linux

### 1. Open a terminal

How you do this depends on your desktop environment. On most distributions you can search your application menu for "Terminal" (e.g. GNOME, KDE, XFCE all have a Terminal app). A common keyboard shortcut is `Ctrl+Alt+T`.

### 2. Check if Rust is already installed

```bash
rustc --version
cargo --version
```

If you see version numbers, Rust is installed — skip to step 3.

If you see `command not found: rustc`, Rust isn't installed or isn't on your PATH.

**Install Rust:**

1. Go to [https://rustup.rs](https://rustup.rs)
2. Run the curl one-liner shown there, typically:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Follow the prompts (default options are fine)
4. Close and reopen your terminal, or run `source "$HOME/.cargo/env"`
5. Confirm with `rustc --version`

### 3. Get the code (no Git knowledge needed)

1. Go to the repository page: [https://github.com/9t29zhmwdh-coder/private-model-orchestrator](https://github.com/9t29zhmwdh-coder/private-model-orchestrator)
2. Click the green **Code** button
3. Click **Download ZIP**
4. Extract it with your file manager, or with `unzip private-model-orchestrator-main.zip`

If you have Git installed and prefer it:

```bash
git clone https://github.com/9t29zhmwdh-coder/private-model-orchestrator.git
```

### 4. Build the project

```bash
cargo build --release
```

### 5. Run it

```bash
./target/release/pmo-cli
```

### What you should see

The CLI prints its version and status, initializes its subsystems in memory, and exits — no network, no credentials, nothing written to disk:

```
Private Model Orchestrator v0.1.0
Device registry : ...
Model registry  : ...
Profiling       : false

All subsystems initialised. This is a v0.1.0 bootstrap check; interactive subcommands (device list, model register, quota status) land in v0.2.0, see ROADMAP.md.
```

### Troubleshooting

| Issue | Fix |
|---|---|
| `rustc`/`cargo`: command not found, even after installing | Close and reopen your terminal so it picks up the updated PATH, or run `source "$HOME/.cargo/env"` in the current session. |
| `cargo build` fails with missing linker (`cc` not found) | Install your distribution's C build tools, e.g. `sudo apt install build-essential` (Debian/Ubuntu) or `sudo dnf groupinstall "Development Tools"` (Fedora). |
| Permission denied when running `./target/release/pmo-cli` | Make sure the file is executable: `chmod +x ./target/release/pmo-cli`. |

---

## macOS

### 1. Open a terminal

Press `Cmd+Space` to open Spotlight, type "Terminal", and press Enter.

### 2. Check if Rust is already installed

```bash
rustc --version
cargo --version
```

If you see version numbers, Rust is installed — skip to step 3.

If you see `command not found: rustc`, Rust isn't installed or isn't on your PATH.

**Install Rust:**

1. Go to [https://rustup.rs](https://rustup.rs)
2. Run the curl one-liner shown there, typically:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Follow the prompts (default options are fine)
4. Close and reopen Terminal, or run `source "$HOME/.cargo/env"`
5. Confirm with `rustc --version`

### 3. Get the code (no Git knowledge needed)

1. Go to the repository page: [https://github.com/9t29zhmwdh-coder/private-model-orchestrator](https://github.com/9t29zhmwdh-coder/private-model-orchestrator)
2. Click the green **Code** button
3. Click **Download ZIP**
4. Double-click the downloaded ZIP file in Finder to extract it

If you have Git installed and prefer it:

```bash
git clone https://github.com/9t29zhmwdh-coder/private-model-orchestrator.git
```

### 4. Build the project

```bash
cargo build --release
```

### 5. Run it

```bash
./target/release/pmo-cli
```

### What you should see

The CLI prints its version and status, initializes its subsystems in memory, and exits. This is also the only environment where PMO's Core ML / on-device AI features are relevant — but the basic status check works with no models, no network access, and no credentials:

```
Private Model Orchestrator v0.1.0
Device registry : ...
Model registry  : ...
Profiling       : false

All subsystems initialised. This is a v0.1.0 bootstrap check; interactive subcommands (device list, model register, quota status) land in v0.2.0, see ROADMAP.md.
```

<!-- TODO: Screenshot of the CLI output above -->

### Troubleshooting

| Issue | Fix |
|---|---|
| `rustc`/`cargo`: command not found, even after installing | Close and reopen Terminal so it picks up the updated PATH, or run `source "$HOME/.cargo/env"` in the current session. |
| `cargo build` fails with Xcode Command Line Tools errors | Run `xcode-select --install` to install Apple's command line developer tools, then try building again. |
| Core ML AOT features (`.mlpackage` conversion) don't work | These require macOS 14+ as noted in the [Requirements](README.md#requirements) section. Check your macOS version via Apple menu → About This Mac. |

---

## Where to go next

Once PMO builds and runs, see the main [README.md](README.md) for an overview of features, and [ROADMAP.md](ROADMAP.md) to see what's planned versus what's already implemented in this v0.1.0 release.
