import SwiftUI

struct ContentView: View {
    @State private var selection: AppSection? = .devices

    var body: some View {
        NavigationSplitView {
            List(AppSection.allCases, selection: $selection) { section in
                Label(section.rawValue, systemImage: section.systemImage)
                    .tag(section)
            }
            .navigationTitle("PMO")
            .listStyle(.sidebar)
        } detail: {
            switch selection {
            case .devices:
                DevicesView()
            case .models:
                ModelsView()
            case .quotas:
                QuotasView()
            case .policy:
                PolicyView()
            case nil:
                Text("Select a section")
                    .foregroundStyle(.secondary)
            }
        }
    }
}
