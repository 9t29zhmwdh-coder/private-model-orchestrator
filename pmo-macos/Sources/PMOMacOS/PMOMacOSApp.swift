import SwiftUI

@main
struct PMOMacOSApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
                .frame(minWidth: 760, minHeight: 480)
                .preferredColorScheme(.dark)
        }
        .windowResizability(.contentSize)
    }
}
