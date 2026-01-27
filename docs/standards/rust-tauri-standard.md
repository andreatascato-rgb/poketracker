# Standard: Rust (backend Tauri)

## Obiettivo

Definisce edition, crates, error handling, logging e uso del database nel backend Rust di PokeTracker (Tauri 2).

## Edition

- Usare **Rust edition 2021** in `Cargo.toml`.
- **Non** usare `edition = "2024"` finché Tauri non aggiorna `tauri-build`: esiste un [bug noto](https://github.com/tauri-apps/tauri/issues/10412) per cui il parsing di `Cargo.toml` fallisce con edition 2024.

## Crates di base

- **serde** + **serde_json**: (de)serializzazione per IPC e dati verso/frontend. Già impliciti con Tauri.
- **thiserror**: per definire tipi di errore domain-specific con `#[derive(thiserror::Error)]` e messaggi chiari.
- **anyhow**: per propagare errori eterogenei con `?` in funzioni helper; **non** usare `anyhow::Result` come tipo di ritorno dei command senza convertirlo (vedi Error handling).

## Error handling nei command

- I command devono restituire **`Result<T, E>`** con **`E: Serialize`** (e compatibile con il layer IPC Tauri).
- **Non** fare `panic!` nei command: in sync crasano l’app; in async è comunque sconsigliato.
- Opzioni per `E`:
  1. **`Result<T, String>`**: ` Err(e.to_string())` o `map_err(|e| e.to_string())` — semplice, messaggio testuale al frontend.
  2. **Enum custom**: definire un enum di errori, implementare `serde::Serialize` (es. serializzare come stringa o come oggetto `{ code, message }`) e usarlo come `E`.
  3. **anyhow-tauri**: crate che permette di usare `anyhow::Result<T>` come ritorno dei command; valutare se introdurla per ridurre boilerplate di conversione.

Esempio con stringa:

```rust
#[tauri::command]
fn my_command() -> Result<MyData, String> {
    do_something().map_err(|e| e.to_string())
}
```

## Logging

- Usare **tauri-plugin-log** (`tauri-plugin-log` in Cargo, `@tauri-apps/plugin-log` in npm se servono log dal frontend).
- Setup in `lib.rs` o `main.rs`:

```rust
tauri::Builder::default()
  .plugin(tauri_plugin_log::Builder::new().build())
  // ...
```

- Livelli: `trace`, `debug`, `info`, `warn`, `error`; output verso stdout, file (opzionale) e console webview.
- Per logging strutturato in Rust si può abilitare il feature **tracing** del plugin (se disponibile) e usare la crate **tracing** in modo coerente con il resto dell’app.

## Database (SQLite)

Due approcci possibili, da scegliere in base a dove si vuole tenere la logica e il controllo:

### Opzione A: tauri-plugin-sql (ufficiale)

- **Crate:** `tauri-plugin-sql` con feature `sqlite`.
- **Pro:** integrato con Tauri, permessi e capability già pensati; il frontend può eseguire query (con i permessi configurati); migrazioni definite in Rust con `Migration` (version, description, sql, kind).
- **Contro:** meno controllo “puro Rust”; le query possono essere esposte al frontend.
- **Permessi:** in capability usare almeno `sql:default` o i permessi granulari (`sql:allow-load`, `sql:allow-execute`, `sql:allow-select`, `sql:allow-close`).
- **Migrazioni:** definire un vettore di `Migration` e passarlo al builder del plugin; eseguirle all’avvio (come da doc del plugin).

### Opzione B: rusqlite / sqlx in Rust

- **rusqlite**: sincrono, semplice; per pool si può usare **rusqlite-pool** (sync o async con tokio).
- **sqlx**: async, connection pool nativo, migrazioni con `sqlx-cli` e macro `sqlx::migrate!()`.
- **Pro:** tutta la logica e le query restano in Rust; nessuna query SQL esposta al frontend; adatto a repository/service layer come in [project-structure](../project/project-structure.md).
- **Path DB:** usare `app.path().app_data_dir()` (o simile) per il path del file SQLite; creare la dir se non esiste.
- **Migrazioni:** con sqlx: `sqlx migrate run` e cartella `migrations/`; con rusqlite: migrazioni custom (script SQL o crate dedicate) eseguite all’avvio.

Per PokeTracker, la documentazione in [database-storage](../project/database-storage.md) indica SQLite e rusqlite; è coerente con l’**opzione B** (repository in Rust). Se si preferisce velocità di integrazione e query anche dal frontend, si può usare l’**opzione A**.

## Riferimenti

- [Tauri 2 – Calling Rust](https://v2.tauri.app/develop/calling-rust/)
- [Tauri 2 – Logging plugin](https://v2.tauri.app/plugin/logging/)
- [Tauri 2 – SQL plugin](https://v2.tauri.app/plugin/sql/)
- [Handling errors in Tauri](https://tauritutorials.com/blog/handling-errors-in-tauri)
- [database-storage](../project/database-storage.md) (scelte storage PokeTracker)

## Data Decisione

2026-01-27

## Note

- Allineare le versioni delle crate Tauri (tauri, tauri-plugin-*) con la [guida ufficiale](https://v2.tauri.app/develop/updating-dependencies/).
- Per il parser sidecar e l’integrazione con i dati estratti si applicano anche [tauri2-sidecar-standard](./tauri2-sidecar-standard.md) e [parser-architecture](../project/parser-architecture.md).
