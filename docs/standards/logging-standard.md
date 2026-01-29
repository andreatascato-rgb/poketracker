# Standard: Logging (Backend e Frontend)

## Obiettivo

Definisce livelli, formato e dove loggare in PokeTracker: backend Rust (tauri-plugin-log) e frontend (console o logger strutturato). Si appoggia a [rust-tauri-standard](./rust-tauri-standard.md) per il backend.

## Livelli

- **error:** fallimenti che impattano l’utente o l’operazione; richiedono attenzione.
- **warn:** situazioni anomale ma recuperabili (es. retry, fallback).
- **info:** flussi principali (avvio app, operazioni completate, eventi significativi).
- **debug:** dettaglio per diagnostica; disattivabile in release.
- **trace:** massimo dettaglio; solo per sviluppo o trace specifici.

In **produzione** preferire livello ≥ `info` (o `warn`) per ridurre volume e costi; `debug`/`trace` solo per sessioni di troubleshooting.

## Backend (Rust)

- Usare **tauri-plugin-log** come da [rust-tauri-standard](./rust-tauri-standard.md). Livelli: `error!`, `warn!`, `info!`, `debug!`, `trace!`.
- **Formato:** in sviluppo output leggibile; in produzione preferire log strutturati (JSON) se il plugin o la pipeline lo supportano.
- **Contenuto:** includere contesto (es. nome command, tipo errore); **non** loggare segreti, token, password o dati personali in chiaro. Per path sensibili troncare o offuscare se necessario.

## Frontend

- **Console:** `console.error` per errori da `invoke` e per errori di UI; `console.warn` per situazioni anomale; `console.log`/`debug` solo in sviluppo.
- **Contenuto:** messaggio breve + contesto (nome command, payload essenziale); evitare di loggare oggetti grandi o dati personali.
- **Produzione:** valutare riduzione o disattivazione di log non error/warn; in Tauri la console del webview va gestita con la stessa attenzione del backend.

## Canonical Log Lines

- Preferire **un log per operazione significativa** invece di molti log frammentati; raggruppare contesto (es. request id, command name) in un’unica riga o in oggetto strutturato quando possibile.

## Riferimenti

- [rust-tauri-standard](./rust-tauri-standard.md) — tauri-plugin-log, livelli
- [error-handling-standard](./error-handling-standard.md) — separazione messaggio utente vs log
- [structlog – Logging best practices](https://www.structlog.org/en/stable/logging-best-practices.html)

## Data Decisione

2026-01-27

## Note

- Per “aggiungi logging”, “configura log level” usare questo standard e la procedura corrispondente.
