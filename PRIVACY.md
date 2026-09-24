# Privacy Policy : Private Model Orchestrator

## Summary / Zusammenfassung

PMO keeps its register in a local SQLite file on the machine where you run it. It makes no network connections.

PMO führt sein Register in einer lokalen SQLite-Datei auf dem Rechner, auf dem du es startest. Es baut keine Netzwerkverbindungen auf.

---

## What I Collect / Was ich erfasse

**Nothing.** PMO sends nothing to me or to anyone else. There is no telemetry.

**Nichts.** PMO schickt weder mir noch sonst jemandem etwas. Es gibt keine Telemetrie.

---

## What PMO stores / Was PMO speichert

| Data | Where | Detail |
|--------|--------|--------|
| Devices | SQLite (`pmo.db` or Application Support) | Serial number, hardware model, macOS version, group, as you enter them |
| Model bundles | same file | Name, version, variant and the checksum you enter; files are not read or checked |
| Quotas | same file | Limits and usage counters per device; counters change only when someone records usage or resets them |
| Policy | read from a JSON file you choose | Minimum macOS version and allowed model IDs; the app reloads it when the file changes |
| Network | none | No outbound connections, not even `localhost` |

PMO does not run models, so it never sees a prompt or an inference result.

---

## Personal data / Personenbezogene Daten

Serial numbers identify devices. Once a device is assigned to a person, its serial number is personal data under GDPR and the Swiss nDSG. PMO does not store names, but whoever operates it and links devices to people is the controller for that data and should treat the database file accordingly (access rights, retention, deletion).

Seriennummern identifizieren Geräte. Ist ein Gerät einer Person zugeordnet, ist seine Seriennummer ein Personendatum im Sinne der DSGVO und des Schweizer nDSG. PMO speichert keine Namen; wer es betreibt und Geräte Personen zuordnet, ist dafür verantwortlich und sollte die Datenbankdatei entsprechend behandeln (Zugriffsrechte, Aufbewahrung, Löschung).

---

## Contact / Kontakt

Security issues: see [SECURITY.md](SECURITY.md)

**Last updated: 2026-09-24**
