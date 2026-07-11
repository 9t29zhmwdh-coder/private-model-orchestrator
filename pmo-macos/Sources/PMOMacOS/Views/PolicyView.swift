import AppKit
import SwiftUI
import PMOCore

struct PolicyView: View {
    @EnvironmentObject private var appModel: AppModel
    @State private var policy: FfiMdmPolicy?
    @State private var loadedFrom: String?

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Policy").font(.title2).bold()
            Text("Loads an MDM-style Configuration Profile (JSON) from disk, the same format PolicyWatcher hot-reloads in pmo-core.")
                .font(.caption)
                .foregroundStyle(.secondary)

            Button("Load Configuration Profile...") {
                pickAndLoadPolicyFile()
            }

            if let policy {
                VStack(alignment: .leading, spacing: 6) {
                    if let loadedFrom {
                        Text("Loaded from \(loadedFrom)").font(.caption).foregroundStyle(.secondary)
                    }
                    Label("Inference \(policy.inferenceAllowed ? "allowed" : "blocked")", systemImage: policy.inferenceAllowed ? "checkmark.circle" : "xmark.circle")
                    Label("Profiling \(policy.disableProfiling ? "disabled" : "allowed")", systemImage: policy.disableProfiling ? "xmark.circle" : "checkmark.circle")
                    if let minOsVersion = policy.minOsVersion {
                        Text("Minimum OS version: \(minOsVersion)")
                    }
                    if policy.allowedModelIds.isEmpty {
                        Text("Allowed models: all registered bundles")
                    } else {
                        Text("Allowed models: \(policy.allowedModelIds.joined(separator: ", "))")
                    }
                }
            } else {
                Text("No Configuration Profile loaded yet.").foregroundStyle(.secondary)
            }

            if let error = appModel.lastError {
                Text(error).font(.caption).foregroundStyle(.red)
            }

            Spacer()
        }
        .padding()
    }

    private func pickAndLoadPolicyFile() {
        let panel = NSOpenPanel()
        panel.allowedContentTypes = [.json]
        panel.allowsMultipleSelection = false
        guard panel.runModal() == .OK, let url = panel.url else { return }

        appModel.run {
            policy = try loadPolicyFile(path: url.path)
            loadedFrom = url.lastPathComponent
        }
    }
}
