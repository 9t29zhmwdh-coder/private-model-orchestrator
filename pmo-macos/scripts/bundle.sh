#!/usr/bin/env bash
# Packages the pmo-macos build produced by build.sh into a real
# PMOMacOS.app bundle and a DMG, so the app can be distributed and
# double-clicked instead of only run from the build tree.
#
# `swift build`'s executable links against the workspace's shared Rust
# dylib via an ABSOLUTE path into this machine's target/debug directory
# (see Package.swift's linker settings). That path does not exist on an
# end user's machine, so the dylib is copied into Contents/Frameworks
# and the executable's load path is rewritten with install_name_tool to
# @executable_path/../Frameworks before signing.
#
# PMOMacOS.entitlements (App Sandbox) is intentionally NOT applied here,
# for the same reason build.sh doesn't apply it: see the Phase 3 gap
# note in ROADMAP.md. This produces an unsandboxed, ad-hoc signed .app,
# not a fully sandbox-compliant one.
set -euo pipefail

cd "$(dirname "$0")/.."

VERSION="${1:-0.0.0-dev}"
APP_NAME="PMOMacOS"
BUNDLE="$APP_NAME.app"
CONFIGURATION="${CONFIGURATION:-release}"

CONFIGURATION="$CONFIGURATION" ./scripts/build.sh

BIN_PATH="$(swift build --configuration "$CONFIGURATION" --show-bin-path)/$APP_NAME"
DYLIB_PATH="../target/debug/libpmo_core.dylib"

rm -rf "$BUNDLE" dist
mkdir -p "$BUNDLE/Contents/MacOS" "$BUNDLE/Contents/Resources" "$BUNDLE/Contents/Frameworks"

cp "$BIN_PATH" "$BUNDLE/Contents/MacOS/$APP_NAME"
cp "$DYLIB_PATH" "$BUNDLE/Contents/Frameworks/"
cp Info.plist "$BUNDLE/Contents/Info.plist"

# Fix CFBundleVersion/CFBundleShortVersionString to the real release version.
plutil -replace CFBundleVersion -string "$VERSION" "$BUNDLE/Contents/Info.plist"
plutil -replace CFBundleShortVersionString -string "$VERSION" "$BUNDLE/Contents/Info.plist"

OLD_DYLIB_PATH="$(otool -L "$BUNDLE/Contents/MacOS/$APP_NAME" | grep libpmo_core.dylib | awk '{print $1}')"
install_name_tool -change "$OLD_DYLIB_PATH" "@executable_path/../Frameworks/libpmo_core.dylib" "$BUNDLE/Contents/MacOS/$APP_NAME"

codesign --force --deep --sign - "$BUNDLE"
codesign --verify --verbose "$BUNDLE"

mkdir -p dist
STAGING="$(mktemp -d)"
cp -r "$BUNDLE" "$STAGING/"
ln -s /Applications "$STAGING/Applications"
hdiutil create -volname "Private Model Orchestrator" -srcfolder "$STAGING" -ov -format UDZO "dist/PMOMacOS_${VERSION}.dmg"
rm -rf "$STAGING"

echo "Bundled and packaged: $(pwd)/dist/PMOMacOS_${VERSION}.dmg"
