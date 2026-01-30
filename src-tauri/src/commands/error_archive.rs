//! Comandi per Archivio errori (Impostazioni → Errori).
//! Persistenza in SQLite; voci usate per log copiabile (supporto / assistente AI).
//! Vedi docs/project/notifications-and-error-archive.md.

use crate::DbState;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorArchiveEntry {
    pub id: String,
    pub at: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub detail: String,
}

/// Payload per aggiungere una voce (id e at generati in Rust).
#[derive(Debug, Deserialize)]
pub struct AddErrorArchiveEntryPayload {
    #[serde(rename = "type")]
    pub kind: String,
    pub detail: String,
}

/// Restituisce tutte le voci dell'archivio errori, ordinate per data discendente.
#[tauri::command]
pub fn get_error_archive_entries(state: State<'_, DbState>) -> Result<Vec<ErrorArchiveEntry>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, at, type, detail FROM error_archive ORDER BY at DESC")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(ErrorArchiveEntry {
                id: row.get(0)?,
                at: row.get(1)?,
                kind: row.get(2)?,
                detail: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

/// Aggiunge una voce all'archivio. Genera id (UUID) e at (ISO 8601) se non forniti.
#[tauri::command]
pub fn add_error_archive_entry(
    state: State<'_, DbState>,
    payload: AddErrorArchiveEntryPayload,
) -> Result<ErrorArchiveEntry, String> {
    let id = Uuid::new_v4().to_string();
    let at = Utc::now().to_rfc3339();
    let kind = payload.kind.trim().to_string();
    let detail = payload.detail.trim().to_string();
    if kind.is_empty() {
        return Err("type non può essere vuoto".to_string());
    }
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO error_archive (id, at, type, detail) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![&id, &at, &kind, &detail],
    )
    .map_err(|e| e.to_string())?;
    Ok(ErrorArchiveEntry {
        id,
        at,
        kind,
        detail,
    })
}

/// Rimuove una voce per id.
#[tauri::command]
pub fn remove_error_archive_entry(state: State<'_, DbState>, id: String) -> Result<(), String> {
    let id = id.trim();
    if id.is_empty() {
        return Ok(());
    }
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM error_archive WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
