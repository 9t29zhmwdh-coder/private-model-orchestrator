import Foundation

func check(_ condition: Bool, _ message: String) {
    if condition {
        print("PASS: \(message)")
    } else {
        print("FAIL: \(message)")
        exit(1)
    }
}

let devices = FfiDeviceRegistry()
let modelId = "11111111-1111-1111-1111-111111111111"
let deviceId = devices.registerDevice(serial: "C02XJ1ABCD12", hardwareModel: "MacBookPro18,3", osVersion: "14.5")
check(devices.deviceCount() == 1, "device count after one registration")

let groupId = devices.createGroup(name: "Engineering Fleet")
check(devices.groupCount() == 1, "group count after one creation")
check(devices.assignModel(groupId: groupId, modelId: modelId), "assign model to group succeeds")
check(!devices.assignModel(groupId: "not-a-uuid", modelId: modelId), "assign model with malformed group id fails")

let all = devices.allDevices()
check(all.count == 1, "allDevices returns the registered device")
check(all[0].serial == "C02XJ1ABCD12", "round-tripped device serial matches")
check(all[0].id == deviceId, "round-tripped device id matches the generated id")

let models = FfiModelRegistry()
let bundleId = models.register(name: "mistral-7b", version: "0.1.0", variant: .mlModelC, checksum: "abc123", minOsVersion: "14.0")
check(models.bundleCount() == 1, "bundle count after one registration")
let found = models.findByName(name: "mistral-7b")
check(found != nil, "findByName locates the registered bundle")
check(found!.id == bundleId, "found bundle id matches the generated id")
check(models.findByName(name: "unknown") == nil, "findByName returns nil for unknown names")

let quota = FfiQuotaEngine()
check(quota.setLimit(deviceId: deviceId, dailyMax: 2, hourlyMax: nil), "setLimit succeeds for a known device")
check(quota.isAllowed(deviceId: deviceId), "inference allowed before hitting the limit")
_ = quota.recordInference(deviceId: deviceId)
_ = quota.recordInference(deviceId: deviceId)
check(!quota.isAllowed(deviceId: deviceId), "inference blocked after hitting the daily limit")
let usage = quota.usageFor(deviceId: deviceId)
check(usage != nil && usage!.dailyCount == 2, "usageFor reports the recorded inference count")

let policy = FfiPolicyEngine()
check(policy.isInferenceAllowed() == false, "inference disallowed by default policy")
policy.loadPolicy(policy: FfiMdmPolicy(inferenceAllowed: true, allowedModelIds: [], minOsVersion: nil, disableProfiling: false))
check(policy.isInferenceAllowed(), "inference allowed after loading a permissive policy")
check(policy.isModelAllowed(modelId: "anything"), "any model allowed when allowedModelIds is empty")

print("All UniFFI round-trip checks passed.")
