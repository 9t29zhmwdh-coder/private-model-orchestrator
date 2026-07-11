import SwiftUI
import PMOCore

struct QuotasView: View {
    @EnvironmentObject private var appModel: AppModel
    @State private var devices: [FfiDevice] = []
    @State private var selectedDeviceId: String?
    @State private var usage: FfiQuotaUsage?
    @State private var isAllowed = true
    @State private var dailyMax = ""

    private let dailyMaxBarCap: Double = 20

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            HStack {
                Text("Quotas").font(.title2).bold()
                Spacer()
                Button("Reset hourly") {
                    appModel.run { try appModel.storage.resetHourlyQuota() }
                    refreshUsage()
                }
                Button("Reset daily") {
                    appModel.run { try appModel.storage.resetDailyQuota() }
                    refreshUsage()
                }
            }

            Picker("Device", selection: $selectedDeviceId) {
                Text("Select a device").tag(Optional<String>.none)
                ForEach(devices, id: \.id) { device in
                    Text(device.serial).tag(Optional(device.id))
                }
            }
            .onChange(of: selectedDeviceId) { refreshUsage() }

            if let deviceId = selectedDeviceId {
                HStack {
                    TextField("Daily limit", text: $dailyMax)
                    Button("Set limit") {
                        guard let limit = UInt64(dailyMax) else { return }
                        appModel.run { try appModel.storage.setQuotaLimit(deviceId: deviceId, dailyMax: limit, hourlyMax: nil) }
                        refreshUsage()
                    }
                    Button("Record inference") {
                        appModel.run { try appModel.storage.recordInference(deviceId: deviceId) }
                        refreshUsage()
                    }
                }

                if let usage {
                    VStack(alignment: .leading, spacing: 6) {
                        Text("Daily: \(usage.dailyCount) inferences")
                        ProgressView(value: Double(usage.dailyCount), total: dailyMaxBarCap)
                        Text("Hourly: \(usage.hourlyCount) inferences")
                        ProgressView(value: Double(usage.hourlyCount), total: dailyMaxBarCap)
                        Text(isAllowed ? "Currently allowed" : "Currently blocked by quota")
                            .foregroundStyle(isAllowed ? .green : .red)
                    }
                }
            }

            if let error = appModel.lastError {
                Text(error).font(.caption).foregroundStyle(.red)
            }

            Spacer()
        }
        .padding()
        .onAppear(perform: refreshDevices)
    }

    private func refreshDevices() {
        appModel.run { devices = try appModel.storage.allDevices() }
    }

    private func refreshUsage() {
        guard let deviceId = selectedDeviceId else {
            usage = nil
            return
        }
        appModel.run {
            usage = try appModel.storage.quotaUsage(deviceId: deviceId)
            isAllowed = try appModel.storage.quotaIsAllowed(deviceId: deviceId)
        }
    }
}
