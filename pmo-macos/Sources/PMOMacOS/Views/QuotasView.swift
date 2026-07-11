import SwiftUI
import PMOCore

struct QuotasView: View {
    private let engine = FfiQuotaEngine()
    @State private var deviceId = ""
    @State private var dailyMax = ""
    @State private var status: String?

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Quotas").font(.title2).bold()
            Text("This session's engine lives in memory only; Phase 4 wires it up to the SQLite-backed storage layer from pmo-core. Paste a device ID from the Devices tab.")
                .font(.caption)
                .foregroundStyle(.secondary)

            HStack {
                TextField("Device ID", text: $deviceId)
                TextField("Daily limit", text: $dailyMax)
                Button("Set limit") {
                    guard let limit = UInt64(dailyMax) else { return }
                    _ = engine.setLimit(deviceId: deviceId, dailyMax: limit, hourlyMax: nil)
                    refresh()
                }
                Button("Record inference") {
                    _ = engine.recordInference(deviceId: deviceId)
                    refresh()
                }
            }
            .disabled(deviceId.isEmpty)

            if let status {
                Text(status)
            }
        }
        .padding()
    }

    private func refresh() {
        guard let usage = engine.usageFor(deviceId: deviceId) else {
            status = "No usage recorded yet for this device."
            return
        }
        let allowed = engine.isAllowed(deviceId: deviceId)
        status = "\(usage.dailyCount) inferences today, \(usage.hourlyCount) this hour. Currently \(allowed ? "allowed" : "blocked by quota")."
    }
}
