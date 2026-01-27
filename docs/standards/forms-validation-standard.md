# Standard: Form e Validazione

## Obiettivo

Definisce come creare form e validazione in PokeTracker: schema condivisi, chi valida cosa (frontend vs backend), e dove tenere logica e messaggi. Per Tauri il “backend” sono i command Rust; il frontend è Svelte/SvelteKit.

## Principio: Validazione Backend Obbligatoria

- **Ogni dato che arriva ai command via `invoke` va validato in Rust.** Il frontend può validare per UX (feedback immediato), ma non sostituisce la validazione backend.
- **Never trust frontend:** trattare i payload di `invoke` come untrusted fino a validazione esplicita.

## Schema e Tipi

- **Schema condivisi:** dove possibile, definire una sola fonte di verità per struttura e regole (es. tipo Rust + validazione in Rust; in frontend eventuali “spec” Zod o tipi derivati da documentazione/typegen). Per app con molti form, valutare **Zod** (o simile) in TypeScript e **validazione manuale o crate** in Rust allineate alle stesse regole.
- **Tipi:** i tipi TypeScript per argomenti/ritorno di `invoke` devono riflettere i contratti Rust; usare `invoke<T>()` o typegen come da [typescript-frontend-standard](./typescript-frontend-standard.md).

## Frontend (Svelte)

- **UX:** validazione lato client per feedback immediato (required, format, length). Usare lo stesso insieme di regole del backend dove possibile per evitare “validato in frontend ma rifiutato dal backend”.
- **Messaggi:** messaggi di errore chiari e vicini al campo; stile coerente con [error-handling-standard](./error-handling-standard.md) per messaggi user-facing.
- **Librerie:** Superforms, Felte o form library compatibile con Svelte 5; se si usa Zod, schema Zod condivisi e `safeParse` per non lanciare in pagina.

## Backend (Rust / Command)

- Validare argomenti all’ingresso del command: tipo, range, formato, lunghezza. Restituire `Result<T, String>` (o tipo custom serializzabile) con messaggio chiaro in caso di errore.
- Per input da utente (path, stringhe, numeri): sanitizzare dove necessario (trim, encoding, path normalization) e validare prima di usare in DB o filesystem.

## Riferimenti

- [typescript-frontend-standard](./typescript-frontend-standard.md) — invoke, tipi
- [rust-tauri-standard](./rust-tauri-standard.md) — command, Result
- [error-handling-standard](./error-handling-standard.md) — messaggi utente
- [input-validation-standard](./input-validation-standard.md) — principi sicurezza e never-trust-frontend

## Data Decisione

2026-01-27

## Note

- Per “aggiungi form”, “validazione form”, “form schema” usare questo standard e la procedure form-creation o validazione.
