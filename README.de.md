<div align="center">

<img src="RayStudio.png" alt="RayStudio" width="80" />

# Private Model Orchestrator

**Privacy-first Orchestrierung von Foundation Models für Apple-Geräteflotten.**

KI-Modelle auf Unternehmensebene verteilen, versionieren und bereitstellen. Daten verlassen das Gerät dabei nie.

[![CI](https://github.com/9t29zhmwdh-coder/private-model-orchestrator/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/private-model-orchestrator/actions) ![Apple Silicon](https://img.shields.io/badge/Apple-Silicon-000000?logo=apple&logoColor=white) ![Platform](https://img.shields.io/badge/Platform-macOS-lightgrey?logo=apple&logoColor=black) ![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white) ![AI | Claude Code](https://img.shields.io/badge/AI-Claude_Code-black?logo=anthropic&logoColor=white) ![AI | Copilot](https://img.shields.io/badge/AI-Copilot-black?logo=github&logoColor=white) ![AI | Ollama](https://img.shields.io/badge/AI-Ollama-black?logo=ollama&logoColor=white)

</div>

---

## Übersicht

Private Model Orchestrator (PMO) ist ein Enterprise-Toolkit zur Verwaltung von Foundation-Model-Deployments auf Apple-Geräteflotten. Alle Inferenzen laufen vollständig auf dem Gerät via Core ML; keine Telemetrie, kein Cloud-Egress.

## Funktionen

| Funktion | Beschreibung |
|----------|--------------|
| **AOT-Konvertierung** | Referenzpipeline für `.mlpackage` → `.mlmodelc` Ahead-of-Time-Kompilierung |
| **Modell-Packaging** | Versionierte, prüfsummenverifizierte Modellbündel mit Variant-Tracking |
| **Gerätegruppen** | Flottensegmentierung mit gruppenspezifischen Modellzuweisungen |
| **Quota-Management** | Gerätebezogene stündliche/tägliche Inferenz-Kontingente mit Reset |
| **MDM-Integration** | Configuration-Profile-Hinweise für Jamf / Apple Business Manager |
| **Performance-Profiling** | Instrumentierte Stubs für die Integration des Core ML Profilers |

## Schnellstart

```bash
# Build
cargo build --workspace

# CLI ausführen
cargo run --bin pmo-cli

# Tests
cargo test --workspace
```

## Dokumentation

- [Architektur](ARCHITECTURE.md)
- [MDM-Integrationshandbuch](docs/mdm_integration.md)
- [AOT-Konvertierungsreferenz](docs/aot_conversion.md)
- [API-Referenz](docs/api_reference.md)
- [Roadmap](ROADMAP.md)
- [Datenschutzrichtlinie](PRIVACY.md)

## Voraussetzungen

- Rust 1.78+
- macOS 14+ (für Core ML AOT-Funktionen)
- Jamf Pro oder Apple Business Manager (optional, für MDM-Integration)

## Sicherheit

Siehe [SECURITY.md](SECURITY.md) für die Meldung von Sicherheitslücken.

## Mitwirken

Siehe [CONTRIBUTING.md](CONTRIBUTING.md).

---

<div align="center">

**RayStudio · Rafael Yilmaz · MIT-Lizenz · 2026**

</div>

**Autor:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Active · v0.1.0 · **Lizenz:** MIT
