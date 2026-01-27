# Standard: Validazione Input e Sicurezza

## Obiettivo

Definisce il principio “never trust frontend” e dove validare/sanitizzare l’input in PokeTracker (Rust backend, command Tauri). Si appoggia a [security](../project/security.md) per la strategia e a [rust-tauri-standard](./rust-tauri-standard.md) per i command.

## Principio: Never Trust Frontend

- **Ogni dato che arriva ai command via `invoke` è untrusted** fino a validazione esplicita in Rust.
- La validazione frontend serve solo per UX (feedback immediato); **non** sostituisce la validazione backend.
- Validare **sempre** in Rust prima di: persistenza (DB, file), path sul filesystem, passaggio a sidecar o a librerie esterne.

## Dove Validare

- **Command Tauri:** all’ingresso del command, prima di qualsiasi logica di business. Restituire `Result<T, String>` (o tipo custom serializzabile) con messaggio chiaro in caso di errore.
- **Path:** normalizzare e validare path (no `..` fuori scope, no caratteri invalidi); usare API sicure per join e risoluzione.
- **Stringhe:** lunghezza max, encoding (UTF-8); sanitizzare se usate in HTML/JSON generato (in Tauri/desktop il rischio XSS è diverso dal web, ma evitare di esporre input raw in contesti sensibili).
- **Numeri:** range, tipo (intero vs float); evitare overflow/underflow dove rilevante.

## Strumenti (Rust)

- **Validazione:** crate `validator` con derive, o match/guard manuali; per JSON da frontend usare `serde` + controlli post-deserializzazione.
- **Sanitizzazione:** trim, normalizzazione path; per HTML/rich text valutare crate dedicate (es. ammonia) solo se si renderizza input utente.
- **Query DB:** usare parametri preparati (rusqlite/sqlx); mai concatenare stringhe utente in SQL.

## Messaggi di Errore

- **User-facing:** messaggio breve e actionable (es. “Path non valido”) come da [error-handling-standard](./error-handling-standard.md).
- **Log:** dettaglio tecnico (tipo di violazione, valore rifiutato troncato se necessario) solo in log; non esporre stack o dati sensibili all’utente.

## Riferimenti

- [security](../project/security.md) — strategia sicurezza
- [rust-tauri-standard](./rust-tauri-standard.md) — command, Result
- [forms-validation-standard](./forms-validation-standard.md) — form e schema
- [error-handling-standard](./error-handling-standard.md) — messaggi utente

## Data Decisione

2026-01-27

## Note

- Per “validazione input”, “sanitizza”, “sicurezza dati” usare questo standard e la procedure corrispondente.
