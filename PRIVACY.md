# Privacy Policy — Private Model Orchestrator

## Summary / Zusammenfassung

PMO processes all data locally on the device. No data ever leaves the device fleet perimeter.

PMO verarbeitet alle Daten lokal auf dem Gerät. Keine Daten verlassen den Perimeter der Geräteflotte.

---

## What We Collect / Was wir erfassen

**Nothing.** PMO does not collect, transmit, or store any personal or organisational data outside the device.

**Nichts.** PMO erfasst, überträgt oder speichert keine personenbezogenen oder organisationsbezogenen Daten ausserhalb des Geräts.

---

## Data Processing / Datenverarbeitung

| Aspect | Detail |
|--------|--------|
| Inference | Runs entirely on-device via Core ML / ANE |
| Model bundles | Stored locally; checksum-verified at load time |
| Device registry | In-process / local DB only |
| Quota counters | In-process; reset on schedule |
| Policy payload | Received via MDM Configuration Profile; parsed in RAM |
| Profiling output | Local only; never transmitted |
| Network | No outbound connections — not even `localhost` |

---

## Enterprise Considerations / Enterprise-Hinweise

- **MDM Policy:** The `MdmPolicy` struct gives administrators full control over which devices may run inference and which model IDs are allowed.
- **Audit Logging:** No built-in audit log is transmitted. Fleet logs remain on MDM-managed devices under the operator's control.
- **Data Residency:** All model weights and inference outputs are bound to the device. No shared storage, no cloud sync.
- **GDPR / nDSG:** Because PMO processes no personal data, it does not act as a data processor under GDPR or the Swiss nDSG. Verify with your DPO for fleet-specific configurations.

---

## Contact / Kontakt

Security issues: see [SECURITY.md](SECURITY.md)

**Last updated: 2026-06-16**
