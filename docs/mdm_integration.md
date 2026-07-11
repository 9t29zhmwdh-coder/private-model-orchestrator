# MDM Integration Guide: Private Model Orchestrator

## Overview

PMO policy is delivered via a custom **Configuration Profile payload** (`com.raystudio.pmo.policy`). The profile is distributed through Jamf Pro or Apple Business Manager (ABM) and scoped to device groups.

## Configuration Profile Payload Schema

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>PayloadType</key>
  <string>com.raystudio.pmo.policy</string>
  <key>PayloadVersion</key>
  <integer>1</integer>
  <key>PayloadIdentifier</key>
  <string>com.raystudio.pmo.policy.engineering</string>
  <key>PayloadUUID</key>
  <string><!-- generate with uuidgen --></string>

  <!-- Master inference switch -->
  <key>inference_allowed</key>
  <true/>

  <!-- Model allowlist (empty array = allow all registered bundles) -->
  <key>allowed_model_ids</key>
  <array>
    <string>mistral-7b-aot</string>
  </array>

  <!-- Minimum macOS version (informational, not enforced by OS) -->
  <key>min_os_version</key>
  <string>14.0</string>

  <!-- Disable profiling export on privacy-hardened fleets -->
  <key>disable_profiling</key>
  <false/>
</dict>
</plist>
```

## Jamf Pro Deployment

1. **Jamf Pro → Computers → Configuration Profiles → New**
2. General: Name `PMO Policy: Engineering`, Category `AI Tools`
3. Application & Custom Settings → Upload `.plist` payload
4. Scope: target device group (e.g. `Engineering Fleet`)
5. Save and distribute

## ABM Deployment

Use Apple Configurator 2 to embed the profile in an MDM enrollment package, or use ABM Device Assignments to distribute via Automated Device Enrollment (ADE).

## Policy Hot-Reload (v0.2.0+)

In v0.2.0, PMO will watch a local policy cache file updated by a LaunchDaemon on profile delivery. Use `launchctl` to trigger reload:

```bash
# Trigger policy refresh after profile delivery
launchctl kickstart -k system/com.raystudio.pmo.agent
```

## Troubleshooting

| Symptom | Check |
|---------|-------|
| `inference_allowed: false` | Verify profile scope includes device |
| Model not in allowlist | Add model ID to `allowed_model_ids` |
| Profile not appearing | Run `profiles list` on device |
| MDM enrolment not detected | Run `scripts/mdm_preflight.sh` |
