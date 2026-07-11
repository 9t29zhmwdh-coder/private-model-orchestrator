import SwiftUI

@main
struct PMOMacOSApp: App {
    @StateObject private var appModel = AppModel()

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(appModel)
                .frame(minWidth: 760, minHeight: 480)
                .preferredColorScheme(.dark)
        }
        .windowResizability(.contentSize)
    }
}
