// swift-tools-version:5.9
import Foundation
import PackageDescription

// The Rust static library lives in the workspace's shared `target/` dir
// (this package is a member of the pmo-core/pmo-cli Cargo workspace, one
// level up). Resolved relative to this manifest file so it works
// regardless of the caller's working directory.
let packageDir = URL(fileURLWithPath: #filePath).deletingLastPathComponent()
let rustTargetDir = packageDir.appendingPathComponent("../target/debug").standardizedFileURL.path

let package = Package(
    name: "pmo-macos",
    platforms: [.macOS(.v14)],
    targets: [
        .target(
            name: "CPMOCoreFFI",
            linkerSettings: [
                .unsafeFlags(["-L\(rustTargetDir)", "-lpmo_core"])
            ]
        ),
        .target(
            name: "PMOCore",
            dependencies: ["CPMOCoreFFI"]
        ),
        .executableTarget(
            name: "PMOMacOS",
            dependencies: ["PMOCore"]
        ),
    ]
)
