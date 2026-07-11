import SwiftUI
import PMOCore

struct DevicesView: View {
    @EnvironmentObject private var appModel: AppModel
    @State private var devices: [FfiDevice] = []
    @State private var groups: [FfiDeviceGroup] = []
    @State private var serial = ""
    @State private var hardwareModel = ""
    @State private var osVersion = ""
    @State private var newGroupName = ""

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Devices").font(.title2).bold()

            HStack {
                TextField("Serial", text: $serial)
                TextField("Hardware model", text: $hardwareModel)
                TextField("OS version", text: $osVersion)
                Button("Register") {
                    appModel.run {
                        _ = try appModel.storage.registerDevice(serial: serial, hardwareModel: hardwareModel, osVersion: osVersion)
                    }
                    serial = ""
                    hardwareModel = ""
                    osVersion = ""
                    refresh()
                }
                .disabled(serial.isEmpty || hardwareModel.isEmpty || osVersion.isEmpty)
            }

            HStack {
                TextField("New group name", text: $newGroupName)
                Button("Create group") {
                    appModel.run { _ = try appModel.storage.createGroup(name: newGroupName) }
                    newGroupName = ""
                    refresh()
                }
                .disabled(newGroupName.isEmpty)
            }

            List(devices, id: \.id) { device in
                HStack {
                    VStack(alignment: .leading) {
                        Text(device.serial).bold()
                        Text("\(device.hardwareModel) · macOS \(device.osVersion)")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }

                    Spacer()

                    Picker("Group", selection: groupBinding(for: device)) {
                        Text("Unassigned").tag(Optional<String>.none)
                        ForEach(groups, id: \.id) { group in
                            Text(group.name).tag(Optional(group.id))
                        }
                    }
                    .frame(width: 180)
                    .labelsHidden()

                    Button(role: .destructive) {
                        appModel.run { _ = try appModel.storage.removeDevice(deviceId: device.id) }
                        refresh()
                    } label: {
                        Image(systemName: "trash")
                    }
                }
            }
            .overlay {
                if devices.isEmpty {
                    Text("No devices registered yet.").foregroundStyle(.secondary)
                }
            }

            if let error = appModel.lastError {
                Text(error).font(.caption).foregroundStyle(.red)
            }
        }
        .padding()
        .onAppear(perform: refresh)
    }

    private func groupBinding(for device: FfiDevice) -> Binding<String?> {
        Binding(
            get: { device.groupId },
            set: { newGroupId in
                appModel.run { _ = try appModel.storage.setDeviceGroup(deviceId: device.id, groupId: newGroupId) }
                refresh()
            }
        )
    }

    private func refresh() {
        appModel.run {
            devices = try appModel.storage.allDevices()
            groups = try appModel.storage.allGroups()
        }
    }
}
