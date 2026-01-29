# Standard: Evoluzione Contratto API (Frontend–Backend)

## Obiettivo

Definisce come evolvere i contratti tra frontend e backend (comandi Tauri, tipi condivisi) senza rompere l’esistente: semver, breaking vs non-breaking, dove documentare.

## Principio: Semver per contratti

- **Major (breaking):** rimozione o rinomina di command o di argomenti/ritorno; cambio di tipo che invalida il frontend esistente (es. `string` → `number` per un campo).
- **Minor (backward-compatible):** nuovi command; nuovi campi opzionali in argomenti o in strutture di ritorno; nuovi valori in enum con `#[non_exhaustive]` in Rust dove applicabile.
- **Patch:** correzioni che non cambiano tipo né signature (fix comportamentali).

## Aggiungere un campo (non-breaking)

- **Strutture di ritorno:** aggiungere un nuovo campo (Rust: opzionale con `Option<T>` o con default) permette al frontend vecchio di ignorarlo; frontend nuovo può usarlo.
- **Argomenti:** aggiungere parametri opzionali (es. con `Default` in Rust) non obbliga il frontend a inviarli.
- **Tipi condivisi:** aggiornare la definizione (Rust e TypeScript) in modo che i vecchi call site continuino a compilare; il frontend deve poter restare indietro almeno di una minor.

## Breaking change

- **Rimuovere campo o command:** bump major; pianificare deprecation (mantenere vecchio per una release, log di deprecation, poi rimuovere).
- **Rinominare:** come rimozione + aggiunta; esporre alias temporaneo se serve transizione.
- **Cambio tipo:** stesso trattamento della rimozione per il vecchio “contratto”; il frontend deve adattarsi.

## Dove tenere i tipi

- **Rust:** struct/enum in `src-tauri/src/models/` (o in moduli dei command); serializzazione con serde.
- **TypeScript:** tipi manuali in `src/lib/types/` (o generati con typegen se usato); allineati alle struct Rust per i payload di `invoke`.
- **Documentazione:** per API pubbliche (command esposti) documentare parametri, ritorno e eventuali errori; in caso di breaking, changelog o doc “migrazione”.

## Riferimenti

- [RFC 1105 – API evolution](https://rust-lang.github.io/rfcs/1105-api-evolution.html)
- [Cargo – Semver](https://doc.rust-lang.org/cargo/reference/semver.html)
- [typescript-frontend-standard](./typescript-frontend-standard.md) — Tipi condivisi, invoke
- [rust-tauri-standard](./rust-tauri-standard.md) — Command, Result

## Data Decisione

2026-01-27

## Note

- Per “cambia contratto”, “aggiungi campo”, “breaking change API” usare questo standard e la procedura api-contract-change.
