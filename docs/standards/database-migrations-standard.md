# Standard: Migrazioni Database (Rust / SQLite)

## Obiettivo

Definisce come gestire le migrazioni dello schema SQLite in PokeTracker quando si usa rusqlite (o sqlx) in Rust, senza esporre SQL al frontend.

## Contesto PokeTracker

- Database SQLite in Rust (opzione B da [rust-tauri-standard](./rust-tauri-standard.md)); path DB da `app.path().app_data_dir()` (o equivalente).
- Logica e query solo in Rust; nessuna query diretta dal frontend.

## Opzione A: rusqlite_migration

- **Crate:** `rusqlite_migration` — leggero, usa `user_version` di SQLite invece di tabelle di tracking.
- **Definizione:** vettore di `M` (solo up o up+down). Esempio:
  ```rust
  use rusqlite_migration::{Migrations, M};

  let migrations = Migrations::new(vec![
      M::up("CREATE TABLE profiles (id INTEGER PRIMARY KEY, name TEXT NOT NULL);"),
      M::up("CREATE TABLE pokedex_entries (profile_id INTEGER, pokemon_id INTEGER, seen INTEGER, caught INTEGER, PRIMARY KEY (profile_id, pokemon_id));"),
  ]);
  ```
- **Applicazione:** alla connessione/ap startup: `migrations.to_latest(&mut conn)`.
- **Vantaggi:** niente CLI esterna, SQL inline in Rust, compilazione veloce.
- **Pragma:** impostare `journal_mode`, `foreign_keys` ecc. prima di `to_latest` (fuori dalle migrazioni).

## Opzione B: sqlx migrate

- **Strumento:** `sqlx-cli`; migrazioni in cartella **`migrations/`** (file `.sql` con nome `YYYYMMDDHHMMSS_description.sql`).
- **Runtime:** `sqlx::migrate!("./migrations").run(&pool).await` (o equivalente).
- **Vantaggi:** async, pool nativo, migrazioni versionate in file SQL separati.
- **Uso in Tauri:** adatto se il backend usa già sqlx e async (tokio).

## Scelta per PokeTracker

- Se backend è **sincrono** e usa **rusqlite**: preferire **rusqlite_migration**.
- Se backend è **async** e usa **sqlx**: preferire **sqlx migrate**.
- La documentazione in [database-storage](../project/database-storage.md) e [rust-tauri-standard](./rust-tauri-standard.md) indica rusqlite per PokeTracker → **rusqlite_migration** come riferimento per le migrazioni.

## Best Practice

- Ogni migrazione deve essere **incrementale** e **reversibile** (quando possibile definire down).
- Non modificare migrazioni già applicate; aggiungere sempre una nuova migrazione per cambi schema.
- Eseguire migrazioni **all'avvio** dell'app, dopo aver aperto/creato il file DB e prima di esporre dati ai command.

## Riferimenti

- [rusqlite_migration](https://docs.rs/rusqlite_migration)
- [sqlx migrate](https://docs.rs/sqlx/latest/sqlx/migrate/index.html)
- [rust-tauri-standard](./rust-tauri-standard.md) (sezione Database)
- [database-storage](../project/database-storage.md)

## Data Decisione

2026-01-27

## Note

- Path DB: creare la directory di `app_data_dir` se non esiste; file tipo `app.db` o `poketracker.db` in quella directory.
