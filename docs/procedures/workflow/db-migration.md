# Procedure: Aggiungere Migrazione Database

## Obiettivo

Definisce come aggiungere una nuova migrazione allo schema SQLite in PokeTracker (rusqlite o sqlx in Rust).

## Quando Usare Questa Procedura

- Query: "aggiungi migrazione", "migrazione db", "cambia schema", "nuova migrazione", "migration", "schema db", "aggiungi tabella", "modifica tabella"
- Quando si deve cambiare lo schema del database (nuove tabelle, colonne, indici) in modo versionato
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Migrazioni DB**: `docs/standards/database-migrations-standard.md:1-80`
   - rusqlite_migration (M::up, to_latest): righe 14-28
   - sqlx migrate: righe 30-38
   - Best practice: righe 44-50

2. **Rust/DB**: `docs/standards/rust-tauri-standard.md:50-68`
   - Path DB, opzione rusqlite vs sqlx

3. **Storage**: `docs/project/database-storage.md:40-60`
   - Tabelle e struttura dati PokeTracker

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/database-migrations-standard.md:14-28` — Per rusqlite: usare `M::up("SQL")` e aggiungere al vettore `Migrations::new(vec![...])`
2. [ ] Non modificare migrazioni già applicate; **sempre** aggiungere una nuova voce/file (`database-migrations-standard.md:46-47`)
3. [ ] Verifica che la migrazione sia eseguita all'avvio (dopo apertura DB, prima di esporre command) (`database-migrations-standard.md:50`)
4. [ ] Se si usa sqlx: nuovo file in `migrations/` con naming `YYYYMMDDHHMMSS_descrizione.sql` e `sqlx migrate run` / `Migrate::run`
5. [ ] Allineare schema alle entità in `docs/project/database-storage.md` e a eventuali modelli in `src-tauri/src/models/`

## Riferimenti Standard

- `docs/standards/database-migrations-standard.md:1-80` — rusqlite_migration, sqlx, best practice
- `docs/standards/rust-tauri-standard.md:50-68` — Database, path, opzioni
- `docs/project/database-storage.md` — Struttura dati PokeTracker

## Note

- Path DB: `app_data_dir` (o equivalente); creare la directory se non esiste. Vedi rust-tauri-standard e database-storage.
