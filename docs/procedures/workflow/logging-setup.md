# Procedure: Logging (Backend e Frontend)

## Obiettivo

Definisce come aggiungere o configurare il logging in linea con [logging-standard](../../standards/logging-standard.md).

## Quando Usare Questa Procedura

- Query: "aggiungi logging", "logging", "log level", "configura log", "debug log", "log livello"
- Quando si deve introdurre o modificare log in backend (Rust) o frontend
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Logging Standard**: `docs/standards/logging-standard.md:1-55`
   - Livelli (error, warn, info, debug, trace): righe 7-18
   - Backend (Rust), frontend, canonical log: righe 20-42

2. **Rust/Log**: `docs/standards/rust-tauri-standard.md`
   - tauri-plugin-log, livelli

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/logging-standard.md:7-18` — Usa livelli appropriati: error per fallimenti; warn per anomalie recuperabili; info per flussi principali; debug/trace per diagnostica
2. [ ] In produzione: livello ≥ info (o warn); debug/trace solo per troubleshooting (`logging-standard.md:16-18`)
3. [ ] Backend: tauri-plugin-log come da [rust-tauri-standard](../../standards/rust-tauri-standard.md); non loggare secret, token, PII in chiaro (`logging-standard.md:20-26`)
4. [ ] Frontend: `console.error`/`warn` per errori; contesto (command, payload essenziale); evitare oggetti grossi o dati personali (`logging-standard.md:28-34`)
5. [ ] Preferire un log per operazione significativa; raggruppare contesto dove possibile (`logging-standard.md:36-40`)
6. [ ] Formato: in sviluppo output leggibile; in produzione preferire strutturato (JSON) se supportato (`logging-standard.md:24-26`)

## Riferimenti Standard

- `docs/standards/logging-standard.md:1-55` — Livelli, backend, frontend
- `docs/standards/rust-tauri-standard.md` — tauri-plugin-log

## Note

- Per “gestisci errore” (dove mostrare all’utente) usare `docs/procedures/workflow/error-handling.md`; questa procedure si applica quando si aggiunge o modifica *dove* e *come* si logga.
