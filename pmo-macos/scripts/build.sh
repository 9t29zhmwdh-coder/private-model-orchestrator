#!/usr/bin/env bash
# Regenerates the UniFFI bindings and builds pmo-macos, ad-hoc signed (no
# Developer ID; matches the other RayStudio desktop apps: not notarized,
# Gatekeeper will warn on a downloaded build).
#
# PMOMacOS.entitlements (App Sandbox) is intentionally NOT applied here.
# A bare executable signed with com.apple.security.app-sandbox but no
# real .app bundle (Info.plist, bundle ID) fails to establish a
# WindowServer connection under the sandbox and crashes with SIGTRAP on
# launch: verified locally. Actually running pmo-macos inside the
# sandbox needs proper .app bundling, which is Phase 5's "Sandboxed App
# Container compliance" work, not this script's job.
set -euo pipefail

cd "$(dirname "$0")/.."

./scripts/generate-bindings.sh
swift build --configuration "${CONFIGURATION:-debug}"

BIN_PATH="$(swift build --configuration "${CONFIGURATION:-debug}" --show-bin-path)/PMOMacOS"
codesign --force --sign - "$BIN_PATH"

echo "Built and signed: $BIN_PATH"
