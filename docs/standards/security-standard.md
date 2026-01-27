# Standard: Sicurezza

## Obiettivo

Definisce le regole operative di sicurezza per PokeTracker (file system, DB, sidecar, Tauri, dati sensibili). Fonte di strategia: [security](../project/security.md); questo standard è il “come” tecnico da applicare.

## Validazione Input

- **Never trust frontend:** ogni input verso backend va validato in Rust; vedi [input-validation-standard](./input-validation-standard.md).
- Path, stringhe, numeri e payload da frontend/sidecar: validare all’ingresso del command prima di qualsiasi logica.

## File System

- **Path:** normalizzare e validare; no `..` fuori scope; usare API sicure per join/risoluzione; limitare accesso a directory autorizzate (scope Tauri).
- **Operazioni:** validare dimensione prima di letture grandi; timeout dove possibile; gestione errori senza esporre path sensibili all’utente.
- **Asset:** scope asset come da [tauri2-standard](./tauri2-standard.md) (`assetScope`, `assetProtocol`).

## Database

- **SQL:** prepared statements sempre; mai concatenare input utente in query; validazione tipo e range prima della query.
- **Accesso:** isolamento dati per profilo; transazioni per operazioni critiche; rollback su errore.
- **Constrainte:** usare constraint DB per integrità; validazione applicativa non sostituisce constraint.

## Sidecar

- **Isolamento:** sidecar in processo separato; validare argomenti e path prima di invocare.
- **Output:** validare output JSON del sidecar prima di usarlo nel backend; timeout e gestione errori di processo.
- **Scope:** vedere [tauri2-sidecar-standard](./tauri2-sidecar-standard.md) per command scope e external binaries.

## Tauri (IPC e permessi)

- **Comandi:** whitelist esplicita; validazione parametri in ogni command; vedi [tauri2-permissions-standard](./tauri2-permissions-standard.md).
- **Permessi:** richiedere solo i permessi necessari; messaggi chiari se permessi mancanti; non esporre dettagli interni all’utente.

## Dati sensibili e errori

- **Log:** non loggare dati sensibili (password, token, path completi di home); per debug usare valori troncati o mascherati.
- **User-facing:** non esporre stack trace né dettagli tecnici; messaggi generici e actionable; vedi [error-handling-standard](./error-handling-standard.md).
- **Backup:** validare integrità backup; rotazione e accesso limitato; no log di contenuti sensibili dei backup.

## Riferimenti

- [security](../project/security.md) — strategia e best practice
- [input-validation-standard](./input-validation-standard.md) — never trust frontend, validazione in Rust
- [tauri2-standard](./tauri2-standard.md) — asset scope
- [tauri2-permissions-standard](./tauri2-permissions-standard.md) — capabilities, scope comandi
- [tauri2-sidecar-standard](./tauri2-sidecar-standard.md) — sidecar, scope
- [error-handling-standard](./error-handling-standard.md) — messaggi utente e log

## Data Decisione

2026-01-27

## Note

- Per “validazione input” usare [input-validation-standard](./input-validation-standard.md) e la procedure corrispondente.
- Per “verifica sicurezza”, “security audit”, “sicurezza generale” usare questo standard e la procedure `docs/procedures/workflow/security-audit.md`.
