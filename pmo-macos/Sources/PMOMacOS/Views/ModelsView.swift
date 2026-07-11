import SwiftUI
import PMOCore

struct ModelsView: View {
    @EnvironmentObject private var appModel: AppModel
    @State private var bundles: [FfiModelBundle] = []
    @State private var name = ""
    @State private var version = ""
    @State private var checksum = ""
    @State private var variant: FfiModelVariant = .mlModelC

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Models").font(.title2).bold()

            HStack {
                TextField("Name", text: $name)
                TextField("Version", text: $version)
                Picker("Variant", selection: $variant) {
                    Text("MlModelC").tag(FfiModelVariant.mlModelC)
                    Text("MlPackage").tag(FfiModelVariant.mlPackage)
                }
                TextField("Checksum", text: $checksum)
                Button("Register") {
                    appModel.run {
                        _ = try appModel.storage.registerModel(name: name, version: version, variant: variant, checksum: checksum, minOsVersion: nil)
                    }
                    name = ""
                    version = ""
                    checksum = ""
                    refresh()
                }
                .disabled(name.isEmpty || version.isEmpty || checksum.isEmpty)
            }

            List(bundles, id: \.id) { bundle in
                VStack(alignment: .leading) {
                    Text("\(bundle.name) v\(bundle.version)").bold()
                    Text("\(String(describing: bundle.variant)) · checksum \(bundle.checksum)")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }
            .overlay {
                if bundles.isEmpty {
                    Text("No model bundles registered yet.").foregroundStyle(.secondary)
                }
            }

            if let error = appModel.lastError {
                Text(error).font(.caption).foregroundStyle(.red)
            }
        }
        .padding()
        .onAppear(perform: refresh)
    }

    private func refresh() {
        appModel.run { bundles = try appModel.storage.allModels() }
    }
}
