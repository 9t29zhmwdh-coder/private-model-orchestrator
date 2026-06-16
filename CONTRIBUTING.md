# Contributing — Private Model Orchestrator

## Contributing / Mitwirken

Contributions are welcome. Please read this guide before opening a pull request.

Beiträge sind willkommen. Bitte diesen Leitfaden lesen, bevor ein Pull Request geöffnet wird.

---

## Workflow

1. Fork the repository
2. Create a branch: `git checkout -b feat/your-feature`
3. Make your changes
4. Run checks:
   ```bash
   cargo fmt --all
   cargo clippy --workspace -- -D warnings
   cargo test --workspace
   ```
5. Push and open a PR against `main`

## Commit Style

```
type: short description (≤72 chars)

Optional longer body.
```

Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`

## Privacy Rules

- No network calls in `pmo-core` — ever
- No personal device data in test fixtures
- No MDM credentials or configuration profile payloads in source

## Code Style

- `rustfmt` default settings
- `clippy --workspace -- -D warnings` must pass
- No `unwrap()` in library code — use `Result` / `Option` properly
- Document public API items with `///` doc comments

## Reporting Issues

Use the GitHub Issue templates: [Bug Report](.github/ISSUE_TEMPLATE/bug_report.md) · [Feature Request](.github/ISSUE_TEMPLATE/feature_request.md)
