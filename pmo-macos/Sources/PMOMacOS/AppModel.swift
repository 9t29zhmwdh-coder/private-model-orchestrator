import Foundation
import PMOCore

/// Owns the single `FfiStorage` handle the whole app shares, so every view
/// reads and writes the same SQLite database instead of its own in-memory
/// copy. Created once in `PMOMacOSApp` and injected via the environment.
final class AppModel: ObservableObject {
    let storage: FfiStorage
    @Published var lastError: String?

    init() {
        let supportDir = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask)[0]
            .appendingPathComponent("PrivateModelOrchestrator", isDirectory: true)
        try? FileManager.default.createDirectory(at: supportDir, withIntermediateDirectories: true)
        let dbPath = supportDir.appendingPathComponent("pmo.db").path

        do {
            storage = try FfiStorage(path: dbPath)
        } catch {
            // Falling back to an in-memory-only path keeps the app usable
            // (data just won't survive a restart) instead of crashing on
            // launch if the Application Support directory is unwritable.
            storage = try! FfiStorage(path: ":memory:")
        }
    }

    /// Runs `body`, surfacing any thrown FfiStorageError as `lastError`
    /// instead of propagating it, since dashboard actions should show an
    /// inline message rather than crash the app.
    func run(_ body: () throws -> Void) {
        do {
            try body()
            lastError = nil
        } catch {
            lastError = "\(error)"
        }
    }
}
