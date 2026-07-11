import SwiftUI

enum AppSection: String, CaseIterable, Identifiable {
    case devices = "Devices"
    case models = "Models"
    case quotas = "Quotas"
    case policy = "Policy"

    var id: String { rawValue }

    var systemImage: String {
        switch self {
        case .devices: "laptopcomputer"
        case .models: "cube.box"
        case .quotas: "gauge"
        case .policy: "shield.lefthalf.filled"
        }
    }
}
