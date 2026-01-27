# Procedure: Comando che Usa il Database

## Obiettivo

Definisce come aggiungere o modificare un comando Tauri che legge o scrive sul database SQLite (Rust). Estende [command-creation](../rust/command-creation.md) con passi specifici per DB: schema, repository, prepared statements.

## Quando Usare Questa Procedure

- Query: "comando che usa DB", "comando con query", "comando che legge/scrive DB", "aggiungi comando con database", "comando che usa SQLite", "comando con persistenza"
- Quando il comando deve accedere ai dati persistenti (profili, Pokedex, wiki, ecc.) tramite il layer DB Rust
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Creazione comando**: `docs/procedures/rust/command-creation.md` — Base: signature, Result, registrazione, invoke frontend

2. **Migrazioni e schema**: `docs/standards/database-migrations-standard.md:1-50`
   - Contesto PokeTracker: DB in Rust, nessuna query dal frontend
   - Migrazioni, best practice, esecuzione all'avvio

3. **Struttura backend**: `docs/project/project-structure.md:48-105`
   - `commands/`, `services/`, `db/` (connection, migrations, repositories)
   - Dove vivono dati: [architecture-overview](../../project/architecture-overview.md)

4. **Rust/Tauri**: `docs/standards/rust-tauri-standard.md` — Sezione Database se presente; Result, no panic nei command

## Checklist Obbligatoria

1. [ ] Segui la checklist di [command-creation](../rust/command-creation.md) per signature, `#[tauri::command]`, Result, registrazione e invoke frontend
2. [ ] **Schema:** verifica che le tabelle/colonne usate esistano; se servono nuove colonne o tabelle, usa [db-migration](./db-migration.md) **prima** di implementare il comando
3. [ ] **Accesso DB:** il comando deve ricevere la connessione (o pool) dallo state dell'app (es. `AppState` in main) oppure usare un servizio/repository che la incapsula; non creare connessioni ad hoc dentro il command
4. [ ] **Query:** usare prepared statements / parametri; mai concatenare input utente in SQL. Vedi [input-validation-standard](../../standards/input-validation-standard.md)
5. [ ] **Profilo:** se i dati sono per-profilo, il comando deve ricevere l'id profilo (o contesto) e filtrare le query di conseguenza
6. [ ] **Errori:** mappare errori DB in messaggi user-facing come da [error-handling-standard](../../standards/error-handling-standard.md); log tecnico interno, messaggio breve per l'utente

## Riferimenti Standard

- `docs/procedures/rust/command-creation.md` — Base comando
- `docs/standards/database-migrations-standard.md` — Schema, migrazioni, best practice
- `docs/project/project-structure.md:48-105` — commands, services, db
- `docs/project/architecture-overview.md` — Dove vivono i dati
- `docs/standards/input-validation-standard.md` — Prepared statements, never trust frontend
- `docs/standards/error-handling-standard.md` — Messaggi utente e log

## Note

- Per "crea comando" generico senza DB usare [command-creation](../rust/command-creation.md).
- Per "aggiungi migrazione" / "cambia schema" usare [db-migration](./db-migration.md) prima di questa procedure.
