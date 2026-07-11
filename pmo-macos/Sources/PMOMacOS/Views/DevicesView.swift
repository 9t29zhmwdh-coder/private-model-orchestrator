import SwiftUI
import PMOCore

struct DevicesView: View {
    private let registry = FfiDeviceRegistry()
    @State private var devices: [FfiDevice] = []
    @State private var serial = ""
    @State private var hardwareModel = ""
    @State private var osVersion = ""

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Devices").font(.title2).bold()
            Text("This session's registry lives in memory only; Phase 4 wires it up to the SQLite-backed storage layer from pmo-core.")
                .font(.caption)
                .foregroundStyle(.secondary)

            HStack {
                TextField("Serial", text: $serial)
                TextField("Hardware model", text: $hardwareModel)
                TextField("OS version", text: $osVersion)
                Button("Register") {
                    _ = registry.registerDevice(serial: serial, hardwareModel: hardwareModel, osVersion: osVersion)
                    serial = ""
                    hardwareModel = ""
                    osVersion = ""
                    refresh()
                }
                .disabled(serial.isEmpty || hardwareModel.isEmpty || osVersion.isEmpty)
            }

            List(devices, id: \.id) { device in
                VStack(alignment: .leading) {
                    Text(device.serial).bold()
                    Text("\(device.hardwareModel) · macOS \(device.osVersion)")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }
            .overlay {
                if devices.isEmpty {
                    Text("No devices registered yet.").foregroundStyle(.secondary)
                }
            }
        }
        .padding()
        .onAppear(perform: refresh)
    }

    private func refresh() {
        devices = registry.allDevices()
    }
}
