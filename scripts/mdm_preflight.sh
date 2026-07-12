#!/usr/bin/env bash
# mdm_preflight.sh: Verify device readiness for PMO deployment
#
# Run on a managed macOS device to check MDM policy preconditions.
# Output is JSON for easy ingestion by fleet management tools.
set -euo pipefail

OS_VERSION=$(sw_vers -productVersion)
ARCH=$(uname -m)
SERIAL=$(system_profiler SPHardwareDataType | awk '/Serial Number/ {print $NF}')

COREML_OK=false
if /usr/bin/python3 -c "import coremltools" 2>/dev/null; then
  COREML_OK=true
fi

MDM_ENROLLED=false
if profiles status -type enrollment 2>/dev/null | grep -q "MDM enrollment: Yes"; then
  MDM_ENROLLED=true
fi

cat <<JSON
{
  "serial":        "${SERIAL}",
  "os_version":    "${OS_VERSION}",
  "arch":          "${ARCH}",
  "mdm_enrolled":  ${MDM_ENROLLED},
  "coremltools":   ${COREML_OK},
  "timestamp":     "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
JSON
