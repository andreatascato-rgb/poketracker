//! Comandi cartella export/backup e apertura nel file manager.
//! Vedi docs/temp/analisi-import-export-cartelle.md: cartella dedicata per export; nessuna per import.

use rusqlite::OptionalExtension;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_opener::OpenerExt;

use crate::DbState;

const APP_STATE_EXPORT_DIR: &str = "export_dir_override";

/// Restituisce il path della cartella export (override da app_state o default app_data_dir/exports).
/// Crea la cartella se non esiste.
#[tauri::command]
pub fn get_export_dir(app: AppHandle, state: State<'_, DbState>) -> Result<String, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let override_path: Option<String> = conn
        .query_row(
            "SELECT value FROM app_state WHERE key = ?1",
            [APP_STATE_EXPORT_DIR],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;

    let path = if let Some(ref p) = override_path {
        let trimmed = p.trim();
        if trimmed.is_empty() {
            default_export_dir(&app)?
        } else {
            std::path::PathBuf::from(trimmed)
        }
    } else {
        default_export_dir(&app)?
    };

    std::fs::create_dir_all(&path).map_err(|e| format!("create export dir: {}", e))?;
    Ok(path.to_string_lossy().to_string())
}

fn default_export_dir(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    let app_data = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(app_data.join("exports"))
}

/// Imposta la cartella export (None = usa predefinita app_data_dir/exports).
#[tauri::command]
pub fn set_export_dir(state: State<'_, DbState>, path: Option<String>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let value = path
        .as_ref()
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    conn.execute(
        "INSERT INTO app_state (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
        rusqlite::params![APP_STATE_EXPORT_DIR, value],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Apre la cartella export nel file manager di sistema.
#[tauri::command]
pub fn open_export_folder(app: AppHandle, state: State<'_, DbState>) -> Result<(), String> {
    let path = get_export_dir(app.clone(), state)?;
    app.opener()
        .open_path(path, None::<&str>)
        .map_err(|e| format!("open folder: {}", e))?;
    Ok(())
}
