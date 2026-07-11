import SwiftUI
import PMOCore

struct ModelsView: View {
    private let registry = FfiModelRegistry()
    @State private var bundles: [FfiModelBundle] = []
    @State private var name = ""
    @State private var version = ""
    @State private var checksum = ""
    @State private var variant: FfiModelVariant = .mlModelC

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Models").font(.title2).bold()
            Text("This session's registry lives in memory only; Phase 4 wires it up to the SQLite-backed storage layer from pmo-core.")
                .font(.caption)
                .foregroundStyle(.secondary)

            HStack {
                TextField("Name", text: $name)
                TextField("Version", text: $version)
                Picker("Variant", selection: $variant) {
                    Text("MlModelC").tag(FfiModelVariant.mlModelC)
                    Text("MlPackage").tag(FfiModelVariant.mlPackage)
                }
                TextField("Checksum", text: $checksum)
                Button("Register") {
                    _ = registry.register(name: name, version: version, variant: variant, checksum: checksum, minOsVersion: nil)
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
                    Text("checksum \(bundle.checksum)")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }
            .overlay {
                if bundles.isEmpty {
                    Text("No model bundles registered yet.").foregroundStyle(.secondary)
                }
            }
        }
        .padding()
        .onAppear(perform: refresh)
    }

    private func refresh() {
        bundles = registry.allBundles()
    }
}
