#!/usr/bin/env bash
# Builds pmo-core with the UniFFI bridge and (re)generates the Swift
# bindings consumed by the pmo-macos SwiftUI app. Run this before
# `swift build`/`swift run` whenever pmo-core's FFI surface changes.
set -euo pipefail

cd "$(dirname "$0")/../.."

cargo build -p pmo-core --features ffi --lib

OUT_DIR="$(mktemp -d)"
cargo run -p pmo-core --features ffi --bin uniffi-bindgen -- \
  generate --library target/debug/libpmo_core.dylib --language swift --out-dir "$OUT_DIR"

mkdir -p pmo-macos/Sources/CPMOCoreFFI/include pmo-macos/Sources/PMOCore
cp "$OUT_DIR/pmo_coreFFI.h" pmo-macos/Sources/CPMOCoreFFI/include/pmo_coreFFI.h
cp "$OUT_DIR/pmo_coreFFI.modulemap" pmo-macos/Sources/CPMOCoreFFI/include/module.modulemap
cp "$OUT_DIR/pmo_core.swift" pmo-macos/Sources/PMOCore/pmo_core.swift
rm -rf "$OUT_DIR"

echo "Bindings regenerated in pmo-macos/Sources/{CPMOCoreFFI,PMOCore}."
