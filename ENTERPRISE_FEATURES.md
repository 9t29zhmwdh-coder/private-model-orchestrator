# Enterprise Features

This document lists features planned for the Enterprise Edition of this
project, licensed separately under
[LICENSE.COMMERCIAL](LICENSE.COMMERCIAL). See [COMMERCIAL.md](COMMERCIAL.md)
for the licensing model.

## Status

No Enterprise features have shipped yet. This list is a forward-looking plan,
not a changelog of existing functionality: everything currently in this
repository is part of the Community Edition and remains MIT-licensed. See the
repository's own [ROADMAP.md](ROADMAP.md), "Dual-Licensing Readiness"
section, for the prerequisites that need to land first.

## Planned

- Live MDM server integration (Jamf Pro API): pulling device groups and
  policy assignments directly from a Jamf Pro tenant instead of a
  manually-picked JSON file.
- Apple Business Manager fleet provisioning automation: enrolling and
  provisioning devices at scale, beyond the current documentation-only guide.
- Compliance audit export: structured reporting suitable for handing to an
  auditor or compliance team, covering policy assignment and quota
  enforcement history across a fleet.

## Not planned

The core device, model, and quota registries, the policy engine, and the
desktop app stay in the Community Edition permanently. Dual-licensing governs
only new, enterprise-shaped capabilities such as the ones listed above, not
the tool's standalone usefulness on a single machine.
