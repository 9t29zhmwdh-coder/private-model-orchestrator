#!/usr/bin/env bash
# Builds pmo-core with the UniFFI bridge, generates Swift bindings, compiles
# the round-trip test in pmo-core/tests/swift_bridge and runs it. Used by
# CI (macos-latest) and locally to catch UniFFI bridge regressions.
set -euo pipefail

cd "$(dirname "$0")/.."

cargo build -p pmo-core --features ffi --lib

OUT_DIR="target/swift-bridge-bindings"
rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

cargo run -p pmo-core --features ffi --bin uniffi-bindgen -- \
  generate --library target/debug/libpmo_core.dylib --language swift --out-dir "$OUT_DIR"

swiftc \
  -I "$OUT_DIR" \
  -L target/debug \
  -lpmo_core \
  -Xcc -fmodule-map-file="$OUT_DIR/pmo_coreFFI.modulemap" \
  "$OUT_DIR/pmo_core.swift" \
  pmo-core/tests/swift_bridge/main.swift \
  -o target/swift-bridge-test

DYLD_LIBRARY_PATH=target/debug target/swift-bridge-test
