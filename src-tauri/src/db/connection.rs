//! Apertura connessione SQLite e applicazione migrazioni.
//! Path DB da app_data_dir; directory creata se non esiste (rust-tauri-standard, database-storage).

use rusqlite::Connection;
use std::path::Path;

use super::migrations;

/// Apre il DB nel path indicato, crea la directory se non esiste, applica le migrazioni.
/// Imposta pragma utili prima di `to_latest` (database-migrations-standard).
pub fn open_or_create(db_path: &Path) -> Result<Connection, String> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("create app_data dir: {}", e))?;
    }
    let mut conn = Connection::open(db_path).map_err(|e| format!("open db: {}", e))?;
    conn.pragma_update(None, "journal_mode", "WAL")
        .map_err(|e| format!("pragma journal_mode: {}", e))?;
    conn.pragma_update(None, "foreign_keys", 1)
        .map_err(|e| format!("pragma foreign_keys: {}", e))?;
    let migs = migrations::migrations();
    migs.to_latest(&mut conn).map_err(|e| format!("migrate: {}", e))?;
    Ok(conn)
}
