import SwiftUI
import PMOCore

struct PolicyView: View {
    private let engine = FfiPolicyEngine()
    @State private var inferenceAllowed = false
    @State private var disableProfiling = false
    @State private var appliedSummary = "No policy applied yet."

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Policy").font(.title2).bold()
            Text("This session's engine lives in memory only; a future update wires PolicyWatcher's file hot-reload (from pmo-core) into this view.")
                .font(.caption)
                .foregroundStyle(.secondary)

            Toggle("Inference allowed", isOn: $inferenceAllowed)
            Toggle("Disable profiling", isOn: $disableProfiling)

            Button("Apply") {
                engine.loadPolicy(policy: FfiMdmPolicy(
                    inferenceAllowed: inferenceAllowed,
                    allowedModelIds: [],
                    minOsVersion: nil,
                    disableProfiling: disableProfiling
                ))
                appliedSummary = "Current policy: inference \(engine.isInferenceAllowed() ? "allowed" : "blocked"), profiling \(engine.isProfilingAllowed() ? "allowed" : "disabled")."
            }

            Divider()

            Text(appliedSummary)
        }
        .padding()
    }
}
