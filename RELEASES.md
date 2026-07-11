# Release Guide: Private Model Orchestrator

## Creating the Initial Import Release (v0.1.0)

Replace `<EARLIEST_COMMIT_DATE>` and `<INITIAL_SHA>` with actual values before running.

### Step 1: Tag the initial commit

```bash
gh api repos/9t29zhmwdh-coder/private-model-orchestrator/git/refs \
  --method POST \
  -f ref="refs/tags/v0.1.0" \
  -f sha="<INITIAL_SHA>"
```

### Step 2: Create the GitHub Release

```bash
gh release create v0.1.0 \
  --repo 9t29zhmwdh-coder/private-model-orchestrator \
  --title "v0.1.0: Initial import" \
  --notes "Initial import (earliest commit date: <EARLIEST_COMMIT_DATE>)

Privacy-first orchestration of Foundation Models for Apple device fleets.
Device registry, model packaging, quota management, MDM policy stubs." \
  --prerelease
```

### Step 3: Verify

```bash
gh release list --repo 9t29zhmwdh-coder/private-model-orchestrator
```

---

## Release Checklist for Future Versions

- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --workspace -- -D warnings` clean
- [ ] `CHANGELOG.md` updated
- [ ] Version bumped in `Cargo.toml` (workspace root)
- [ ] PR merged to `main`
- [ ] Tag created: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
- [ ] GitHub Release created with release notes
